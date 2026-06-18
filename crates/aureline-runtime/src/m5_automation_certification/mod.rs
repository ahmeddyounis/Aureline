//! Claim-bearing safe-automation certification matrix for the M5 command
//! surfaces.
//!
//! The automation contract baseline ([`crate::m5_automation_contract_baseline`])
//! froze the object families, the controlled safety-label vocabulary, and the
//! reuse rules; the builder, parameter-review, dry-run/explain, run-history,
//! macro-recorder, and label-parity lanes then each proved one slice of the
//! safe-automation story. This module closes the loop at the *claim* level: it
//! turns those frozen proofs into a claim-bearing **surface certification
//! matrix** so each claimed M5 automation surface can only present itself as
//! *safe or shareable* when its own current evidence proves builder parity,
//! parameter review, dry-run/explain coverage, run-history/evidence integrity,
//! macro-scope safety, and label reuse.
//!
//! Each claimed [`AutomationSurface`] carries one
//! [`AutomationSurfaceCertification`] graded on the six certification
//! [`AutomationCertificationDimension`]s the docs now require:
//!
//! - [`AutomationCertificationDimension::BuilderParity`] — the surface authors
//!   automation through the canonical declarative recipe builder and cites the
//!   builder proof, instead of an ad-hoc feature dialog, hidden command metadata,
//!   or unreviewed free text.
//! - [`AutomationCertificationDimension::ParameterReview`] — inputs route through
//!   a typed parameter-review sheet with validation and safe secret-reference
//!   handling.
//! - [`AutomationCertificationDimension::DryRunExplainCoverage`] — a dry-run /
//!   explain preview discloses the predicted writes, process, network, and remote
//!   effects before any apply.
//! - [`AutomationCertificationDimension::RunHistoryIntegrity`] — durable run
//!   history / evidence is recorded with retention/redaction and a
//!   rerun-under-current-policy resolution.
//! - [`AutomationCertificationDimension::MacroScopeSafety`] — recorded macros
//!   declare their target scope and fail closed on a context, scope, or
//!   supported-command mismatch.
//! - [`AutomationCertificationDimension::LabelReuse`] — the surface reuses the
//!   controlled safety-label vocabulary rather than minting surface-local
//!   synonyms.
//!
//! A surface that fails any dimension *blocks stable*; a surface whose proof has
//! aged past its freshness window *narrows below stable* (a warning, not a
//! blocker) so a claim cannot coast on aged proof. A surface that *presents
//! itself as shareable* without full current proof is itself a finding. The
//! derived [`AutomationCertificationIndex`] names which surfaces are shareable,
//! narrowed, or blocked so release, support, AI, and docs/help surfaces can ingest
//! one canonical automation-evidence index instead of re-deriving surface maturity
//! by hand.
//!
//! The packet deliberately reuses the safety-label, promotion-state, and
//! finding-severity vocabulary frozen in
//! [`crate::m5_automation_contract_baseline`]; it adds the surface/claim/index
//! layer and nothing that re-derives automation truth.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/automation-certification.md`](../../../docs/m5/automation-certification.md);
//! the machine-readable boundary lives at
//! [`/schemas/automation/m5-automation-certification.schema.json`](../../../schemas/automation/m5-automation-certification.schema.json).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m5_automation_contract_baseline::{
    AutomationBaselinePromotionState, AutomationSafetyLabelId, BaselineFindingSeverity,
    AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF,
};

/// Stable record-kind tag for [`AutomationCertificationPacket`].
pub const AUTOMATION_CERTIFICATION_RECORD_KIND: &str = "m5_automation_certification_packet";

/// Stable record-kind tag for [`AutomationCertificationSupportExport`].
pub const AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_automation_certification_support_export";

/// Stable record-kind tag for [`AutomationCertificationEvidenceJoinView`].
pub const AUTOMATION_CERTIFICATION_EVIDENCE_JOIN_RECORD_KIND: &str =
    "m5_automation_certification_evidence_join";

/// Stable record-kind tag for [`AutomationCertificationCliHeadlessView`].
pub const AUTOMATION_CERTIFICATION_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_automation_certification_cli_headless";

/// Integer schema version for the certification packet family.
pub const AUTOMATION_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the certification boundary schema.
pub const AUTOMATION_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/automation/m5-automation-certification.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const AUTOMATION_CERTIFICATION_DOC_REF: &str = "docs/m5/automation-certification.md";

/// Repo-relative path of the reused automation-contract-baseline schema.
pub const AUTOMATION_CERTIFICATION_CONTRACT_BASELINE_SCHEMA_REF: &str =
    AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF;

/// Repo-relative path of the checked-in packet artifact.
pub const AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/automation-certification/packet.json";

/// Repo-relative root the worked-example certification fixtures live under.
pub const AUTOMATION_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/automation/m5/automation-certification";

/// Logical certification-index binding ref minted by the seed.
pub const AUTOMATION_CERTIFICATION_INDEX_REF: &str =
    "release-evidence:automation:m5:automation-certification";

/// Stable packet id minted by the seed.
pub const AUTOMATION_CERTIFICATION_ID: &str = "automation:m5:automation-certification:v1";

/// Stable support-export id minted by the seed inspector.
pub const AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:automation:m5:automation-certification";

/// Stable AI evidence join id minted by the seed inspector.
pub const AUTOMATION_CERTIFICATION_AI_EVIDENCE_ID: &str =
    "ai-evidence:automation:m5:automation-certification";

/// Stable incident packet join id minted by the seed inspector.
pub const AUTOMATION_CERTIFICATION_INCIDENT_PACKET_ID: &str =
    "incident:automation:m5:automation-certification";

/// Stable CLI/headless view id minted by the seed inspector.
pub const AUTOMATION_CERTIFICATION_CLI_HEADLESS_ID: &str =
    "cli-headless:automation:m5:automation-certification";

/// Canonical upstream automation proofs every claimed surface draws evidence from.
///
/// Each ref points at a checked-in upstream packet: the declarative recipe
/// builder, the typed parameter-review sheets, the dry-run/explain previews, the
/// run-history / evidence panel, the macro recorder, the cross-surface
/// label-parity proof, and the automation contract baseline that froze the
/// vocabulary they all reuse.
pub const AUTOMATION_CERTIFICATION_EVIDENCE_REFS: [&str; 7] = [
    "artifacts/m5/automation/recipe-builder-first-consumers/packet.json",
    "artifacts/m5/automation/parameter-review/packet.json",
    "artifacts/m5/automation/dry-run-explain/packet.json",
    "artifacts/m5/automation/run-history/packet.json",
    "artifacts/m5/automation/macro-recorder/packet.json",
    "artifacts/m5/automation/label-parity/packet.json",
    "artifacts/m5/automation/automation-contract-baseline/packet.json",
];

// ---------------------------------------------------------------------------
// Surfaces
// ---------------------------------------------------------------------------

/// One claimed M5 automation surface the certification matrix grades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationSurface {
    /// Notebook run / cell automation.
    NotebookAutomation,
    /// Saved request / API automation.
    RequestApiAutomation,
    /// Dependency / package automation.
    PackageAutomation,
    /// Task, test, and debug automation.
    TestDebugAutomation,
    /// Incident response automation.
    IncidentAutomation,
    /// AI-linked / assistant-proposed automation.
    AiLinkedAutomation,
}

impl AutomationSurface {
    /// Every claimed surface in stable declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotebookAutomation,
        Self::RequestApiAutomation,
        Self::PackageAutomation,
        Self::TestDebugAutomation,
        Self::IncidentAutomation,
        Self::AiLinkedAutomation,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookAutomation => "notebook_automation",
            Self::RequestApiAutomation => "request_api_automation",
            Self::PackageAutomation => "package_automation",
            Self::TestDebugAutomation => "test_debug_automation",
            Self::IncidentAutomation => "incident_automation",
            Self::AiLinkedAutomation => "ai_linked_automation",
        }
    }

    /// Reviewable title.
    pub const fn title(self) -> &'static str {
        match self {
            Self::NotebookAutomation => "Notebook automation",
            Self::RequestApiAutomation => "Request and API automation",
            Self::PackageAutomation => "Package automation",
            Self::TestDebugAutomation => "Test and debug automation",
            Self::IncidentAutomation => "Incident automation",
            Self::AiLinkedAutomation => "AI-linked automation",
        }
    }
}

/// How a surface authors the automation it presents.
///
/// Only [`AutomationAuthoringPath::DeclarativeRecipeBuilder`] is conformant; every
/// other variant names a way a surface can sneak automation semantics outside the
/// reviewed builder and blocks the surface's certification claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationAuthoringPath {
    /// The surface authors automation through the canonical declarative builder.
    DeclarativeRecipeBuilder,
    /// The surface authors automation in an ad-hoc, feature-specific dialog.
    AdHocFeatureDialog,
    /// The surface drives automation from hidden command metadata.
    HiddenCommandMetadata,
    /// The surface authors automation from unreviewed free text.
    UnreviewedFreeText,
}

impl AutomationAuthoringPath {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclarativeRecipeBuilder => "declarative_recipe_builder",
            Self::AdHocFeatureDialog => "ad_hoc_feature_dialog",
            Self::HiddenCommandMetadata => "hidden_command_metadata",
            Self::UnreviewedFreeText => "unreviewed_free_text",
        }
    }

    /// True when the surface authors through the canonical declarative builder.
    pub const fn is_conformant(self) -> bool {
        matches!(self, Self::DeclarativeRecipeBuilder)
    }
}

/// One certification dimension graded for every claimed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationCertificationDimension {
    /// The surface authors through the canonical declarative builder and cites it.
    BuilderParity,
    /// Inputs route through a typed parameter-review sheet with secret handling.
    ParameterReview,
    /// A dry-run/explain preview discloses predicted side effects before apply.
    DryRunExplainCoverage,
    /// Durable run history / evidence is recorded with retention and rerun policy.
    RunHistoryIntegrity,
    /// Recorded macros declare their scope and fail closed on a mismatch.
    MacroScopeSafety,
    /// The surface reuses the controlled safety-label vocabulary.
    LabelReuse,
}

impl AutomationCertificationDimension {
    /// Every graded dimension in stable declaration order.
    pub const ALL: [Self; 6] = [
        Self::BuilderParity,
        Self::ParameterReview,
        Self::DryRunExplainCoverage,
        Self::RunHistoryIntegrity,
        Self::MacroScopeSafety,
        Self::LabelReuse,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuilderParity => "builder_parity",
            Self::ParameterReview => "parameter_review",
            Self::DryRunExplainCoverage => "dry_run_explain_coverage",
            Self::RunHistoryIntegrity => "run_history_integrity",
            Self::MacroScopeSafety => "macro_scope_safety",
            Self::LabelReuse => "label_reuse",
        }
    }
}

/// Derived freshness state for a surface's recorded proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFreshnessState {
    /// Proof age is within the freshness window.
    Current,
    /// Proof age has exceeded the freshness window (narrows below stable).
    Stale,
}

impl CertificationFreshnessState {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }
}

/// Derived claim state for one surface in the certification matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClaimState {
    /// The surface is current and certified across every dimension; shareable.
    Shareable,
    /// The surface is certified but its proof has aged out (narrows below stable).
    NarrowedBelowStable,
    /// The surface fails a certification dimension and blocks stable.
    Blocked,
}

impl SurfaceClaimState {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shareable => "shareable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::Blocked => "blocked",
        }
    }
}

/// Evidence-join surface that presents the certification matrix across a boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationEvidenceSurface {
    /// Support bundle / support export.
    SupportBundle,
    /// Incident timeline packet.
    IncidentPacket,
    /// AI evidence packet.
    AiEvidence,
}

impl CertificationEvidenceSurface {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundle => "support_bundle",
            Self::IncidentPacket => "incident_packet",
            Self::AiEvidence => "ai_evidence",
        }
    }
}

/// Closed validation finding vocabulary for the certification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// A required automation surface is absent.
    MissingSurface,
    /// Two certifications declare the same surface.
    DuplicateSurface,
    /// A surface authors automation outside the declarative recipe builder.
    AdHocAuthoring,
    /// A builder-conformant surface cites no upstream builder proof.
    MissingBuilderEvidence,
    /// A surface routes inputs without a typed, secret-safe parameter review.
    ParameterReviewMissing,
    /// A surface applies automation with no dry-run/explain side-effect preview.
    SideEffectPreviewMissing,
    /// A surface keeps no durable, redaction-safe, rerun-under-policy run history.
    RunHistoryIntegrityMissing,
    /// A surface's recorded macros are not scope-safe and fail-closed.
    MacroScopeUnsafe,
    /// A surface invents a label vocabulary instead of reusing the controlled set.
    LabelReuseBroken,
    /// A surface cites no upstream automation proof.
    MissingEvidenceRef,
    /// A surface presents as shareable without full current proof (blocked).
    ShareableClaimUnproven,
    /// A surface presents as shareable on aged proof (narrows below stable).
    ShareableClaimNarrowed,
    /// A surface's recorded proof has aged past its freshness window.
    SurfaceEvidenceStale,
    /// Stored per-dimension outcomes disagree with the derivation.
    DimensionOutcomeDrift,
    /// Stored surface certified flag disagrees with the derivation.
    SurfaceCertificationDrift,
    /// Stored surface claim state disagrees with the derivation.
    SurfaceClaimStateDrift,
    /// Stored surface freshness state disagrees with the derivation.
    SurfaceFreshnessDrift,
    /// The certification-index binding ref is missing.
    CertificationIndexMissing,
    /// Stored certification index disagrees with the derivation.
    CertificationIndexDrift,
    /// Stored surface digest disagrees with the derivation.
    SurfaceDigestDrift,
    /// Stored promotion state disagrees with the derivation.
    PromotionStateMismatch,
}

impl CertificationFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSurface => "missing_surface",
            Self::DuplicateSurface => "duplicate_surface",
            Self::AdHocAuthoring => "ad_hoc_authoring",
            Self::MissingBuilderEvidence => "missing_builder_evidence",
            Self::ParameterReviewMissing => "parameter_review_missing",
            Self::SideEffectPreviewMissing => "side_effect_preview_missing",
            Self::RunHistoryIntegrityMissing => "run_history_integrity_missing",
            Self::MacroScopeUnsafe => "macro_scope_unsafe",
            Self::LabelReuseBroken => "label_reuse_broken",
            Self::MissingEvidenceRef => "missing_evidence_ref",
            Self::ShareableClaimUnproven => "shareable_claim_unproven",
            Self::ShareableClaimNarrowed => "shareable_claim_narrowed",
            Self::SurfaceEvidenceStale => "surface_evidence_stale",
            Self::DimensionOutcomeDrift => "dimension_outcome_drift",
            Self::SurfaceCertificationDrift => "surface_certification_drift",
            Self::SurfaceClaimStateDrift => "surface_claim_state_drift",
            Self::SurfaceFreshnessDrift => "surface_freshness_drift",
            Self::CertificationIndexMissing => "certification_index_missing",
            Self::CertificationIndexDrift => "certification_index_drift",
            Self::SurfaceDigestDrift => "surface_digest_drift",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the certification validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationValidationFinding {
    /// Closed finding kind.
    pub finding_kind: CertificationFindingKind,
    /// Finding severity.
    pub severity: BaselineFindingSeverity,
    /// Optional subject the finding is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Short support-safe summary.
    pub summary: String,
}

impl CertificationValidationFinding {
    fn blocker(
        finding_kind: CertificationFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity: BaselineFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }

    fn warning(
        finding_kind: CertificationFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity: BaselineFindingSeverity::Warning,
            subject,
            summary: summary.into(),
        }
    }
}

/// One graded certification dimension outcome (derived at materialization).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationDimensionOutcome {
    /// Certification dimension.
    pub dimension: AutomationCertificationDimension,
    /// True when the surface satisfies the dimension.
    pub passed: bool,
    /// Support-safe note describing the result.
    pub detail: String,
}

/// One surface certification: a claimed M5 automation surface graded across every
/// certification dimension, with its freshness window and cited evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationSurfaceCertification {
    /// Claimed automation surface.
    pub surface: AutomationSurface,
    /// Support-safe summary of the automation claim under certification.
    pub claim_summary: String,
    /// Whether the surface presents itself as safe / shareable to the user.
    pub presents_as_shareable: bool,
    /// How the surface authors the automation it presents.
    pub authoring_path: AutomationAuthoringPath,
    /// True when inputs route through a typed parameter-review sheet.
    pub parameters_reviewed: bool,
    /// True when secret references are resolved safely, not inlined.
    pub secret_references_safe: bool,
    /// True when a dry-run/explain side-effect preview is shown before apply.
    pub side_effect_preview_shown: bool,
    /// True when the preview discloses predicted writes/process/network/remote.
    pub predicted_effects_disclosed: bool,
    /// True when durable run history / evidence is recorded.
    pub run_history_durable: bool,
    /// True when run history honors retention/redaction (no raw payload bodies).
    pub run_history_redaction_safe: bool,
    /// True when run history resolves a rerun under the current policy.
    pub rerun_under_current_policy: bool,
    /// True when recorded macros declare their target scope.
    pub macro_scope_declared: bool,
    /// True when macro replay fails closed on context/scope/command mismatch.
    pub macro_fails_closed_on_mismatch: bool,
    /// True when the surface reuses the controlled safety-label vocabulary.
    pub reuses_controlled_labels: bool,
    /// The controlled safety labels the surface reuses, in canonical order.
    #[serde(default)]
    pub safety_labels: Vec<AutomationSafetyLabelId>,
    /// Upstream automation proofs the surface draws its evidence from.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Timestamp of the last recorded certification run.
    pub last_certified_at: String,
    /// Age in days of the recorded proof at the packet's capture time.
    pub proof_age_days: u32,
    /// Freshness window in days before the proof narrows below stable.
    pub freshness_window_days: u32,
    /// Derived freshness state.
    pub freshness_state: CertificationFreshnessState,
    /// Per-dimension outcomes (derived at materialization).
    #[serde(default)]
    pub dimension_outcomes: Vec<AutomationDimensionOutcome>,
    /// True when every dimension passes (derived at materialization).
    pub certified: bool,
    /// Derived claim state for the matrix (derived at materialization).
    pub claim_state: SurfaceClaimState,
}

impl AutomationSurfaceCertification {
    fn evidence_refs_empty(&self) -> bool {
        self.evidence_refs
            .iter()
            .all(|reference| reference.trim().is_empty())
    }

    fn reuses_builder(&self) -> bool {
        self.authoring_path.is_conformant() && !self.evidence_refs_empty()
    }

    fn labels_conformant(&self) -> bool {
        // The labels are the typed controlled vocabulary, so reuse is proven by the
        // surface flagging reuse and naming at least one frozen label.
        self.reuses_controlled_labels && !self.safety_labels.is_empty()
    }

    /// Evaluates every certification dimension from the surface's explicit fields.
    fn evaluate_dimensions(&self) -> Vec<AutomationDimensionOutcome> {
        let builder_ok = self.reuses_builder();
        let parameter_ok = self.parameters_reviewed && self.secret_references_safe;
        let preview_ok = self.side_effect_preview_shown && self.predicted_effects_disclosed;
        let history_ok = self.run_history_durable
            && self.run_history_redaction_safe
            && self.rerun_under_current_policy;
        let macro_ok = self.macro_scope_declared && self.macro_fails_closed_on_mismatch;
        let label_ok = self.labels_conformant();

        vec![
            AutomationDimensionOutcome {
                dimension: AutomationCertificationDimension::BuilderParity,
                passed: builder_ok,
                detail: format!(
                    "authoring path {} with {} evidence ref(s)",
                    self.authoring_path.as_str(),
                    self.evidence_refs.len(),
                ),
            },
            AutomationDimensionOutcome {
                dimension: AutomationCertificationDimension::ParameterReview,
                passed: parameter_ok,
                detail: format!(
                    "parameters_reviewed={} secret_references_safe={}",
                    self.parameters_reviewed, self.secret_references_safe,
                ),
            },
            AutomationDimensionOutcome {
                dimension: AutomationCertificationDimension::DryRunExplainCoverage,
                passed: preview_ok,
                detail: format!(
                    "side_effect_preview_shown={} predicted_effects_disclosed={}",
                    self.side_effect_preview_shown, self.predicted_effects_disclosed,
                ),
            },
            AutomationDimensionOutcome {
                dimension: AutomationCertificationDimension::RunHistoryIntegrity,
                passed: history_ok,
                detail: format!(
                    "durable={} redaction_safe={} rerun_under_current_policy={}",
                    self.run_history_durable,
                    self.run_history_redaction_safe,
                    self.rerun_under_current_policy,
                ),
            },
            AutomationDimensionOutcome {
                dimension: AutomationCertificationDimension::MacroScopeSafety,
                passed: macro_ok,
                detail: format!(
                    "scope_declared={} fails_closed_on_mismatch={}",
                    self.macro_scope_declared, self.macro_fails_closed_on_mismatch,
                ),
            },
            AutomationDimensionOutcome {
                dimension: AutomationCertificationDimension::LabelReuse,
                passed: label_ok,
                detail: format!(
                    "reuses_controlled_labels={} labels={}",
                    self.reuses_controlled_labels,
                    self.safety_labels.len(),
                ),
            },
        ]
    }

    /// Returns true when the surface is current and certified.
    pub fn is_shareable(&self) -> bool {
        self.freshness_state == CertificationFreshnessState::Current && self.certified
    }

    /// The safety-label tokens the surface reuses, in declaration order.
    pub fn safety_label_tokens(&self) -> Vec<&'static str> {
        AutomationSafetyLabelId::ALL
            .into_iter()
            .filter(|label| self.safety_labels.contains(label))
            .map(AutomationSafetyLabelId::as_str)
            .collect()
    }
}

/// Certification index rolled up from the surface certifications (derived).
///
/// This is the one canonical automation-evidence index release, support, AI, and
/// docs/help surfaces ingest: it names which surfaces are shareable, which have
/// narrowed below stable on aged proof, and which are blocked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationCertificationIndex {
    /// Logical certification-index binding ref.
    pub certification_ref: String,
    /// True when every surface's proof is current.
    pub all_surfaces_current: bool,
    /// True when every surface certifies across all dimensions.
    pub all_surfaces_certified: bool,
    /// Surface tokens that are current, certified, and shareable.
    #[serde(default)]
    pub shareable_surfaces: Vec<String>,
    /// Surface tokens that are certified but narrowed on aged proof.
    #[serde(default)]
    pub narrowed_surfaces: Vec<String>,
    /// Surface tokens that fail a dimension and block stable.
    #[serde(default)]
    pub blocked_surfaces: Vec<String>,
    /// Support-safe roll-up summary.
    pub certification_summary: String,
}

/// Constructor input for [`AutomationCertificationPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Logical certification-index binding ref.
    pub certification_ref: String,
    /// Surface certifications (outcomes/roll-ups derived at materialization).
    #[serde(default)]
    pub surfaces: Vec<AutomationSurfaceCertification>,
}

/// Canonical automation certification packet: the claimed surface matrix, the
/// per-surface dimension grades, the freshness/stale-narrowing roll-up, and the
/// certification index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Certification boundary schema ref.
    pub certification_schema_ref: String,
    /// Reused automation-contract-baseline schema ref.
    pub contract_baseline_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Surface certifications.
    #[serde(default)]
    pub surfaces: Vec<AutomationSurfaceCertification>,
    /// Order-invariant digest of every surface token.
    pub surface_digest: String,
    /// Certification index rolled up from the surfaces.
    pub certification_index: AutomationCertificationIndex,
    /// Derived promotion state.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<CertificationValidationFinding>,
}

impl AutomationCertificationPacket {
    /// Materializes a packet, deriving per-surface dimension outcomes, freshness,
    /// claim states, the surface digest, and the certification index, then records
    /// findings and the promotion state.
    pub fn materialize(input: AutomationCertificationPacketInput) -> Self {
        let surfaces: Vec<AutomationSurfaceCertification> =
            input.surfaces.into_iter().map(derive_surface).collect();
        let surface_digest = surface_digest(&surfaces);
        let certification_index = derive_certification_index(&input.certification_ref, &surfaces);

        let mut packet = Self {
            record_kind: AUTOMATION_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: AUTOMATION_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            certification_schema_ref: AUTOMATION_CERTIFICATION_SCHEMA_REF.to_owned(),
            contract_baseline_schema_ref: AUTOMATION_CERTIFICATION_CONTRACT_BASELINE_SCHEMA_REF
                .to_owned(),
            doc_ref: AUTOMATION_CERTIFICATION_DOC_REF.to_owned(),
            surfaces,
            surface_digest,
            certification_index,
            promotion_state: AutomationBaselinePromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against the frozen invariants.
    pub fn validate(&self) -> Vec<CertificationValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BaselineFindingSeverity::Blocker)
    }

    /// Returns the certification for the given surface, if present.
    pub fn surface_for(
        &self,
        surface: AutomationSurface,
    ) -> Option<&AutomationSurfaceCertification> {
        self.surfaces.iter().find(|row| row.surface == surface)
    }

    /// Builds an evidence join for one export/evidence surface.
    pub fn evidence_join(
        &self,
        surface: CertificationEvidenceSurface,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AutomationCertificationEvidenceJoinView {
        AutomationCertificationEvidenceJoinView {
            record_kind: AUTOMATION_CERTIFICATION_EVIDENCE_JOIN_RECORD_KIND.to_owned(),
            schema_version: AUTOMATION_CERTIFICATION_SCHEMA_VERSION,
            view_id: view_id.into(),
            surface,
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            surface_digest: self.surface_digest.clone(),
            certification_index: self.certification_index.clone(),
            surface_rows: self
                .surfaces
                .iter()
                .map(SurfaceCertificationRow::from_surface)
                .collect(),
        }
    }

    /// Builds the CLI/headless stable view of the certification matrix.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AutomationCertificationCliHeadlessView {
        AutomationCertificationCliHeadlessView {
            record_kind: AUTOMATION_CERTIFICATION_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: AUTOMATION_CERTIFICATION_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id_ref: self.packet_id.clone(),
            surface_digest: self.surface_digest.clone(),
            promotion_state: self.promotion_state,
            certification_index: self.certification_index.clone(),
            surface_rows: self
                .surfaces
                .iter()
                .map(SurfaceCertificationRow::from_surface)
                .collect(),
        }
    }

    /// Builds an export-safe support bundle carrying the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> AutomationCertificationSupportExport {
        AutomationCertificationSupportExport {
            record_kind: AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: AUTOMATION_CERTIFICATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id_ref: self.packet_id.clone(),
            packet: self.clone(),
        }
    }

    /// Returns the surface tokens present in the packet.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.surfaces {
            set.insert(row.surface);
        }
        set.into_iter().map(AutomationSurface::as_str).collect()
    }

    /// Returns the authoring-path tokens present across every surface.
    pub fn authoring_path_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.surfaces {
            set.insert(row.authoring_path);
        }
        set.into_iter()
            .map(AutomationAuthoringPath::as_str)
            .collect()
    }

    /// Returns the graded certification-dimension tokens.
    pub fn dimension_tokens(&self) -> Vec<&'static str> {
        AutomationCertificationDimension::ALL
            .into_iter()
            .map(AutomationCertificationDimension::as_str)
            .collect()
    }

    /// Compact, support-safe one-line-per-row rendering for the inspector.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet {} schema_version={} promotion={} surfaces={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.surfaces.len(),
            self.surface_digest,
        ));
        lines.push(format!(
            "index ref={} current={} certified={} shareable=[{}] narrowed=[{}] blocked=[{}]",
            self.certification_index.certification_ref,
            self.certification_index.all_surfaces_current,
            self.certification_index.all_surfaces_certified,
            self.certification_index.shareable_surfaces.join(","),
            self.certification_index.narrowed_surfaces.join(","),
            self.certification_index.blocked_surfaces.join(","),
        ));
        for row in &self.surfaces {
            lines.push(format!(
                "surface {} authoring={} shareable_claim={} claim={} labels={} age={}/{}d",
                row.surface.as_str(),
                row.authoring_path.as_str(),
                row.presents_as_shareable,
                row.claim_state.as_str(),
                row.safety_labels.len(),
                row.proof_age_days,
                row.freshness_window_days,
            ));
        }
        lines
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<CertificationValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != AUTOMATION_CERTIFICATION_RECORD_KIND {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::WrongRecordKind,
                None,
                "packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != AUTOMATION_CERTIFICATION_SCHEMA_VERSION {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::WrongSchemaVersion,
                None,
                "packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::MissingIdentity,
                None,
                "packet id and timestamp are required",
            ));
        }
        for (label, value) in [
            (
                "certification schema",
                self.certification_schema_ref.as_str(),
            ),
            (
                "contract baseline schema",
                self.contract_baseline_schema_ref.as_str(),
            ),
            ("doc", self.doc_ref.as_str()),
        ] {
            if value.trim().is_empty() {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::MissingIdentity,
                    None,
                    format!("{label} ref is required"),
                ));
            }
        }

        self.check_surfaces(&mut findings, include_record_fields);
        self.check_certification_index(&mut findings, include_record_fields);

        if include_record_fields {
            let expected_digest = surface_digest(&self.surfaces);
            if self.surface_digest != expected_digest {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::SurfaceDigestDrift,
                    None,
                    "stored surface digest does not match the surfaces",
                ));
            }
            let expected = promotion_state_for_findings(&findings);
            if self.promotion_state != expected {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::PromotionStateMismatch,
                    None,
                    format!(
                        "stored promotion state {} does not match derived {}",
                        self.promotion_state.as_str(),
                        expected.as_str()
                    ),
                ));
            }
        }

        findings
    }

    fn check_surfaces(
        &self,
        findings: &mut Vec<CertificationValidationFinding>,
        include_record_fields: bool,
    ) {
        let mut seen: BTreeMap<AutomationSurface, usize> = BTreeMap::new();
        for row in &self.surfaces {
            *seen.entry(row.surface).or_insert(0) += 1;
        }
        for surface in AutomationSurface::ALL {
            match seen.get(&surface) {
                None => findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::MissingSurface,
                    Some(surface.as_str().to_owned()),
                    format!("automation surface {} is missing", surface.as_str()),
                )),
                Some(count) if *count > 1 => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::DuplicateSurface,
                        Some(surface.as_str().to_owned()),
                        format!(
                            "automation surface {} is declared more than once",
                            surface.as_str()
                        ),
                    ));
                }
                Some(_) => {}
            }
        }

        for row in &self.surfaces {
            self.check_surface(row, findings, include_record_fields);
        }
    }

    fn check_surface(
        &self,
        row: &AutomationSurfaceCertification,
        findings: &mut Vec<CertificationValidationFinding>,
        include_record_fields: bool,
    ) {
        let label = row.surface.as_str();
        let subject = || Some(label.to_owned());

        // Stale evidence narrows below stable but does not block.
        let expected_freshness = freshness_for(row.proof_age_days, row.freshness_window_days);
        if expected_freshness == CertificationFreshnessState::Stale {
            findings.push(CertificationValidationFinding::warning(
                CertificationFindingKind::SurfaceEvidenceStale,
                subject(),
                format!(
                    "surface {} proof aged {} days past its {}-day window",
                    label, row.proof_age_days, row.freshness_window_days,
                ),
            ));
        }

        let outcomes = row.evaluate_dimensions();
        for outcome in &outcomes {
            if outcome.passed {
                continue;
            }
            match outcome.dimension {
                AutomationCertificationDimension::BuilderParity => {
                    if !row.authoring_path.is_conformant() {
                        findings.push(CertificationValidationFinding::blocker(
                            CertificationFindingKind::AdHocAuthoring,
                            subject(),
                            format!(
                                "surface {label} authors automation via {} instead of the declarative recipe builder",
                                row.authoring_path.as_str()
                            ),
                        ));
                    } else {
                        findings.push(CertificationValidationFinding::blocker(
                            CertificationFindingKind::MissingBuilderEvidence,
                            subject(),
                            format!("surface {label} cites no upstream builder proof"),
                        ));
                    }
                }
                AutomationCertificationDimension::ParameterReview => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::ParameterReviewMissing,
                        subject(),
                        format!(
                            "surface {label} routes inputs without a typed, secret-safe parameter review"
                        ),
                    ));
                }
                AutomationCertificationDimension::DryRunExplainCoverage => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::SideEffectPreviewMissing,
                        subject(),
                        format!(
                            "surface {label} applies automation with no dry-run/explain side-effect preview"
                        ),
                    ));
                }
                AutomationCertificationDimension::RunHistoryIntegrity => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::RunHistoryIntegrityMissing,
                        subject(),
                        format!(
                            "surface {label} keeps no durable, redaction-safe, rerun-under-policy run history"
                        ),
                    ));
                }
                AutomationCertificationDimension::MacroScopeSafety => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::MacroScopeUnsafe,
                        subject(),
                        format!("surface {label} records macros that are not scope-safe and fail-closed"),
                    ));
                }
                AutomationCertificationDimension::LabelReuse => {
                    findings.push(CertificationValidationFinding::blocker(
                        CertificationFindingKind::LabelReuseBroken,
                        subject(),
                        format!(
                            "surface {label} invents a label vocabulary instead of reusing the controlled set"
                        ),
                    ));
                }
            }
        }

        if row.evidence_refs_empty() {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::MissingEvidenceRef,
                subject(),
                format!("surface {label} cites no upstream automation proof"),
            ));
        }

        // The track invariant: a surface may only present itself as safe/shareable
        // when its current evidence proves every dimension. A shareable claim
        // without full proof blocks; a shareable claim on aged proof narrows.
        let certified = profile_certified(&outcomes);
        if row.presents_as_shareable {
            if !certified {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::ShareableClaimUnproven,
                    subject(),
                    format!(
                        "surface {label} presents as shareable but its evidence does not prove every dimension"
                    ),
                ));
            } else if expected_freshness == CertificationFreshnessState::Stale {
                findings.push(CertificationValidationFinding::warning(
                    CertificationFindingKind::ShareableClaimNarrowed,
                    subject(),
                    format!("surface {label} presents as shareable on proof aged past its window"),
                ));
            }
        }

        if include_record_fields {
            if row.dimension_outcomes != outcomes {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::DimensionOutcomeDrift,
                    subject(),
                    format!(
                        "surface {label} stored dimension outcomes disagree with the derivation"
                    ),
                ));
            }
            if row.certified != certified {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::SurfaceCertificationDrift,
                    subject(),
                    format!("surface {label} stored certified flag disagrees with the derivation"),
                ));
            }
            if row.freshness_state != expected_freshness {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::SurfaceFreshnessDrift,
                    subject(),
                    format!("surface {label} freshness state disagrees with proof age"),
                ));
            }
            let expected_claim = claim_state_for(certified, expected_freshness);
            if row.claim_state != expected_claim {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::SurfaceClaimStateDrift,
                    subject(),
                    format!("surface {label} stored claim state disagrees with the derivation"),
                ));
            }
        }
    }

    fn check_certification_index(
        &self,
        findings: &mut Vec<CertificationValidationFinding>,
        include_record_fields: bool,
    ) {
        if self.certification_index.certification_ref.trim().is_empty() {
            findings.push(CertificationValidationFinding::blocker(
                CertificationFindingKind::CertificationIndexMissing,
                None,
                "certification-index binding ref is required",
            ));
        }
        if include_record_fields {
            let expected = derive_certification_index(
                &self.certification_index.certification_ref,
                &self.surfaces,
            );
            if self.certification_index != expected {
                findings.push(CertificationValidationFinding::blocker(
                    CertificationFindingKind::CertificationIndexDrift,
                    None,
                    "stored certification index disagrees with the surfaces",
                ));
            }
        }
    }
}

/// Support-export wrapper carrying the exact certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationCertificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Exact packet exported.
    pub packet: AutomationCertificationPacket,
}

impl AutomationCertificationSupportExport {
    /// Returns true when the export is safe for support/review packets.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == AUTOMATION_CERTIFICATION_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.exported_at.trim().is_empty()
            && self.packet_id_ref == self.packet.packet_id
            && self.packet.is_stable()
    }
}

/// One surface row for an evidence join or CLI/headless view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCertificationRow {
    /// Surface token.
    pub surface: String,
    /// Support-safe claim summary.
    pub claim_summary: String,
    /// Whether the surface presents itself as shareable.
    pub presents_as_shareable: bool,
    /// Authoring-path token.
    pub authoring_path: String,
    /// Claim-state token.
    pub claim_state: String,
    /// Freshness-state token.
    pub freshness_state: String,
    /// Proof age in days.
    pub proof_age_days: u32,
    /// Freshness window in days.
    pub freshness_window_days: u32,
    /// True when every dimension passes.
    pub certified: bool,
    /// True when the surface is current and certified.
    pub shareable: bool,
    /// Number of upstream evidence refs cited.
    pub evidence_ref_count: usize,
    /// Controlled safety-label tokens the surface reuses.
    #[serde(default)]
    pub safety_label_tokens: Vec<String>,
    /// Dimension tokens that failed (empty when certified).
    #[serde(default)]
    pub failed_dimensions: Vec<String>,
    /// Support-safe explanation of the surface certification.
    pub explanation: String,
}

impl SurfaceCertificationRow {
    fn from_surface(row: &AutomationSurfaceCertification) -> Self {
        let failed_dimensions: Vec<String> = row
            .dimension_outcomes
            .iter()
            .filter(|outcome| !outcome.passed)
            .map(|outcome| outcome.dimension.as_str().to_owned())
            .collect();
        let explanation = format!(
            "{} via {} authoring; claim={}{}",
            row.surface.as_str(),
            row.authoring_path.as_str(),
            row.claim_state.as_str(),
            if failed_dimensions.is_empty() {
                String::new()
            } else {
                format!(", failed=[{}]", failed_dimensions.join(","))
            },
        );
        Self {
            surface: row.surface.as_str().to_owned(),
            claim_summary: row.claim_summary.clone(),
            presents_as_shareable: row.presents_as_shareable,
            authoring_path: row.authoring_path.as_str().to_owned(),
            claim_state: row.claim_state.as_str().to_owned(),
            freshness_state: row.freshness_state.as_str().to_owned(),
            proof_age_days: row.proof_age_days,
            freshness_window_days: row.freshness_window_days,
            certified: row.certified,
            shareable: row.is_shareable(),
            evidence_ref_count: row.evidence_refs.len(),
            safety_label_tokens: row
                .safety_label_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            failed_dimensions,
            explanation,
        }
    }
}

/// Evidence-join view for one export/evidence surface (support, incident, AI).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationCertificationEvidenceJoinView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Evidence surface this view serves.
    pub surface: CertificationEvidenceSurface,
    /// View timestamp.
    pub generated_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Order-invariant digest of the surfaces.
    pub surface_digest: String,
    /// Certification index.
    pub certification_index: AutomationCertificationIndex,
    /// Surface rows.
    #[serde(default)]
    pub surface_rows: Vec<SurfaceCertificationRow>,
}

impl AutomationCertificationEvidenceJoinView {
    /// Returns true when every row keeps its explanation and provenance fields.
    pub fn explains_consistently(&self) -> bool {
        self.surface_rows.iter().all(|row| {
            !row.surface.trim().is_empty()
                && !row.authoring_path.trim().is_empty()
                && !row.claim_summary.trim().is_empty()
                && !row.explanation.trim().is_empty()
        })
    }
}

/// CLI/headless stable view of the certification matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationCertificationCliHeadlessView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// View timestamp.
    pub generated_at: String,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Order-invariant digest of the surfaces.
    pub surface_digest: String,
    /// Derived promotion state.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Certification index.
    pub certification_index: AutomationCertificationIndex,
    /// Surface rows.
    #[serde(default)]
    pub surface_rows: Vec<SurfaceCertificationRow>,
}

impl AutomationCertificationCliHeadlessView {
    /// Returns true when every surface row is explained and cites evidence.
    pub fn every_surface_explained(&self) -> bool {
        self.surface_rows
            .iter()
            .all(|row| row.evidence_ref_count > 0 && !row.explanation.trim().is_empty())
    }
}

fn profile_certified(outcomes: &[AutomationDimensionOutcome]) -> bool {
    outcomes.iter().all(|outcome| outcome.passed)
}

fn freshness_for(proof_age_days: u32, freshness_window_days: u32) -> CertificationFreshnessState {
    if proof_age_days > freshness_window_days {
        CertificationFreshnessState::Stale
    } else {
        CertificationFreshnessState::Current
    }
}

fn claim_state_for(certified: bool, freshness: CertificationFreshnessState) -> SurfaceClaimState {
    if !certified {
        SurfaceClaimState::Blocked
    } else if freshness == CertificationFreshnessState::Stale {
        SurfaceClaimState::NarrowedBelowStable
    } else {
        SurfaceClaimState::Shareable
    }
}

fn derive_surface(mut row: AutomationSurfaceCertification) -> AutomationSurfaceCertification {
    row.dimension_outcomes = row.evaluate_dimensions();
    row.certified = profile_certified(&row.dimension_outcomes);
    row.freshness_state = freshness_for(row.proof_age_days, row.freshness_window_days);
    row.claim_state = claim_state_for(row.certified, row.freshness_state);
    row
}

fn derive_certification_index(
    certification_ref: &str,
    surfaces: &[AutomationSurfaceCertification],
) -> AutomationCertificationIndex {
    let all_surfaces_current = surfaces
        .iter()
        .all(|row| row.freshness_state == CertificationFreshnessState::Current);
    let all_surfaces_certified = !surfaces.is_empty() && surfaces.iter().all(|row| row.certified);
    let shareable_surfaces = surfaces_with_state(surfaces, SurfaceClaimState::Shareable);
    let narrowed_surfaces = surfaces_with_state(surfaces, SurfaceClaimState::NarrowedBelowStable);
    let blocked_surfaces = surfaces_with_state(surfaces, SurfaceClaimState::Blocked);
    let certification_summary = format!(
        "{} surfaces; shareable={}, narrowed={}, blocked={}",
        surfaces.len(),
        shareable_surfaces.len(),
        narrowed_surfaces.len(),
        blocked_surfaces.len(),
    );
    AutomationCertificationIndex {
        certification_ref: certification_ref.to_owned(),
        all_surfaces_current,
        all_surfaces_certified,
        shareable_surfaces,
        narrowed_surfaces,
        blocked_surfaces,
        certification_summary,
    }
}

fn surfaces_with_state(
    surfaces: &[AutomationSurfaceCertification],
    state: SurfaceClaimState,
) -> Vec<String> {
    surfaces
        .iter()
        .filter(|row| row.claim_state == state)
        .map(|row| row.surface.as_str().to_owned())
        .collect()
}

/// Order-invariant FNV-1a 64-bit digest of every surface token.
fn surface_digest(surfaces: &[AutomationSurfaceCertification]) -> String {
    let mut tokens: Vec<&str> = surfaces.iter().map(|row| row.surface.as_str()).collect();
    tokens.sort_unstable();
    fnv1a64(&tokens)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for item in items_in_order {
        for byte in item.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

fn promotion_state_for_findings(
    findings: &[CertificationValidationFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == BaselineFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == BaselineFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

// ---------------------------------------------------------------------------
// Seeds
// ---------------------------------------------------------------------------

fn canonical_surface(surface: AutomationSurface) -> AutomationSurfaceCertification {
    use AutomationSafetyLabelId::{
        ApprovalRequired, HeadlessSafe, MacroSafe, NetworkCall, RecipeSafe, RemoteMutation,
        RunsProcess, WritesFiles,
    };

    let (claim_summary, safety_labels): (&str, Vec<AutomationSafetyLabelId>) = match surface {
        AutomationSurface::NotebookAutomation => (
            "notebook run automation authored in the declarative builder with reviewed parameters, a side-effect preview, durable history, scope-safe macros, and reused labels",
            vec![RecipeSafe, MacroSafe, HeadlessSafe, RunsProcess, WritesFiles],
        ),
        AutomationSurface::RequestApiAutomation => (
            "saved request automation authored in the declarative builder with reviewed parameters, a side-effect preview, durable history, scope-safe macros, and reused labels",
            vec![RecipeSafe, HeadlessSafe, NetworkCall],
        ),
        AutomationSurface::PackageAutomation => (
            "package automation authored in the declarative builder with reviewed parameters, a side-effect preview, durable history, scope-safe macros, and reused labels",
            vec![RecipeSafe, ApprovalRequired, NetworkCall, WritesFiles],
        ),
        AutomationSurface::TestDebugAutomation => (
            "task/test/debug automation authored in the declarative builder with reviewed parameters, a side-effect preview, durable history, scope-safe macros, and reused labels",
            vec![RecipeSafe, MacroSafe, HeadlessSafe, RunsProcess],
        ),
        AutomationSurface::IncidentAutomation => (
            "incident automation authored in the declarative builder with reviewed parameters, a side-effect preview, durable history, scope-safe macros, and reused labels",
            vec![RecipeSafe, ApprovalRequired, NetworkCall, RemoteMutation],
        ),
        AutomationSurface::AiLinkedAutomation => (
            "AI-linked automation authored in the declarative builder with reviewed parameters, a side-effect preview, durable history, scope-safe macros, and reused labels",
            vec![RecipeSafe, ApprovalRequired, RunsProcess, WritesFiles],
        ),
    };

    AutomationSurfaceCertification {
        surface,
        claim_summary: claim_summary.to_owned(),
        presents_as_shareable: true,
        authoring_path: AutomationAuthoringPath::DeclarativeRecipeBuilder,
        parameters_reviewed: true,
        secret_references_safe: true,
        side_effect_preview_shown: true,
        predicted_effects_disclosed: true,
        run_history_durable: true,
        run_history_redaction_safe: true,
        rerun_under_current_policy: true,
        macro_scope_declared: true,
        macro_fails_closed_on_mismatch: true,
        reuses_controlled_labels: true,
        safety_labels,
        evidence_refs: AUTOMATION_CERTIFICATION_EVIDENCE_REFS
            .iter()
            .map(|reference| (*reference).to_owned())
            .collect(),
        last_certified_at: "2026-06-15T00:00:00Z".to_owned(),
        proof_age_days: 3,
        freshness_window_days: 30,
        // Overwritten by `derive_surface` at materialization.
        freshness_state: CertificationFreshnessState::Current,
        dimension_outcomes: Vec::new(),
        certified: false,
        claim_state: SurfaceClaimState::Blocked,
    }
}

/// Builds the canonical stable automation certification packet input.
pub fn current_stable_automation_certification_input() -> AutomationCertificationPacketInput {
    AutomationCertificationPacketInput {
        packet_id: AUTOMATION_CERTIFICATION_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        certification_ref: AUTOMATION_CERTIFICATION_INDEX_REF.to_owned(),
        surfaces: AutomationSurface::ALL
            .into_iter()
            .map(canonical_surface)
            .collect(),
    }
}

/// Materializes the canonical stable automation certification packet.
pub fn seeded_automation_certification_packet() -> AutomationCertificationPacket {
    AutomationCertificationPacket::materialize(current_stable_automation_certification_input())
}

/// Validates a packet and returns an `Ok(())` / findings result.
pub fn validate_automation_certification_packet(
    packet: &AutomationCertificationPacket,
) -> Result<(), Vec<CertificationValidationFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}
