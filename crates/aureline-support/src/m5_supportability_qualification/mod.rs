//! M5 Support Center, environment-explainability, consent, crash-intake, and
//! handoff qualification.
//!
//! This module composes the already-landed M5 supportability surfaces into one
//! qualification index that shiproom, Help/About, desktop, CLI/headless, and
//! support-export consumers can ingest verbatim. It mints no new supportability
//! behavior; it binds the Support Center matrix, environment-explainability,
//! precedence-inspection, support-bundle consent, crash-intake/recovery, and
//! supportability-handoff lanes into one shared decision surface and proves that
//! no claimed supportability surface can stay green while its own evidence or a
//! bound drill is stale, missing, or downgraded.
//!
//! The packet answers, for every claimed supportability surface on every claimed
//! M5 profile:
//!
//! - which surface is being qualified and which deployment modes (desktop,
//!   CLI/headless) it must cover;
//! - which lane schema, canonical artifact, fixture corpus, and record kind back
//!   the surface — its *own* proof, never an adjacent surface's;
//! - which supportability drills (diagnosis latency, consent-sheet accuracy,
//!   export-mode parity, crash-loop recovery, exact-build intake) back the row;
//! - whether the surface offers a share path and whether local-save stays
//!   first-class beside any team-share or formal-support send; and
//! - the published qualification state plus the stale-proof tokens and
//!   downgrade-rule ids that explain any narrowing.
//!
//! Rows that lose their own surface evidence or a bound drill narrow
//! automatically instead of inheriting a greener neighboring claim, and a
//! send-capable surface may never publish a row that demotes local-save below a
//! send path.

use serde::{Deserialize, Serialize};

use crate::m5_crash_intake_and_recovery::{
    M5_CRASH_INTAKE_RECOVERY_FIXTURE_DIR, M5_CRASH_INTAKE_RECOVERY_PATH,
    M5_CRASH_INTAKE_RECOVERY_RECORD_KIND, M5_CRASH_INTAKE_RECOVERY_SCHEMA_REF,
};
use crate::m5_fault_crash_certification::ClaimedM5Profile;
use crate::m5_precedence_inspector::{
    M5_PRECEDENCE_INSPECTOR_FIXTURE_DIR, M5_PRECEDENCE_INSPECTOR_PATH,
    M5_PRECEDENCE_INSPECTOR_RECORD_KIND, M5_PRECEDENCE_INSPECTOR_SCHEMA_REF,
};
use crate::m5_support_bundle_consent::{
    M5_SUPPORT_BUNDLE_CONSENT_FIXTURE_DIR, M5_SUPPORT_BUNDLE_CONSENT_PATH,
    M5_SUPPORT_BUNDLE_CONSENT_RECORD_KIND, M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_REF,
};
use crate::m5_support_center_matrix::{
    M5_SUPPORT_CENTER_MATRIX_ARTIFACT_DOC_REF, M5_SUPPORT_CENTER_MATRIX_FIXTURE_DIR,
    M5_SUPPORT_CENTER_MATRIX_PATH, M5_SUPPORT_CENTER_MATRIX_RECORD_KIND,
    M5_SUPPORT_CENTER_MATRIX_SCHEMA_REF,
};
use crate::m5_supportability_handoff_packets::{
    M5_SUPPORTABILITY_HANDOFF_FIXTURE_DIR, M5_SUPPORTABILITY_HANDOFF_PATH,
    M5_SUPPORTABILITY_HANDOFF_RECORD_KIND, M5_SUPPORTABILITY_HANDOFF_SCHEMA_REF,
};

use aureline_runtime::m5_environment_status_strips::{
    M5_ENVIRONMENT_STATUS_STRIP_FIXTURE_DIR, M5_ENVIRONMENT_STATUS_STRIP_PATH,
    M5_ENVIRONMENT_STATUS_STRIP_RECORD_KIND, M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_REF,
};

// Supportability-drill evidence each drill already checks in. The qualification
// cites the same evidence the drills regenerate so a stale or deleted drill
// fixture is visible from the qualification, not just inside the owning lane.
const DIAGNOSIS_LATENCY_SCOREBOARD_REF: &str =
    "artifacts/support/diagnosis_latency_scoreboard.yaml";
const DIAGNOSIS_SLO_TARGETS_REF: &str = "artifacts/support/diagnosis_slo_targets.yaml";
const REDACTION_ACCURACY_CHECKS_REF: &str = "artifacts/support/redaction_accuracy_checks.yaml";
const RECOVERY_LADDER_CASES_REF: &str = "artifacts/support/recovery_ladder_cases.yaml";

// Checked consumer surfaces that must ingest the qualification index verbatim.
const SUPPORT_EXPORT_CONSUMER_REF: &str = "schemas/support/support_bundle_manifest.schema.json";
const RELEASE_MANIFEST_CONSUMER_REF: &str =
    "artifacts/release/stable/claim-publication-manifest/manifest.json";

const REQUIRED_PROJECTION_FIELDS: &[&str] = &[
    "qualification_row_id",
    "surface",
    "profile",
    "published_state",
    "deployment_mode_coverage",
    "stale_proof_tokens",
    "downgrade_rule_ids",
];

/// Stable record-kind tag carried by [`M5SupportabilityQualificationPacket`].
pub const M5_SUPPORTABILITY_QUALIFICATION_PACKET_RECORD_KIND: &str =
    "m5_supportability_qualification_packet";

/// Frozen schema version for the M5 supportability qualification packet.
pub const M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema.
pub const M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/support/m5-supportability-qualification.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const M5_SUPPORTABILITY_QUALIFICATION_DOC_REF: &str =
    "docs/help/support/m5-supportability-qualification.md";

/// Repository-relative path of the checked review artifact.
pub const M5_SUPPORTABILITY_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/support/m5/m5-supportability-qualification.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_SUPPORTABILITY_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/support/m5/m5-supportability-qualification";

/// Repository-relative path of the shiproom-facing claim packet derived from
/// this qualification index.
pub const M5_SUPPORTABILITY_QUALIFICATION_CLAIM_PACKET_REF: &str =
    "artifacts/shiproom/m5-supportability-claim-packet/m5_supportability_claim_packet.md";

/// Stable packet identifier reused by every surface binding.
pub const M5_SUPPORTABILITY_QUALIFICATION_PACKET_ID: &str =
    "support.m5.supportability_qualification.v1";

/// One claimed supportability surface the qualification certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportabilitySurfaceClass {
    /// The Support Center IA and module matrix.
    SupportCenter,
    /// Per-surface environment status strips and "why this execution context?".
    EnvironmentExplainability,
    /// Resolver precedence inspectors.
    PrecedenceInspection,
    /// Support-bundle consent sheets.
    SupportBundleConsent,
    /// Crash-loop recovery screens and issue-report / crash-intake.
    CrashIntakeRecovery,
    /// The supportability handoff packet family and its modes.
    SupportabilityHandoff,
}

impl SupportabilitySurfaceClass {
    /// All claimed supportability surfaces in canonical order.
    pub const ALL: [Self; 6] = [
        Self::SupportCenter,
        Self::EnvironmentExplainability,
        Self::PrecedenceInspection,
        Self::SupportBundleConsent,
        Self::CrashIntakeRecovery,
        Self::SupportabilityHandoff,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportCenter => "support_center",
            Self::EnvironmentExplainability => "environment_explainability",
            Self::PrecedenceInspection => "precedence_inspection",
            Self::SupportBundleConsent => "support_bundle_consent",
            Self::CrashIntakeRecovery => "crash_intake_recovery",
            Self::SupportabilityHandoff => "supportability_handoff",
        }
    }

    /// Returns a review-safe label for the surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SupportCenter => "Support Center",
            Self::EnvironmentExplainability => "Environment explainability",
            Self::PrecedenceInspection => "Precedence inspection",
            Self::SupportBundleConsent => "Support-bundle consent",
            Self::CrashIntakeRecovery => "Crash intake and recovery",
            Self::SupportabilityHandoff => "Supportability handoff",
        }
    }

    /// True when the surface offers a team-share or formal-support send path and
    /// must therefore keep local-save first-class.
    pub const fn share_capable(self) -> bool {
        matches!(
            self,
            Self::SupportBundleConsent | Self::CrashIntakeRecovery | Self::SupportabilityHandoff
        )
    }

    /// Returns the canonical lane refs that back the surface's own proof.
    fn backing_refs(self) -> SurfaceBackingRefs {
        match self {
            Self::SupportCenter => SurfaceBackingRefs {
                schema_ref: M5_SUPPORT_CENTER_MATRIX_SCHEMA_REF,
                artifact_ref: M5_SUPPORT_CENTER_MATRIX_PATH,
                fixture_ref: M5_SUPPORT_CENTER_MATRIX_FIXTURE_DIR,
                record_kind: M5_SUPPORT_CENTER_MATRIX_RECORD_KIND,
            },
            Self::EnvironmentExplainability => SurfaceBackingRefs {
                schema_ref: M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_REF,
                artifact_ref: M5_ENVIRONMENT_STATUS_STRIP_PATH,
                fixture_ref: M5_ENVIRONMENT_STATUS_STRIP_FIXTURE_DIR,
                record_kind: M5_ENVIRONMENT_STATUS_STRIP_RECORD_KIND,
            },
            Self::PrecedenceInspection => SurfaceBackingRefs {
                schema_ref: M5_PRECEDENCE_INSPECTOR_SCHEMA_REF,
                artifact_ref: M5_PRECEDENCE_INSPECTOR_PATH,
                fixture_ref: M5_PRECEDENCE_INSPECTOR_FIXTURE_DIR,
                record_kind: M5_PRECEDENCE_INSPECTOR_RECORD_KIND,
            },
            Self::SupportBundleConsent => SurfaceBackingRefs {
                schema_ref: M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_REF,
                artifact_ref: M5_SUPPORT_BUNDLE_CONSENT_PATH,
                fixture_ref: M5_SUPPORT_BUNDLE_CONSENT_FIXTURE_DIR,
                record_kind: M5_SUPPORT_BUNDLE_CONSENT_RECORD_KIND,
            },
            Self::CrashIntakeRecovery => SurfaceBackingRefs {
                schema_ref: M5_CRASH_INTAKE_RECOVERY_SCHEMA_REF,
                artifact_ref: M5_CRASH_INTAKE_RECOVERY_PATH,
                fixture_ref: M5_CRASH_INTAKE_RECOVERY_FIXTURE_DIR,
                record_kind: M5_CRASH_INTAKE_RECOVERY_RECORD_KIND,
            },
            Self::SupportabilityHandoff => SurfaceBackingRefs {
                schema_ref: M5_SUPPORTABILITY_HANDOFF_SCHEMA_REF,
                artifact_ref: M5_SUPPORTABILITY_HANDOFF_PATH,
                fixture_ref: M5_SUPPORTABILITY_HANDOFF_FIXTURE_DIR,
                record_kind: M5_SUPPORTABILITY_HANDOFF_RECORD_KIND,
            },
        }
    }

    /// Returns the supportability drills that back the surface, in canonical
    /// order.
    fn drills(self) -> &'static [SupportabilityDrillClass] {
        match self {
            Self::SupportCenter => &[SupportabilityDrillClass::DiagnosisLatency],
            Self::EnvironmentExplainability | Self::PrecedenceInspection => &[],
            Self::SupportBundleConsent => &[
                SupportabilityDrillClass::ConsentSheetAccuracy,
                SupportabilityDrillClass::ExportModeParity,
            ],
            Self::CrashIntakeRecovery => &[
                SupportabilityDrillClass::CrashLoopRecovery,
                SupportabilityDrillClass::ExactBuildIntake,
                SupportabilityDrillClass::ExportModeParity,
            ],
            Self::SupportabilityHandoff => &[
                SupportabilityDrillClass::ConsentSheetAccuracy,
                SupportabilityDrillClass::ExactBuildIntake,
                SupportabilityDrillClass::ExportModeParity,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SurfaceBackingRefs {
    schema_ref: &'static str,
    artifact_ref: &'static str,
    fixture_ref: &'static str,
    record_kind: &'static str,
}

/// Deployment mode a supportability surface must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentMode {
    /// The desktop shell.
    Desktop,
    /// The CLI / headless runner.
    CliHeadless,
}

impl DeploymentMode {
    /// All deployment modes in canonical order.
    pub const ALL: [Self; 2] = [Self::Desktop, Self::CliHeadless];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// One supportability drill bound into the qualification corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportabilityDrillClass {
    /// Project Doctor diagnosis latency against the SLO targets.
    DiagnosisLatency,
    /// Consent-sheet accuracy: included / excluded / policy-locked counts match.
    ConsentSheetAccuracy,
    /// Local-save / team-share / formal-support export-mode parity.
    ExportModeParity,
    /// Crash-loop recovery routing without a dead end.
    CrashLoopRecovery,
    /// Exact-build-aware crash intake fidelity.
    ExactBuildIntake,
}

impl SupportabilityDrillClass {
    /// All supportability drills in canonical order.
    pub const ALL: [Self; 5] = [
        Self::DiagnosisLatency,
        Self::ConsentSheetAccuracy,
        Self::ExportModeParity,
        Self::CrashLoopRecovery,
        Self::ExactBuildIntake,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosisLatency => "diagnosis_latency",
            Self::ConsentSheetAccuracy => "consent_sheet_accuracy",
            Self::ExportModeParity => "export_mode_parity",
            Self::CrashLoopRecovery => "crash_loop_recovery",
            Self::ExactBuildIntake => "exact_build_intake",
        }
    }

    /// Stable drill identifier reused by row bindings.
    fn drill_id(self) -> String {
        format!("drill:{}", self.as_str())
    }

    /// Returns a review-safe label.
    fn label(self) -> &'static str {
        match self {
            Self::DiagnosisLatency => "Diagnosis latency",
            Self::ConsentSheetAccuracy => "Consent-sheet accuracy",
            Self::ExportModeParity => "Export-mode parity",
            Self::CrashLoopRecovery => "Crash-loop recovery",
            Self::ExactBuildIntake => "Exact-build intake",
        }
    }

    /// Surfaces this drill covers, derived from each surface's drill set.
    fn covered_surfaces(self) -> Vec<SupportabilitySurfaceClass> {
        SupportabilitySurfaceClass::ALL
            .into_iter()
            .filter(|surface| surface.drills().contains(&self))
            .collect()
    }

    /// Checked evidence refs the drill regenerates.
    fn evidence_refs(self) -> Vec<String> {
        match self {
            Self::DiagnosisLatency => vec![
                DIAGNOSIS_LATENCY_SCOREBOARD_REF.to_owned(),
                DIAGNOSIS_SLO_TARGETS_REF.to_owned(),
            ],
            Self::ConsentSheetAccuracy => vec![
                M5_SUPPORT_BUNDLE_CONSENT_PATH.to_owned(),
                REDACTION_ACCURACY_CHECKS_REF.to_owned(),
            ],
            Self::ExportModeParity => vec![
                M5_SUPPORT_BUNDLE_CONSENT_PATH.to_owned(),
                M5_SUPPORTABILITY_HANDOFF_PATH.to_owned(),
            ],
            Self::CrashLoopRecovery => vec![
                M5_CRASH_INTAKE_RECOVERY_PATH.to_owned(),
                RECOVERY_LADDER_CASES_REF.to_owned(),
            ],
            Self::ExactBuildIntake => vec![
                M5_CRASH_INTAKE_RECOVERY_PATH.to_owned(),
                M5_SUPPORTABILITY_HANDOFF_PATH.to_owned(),
            ],
        }
    }
}

/// Qualification result published for one surface/profile row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationStateClass {
    /// The surface and all its bound drills are current on the claimed profile.
    Qualified,
    /// The surface keeps a narrower, profile-scoped claim only.
    LimitedProfileScoped,
    /// Only the local-save / self-diagnosis path may be claimed; any send path
    /// is unverified pending fresh proof.
    LocalSelfDiagnosisOnly,
    /// The broad surface claim is blocked pending fresh proof.
    BlockedUnverified,
}

impl QualificationStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::LimitedProfileScoped => "limited_profile_scoped",
            Self::LocalSelfDiagnosisOnly => "local_self_diagnosis_only",
            Self::BlockedUnverified => "blocked_unverified",
        }
    }
}

/// Downgrade trigger automated by the qualification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDowngradeTriggerClass {
    /// The surface's own lane evidence (schema, artifact, or fixture) is stale or
    /// missing.
    SurfaceEvidenceStale,
    /// A bound supportability drill is stale or missing.
    SupportabilityDrillStale,
    /// A deployment mode the surface must cover dropped out of parity.
    DeploymentModeParityLost,
    /// A profile policy-blocks a send path; only local-save / self-diagnosis may
    /// be claimed.
    PolicyBlockedSend,
    /// One downstream surface stopped ingesting the qualification by reference.
    ConsumerBindingMissing,
}

impl QualificationDowngradeTriggerClass {
    /// All downgrade triggers in canonical order.
    pub const ALL: [Self; 5] = [
        Self::SurfaceEvidenceStale,
        Self::SupportabilityDrillStale,
        Self::DeploymentModeParityLost,
        Self::PolicyBlockedSend,
        Self::ConsumerBindingMissing,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceEvidenceStale => "surface_evidence_stale",
            Self::SupportabilityDrillStale => "supportability_drill_stale",
            Self::DeploymentModeParityLost => "deployment_mode_parity_lost",
            Self::PolicyBlockedSend => "policy_blocked_send",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
        }
    }
}

/// Stable consumer surface that ingests the qualification result verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationConsumerClass {
    /// Help/About supportability cards.
    HelpAbout,
    /// The desktop Support Center.
    SupportCenterDesktop,
    /// CLI / headless support output.
    CliHeadless,
    /// Support-export and handoff surfaces.
    SupportExport,
    /// Shiproom claim and operational-readiness packets.
    Shiproom,
    /// Release manifest and publication control surfaces.
    ReleaseManifest,
}

impl QualificationConsumerClass {
    /// All consumer surfaces in canonical order.
    pub const ALL: [Self; 6] = [
        Self::HelpAbout,
        Self::SupportCenterDesktop,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Shiproom,
        Self::ReleaseManifest,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::SupportCenterDesktop => "support_center_desktop",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Shiproom => "shiproom",
            Self::ReleaseManifest => "release_manifest",
        }
    }
}

/// One surface/profile qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceProfileQualificationRow {
    /// Stable row identifier.
    pub qualification_row_id: String,
    /// Supportability surface covered by the row.
    pub surface: SupportabilitySurfaceClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Claimed M5 profile covered by the row.
    pub profile: ClaimedM5Profile,
    /// Published qualification state for the row.
    pub published_state: QualificationStateClass,
    /// Deployment modes the surface covers on this profile.
    pub deployment_mode_coverage: Vec<DeploymentMode>,
    /// True when the surface offers a team-share or formal-support send path.
    pub share_capable: bool,
    /// True when local-save / self-diagnosis stays first-class beside any send.
    pub local_save_first: bool,
    /// Surface lane boundary schema backing the row's own proof.
    pub backing_schema_ref: String,
    /// Surface lane canonical artifact backing the row's own proof.
    pub backing_artifact_ref: String,
    /// Surface lane fixture corpus backing the row's own proof.
    pub backing_fixture_ref: String,
    /// Surface lane record kind backing the row's own proof.
    pub backing_record_kind: String,
    /// Supportability drill ids backing the row.
    pub drill_ids: Vec<String>,
    /// Active stale or capability-loss tokens narrowing the row.
    pub stale_proof_tokens: Vec<String>,
    /// Active downgrade-rule identifiers explaining the published state.
    pub downgrade_rule_ids: Vec<String>,
    /// Review-safe summary for downstream surfaces.
    pub summary: String,
}

/// One supportability drill bound into the qualification corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportabilityDrillRow {
    /// Stable drill identifier.
    pub drill_id: String,
    /// Drill class.
    pub drill_class: SupportabilityDrillClass,
    /// Human-readable drill label.
    pub label: String,
    /// Surfaces this drill backs.
    pub covered_surfaces: Vec<SupportabilitySurfaceClass>,
    /// Checked evidence refs the drill regenerates.
    pub evidence_refs: Vec<String>,
    /// True when the drill evidence is current.
    pub is_fresh: bool,
    /// Review-safe summary.
    pub summary: String,
}

/// One downgrade rule published by the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationDowngradeRuleRow {
    /// Stable rule identifier.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger_class: QualificationDowngradeTriggerClass,
    /// Source qualification state before the downgrade.
    pub source_state: QualificationStateClass,
    /// Resulting qualification state after the downgrade.
    pub downgraded_state: QualificationStateClass,
    /// User-visible effect of the downgrade.
    pub required_effect: String,
    /// Reviewable rationale for the downgrade.
    pub rationale: String,
    /// Supporting evidence or contract refs used to inspect the rule.
    pub evidence_refs: Vec<String>,
}

/// One consumer-surface binding proving the same qualification result is reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationConsumerBinding {
    /// Consumer surface that ingests the qualification.
    pub consumer: QualificationConsumerClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// Number of qualification rows the consumer exposes by reference.
    pub qualification_row_count: usize,
    /// Fields the consumer must preserve verbatim from the packet.
    pub required_verbatim_fields: Vec<String>,
    /// True when the consumer narrows immediately on stale proof or blocked rows.
    pub narrow_on_stale_proof: bool,
    /// True when limited or local-only states stay labeled explicitly.
    pub explicit_limited_state_labels_required: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

/// One validation error returned by
/// [`M5SupportabilityQualificationPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportabilityQualificationViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 supportability qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportabilityQualificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing contract document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing packets and contracts this qualification composes.
    pub supporting_contract_refs: Vec<String>,
    /// Claimed M5 profiles covered by the packet.
    pub claimed_profiles: Vec<ClaimedM5Profile>,
    /// Deployment modes every surface must cover.
    pub deployment_modes: Vec<DeploymentMode>,
    /// Canonical surface/profile qualification rows.
    pub qualification_rows: Vec<SurfaceProfileQualificationRow>,
    /// Supportability drills bound into the corpus.
    pub drill_catalog: Vec<SupportabilityDrillRow>,
    /// Automatic downgrade rules used by the packet.
    pub downgrade_rules: Vec<QualificationDowngradeRuleRow>,
    /// Consumer-surface bindings that prove one qualification index is reused.
    pub consumer_bindings: Vec<QualificationConsumerBinding>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5SupportabilityQualificationPacket {
    /// Validates profile/surface coverage, drill binding, downgrade automation,
    /// own-proof consistency, and shared-surface bindings.
    pub fn validate(&self) -> Vec<M5SupportabilityQualificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUPPORTABILITY_QUALIFICATION_PACKET_RECORD_KIND {
            push(&mut violations, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_VERSION {
            push(
                &mut violations,
                "schema_version",
                "unexpected schema_version",
            );
        }
        if self.packet_id != M5_SUPPORTABILITY_QUALIFICATION_PACKET_ID {
            push(&mut violations, "packet_id", "unexpected packet_id");
        }
        if self.doc_ref != M5_SUPPORTABILITY_QUALIFICATION_DOC_REF {
            push(
                &mut violations,
                "doc_ref",
                "packet must quote the canonical reviewer doc",
            );
        }
        if self.schema_ref != M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_REF {
            push(
                &mut violations,
                "schema_ref",
                "packet must quote the canonical schema ref",
            );
        }
        if self.artifact_ref != M5_SUPPORTABILITY_QUALIFICATION_ARTIFACT_REF {
            push(
                &mut violations,
                "artifact_ref",
                "packet must quote the checked review artifact ref",
            );
        }
        if self.supporting_contract_refs.is_empty() {
            push(
                &mut violations,
                "supporting_contract_refs",
                "packet must cite the composed lane contracts",
            );
        }

        for required in ClaimedM5Profile::ALL {
            if !self.claimed_profiles.contains(&required) {
                push(
                    &mut violations,
                    "claimed_profiles",
                    &format!("missing claimed profile {}", required.as_str()),
                );
            }
        }
        for required in DeploymentMode::ALL {
            if !self.deployment_modes.contains(&required) {
                push(
                    &mut violations,
                    "deployment_modes",
                    &format!("missing deployment mode {}", required.as_str()),
                );
            }
        }

        for surface in SupportabilitySurfaceClass::ALL {
            for profile in ClaimedM5Profile::ALL {
                if !self
                    .qualification_rows
                    .iter()
                    .any(|row| row.surface == surface && row.profile == profile)
                {
                    push(
                        &mut violations,
                        "qualification_rows",
                        &format!(
                            "missing qualification row for surface {} on profile {}",
                            surface.as_str(),
                            profile.as_str()
                        ),
                    );
                }
            }
        }

        let rule_ids: Vec<&str> = self
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();
        let drill_ids: Vec<&str> = self
            .drill_catalog
            .iter()
            .map(|drill| drill.drill_id.as_str())
            .collect();

        for row in &self.qualification_rows {
            self.validate_row(&mut violations, row, &rule_ids, &drill_ids);
        }

        self.validate_drill_catalog(&mut violations);

        for required in QualificationDowngradeTriggerClass::ALL {
            if !self
                .downgrade_rules
                .iter()
                .any(|rule| rule.trigger_class == required)
            {
                push(
                    &mut violations,
                    "downgrade_rules",
                    &format!("missing downgrade trigger {}", required.as_str()),
                );
            }
        }
        for rule in &self.downgrade_rules {
            if rule.evidence_refs.is_empty() {
                push(
                    &mut violations,
                    &format!("downgrade_rules.{}", rule.rule_id),
                    "downgrade rule must cite at least one evidence ref",
                );
            }
        }

        for required in QualificationConsumerClass::ALL {
            let Some(binding) = self
                .consumer_bindings
                .iter()
                .find(|binding| binding.consumer == required)
            else {
                push(
                    &mut violations,
                    "consumer_bindings",
                    &format!("missing consumer binding {}", required.as_str()),
                );
                continue;
            };
            let base = format!("consumer_bindings.{}", binding.consumer.as_str());
            if binding.ingested_packet_id != self.packet_id {
                push(
                    &mut violations,
                    &base,
                    "consumer binding must ingest the canonical packet id",
                );
            }
            if binding.qualification_row_count != self.qualification_rows.len() {
                push(
                    &mut violations,
                    &base,
                    "consumer binding row count must match qualification rows",
                );
            }
            if !binding.narrow_on_stale_proof {
                push(
                    &mut violations,
                    &base,
                    "consumer binding must narrow on stale proof",
                );
            }
            for field in REQUIRED_PROJECTION_FIELDS {
                if !binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field)
                {
                    push(
                        &mut violations,
                        &base,
                        &format!("consumer binding must preserve {field}"),
                    );
                }
            }
        }

        violations
    }

    fn validate_row(
        &self,
        violations: &mut Vec<M5SupportabilityQualificationViolation>,
        row: &SurfaceProfileQualificationRow,
        rule_ids: &[&str],
        drill_ids: &[&str],
    ) {
        let base = format!("qualification_rows.{}", row.qualification_row_id);
        for (field, value) in [
            ("surface_label", row.surface_label.as_str()),
            ("backing_schema_ref", row.backing_schema_ref.as_str()),
            ("backing_artifact_ref", row.backing_artifact_ref.as_str()),
            ("backing_fixture_ref", row.backing_fixture_ref.as_str()),
            ("backing_record_kind", row.backing_record_kind.as_str()),
            ("summary", row.summary.as_str()),
        ] {
            if value.trim().is_empty() {
                push(
                    violations,
                    &format!("{base}.{field}"),
                    "row field may not be empty",
                );
            }
        }

        // Own-proof guard: a row may not borrow a neighboring surface's evidence.
        // Each backing ref must match the surface's own canonical lane refs.
        let refs = row.surface.backing_refs();
        if row.backing_schema_ref != refs.schema_ref {
            push(
                violations,
                &format!("{base}.backing_schema_ref"),
                "row must cite its own surface's boundary schema, not an adjacent surface's",
            );
        }
        if row.backing_artifact_ref != refs.artifact_ref {
            push(
                violations,
                &format!("{base}.backing_artifact_ref"),
                "row must cite its own surface's canonical artifact",
            );
        }
        if row.backing_fixture_ref != refs.fixture_ref {
            push(
                violations,
                &format!("{base}.backing_fixture_ref"),
                "row must cite its own surface's fixture corpus",
            );
        }
        if row.backing_record_kind != refs.record_kind {
            push(
                violations,
                &format!("{base}.backing_record_kind"),
                "row must cite its own surface's record kind",
            );
        }
        if row.surface_label != row.surface.label() {
            push(
                violations,
                &format!("{base}.surface_label"),
                "surface_label must match the canonical surface label",
            );
        }

        // Deployment-mode coverage must be a non-empty subset of the declared
        // deployment modes.
        if row.deployment_mode_coverage.is_empty() {
            push(
                violations,
                &format!("{base}.deployment_mode_coverage"),
                "row must cover at least one deployment mode",
            );
        }
        for mode in &row.deployment_mode_coverage {
            if !self.deployment_modes.contains(mode) {
                push(
                    violations,
                    &format!("{base}.deployment_mode_coverage"),
                    &format!("row covers undeclared deployment mode {}", mode.as_str()),
                );
            }
        }

        // Local-save invariant: a send-capable surface may never demote local-save
        // below a send path.
        if row.share_capable != row.surface.share_capable() {
            push(
                violations,
                &format!("{base}.share_capable"),
                "share_capable must match the surface's send capability",
            );
        }
        if row.share_capable && !row.local_save_first {
            push(
                violations,
                &format!("{base}.local_save_first"),
                "a send-capable surface must keep local-save first-class",
            );
        }

        // Drill bindings must reference drills in the catalog and stay consistent
        // with the surface's own drill set.
        let allowed: Vec<SupportabilityDrillClass> = row.surface.drills().to_vec();
        for drill_id in &row.drill_ids {
            if !drill_ids.contains(&drill_id.as_str()) {
                push(
                    violations,
                    &format!("{base}.drill_ids"),
                    &format!("row cites unknown drill {drill_id}"),
                );
                continue;
            }
            if !allowed.iter().any(|drill| &drill.drill_id() == drill_id) {
                push(
                    violations,
                    &format!("{base}.drill_ids"),
                    &format!("row cites drill {drill_id} that does not back its surface"),
                );
            }
        }

        if row.published_state == QualificationStateClass::Qualified
            && !row.stale_proof_tokens.is_empty()
        {
            push(
                violations,
                &format!("{base}.stale_proof_tokens"),
                "qualified rows may not carry stale proof tokens",
            );
        }
        if row.published_state != QualificationStateClass::Qualified
            && row.downgrade_rule_ids.is_empty()
        {
            push(
                violations,
                &format!("{base}.downgrade_rule_ids"),
                "non-qualified rows must cite downgrade rules",
            );
        }
        for rule_id in &row.downgrade_rule_ids {
            if !rule_ids.contains(&rule_id.as_str()) {
                push(
                    violations,
                    &format!("{base}.downgrade_rule_ids"),
                    &format!("row cites unknown downgrade rule {rule_id}"),
                );
            }
        }
    }

    fn validate_drill_catalog(&self, violations: &mut Vec<M5SupportabilityQualificationViolation>) {
        for required in SupportabilityDrillClass::ALL {
            let Some(drill) = self
                .drill_catalog
                .iter()
                .find(|drill| drill.drill_class == required)
            else {
                push(
                    violations,
                    "drill_catalog",
                    &format!("missing supportability drill {}", required.as_str()),
                );
                continue;
            };
            let base = format!("drill_catalog.{}", drill.drill_id);
            if drill.covered_surfaces.is_empty() {
                push(violations, &base, "drill must cover at least one surface");
            }
            if drill.evidence_refs.is_empty() {
                push(
                    violations,
                    &base,
                    "drill must cite at least one evidence ref",
                );
            }
            // The drill's covered surfaces must agree with the per-surface drill
            // sets, so a drill and the rows it backs cannot drift apart.
            for surface in &drill.covered_surfaces {
                if !surface.drills().contains(&required) {
                    push(
                        violations,
                        &base,
                        &format!(
                            "drill claims surface {} that does not bind it",
                            surface.as_str()
                        ),
                    );
                }
            }
        }
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self
                .consumer_bindings
                .iter()
                .all(|binding| binding.narrow_on_stale_proof)
    }

    /// Returns the number of rows in each published state, for claim packets.
    pub fn state_counts(&self) -> QualificationStateCounts {
        let mut counts = QualificationStateCounts::default();
        for row in &self.qualification_rows {
            match row.published_state {
                QualificationStateClass::Qualified => counts.qualified += 1,
                QualificationStateClass::LimitedProfileScoped => counts.limited_profile_scoped += 1,
                QualificationStateClass::LocalSelfDiagnosisOnly => {
                    counts.local_self_diagnosis_only += 1
                }
                QualificationStateClass::BlockedUnverified => counts.blocked_unverified += 1,
            }
        }
        counts
    }
}

/// Row counts by published qualification state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QualificationStateCounts {
    /// Rows that remain fully qualified.
    pub qualified: usize,
    /// Rows narrowed to a profile-scoped claim.
    pub limited_profile_scoped: usize,
    /// Rows narrowed to a local-save / self-diagnosis claim.
    pub local_self_diagnosis_only: usize,
    /// Rows blocked pending fresh proof.
    pub blocked_unverified: usize,
}

/// Returns the canonical seeded M5 supportability qualification packet.
pub fn seeded_m5_supportability_qualification_packet() -> M5SupportabilityQualificationPacket {
    build_packet(QualificationVariant::Canonical)
}

/// Returns a seeded packet where the consent-sheet-accuracy drill is stale, so
/// every send-capable surface narrows to a local-save / self-diagnosis claim.
pub fn seeded_consent_drill_stale_m5_supportability_qualification_packet(
) -> M5SupportabilityQualificationPacket {
    build_packet(QualificationVariant::ConsentDrillStale)
}

/// Returns a seeded packet where the environment-explainability evidence is
/// stale, blocking that surface and narrowing the Support Center that depends on
/// it.
pub fn seeded_environment_evidence_stale_m5_supportability_qualification_packet(
) -> M5SupportabilityQualificationPacket {
    build_packet(QualificationVariant::EnvironmentEvidenceStale)
}

#[derive(Debug, Clone, Copy)]
enum QualificationVariant {
    Canonical,
    ConsentDrillStale,
    EnvironmentEvidenceStale,
}

fn build_packet(variant: QualificationVariant) -> M5SupportabilityQualificationPacket {
    let mut qualification_rows = Vec::new();
    for surface in SupportabilitySurfaceClass::ALL {
        for profile in ClaimedM5Profile::ALL {
            qualification_rows.push(seed_row(surface, profile, variant));
        }
    }
    let row_count = qualification_rows.len();

    M5SupportabilityQualificationPacket {
        record_kind: M5_SUPPORTABILITY_QUALIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_VERSION,
        packet_id: M5_SUPPORTABILITY_QUALIFICATION_PACKET_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        doc_ref: M5_SUPPORTABILITY_QUALIFICATION_DOC_REF.to_owned(),
        schema_ref: M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_REF.to_owned(),
        artifact_ref: M5_SUPPORTABILITY_QUALIFICATION_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
            ".t2/docs/Aureline_PRD.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            M5_SUPPORT_CENTER_MATRIX_SCHEMA_REF.to_owned(),
            M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_REF.to_owned(),
            M5_PRECEDENCE_INSPECTOR_SCHEMA_REF.to_owned(),
            M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_REF.to_owned(),
            M5_CRASH_INTAKE_RECOVERY_SCHEMA_REF.to_owned(),
            M5_SUPPORTABILITY_HANDOFF_SCHEMA_REF.to_owned(),
        ],
        claimed_profiles: ClaimedM5Profile::ALL.to_vec(),
        deployment_modes: DeploymentMode::ALL.to_vec(),
        qualification_rows,
        drill_catalog: seeded_drill_catalog(variant),
        downgrade_rules: seeded_downgrade_rules(),
        consumer_bindings: seeded_consumer_bindings(row_count),
        export_safe_summary:
            "This metadata-safe qualification index binds every claimed M5 supportability surface and profile to its own Support Center, environment-explainability, precedence, consent, crash-intake, or handoff proof plus the supportability drills that back it; stale surface evidence or a stale drill narrows the row instead of inheriting an adjacent surface's maturity, local-save stays first-class on every send-capable surface, and no raw payloads cross the boundary."
                .to_owned(),
    }
}

fn seed_row(
    surface: SupportabilitySurfaceClass,
    profile: ClaimedM5Profile,
    variant: QualificationVariant,
) -> SurfaceProfileQualificationRow {
    let refs = surface.backing_refs();
    let share_capable = surface.share_capable();
    let drill_ids: Vec<String> = surface
        .drills()
        .iter()
        .map(|drill| drill.drill_id())
        .collect();
    let mut row = SurfaceProfileQualificationRow {
        qualification_row_id: format!(
            "m5_supportability:{}:{}",
            surface.as_str(),
            profile.as_str()
        ),
        surface,
        surface_label: surface.label().to_owned(),
        profile,
        published_state: QualificationStateClass::Qualified,
        deployment_mode_coverage: DeploymentMode::ALL.to_vec(),
        share_capable,
        local_save_first: share_capable,
        backing_schema_ref: refs.schema_ref.to_owned(),
        backing_artifact_ref: refs.artifact_ref.to_owned(),
        backing_fixture_ref: refs.fixture_ref.to_owned(),
        backing_record_kind: refs.record_kind.to_owned(),
        drill_ids,
        stale_proof_tokens: Vec::new(),
        downgrade_rule_ids: Vec::new(),
        summary: format!(
            "{} on {} reuses its own checked-in lane proof across desktop and CLI/headless{}.",
            surface.label(),
            profile.as_str(),
            if share_capable {
                ", with local-save first-class beside any team-share or formal-support send"
            } else {
                ""
            }
        ),
    };

    match variant {
        QualificationVariant::Canonical => {}
        QualificationVariant::ConsentDrillStale => {
            if surface
                .drills()
                .contains(&SupportabilityDrillClass::ConsentSheetAccuracy)
            {
                apply_downgrade(
                    &mut row,
                    QualificationStateClass::LocalSelfDiagnosisOnly,
                    "consent_sheet_accuracy_drill_stale",
                    "supportability_drill_stale_narrows_send_claim",
                    &format!(
                        "{} on {} narrows to a local-save / self-diagnosis claim because the consent-sheet-accuracy drill is stale; the team-share and formal-support send paths cannot be certified while included / excluded / policy-locked counts are unproven.",
                        surface.label(),
                        profile.as_str()
                    ),
                );
            }
        }
        QualificationVariant::EnvironmentEvidenceStale => match surface {
            SupportabilitySurfaceClass::EnvironmentExplainability => apply_downgrade(
                &mut row,
                QualificationStateClass::BlockedUnverified,
                "environment_explainability_evidence_stale",
                "surface_evidence_stale_blocks_own_claim",
                &format!(
                    "Environment explainability on {} is blocked because its own status-strip evidence is stale; no run-capable surface can answer why this execution context won until it is refreshed.",
                    profile.as_str()
                ),
            ),
            SupportabilitySurfaceClass::SupportCenter => apply_downgrade(
                &mut row,
                QualificationStateClass::LimitedProfileScoped,
                "environment_explainability_evidence_stale",
                "surface_evidence_stale_blocks_own_claim",
                &format!(
                    "Support Center on {} narrows to a profile-scoped claim because it binds environment status and that evidence is stale; the integrated surface may not imply a fresh execution-context strip it does not have.",
                    profile.as_str()
                ),
            ),
            _ => {}
        },
    }

    row
}

fn apply_downgrade(
    row: &mut SurfaceProfileQualificationRow,
    state: QualificationStateClass,
    token: &str,
    rule_id: &str,
    summary: &str,
) {
    row.published_state = state;
    row.stale_proof_tokens.push(token.to_owned());
    row.downgrade_rule_ids.push(rule_id.to_owned());
    row.summary = summary.to_owned();
}

fn seeded_drill_catalog(variant: QualificationVariant) -> Vec<SupportabilityDrillRow> {
    SupportabilityDrillClass::ALL
        .into_iter()
        .map(|drill| {
            let is_fresh = !matches!(
                (variant, drill),
                (
                    QualificationVariant::ConsentDrillStale,
                    SupportabilityDrillClass::ConsentSheetAccuracy
                )
            );
            SupportabilityDrillRow {
                drill_id: drill.drill_id(),
                drill_class: drill,
                label: drill.label().to_owned(),
                covered_surfaces: drill.covered_surfaces(),
                evidence_refs: drill.evidence_refs(),
                is_fresh,
                summary: format!(
                    "The {} drill {} the surfaces that bind it.",
                    drill.label().to_lowercase(),
                    if is_fresh {
                        "is current and backs"
                    } else {
                        "is stale and narrows"
                    }
                ),
            }
        })
        .collect()
}

fn seeded_downgrade_rules() -> Vec<QualificationDowngradeRuleRow> {
    vec![
        QualificationDowngradeRuleRow {
            rule_id: "surface_evidence_stale_blocks_own_claim".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::SurfaceEvidenceStale,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::BlockedUnverified,
            required_effect: "When a supportability surface's own lane evidence (schema, artifact, or fixture) is stale or missing, the surface blocks and any integrated surface that binds it narrows; neither may inherit a greener neighboring claim.".to_owned(),
            rationale: "The integrated Support Center, export, and intake surfaces each need their own proof; a passing crash or Doctor row may not keep a stale surface green.".to_owned(),
            evidence_refs: vec![
                M5_SUPPORTABILITY_QUALIFICATION_DOC_REF.to_owned(),
                M5_SUPPORT_CENTER_MATRIX_SCHEMA_REF.to_owned(),
                M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "supportability_drill_stale_narrows_send_claim".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::SupportabilityDrillStale,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::LocalSelfDiagnosisOnly,
            required_effect: "When a bound supportability drill (diagnosis latency, consent-sheet accuracy, export-mode parity, crash-loop recovery, or exact-build intake) is stale, the rows it backs narrow; a send-capable surface narrows to a local-save / self-diagnosis claim while local-save stays first-class.".to_owned(),
            rationale: "A send or consent claim is only safe while the drill that proves its accuracy and parity is current.".to_owned(),
            evidence_refs: vec![
                DIAGNOSIS_LATENCY_SCOREBOARD_REF.to_owned(),
                REDACTION_ACCURACY_CHECKS_REF.to_owned(),
                RECOVERY_LADDER_CASES_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "deployment_mode_parity_lost_narrows_claim".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::DeploymentModeParityLost,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::LimitedProfileScoped,
            required_effect: "When a surface stops projecting identically across desktop and CLI/headless, its row narrows to the deployment modes it still covers; the qualification may not imply parity it cannot prove.".to_owned(),
            rationale: "Blocked-user recovery must not fragment across deployment modes; a missing mode narrows the claim.".to_owned(),
            evidence_refs: vec![
                M5_SUPPORTABILITY_QUALIFICATION_DOC_REF.to_owned(),
                M5_SUPPORT_CENTER_MATRIX_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "policy_blocked_send_keeps_local_save_first".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::PolicyBlockedSend,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::LocalSelfDiagnosisOnly,
            required_effect: "When a profile policy-blocks a team-share or formal-support send, the surface narrows to a local-save / self-diagnosis claim; local-save and self-diagnosis stay first-class and are never demoted below a blocked send path.".to_owned(),
            rationale: "Support claims may not widen on profiles or deployment modes whose send paths are policy-blocked, but local recovery must remain fully claimable.".to_owned(),
            evidence_refs: vec![
                M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_REF.to_owned(),
                M5_SUPPORTABILITY_HANDOFF_SCHEMA_REF.to_owned(),
            ],
        },
        QualificationDowngradeRuleRow {
            rule_id: "consumer_binding_missing_blocks_shared_truth".to_owned(),
            trigger_class: QualificationDowngradeTriggerClass::ConsumerBindingMissing,
            source_state: QualificationStateClass::Qualified,
            downgraded_state: QualificationStateClass::BlockedUnverified,
            required_effect: "If Help/About, the desktop Support Center, CLI/headless, support export, shiproom, or release manifest stops ingesting this packet by reference, the broad supportability claim blocks until parity is restored.".to_owned(),
            rationale: "The task requires one supportability qualification index; broken consumer bindings invalidate that promise.".to_owned(),
            evidence_refs: vec![
                M5_SUPPORTABILITY_QUALIFICATION_DOC_REF.to_owned(),
                M5_SUPPORTABILITY_QUALIFICATION_CLAIM_PACKET_REF.to_owned(),
                SUPPORT_EXPORT_CONSUMER_REF.to_owned(),
                RELEASE_MANIFEST_CONSUMER_REF.to_owned(),
            ],
        },
    ]
}

fn seeded_consumer_bindings(row_count: usize) -> Vec<QualificationConsumerBinding> {
    let verbatim_fields: Vec<String> = REQUIRED_PROJECTION_FIELDS
        .iter()
        .map(|field| (*field).to_owned())
        .collect();
    let binding = |consumer: QualificationConsumerClass, consumer_ref: &str, summary: &str| {
        QualificationConsumerBinding {
            consumer,
            consumer_ref: consumer_ref.to_owned(),
            ingested_packet_id: M5_SUPPORTABILITY_QUALIFICATION_PACKET_ID.to_owned(),
            qualification_row_count: row_count,
            required_verbatim_fields: verbatim_fields.clone(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: summary.to_owned(),
        }
    };
    vec![
        binding(
            QualificationConsumerClass::HelpAbout,
            M5_SUPPORTABILITY_QUALIFICATION_DOC_REF,
            "Help/About reuses the qualification row ids, surface tokens, profile tokens, published state, and stale-proof tokens verbatim instead of paraphrasing supportability maturity.",
        ),
        binding(
            QualificationConsumerClass::SupportCenterDesktop,
            M5_SUPPORT_CENTER_MATRIX_ARTIFACT_DOC_REF,
            "The desktop Support Center ingests the same qualification index so a module narrows in lockstep with its certified row instead of advertising a recovery the proof no longer supports.",
        ),
        binding(
            QualificationConsumerClass::CliHeadless,
            M5_SUPPORTABILITY_QUALIFICATION_ARTIFACT_REF,
            "CLI/headless support output projects the same rows so blocked-user recovery reads identically off the desktop.",
        ),
        binding(
            QualificationConsumerClass::SupportExport,
            SUPPORT_EXPORT_CONSUMER_REF,
            "Support-export packets attach the same row ids and downgrade tokens instead of inventing a parallel supportability badge.",
        ),
        binding(
            QualificationConsumerClass::Shiproom,
            M5_SUPPORTABILITY_QUALIFICATION_CLAIM_PACKET_REF,
            "The shiproom claim packet derives its publishable / narrowed / withheld scope from this index and narrows automatically when a surface row goes stale or red.",
        ),
        binding(
            QualificationConsumerClass::ReleaseManifest,
            RELEASE_MANIFEST_CONSUMER_REF,
            "Release manifests consume the same qualification index so stale surface evidence or a stale drill cannot keep a broader release claim green.",
        ),
    ]
}

fn push(violations: &mut Vec<M5SupportabilityQualificationViolation>, path: &str, message: &str) {
    violations.push(M5SupportabilityQualificationViolation {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}

#[cfg(test)]
mod tests;
