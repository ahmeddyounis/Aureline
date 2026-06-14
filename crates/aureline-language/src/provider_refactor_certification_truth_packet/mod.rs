//! Provider-arbitration, diagnostic-convergence, and refactor preview/rollback
//! certification truth packet.
//!
//! This module is the language-owned exit-gate contract that certifies — on
//! every claimed framework-pack and structured-artifact lane — that the
//! provider-arbitration, diagnostic-convergence, and refactor preview/rollback
//! truth the matrix already froze is actually *proven* by checked-in evidence
//! drills before the lane keeps a certified grade. Where the
//! [`crate::provider_refactor_matrix_truth_packet`] freezes which posture a lane
//! *may* claim, this packet records whether the lane has earned that claim:
//! provider agreement/disagreement handling, downgrade honesty, diagnostic
//! convergence, refactor preview completeness, and deterministic rollback, each
//! backed by the fixture-repo, notebook/generated/config, partial-scope,
//! provider crash/quarantine, and rollback-determinism drills the exit gate
//! demands.
//!
//! The packet is the single certification truth that the framework-pack panel,
//! structured-artifact runner, preview surface, compatibility report, archetype
//! scorecard, release-narrowing automation, support export, Help/About proof
//! card, service-health feed, and conformance dashboard all read. Surfaces MUST
//! NOT mint local copies or paraphrase certification verdicts; they read this
//! packet verbatim. A lane whose arbitration or refactor truth is unproven,
//! stale, or downgraded narrows automatically rather than inheriting the
//! broader framework-pack marketing language.
//!
//! The certification extends — it does not redefine — the launch-language
//! refactor transaction safety model. A lane that certifies a mutating refactor
//! carries the same typed preview, completeness, and rollback discipline the
//! matrix and refactor-transaction packets already pin; this packet only adds
//! the proof verdicts and evidence drills on top, never weakening the existing
//! safety model.
//!
//! The packet is intentionally metadata-only — it never admits raw source
//! bodies, refactor diffs, generated artifact bodies, notebook cell outputs,
//! provider payloads, secrets, ambient credentials, or any other private
//! material past the boundary. A row that claims `certified` while leaving a
//! required binding unbound is refused; the validator narrows below certified
//! instead of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reuse the canonical leaf vocabularies frozen by the provider/refactor matrix
// packet instead of minting a local synonym set; this packet certifies the
// same families the matrix admits.
pub use crate::provider_refactor_matrix_truth_packet::{
    ArtifactFamilyLaneClass, CompletenessClass, ConfidenceClass, ConflictClass,
    DowngradeAutomationClass, EvidenceClass, GeneratedArtifactPolicyClass, KnownLimitClass,
    ProviderFamilyClass, RefactorTransactionClass, RollbackPathClass, SupportClass,
};

/// Stable record-kind tag for [`ProviderRefactorCertificationTruthPacket`].
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_PACKET_RECORD_KIND: &str =
    "provider_refactor_certification_truth_stable_packet";

/// Stable record-kind tag for [`ProviderRefactorCertificationTruthSupportExport`].
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "provider_refactor_certification_truth_support_export";

/// Integer schema version for the certification truth packet.
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_REF: &str =
    "schemas/language/provider_refactor_certification_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_DOC_REF: &str =
    "docs/m5/certify-language-provider-arbitration-diagnostic-convergence-and-refactor-preview-rollback-truth.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/certify-language-provider-arbitration-diagnostic-convergence-and-refactor-preview-rollback-truth.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/provider_refactor_certification_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const PROVIDER_REFACTOR_CERTIFICATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/provider_refactor_certification_truth_packet.json";

/// Closed certification-row vocabulary the packet evaluates. Each admission row
/// class owns exactly one certification dimension; the headline
/// `lane_certification` row carries the lane's acting provider family and
/// verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationRowClass {
    /// The lane's headline certification row binding acting provider and verdict.
    LaneCertification,
    /// Provider-arbitration certification row binding the arbitration proof and conflict class.
    ProviderArbitrationCertification,
    /// Diagnostic-convergence certification row binding the convergence proof.
    DiagnosticConvergenceCertification,
    /// Refactor-preview certification row binding the refactor class and completeness.
    RefactorPreviewCertification,
    /// Rollback-determinism certification row binding the rollback path and determinism proof.
    RollbackDeterminismCertification,
    /// Generated-artifact policy certification row binding the generated-asset policy.
    GeneratedArtifactPolicyCertification,
    /// Evidence-drill admission row binding one proven drill.
    EvidenceDrillAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
}

impl CertificationRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaneCertification => "lane_certification",
            Self::ProviderArbitrationCertification => "provider_arbitration_certification",
            Self::DiagnosticConvergenceCertification => "diagnostic_convergence_certification",
            Self::RefactorPreviewCertification => "refactor_preview_certification",
            Self::RollbackDeterminismCertification => "rollback_determinism_certification",
            Self::GeneratedArtifactPolicyCertification => "generated_artifact_policy_certification",
            Self::EvidenceDrillAdmission => "evidence_drill_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
            Self::UnsupportedGap => "unsupported_gap",
        }
    }

    /// True when the row class must name a concrete acting provider family.
    pub const fn requires_provider_family(self) -> bool {
        matches!(self, Self::LaneCertification)
    }
}

/// Closed certification-verdict vocabulary. A `lane_certification` row binds
/// exactly one verdict; the losing detail stays inspectable and a narrowed
/// verdict never inherits a broader marketed grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationVerdictClass {
    /// The lane is fully certified; every proof and drill is current.
    Certified,
    /// The lane is provisionally certified pending a disclosed gap.
    ProvisionallyCertified,
    /// The lane narrows below certified until a recorded gap closes.
    NarrowedBelowCertified,
    /// The lane is blocked until required evidence lands.
    BlockedPendingEvidence,
    /// The lane's certification is withdrawn.
    Withdrawn,
    /// Row is not a lane-certification row.
    NotApplicable,
    /// Row has no bound verdict; this never qualifies certified.
    VerdictUnbound,
}

impl CertificationVerdictClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ProvisionallyCertified => "provisionally_certified",
            Self::NarrowedBelowCertified => "narrowed_below_certified",
            Self::BlockedPendingEvidence => "blocked_pending_evidence",
            Self::Withdrawn => "withdrawn",
            Self::NotApplicable => "not_applicable",
            Self::VerdictUnbound => "verdict_unbound",
        }
    }

    /// True when this verdict is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::VerdictUnbound)
    }

    /// True when this verdict is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::VerdictUnbound)
    }

    /// True when this verdict contradicts a `certified` support class.
    pub const fn contradicts_certified_support(self) -> bool {
        matches!(self, Self::BlockedPendingEvidence | Self::Withdrawn)
    }
}

/// Closed provider-arbitration proof vocabulary. A
/// `provider_arbitration_certification` row binds exactly one proven path. The
/// losing provider and downgrade reason stay inspectable; arbitration is never
/// collapsed to a ranking-only result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArbitrationProofClass {
    /// A single provider answered; no arbitration was required.
    SingleProviderNoConflict,
    /// Agreement and disagreement paths both exercised and proven.
    AgreementAndDisagreementProven,
    /// Disagreement preserved the losing provider alongside the winner.
    DisagreementWinnerLoserPreserved,
    /// Downgrade honesty proven: the downgrade reason stays inspectable.
    DowngradeHonestyProven,
    /// Provider crash/quarantine handling proven by a recovery drill.
    ProviderCrashQuarantineProven,
    /// Row is not a provider-arbitration certification row.
    NotApplicable,
    /// Row has no bound arbitration proof; this never qualifies certified.
    ProofUnbound,
}

impl ArbitrationProofClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleProviderNoConflict => "single_provider_no_conflict",
            Self::AgreementAndDisagreementProven => "agreement_and_disagreement_proven",
            Self::DisagreementWinnerLoserPreserved => "disagreement_winner_loser_preserved",
            Self::DowngradeHonestyProven => "downgrade_honesty_proven",
            Self::ProviderCrashQuarantineProven => "provider_crash_quarantine_proven",
            Self::NotApplicable => "not_applicable",
            Self::ProofUnbound => "proof_unbound",
        }
    }

    /// True when this proof is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ProofUnbound)
    }

    /// True when this proof is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ProofUnbound)
    }
}

/// Closed diagnostic-convergence proof vocabulary. A
/// `diagnostic_convergence_certification` row binds exactly one proven path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergenceProofClass {
    /// A single diagnostic source answered; no convergence was required.
    SingleSourceNoConvergence,
    /// Multiple sources converged behind a labeled cluster.
    MultiSourceConvergedLabeled,
    /// Per-source provenance preserved across the converged cluster.
    ProvenancePreservedPerSource,
    /// Suppression state preserved across convergence.
    SuppressionStatePreserved,
    /// Freshness labeled on every converged cluster.
    FreshnessLabeled,
    /// Row is not a diagnostic-convergence certification row.
    NotApplicable,
    /// Row has no bound convergence proof; this never qualifies certified.
    ProofUnbound,
}

impl ConvergenceProofClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleSourceNoConvergence => "single_source_no_convergence",
            Self::MultiSourceConvergedLabeled => "multi_source_converged_labeled",
            Self::ProvenancePreservedPerSource => "provenance_preserved_per_source",
            Self::SuppressionStatePreserved => "suppression_state_preserved",
            Self::FreshnessLabeled => "freshness_labeled",
            Self::NotApplicable => "not_applicable",
            Self::ProofUnbound => "proof_unbound",
        }
    }

    /// True when this proof is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ProofUnbound)
    }

    /// True when this proof is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ProofUnbound)
    }
}

/// Closed rollback-determinism vocabulary. A
/// `rollback_determinism_certification` row binds exactly one determinism
/// outcome alongside the rollback path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDeterminismClass {
    /// Rollback proven deterministic by a replay drill.
    DeterministicRollbackProven,
    /// Checkpoint replay verified to restore the pre-mutation state.
    CheckpointReplayVerified,
    /// Regeneration replay verified for a generated artifact.
    RegenerationReplayVerified,
    /// Manual review is required; no automatic determinism is claimed.
    ManualReviewOnly,
    /// Rollback is nondeterministic / unsafe; this is never certified.
    NondeterministicUnsafe,
    /// Row is not a rollback-determinism certification row.
    NotApplicable,
    /// Row has no bound determinism outcome; this never qualifies certified.
    DeterminismUnbound,
}

impl RollbackDeterminismClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeterministicRollbackProven => "deterministic_rollback_proven",
            Self::CheckpointReplayVerified => "checkpoint_replay_verified",
            Self::RegenerationReplayVerified => "regeneration_replay_verified",
            Self::ManualReviewOnly => "manual_review_only",
            Self::NondeterministicUnsafe => "nondeterministic_unsafe",
            Self::NotApplicable => "not_applicable",
            Self::DeterminismUnbound => "determinism_unbound",
        }
    }

    /// True when this determinism outcome is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::DeterminismUnbound)
    }

    /// True when this determinism outcome is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::DeterminismUnbound)
    }

    /// True when this determinism outcome proves a mutating refactor is safe to apply.
    pub const fn proves_safe_rollback(self) -> bool {
        matches!(
            self,
            Self::DeterministicRollbackProven
                | Self::CheckpointReplayVerified
                | Self::RegenerationReplayVerified
        )
    }
}

/// Closed evidence-drill vocabulary. An `evidence_drill_admission` row binds
/// exactly one proven drill; the packet requires the full drill set before any
/// lane keeps a certified grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDrillClass {
    /// A certified fixture-repo capture.
    FixtureRepoDrill,
    /// A notebook-cell certification case.
    NotebookCaseDrill,
    /// A generated-source certification case.
    GeneratedCaseDrill,
    /// A structured config-artifact certification case.
    ConfigCaseDrill,
    /// A partial-scope refactor drill.
    PartialScopeDrill,
    /// A provider crash / quarantine recovery drill.
    ProviderCrashQuarantineDrill,
    /// A rollback-determinism result.
    RollbackDeterminismDrill,
    /// Row is not an evidence-drill admission row.
    NotApplicable,
    /// Row has no bound drill; this never qualifies certified.
    DrillUnbound,
}

impl EvidenceDrillClass {
    /// Drills the packet requires before any lane keeps a certified grade.
    pub const REQUIRED: [Self; 7] = [
        Self::FixtureRepoDrill,
        Self::NotebookCaseDrill,
        Self::GeneratedCaseDrill,
        Self::ConfigCaseDrill,
        Self::PartialScopeDrill,
        Self::ProviderCrashQuarantineDrill,
        Self::RollbackDeterminismDrill,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixtureRepoDrill => "fixture_repo_drill",
            Self::NotebookCaseDrill => "notebook_case_drill",
            Self::GeneratedCaseDrill => "generated_case_drill",
            Self::ConfigCaseDrill => "config_case_drill",
            Self::PartialScopeDrill => "partial_scope_drill",
            Self::ProviderCrashQuarantineDrill => "provider_crash_quarantine_drill",
            Self::RollbackDeterminismDrill => "rollback_determinism_drill",
            Self::NotApplicable => "not_applicable",
            Self::DrillUnbound => "drill_unbound",
        }
    }

    /// True when this drill is a concrete, bound drill.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::DrillUnbound)
    }

    /// True when this drill is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::DrillUnbound)
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim across all required lanes.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the certification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required artifact-family lane has no row.
    MissingArtifactFamilyLaneCoverage,
    /// A lane claiming certified is missing a provider-arbitration certification.
    MissingProviderArbitrationCoverage,
    /// A lane claiming certified is missing a diagnostic-convergence certification.
    MissingDiagnosticConvergenceCoverage,
    /// A lane claiming certified is missing a refactor-preview certification.
    MissingRefactorPreviewCoverage,
    /// A lane claiming certified is missing a rollback-determinism certification.
    MissingRollbackDeterminismCoverage,
    /// A lane claiming certified is missing a generated-artifact policy certification.
    MissingGeneratedArtifactPolicyCoverage,
    /// A lane claiming certified is missing an evidence-drill admission.
    MissingEvidenceDrillCoverage,
    /// A certified packet is missing a required evidence drill across all lanes.
    MissingRequiredEvidenceDrill,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row that must name a provider family has no concrete provider.
    MissingProviderFamily,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A lane-certification row has no bound verdict.
    MissingVerdictClass,
    /// A provider-arbitration certification row has no bound proof.
    MissingArbitrationProofClass,
    /// A provider-arbitration certification row has no bound conflict class.
    MissingConflictClass,
    /// A diagnostic-convergence certification row has no bound proof.
    MissingConvergenceProofClass,
    /// A refactor-preview certification row has no bound refactor class.
    MissingRefactorTransactionClass,
    /// A refactor-preview certification row has no bound completeness class.
    MissingCompletenessClass,
    /// A rollback-determinism certification row has no bound rollback path.
    MissingRollbackPathClass,
    /// A rollback-determinism certification row has no bound determinism outcome.
    MissingRollbackDeterminismClass,
    /// A generated-artifact policy certification row has no bound policy.
    MissingGeneratedArtifactPolicyClass,
    /// An evidence-drill admission row has no bound drill.
    MissingEvidenceDrillClass,
    /// A row claims certified while one or more bindings is unbound.
    CertifiedWithUnboundBinding,
    /// A lane-certification row claims certified support while its verdict says blocked/withdrawn.
    VerdictSupportMismatch,
    /// A row narrowed below certified drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A verdict is bound on a non-lane-certification row.
    VerdictNotPermittedOnRowClass,
    /// An arbitration proof is bound on a non-arbitration row.
    ArbitrationProofNotPermittedOnRowClass,
    /// A conflict class is bound on a non-arbitration row.
    ConflictNotPermittedOnRowClass,
    /// A convergence proof is bound on a non-convergence row.
    ConvergenceProofNotPermittedOnRowClass,
    /// A refactor class is bound on a non-refactor-preview row.
    RefactorTransactionNotPermittedOnRowClass,
    /// A completeness class is bound on a non-refactor-preview row.
    CompletenessNotPermittedOnRowClass,
    /// A rollback path is bound on a non-rollback-determinism row.
    RollbackPathNotPermittedOnRowClass,
    /// A determinism outcome is bound on a non-rollback-determinism row.
    RollbackDeterminismNotPermittedOnRowClass,
    /// A generated-artifact policy is bound on a non-generated row.
    GeneratedArtifactPolicyNotPermittedOnRowClass,
    /// An evidence drill is bound on a non-evidence-drill row.
    EvidenceDrillNotPermittedOnRowClass,
    /// A mutating refactor-preview row leaves preview completeness unsafe.
    MutationBypassesPreviewOrRollback,
    /// A rollback-determinism row binds a mutating, unproven rollback.
    RollbackDeterminismNotProven,
    /// A provider-arbitration row collapses disagreement into a ranking-only result.
    DisagreementCollapsedToRankingOnly,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops certification truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the provider-family vocabulary.
    ProviderFamilyVocabularyCollapsed,
    /// A projection collapses the verdict vocabulary.
    VerdictVocabularyCollapsed,
    /// A projection collapses the arbitration-proof vocabulary.
    ArbitrationProofVocabularyCollapsed,
    /// A projection collapses the conflict vocabulary.
    ConflictVocabularyCollapsed,
    /// A projection collapses the convergence-proof vocabulary.
    ConvergenceProofVocabularyCollapsed,
    /// A projection collapses the refactor-transaction vocabulary.
    RefactorTransactionVocabularyCollapsed,
    /// A projection collapses the completeness vocabulary.
    CompletenessVocabularyCollapsed,
    /// A projection collapses the rollback-path vocabulary.
    RollbackPathVocabularyCollapsed,
    /// A projection collapses the rollback-determinism vocabulary.
    RollbackDeterminismVocabularyCollapsed,
    /// A projection collapses the generated-artifact policy vocabulary.
    GeneratedArtifactPolicyVocabularyCollapsed,
    /// A projection collapses the evidence-drill vocabulary.
    EvidenceDrillVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingArtifactFamilyLaneCoverage => "missing_artifact_family_lane_coverage",
            Self::MissingProviderArbitrationCoverage => "missing_provider_arbitration_coverage",
            Self::MissingDiagnosticConvergenceCoverage => "missing_diagnostic_convergence_coverage",
            Self::MissingRefactorPreviewCoverage => "missing_refactor_preview_coverage",
            Self::MissingRollbackDeterminismCoverage => "missing_rollback_determinism_coverage",
            Self::MissingGeneratedArtifactPolicyCoverage => {
                "missing_generated_artifact_policy_coverage"
            }
            Self::MissingEvidenceDrillCoverage => "missing_evidence_drill_coverage",
            Self::MissingRequiredEvidenceDrill => "missing_required_evidence_drill",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingProviderFamily => "missing_provider_family",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingVerdictClass => "missing_verdict_class",
            Self::MissingArbitrationProofClass => "missing_arbitration_proof_class",
            Self::MissingConflictClass => "missing_conflict_class",
            Self::MissingConvergenceProofClass => "missing_convergence_proof_class",
            Self::MissingRefactorTransactionClass => "missing_refactor_transaction_class",
            Self::MissingCompletenessClass => "missing_completeness_class",
            Self::MissingRollbackPathClass => "missing_rollback_path_class",
            Self::MissingRollbackDeterminismClass => "missing_rollback_determinism_class",
            Self::MissingGeneratedArtifactPolicyClass => "missing_generated_artifact_policy_class",
            Self::MissingEvidenceDrillClass => "missing_evidence_drill_class",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::VerdictSupportMismatch => "verdict_support_mismatch",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::VerdictNotPermittedOnRowClass => "verdict_not_permitted_on_row_class",
            Self::ArbitrationProofNotPermittedOnRowClass => {
                "arbitration_proof_not_permitted_on_row_class"
            }
            Self::ConflictNotPermittedOnRowClass => "conflict_not_permitted_on_row_class",
            Self::ConvergenceProofNotPermittedOnRowClass => {
                "convergence_proof_not_permitted_on_row_class"
            }
            Self::RefactorTransactionNotPermittedOnRowClass => {
                "refactor_transaction_not_permitted_on_row_class"
            }
            Self::CompletenessNotPermittedOnRowClass => "completeness_not_permitted_on_row_class",
            Self::RollbackPathNotPermittedOnRowClass => "rollback_path_not_permitted_on_row_class",
            Self::RollbackDeterminismNotPermittedOnRowClass => {
                "rollback_determinism_not_permitted_on_row_class"
            }
            Self::GeneratedArtifactPolicyNotPermittedOnRowClass => {
                "generated_artifact_policy_not_permitted_on_row_class"
            }
            Self::EvidenceDrillNotPermittedOnRowClass => {
                "evidence_drill_not_permitted_on_row_class"
            }
            Self::MutationBypassesPreviewOrRollback => "mutation_bypasses_preview_or_rollback",
            Self::RollbackDeterminismNotProven => "rollback_determinism_not_proven",
            Self::DisagreementCollapsedToRankingOnly => "disagreement_collapsed_to_ranking_only",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::ProviderFamilyVocabularyCollapsed => "provider_family_vocabulary_collapsed",
            Self::VerdictVocabularyCollapsed => "verdict_vocabulary_collapsed",
            Self::ArbitrationProofVocabularyCollapsed => "arbitration_proof_vocabulary_collapsed",
            Self::ConflictVocabularyCollapsed => "conflict_vocabulary_collapsed",
            Self::ConvergenceProofVocabularyCollapsed => "convergence_proof_vocabulary_collapsed",
            Self::RefactorTransactionVocabularyCollapsed => {
                "refactor_transaction_vocabulary_collapsed"
            }
            Self::CompletenessVocabularyCollapsed => "completeness_vocabulary_collapsed",
            Self::RollbackPathVocabularyCollapsed => "rollback_path_vocabulary_collapsed",
            Self::RollbackDeterminismVocabularyCollapsed => {
                "rollback_determinism_vocabulary_collapsed"
            }
            Self::GeneratedArtifactPolicyVocabularyCollapsed => {
                "generated_artifact_policy_vocabulary_collapsed"
            }
            Self::EvidenceDrillVocabularyCollapsed => "evidence_drill_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the certification packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Framework-pack panel surface.
    FrameworkPackPanel,
    /// Structured-artifact runner surface.
    StructuredArtifactRunner,
    /// Preview surface.
    PreviewSurface,
    /// Compatibility-report surface.
    CompatibilityReport,
    /// Archetype-scorecard surface.
    ArchetypeScorecard,
    /// Release-narrowing automation surface.
    ReleaseNarrowingAutomation,
    /// Support export bundle surface.
    SupportExport,
    /// Help/About proof card surface.
    HelpAbout,
    /// Service-health feed surface.
    ServiceHealth,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 10] = [
        Self::FrameworkPackPanel,
        Self::StructuredArtifactRunner,
        Self::PreviewSurface,
        Self::CompatibilityReport,
        Self::ArchetypeScorecard,
        Self::ReleaseNarrowingAutomation,
        Self::SupportExport,
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackPanel => "framework_pack_panel",
            Self::StructuredArtifactRunner => "structured_artifact_runner",
            Self::PreviewSurface => "preview_surface",
            Self::CompatibilityReport => "compatibility_report",
            Self::ArchetypeScorecard => "archetype_scorecard",
            Self::ReleaseNarrowingAutomation => "release_narrowing_automation",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One certification row binding an artifact-family lane to its certification proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Artifact-family lane this row certifies.
    pub lane_class: ArtifactFamilyLaneClass,
    /// Row class.
    pub row_class: CertificationRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Acting provider family (or `not_applicable`).
    pub provider_family_class: ProviderFamilyClass,
    /// Certification verdict (or `not_applicable`).
    pub verdict_class: CertificationVerdictClass,
    /// Provider-arbitration proof (or `not_applicable`).
    pub arbitration_proof_class: ArbitrationProofClass,
    /// Provider-conflict class (or `not_applicable`).
    pub conflict_class: ConflictClass,
    /// Diagnostic-convergence proof (or `not_applicable`).
    pub convergence_proof_class: ConvergenceProofClass,
    /// Refactor-transaction class (or `not_applicable`).
    pub refactor_transaction_class: RefactorTransactionClass,
    /// Preview-completeness class (or `not_applicable`).
    pub completeness_class: CompletenessClass,
    /// Rollback path class (or `not_applicable`).
    pub rollback_path_class: RollbackPathClass,
    /// Rollback-determinism outcome (or `not_applicable`).
    pub rollback_determinism_class: RollbackDeterminismClass,
    /// Generated-artifact policy class (or `not_applicable`).
    pub generated_artifact_policy_class: GeneratedArtifactPolicyClass,
    /// Evidence-drill class (or `not_applicable`).
    pub evidence_drill_class: EvidenceDrillClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// True when provider disagreement stays inspectable (never ranking-only).
    pub disagreement_inspectable: bool,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not `certified`,
    /// declares a non-`none_declared` known limit, or binds a non-`none`
    /// automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl CertificationRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.provider_family_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Certification packet id consumed by the projection.
    pub certification_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the provider-family vocabulary is preserved verbatim.
    pub preserves_provider_family_vocabulary: bool,
    /// True when the verdict vocabulary is preserved verbatim.
    pub preserves_verdict_vocabulary: bool,
    /// True when the arbitration-proof vocabulary is preserved verbatim.
    pub preserves_arbitration_proof_vocabulary: bool,
    /// True when the conflict vocabulary is preserved verbatim.
    pub preserves_conflict_vocabulary: bool,
    /// True when the convergence-proof vocabulary is preserved verbatim.
    pub preserves_convergence_proof_vocabulary: bool,
    /// True when the refactor-transaction vocabulary is preserved verbatim.
    pub preserves_refactor_transaction_vocabulary: bool,
    /// True when the completeness vocabulary is preserved verbatim.
    pub preserves_completeness_vocabulary: bool,
    /// True when the rollback-path vocabulary is preserved verbatim.
    pub preserves_rollback_path_vocabulary: bool,
    /// True when the rollback-determinism vocabulary is preserved verbatim.
    pub preserves_rollback_determinism_vocabulary: bool,
    /// True when the generated-artifact policy vocabulary is preserved verbatim.
    pub preserves_generated_artifact_policy_vocabulary: bool,
    /// True when the evidence-drill vocabulary is preserved verbatim.
    pub preserves_evidence_drill_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl CertificationConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.certification_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_provider_family_vocabulary
            && self.preserves_verdict_vocabulary
            && self.preserves_arbitration_proof_vocabulary
            && self.preserves_conflict_vocabulary
            && self.preserves_convergence_proof_vocabulary
            && self.preserves_refactor_transaction_vocabulary
            && self.preserves_completeness_vocabulary
            && self.preserves_rollback_path_vocabulary
            && self.preserves_rollback_determinism_vocabulary
            && self.preserves_generated_artifact_policy_vocabulary
            && self.preserves_evidence_drill_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`ProviderRefactorCertificationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRefactorCertificationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Certification rows.
    #[serde(default)]
    pub rows: Vec<CertificationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CertificationConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying provider-arbitration,
/// diagnostic-convergence, and refactor preview/rollback truth across the M5
/// framework and structured-artifact lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRefactorCertificationTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Certification rows.
    #[serde(default)]
    pub rows: Vec<CertificationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CertificationConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl ProviderRefactorCertificationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: ProviderRefactorCertificationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: PROVIDER_REFACTOR_CERTIFICATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable certification invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns true when the packet has a certified lane-certification row.
    pub fn has_certified_lane(&self) -> bool {
        self.rows.iter().any(|row| {
            matches!(row.row_class, CertificationRowClass::LaneCertification)
                && matches!(row.support_class, SupportClass::Certified)
        })
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.lane_class.as_str())
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.row_class.as_str())
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.support_class.as_str())
    }

    /// Returns the unique provider-family tokens observed across rows.
    pub fn provider_family_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provider_family_class.as_str())
    }

    /// Returns the unique verdict tokens observed across rows.
    pub fn verdict_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.verdict_class.as_str())
    }

    /// Returns the unique arbitration-proof tokens observed across rows.
    pub fn arbitration_proof_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.arbitration_proof_class.as_str())
    }

    /// Returns the unique conflict tokens observed across rows.
    pub fn conflict_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.conflict_class.as_str())
    }

    /// Returns the unique convergence-proof tokens observed across rows.
    pub fn convergence_proof_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.convergence_proof_class.as_str())
    }

    /// Returns the unique refactor-transaction tokens observed across rows.
    pub fn refactor_transaction_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.refactor_transaction_class.as_str())
    }

    /// Returns the unique completeness tokens observed across rows.
    pub fn completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.completeness_class.as_str())
    }

    /// Returns the unique rollback-path tokens observed across rows.
    pub fn rollback_path_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.rollback_path_class.as_str())
    }

    /// Returns the unique rollback-determinism tokens observed across rows.
    pub fn rollback_determinism_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.rollback_determinism_class.as_str())
    }

    /// Returns the unique generated-artifact policy tokens observed across rows.
    pub fn generated_artifact_policy_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.generated_artifact_policy_class.as_str())
    }

    /// Returns the unique evidence-drill tokens observed across rows.
    pub fn evidence_drill_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.evidence_drill_class.as_str())
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.known_limit_class.as_str())
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_automation_class.as_str())
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.evidence_class.as_str())
    }

    fn unique_tokens(
        &self,
        project: impl Fn(&CertificationRow) -> &'static str,
    ) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(project(row));
        }
        set.into_iter().collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ProviderRefactorCertificationTruthSupportExport {
        ProviderRefactorCertificationTruthSupportExport {
            record_kind: PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
                .to_owned(),
            schema_version: PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            certification_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            certification_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != PROVIDER_REFACTOR_CERTIFICATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "certification packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "certification packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingArtifactFamilyLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered artifact-family lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingArtifactFamilyLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers artifact-family lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.append_per_row_findings(row, &mut findings);
        }

        for lane in &self.covered_lanes {
            self.append_per_lane_coverage_findings(*lane, &mut findings);
        }

        self.append_required_drill_findings(&mut findings);

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            self.append_projection_findings(projection, &mut findings);
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }

    fn append_per_row_findings(
        &self,
        row: &CertificationRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                format!("row {} identity or timestamp is empty", row.row_id),
            ));
        }
        if !row.raw_source_material_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::RawSourceMaterialPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits raw source bodies past the boundary",
                    row.row_id
                ),
            ));
        }
        if !row.secrets_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::SecretsPresent,
                FindingSeverity::Blocker,
                format!("row {} admits secrets past the boundary", row.row_id),
            ));
        }
        if !row.ambient_authority_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::AmbientAuthorityPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits ambient authority/credentials past the boundary",
                    row.row_id
                ),
            ));
        }

        if !row.support_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSupportClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound support class", row.row_id),
            ));
        }
        if !row.known_limit_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingKnownLimit,
                FindingSeverity::Blocker,
                format!("row {} has no bound known-limit class", row.row_id),
            ));
        }
        if !row.downgrade_automation_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDowngradeAutomation,
                FindingSeverity::Blocker,
                format!("row {} has no bound downgrade-automation class", row.row_id),
            ));
        }
        if !row.evidence_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound evidence class", row.row_id),
            ));
        }
        if row.row_class.requires_provider_family() && !row.provider_family_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingProviderFamily,
                FindingSeverity::Blocker,
                format!(
                    "row {} must name a concrete acting provider family",
                    row.row_id
                ),
            ));
        }

        if matches!(row.support_class, SupportClass::Certified) && !row.all_bindings_satisfied() {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified while a binding (support, provider family, known limit, downgrade automation, or evidence) is unbound",
                    row.row_id
                ),
            ));
        }

        if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::NarrowedRowMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} has support class {} without a disclosure ref",
                    row.row_id,
                    row.support_class.as_str()
                ),
            ));
        }
        if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::KnownLimitMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} discloses known limit {} without a disclosure ref",
                    row.row_id,
                    row.known_limit_class.as_str()
                ),
            ));
        }
        if row
            .downgrade_automation_class
            .requires_explicit_disclosure()
            && row.disclosure_ref.is_none()
        {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeAutomationMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} binds downgrade automation {} without a disclosure ref",
                    row.row_id,
                    row.downgrade_automation_class.as_str()
                ),
            ));
        }

        if row.evidence_refs.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceRefs,
                FindingSeverity::Blocker,
                format!("row {} carries no evidence refs", row.row_id),
            ));
        }

        self.append_dimension_gating_findings(row, findings);

        if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
            && matches!(row.support_class, SupportClass::Certified)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Warning,
                format!(
                    "row {} claims certified at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn append_dimension_gating_findings(
        &self,
        row: &CertificationRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let is_lane = matches!(row.row_class, CertificationRowClass::LaneCertification);
        let is_arbitration = matches!(
            row.row_class,
            CertificationRowClass::ProviderArbitrationCertification
        );
        let is_convergence = matches!(
            row.row_class,
            CertificationRowClass::DiagnosticConvergenceCertification
        );
        let is_refactor = matches!(
            row.row_class,
            CertificationRowClass::RefactorPreviewCertification
        );
        let is_rollback = matches!(
            row.row_class,
            CertificationRowClass::RollbackDeterminismCertification
        );
        let is_generated = matches!(
            row.row_class,
            CertificationRowClass::GeneratedArtifactPolicyCertification
        );
        let is_drill = matches!(row.row_class, CertificationRowClass::EvidenceDrillAdmission);

        // Verdict dimension — owned by lane_certification.
        if is_lane && !row.verdict_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingVerdictClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound certification verdict", row.row_id),
            ));
        }
        if !is_lane && !row.verdict_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::VerdictNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds verdict {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.verdict_class.as_str()
                ),
            ));
        }
        if is_lane
            && matches!(row.support_class, SupportClass::Certified)
            && row.verdict_class.contradicts_certified_support()
        {
            findings.push(ValidationFinding::new(
                FindingKind::VerdictSupportMismatch,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified support but verdict {} blocks or withdraws the lane",
                    row.row_id,
                    row.verdict_class.as_str()
                ),
            ));
        }

        // Arbitration proof + conflict dimension — owned by provider_arbitration_certification.
        if is_arbitration && !row.arbitration_proof_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingArbitrationProofClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound arbitration proof", row.row_id),
            ));
        }
        if !is_arbitration && !row.arbitration_proof_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ArbitrationProofNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds arbitration proof {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.arbitration_proof_class.as_str()
                ),
            ));
        }
        if is_arbitration && !row.conflict_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingConflictClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound conflict class", row.row_id),
            ));
        }
        if !is_arbitration && !row.conflict_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ConflictNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds conflict class {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.conflict_class.as_str()
                ),
            ));
        }
        // Disagreement must stay inspectable, never collapsed to ranking-only.
        if is_arbitration
            && matches!(
                row.conflict_class,
                ConflictClass::ArbitratedWinnerLoserPreserved
                    | ConflictClass::UnresolvedDisagreementSurfaced
            )
            && !row.disagreement_inspectable
        {
            findings.push(ValidationFinding::new(
                FindingKind::DisagreementCollapsedToRankingOnly,
                FindingSeverity::Blocker,
                format!(
                    "row {} certifies provider disagreement but collapses it to a ranking-only result",
                    row.row_id
                ),
            ));
        }

        // Convergence proof dimension — owned by diagnostic_convergence_certification.
        if is_convergence && !row.convergence_proof_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingConvergenceProofClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound convergence proof", row.row_id),
            ));
        }
        if !is_convergence && !row.convergence_proof_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ConvergenceProofNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds convergence proof {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.convergence_proof_class.as_str()
                ),
            ));
        }

        // Refactor transaction + completeness dimension — owned by refactor_preview_certification.
        if is_refactor && !row.refactor_transaction_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRefactorTransactionClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound refactor-transaction class", row.row_id),
            ));
        }
        if !is_refactor && !row.refactor_transaction_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::RefactorTransactionNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds refactor class {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.refactor_transaction_class.as_str()
                ),
            ));
        }
        if is_refactor && !row.completeness_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingCompletenessClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound completeness class", row.row_id),
            ));
        }
        if !is_refactor && !row.completeness_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::CompletenessNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds completeness {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.completeness_class.as_str()
                ),
            ));
        }
        // A mutating refactor must not certify behind an unsafe or unlabeled preview.
        if is_refactor
            && row.refactor_transaction_class.is_concrete()
            && row.refactor_transaction_class.is_mutating()
            && matches!(
                row.completeness_class,
                CompletenessClass::Unsupported | CompletenessClass::CompletenessUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::MutationBypassesPreviewOrRollback,
                FindingSeverity::Blocker,
                format!(
                    "row {} certifies mutating refactor {} without a typed, labeled preview completeness",
                    row.row_id,
                    row.refactor_transaction_class.as_str()
                ),
            ));
        }

        // Rollback path + determinism dimension — owned by rollback_determinism_certification.
        if is_rollback && !row.rollback_path_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackPathClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound rollback path class", row.row_id),
            ));
        }
        if !is_rollback && !row.rollback_path_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds rollback path {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.rollback_path_class.as_str()
                ),
            ));
        }
        if is_rollback && !row.rollback_determinism_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackDeterminismClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has no bound rollback-determinism outcome",
                    row.row_id
                ),
            ));
        }
        if !is_rollback && !row.rollback_determinism_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackDeterminismNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds rollback determinism {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.rollback_determinism_class.as_str()
                ),
            ));
        }
        // A certified rollback row that names a mutating-capable rollback path must
        // prove determinism, never a nondeterministic/unsafe rollback.
        if is_rollback
            && row.rollback_determinism_class.is_concrete()
            && matches!(
                row.rollback_determinism_class,
                RollbackDeterminismClass::NondeterministicUnsafe
            )
            && matches!(row.support_class, SupportClass::Certified)
        {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackDeterminismNotProven,
                FindingSeverity::Blocker,
                format!(
                    "row {} certifies rollback while determinism is nondeterministic/unsafe",
                    row.row_id
                ),
            ));
        }

        // Generated artifact policy dimension — owned by generated_artifact_policy_certification.
        if is_generated && !row.generated_artifact_policy_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingGeneratedArtifactPolicyClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound generated-artifact policy", row.row_id),
            ));
        }
        if !is_generated && !row.generated_artifact_policy_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::GeneratedArtifactPolicyNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds generated-artifact policy {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.generated_artifact_policy_class.as_str()
                ),
            ));
        }

        // Evidence drill dimension — owned by evidence_drill_admission.
        if is_drill && !row.evidence_drill_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceDrillClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound evidence drill", row.row_id),
            ));
        }
        if !is_drill && !row.evidence_drill_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::EvidenceDrillNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds evidence drill {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.evidence_drill_class.as_str()
                ),
            ));
        }
    }

    fn append_per_lane_coverage_findings(
        &self,
        lane: ArtifactFamilyLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_certified = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(row.row_class, CertificationRowClass::LaneCertification)
                && matches!(row.support_class, SupportClass::Certified)
        });
        if !lane_claims_certified {
            return;
        }

        let required: [(CertificationRowClass, FindingKind, &str); 6] = [
            (
                CertificationRowClass::ProviderArbitrationCertification,
                FindingKind::MissingProviderArbitrationCoverage,
                "provider_arbitration_certification",
            ),
            (
                CertificationRowClass::DiagnosticConvergenceCertification,
                FindingKind::MissingDiagnosticConvergenceCoverage,
                "diagnostic_convergence_certification",
            ),
            (
                CertificationRowClass::RefactorPreviewCertification,
                FindingKind::MissingRefactorPreviewCoverage,
                "refactor_preview_certification",
            ),
            (
                CertificationRowClass::RollbackDeterminismCertification,
                FindingKind::MissingRollbackDeterminismCoverage,
                "rollback_determinism_certification",
            ),
            (
                CertificationRowClass::GeneratedArtifactPolicyCertification,
                FindingKind::MissingGeneratedArtifactPolicyCoverage,
                "generated_artifact_policy_certification",
            ),
            (
                CertificationRowClass::EvidenceDrillAdmission,
                FindingKind::MissingEvidenceDrillCoverage,
                "evidence_drill_admission",
            ),
        ];

        for (row_class, finding_kind, label) in required {
            let covered = self
                .rows
                .iter()
                .any(|row| row.lane_class == lane && row.row_class == row_class);
            if !covered {
                findings.push(ValidationFinding::new(
                    finding_kind,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims certified but has no {} row",
                        lane.as_str(),
                        label
                    ),
                ));
            }
        }
    }

    fn append_required_drill_findings(&self, findings: &mut Vec<ValidationFinding>) {
        if !self.has_certified_lane() {
            return;
        }
        let observed: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| {
                matches!(row.row_class, CertificationRowClass::EvidenceDrillAdmission)
                    && row.evidence_drill_class.is_concrete()
            })
            .map(|row| row.evidence_drill_class.as_str())
            .collect();
        for drill in EvidenceDrillClass::REQUIRED {
            if !observed.contains(drill.as_str()) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRequiredEvidenceDrill,
                    FindingSeverity::Blocker,
                    format!(
                        "certified packet is missing the required {} evidence drill",
                        drill.as_str()
                    ),
                ));
            }
        }
    }

    fn append_projection_findings(
        &self,
        projection: &CertificationConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve certification truth",
                    projection.projection_ref
                ),
            ));
        }
        let collapses: [(bool, FindingKind, &str); 17] = [
            (
                projection.preserves_lane_vocabulary,
                FindingKind::LaneVocabularyCollapsed,
                "lane",
            ),
            (
                projection.preserves_row_class_vocabulary,
                FindingKind::RowClassVocabularyCollapsed,
                "row-class",
            ),
            (
                projection.preserves_support_class_vocabulary,
                FindingKind::SupportClassVocabularyCollapsed,
                "support-class",
            ),
            (
                projection.preserves_provider_family_vocabulary,
                FindingKind::ProviderFamilyVocabularyCollapsed,
                "provider-family",
            ),
            (
                projection.preserves_verdict_vocabulary,
                FindingKind::VerdictVocabularyCollapsed,
                "verdict",
            ),
            (
                projection.preserves_arbitration_proof_vocabulary,
                FindingKind::ArbitrationProofVocabularyCollapsed,
                "arbitration-proof",
            ),
            (
                projection.preserves_conflict_vocabulary,
                FindingKind::ConflictVocabularyCollapsed,
                "conflict",
            ),
            (
                projection.preserves_convergence_proof_vocabulary,
                FindingKind::ConvergenceProofVocabularyCollapsed,
                "convergence-proof",
            ),
            (
                projection.preserves_refactor_transaction_vocabulary,
                FindingKind::RefactorTransactionVocabularyCollapsed,
                "refactor-transaction",
            ),
            (
                projection.preserves_completeness_vocabulary,
                FindingKind::CompletenessVocabularyCollapsed,
                "completeness",
            ),
            (
                projection.preserves_rollback_path_vocabulary,
                FindingKind::RollbackPathVocabularyCollapsed,
                "rollback-path",
            ),
            (
                projection.preserves_rollback_determinism_vocabulary,
                FindingKind::RollbackDeterminismVocabularyCollapsed,
                "rollback-determinism",
            ),
            (
                projection.preserves_generated_artifact_policy_vocabulary,
                FindingKind::GeneratedArtifactPolicyVocabularyCollapsed,
                "generated-artifact-policy",
            ),
            (
                projection.preserves_evidence_drill_vocabulary,
                FindingKind::EvidenceDrillVocabularyCollapsed,
                "evidence-drill",
            ),
            (
                projection.preserves_known_limit_vocabulary,
                FindingKind::KnownLimitVocabularyCollapsed,
                "known-limit",
            ),
            (
                projection.preserves_downgrade_automation_vocabulary,
                FindingKind::DowngradeAutomationVocabularyCollapsed,
                "downgrade-automation",
            ),
            (
                projection.preserves_evidence_class_vocabulary,
                FindingKind::EvidenceClassVocabularyCollapsed,
                "evidence-class",
            ),
        ];
        for (preserved, finding_kind, label) in collapses {
            if !preserved {
                findings.push(ValidationFinding::new(
                    finding_kind,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the {} vocabulary",
                        projection.projection_ref, label
                    ),
                ));
            }
        }
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRefactorCertificationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub certification_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub certification_packet: ProviderRefactorCertificationTruthPacket,
}

impl ProviderRefactorCertificationTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == PROVIDER_REFACTOR_CERTIFICATION_TRUTH_SCHEMA_VERSION
            && self.certification_packet_id_ref == self.certification_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.certification_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable certification packet.
#[derive(Debug)]
pub enum ProviderRefactorCertificationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for ProviderRefactorCertificationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "certification packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "certification packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ProviderRefactorCertificationTruthArtifactError {}

/// Returns the checked-in stable provider/refactor certification truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_provider_refactor_certification_truth_packet(
) -> Result<ProviderRefactorCertificationTruthPacket, ProviderRefactorCertificationTruthArtifactError>
{
    let packet: ProviderRefactorCertificationTruthPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/language/m5/provider_refactor_certification_truth_packet.json"
        )))
        .map_err(ProviderRefactorCertificationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderRefactorCertificationTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests;
