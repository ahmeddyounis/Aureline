//! M3 beta incident-workspace handoff packet.
//!
//! This module mints the single escalation artifact that lets a blocked
//! user, support intake, and security triage read one packet instead of
//! three separate private templates. Each packet pins:
//!
//! - the workspace identity that preserves user-authored files;
//! - the target's exact-build identity and deployment profile;
//! - the degraded-state row that opened the escalation;
//! - the typed findings (Project Doctor, extension bisect, safe-mode
//!   profile, crash envelope, records governance, runtime replay) that
//!   inform the escalation;
//! - the evidence artifacts with explicit custody class so local-only
//!   artifacts, managed copies, and held records never silently merge;
//! - the recovery options the user still has;
//! - the claim-state downgrade tokens the M3 scenario corpus uses, so
//!   review surfaces inherit the same red / yellow / stale signals.
//!
//! The packet shape mirrors
//! [`/schemas/support/incident_workspace_beta_packet.schema.json`].
//! The reviewer doc lives at
//! [`/docs/support/m3/incident_handoff_template.md`] and the protected
//! fixture corpus lives at `/fixtures/support/m3/incident_packets/`.
//!
//! ## What this module does NOT own
//!
//! - Live runtime probe execution, fixture mutation, or the apply path
//!   for any beta lane. The packet composes by reference with each
//!   owning beta-lane evaluator.
//! - Hosted ticket intake, cross-tenant case management, or upload
//!   transport. The packet is the metadata-safe escalation artifact;
//!   transport stays with the support-export pipeline.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the M3 beta incident-workspace packet.
pub const INCIDENT_WORKSPACE_BETA_PACKET_RECORD_KIND: &str =
    "incident_workspace_beta_packet_record";

/// Frozen schema version for the M3 beta packet.
pub const INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_REF: &str =
    "schemas/support/incident_workspace_beta_packet.schema.json";

/// Repo-relative path of the reviewer template doc.
pub const INCIDENT_WORKSPACE_BETA_PACKET_DOC_REF: &str =
    "docs/support/m3/incident_handoff_template.md";

/// Repo-relative path of the M3 scenario corpus reviewer doc.
pub const INCIDENT_WORKSPACE_BETA_SCENARIO_CORPUS_DOC_REF: &str =
    "docs/support/m3/support_scenario_corpus.md";

/// Repo-relative path of the incident-workspace alpha contract.
pub const INCIDENT_WORKSPACE_ALPHA_DOC_REF: &str = "docs/ops/incident_workspace_contract.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const INCIDENT_WORKSPACE_BETA_FIXTURE_DIR: &str = "fixtures/support/m3/incident_packets";

/// Repo-relative path of the fixture manifest.
pub const INCIDENT_WORKSPACE_BETA_FIXTURE_MANIFEST_REF: &str =
    "fixtures/support/m3/incident_packets/manifest.yaml";

/// Repo-relative path of the reviewer-facing artifact summary.
pub const INCIDENT_WORKSPACE_BETA_ARTIFACT_REF: &str =
    "artifacts/support/m3/incident_workspace_packet.md";

const FIXTURE_SOURCES: &[(&str, &str)] = &[
    (
        "fixtures/support/m3/incident_packets/safe_mode_crash_loop_local_only.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/incident_packets/safe_mode_crash_loop_local_only.yaml"
        )),
    ),
    (
        "fixtures/support/m3/incident_packets/extension_quarantine_managed_copy.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/incident_packets/extension_quarantine_managed_copy.yaml"
        )),
    ),
    (
        "fixtures/support/m3/incident_packets/joint_security_support_held_record.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/incident_packets/joint_security_support_held_record.yaml"
        )),
    ),
];

/// Closed deployment-profile vocabulary mirrored on every packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileClass {
    /// Individual user, local workstation.
    IndividualLocal,
    /// Self-hosted shared install.
    SelfHosted,
    /// Enterprise online tenant.
    EnterpriseOnline,
    /// Air-gapped install with no provider lane.
    AirGapped,
    /// Managed-cloud tenant.
    ManagedCloud,
}

impl DeploymentProfileClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }
}

/// Closed handoff-consumer-class vocabulary; names which downstream
/// review surfaces consume the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffConsumerClass {
    /// Only the support intake lane consumes the packet.
    SupportIntakeOnly,
    /// Only the security triage lane consumes the packet.
    SecurityTriageOnly,
    /// Both support intake and security triage consume the same packet.
    SupportIntakeAndSecurityTriage,
}

impl HandoffConsumerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportIntakeOnly => "support_intake_only",
            Self::SecurityTriageOnly => "security_triage_only",
            Self::SupportIntakeAndSecurityTriage => "support_intake_and_security_triage",
        }
    }
}

/// Closed degraded-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedStateClass {
    /// Startup crash loop was detected.
    StartupCrashLoopDetected,
    /// Restart budget was exceeded.
    RestartBudgetExceeded,
    /// Extension regression is suspected.
    ExtensionRegressionSuspected,
    /// Policy-forced safe mode is in effect.
    PolicyForcedSafeMode,
    /// A remote route is unavailable.
    RemoteRouteUnavailable,
    /// State corruption that the recovery ladder can repair.
    StateCorruptionRecoverable,
    /// The user has only the support-export path; no remote help available.
    SupportExportOnlyNoRemote,
}

impl DegradedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartupCrashLoopDetected => "startup_crash_loop_detected",
            Self::RestartBudgetExceeded => "restart_budget_exceeded",
            Self::ExtensionRegressionSuspected => "extension_regression_suspected",
            Self::PolicyForcedSafeMode => "policy_forced_safe_mode",
            Self::RemoteRouteUnavailable => "remote_route_unavailable",
            Self::StateCorruptionRecoverable => "state_corruption_recoverable",
            Self::SupportExportOnlyNoRemote => "support_export_only_no_remote",
        }
    }
}

/// Closed finding-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingClass {
    /// Project Doctor beta finding.
    ProjectDoctorFinding,
    /// Extension-bisect attribution finding.
    ExtensionBisectFinding,
    /// Safe-mode profile entry/exit finding.
    SafeModeProfileFinding,
    /// Crash envelope reference.
    CrashEnvelopeFinding,
    /// Records-governance packet finding.
    RecordsGovernanceFinding,
    /// Runtime replay decision finding.
    RuntimeReplayFinding,
}

impl FindingClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDoctorFinding => "project_doctor_finding",
            Self::ExtensionBisectFinding => "extension_bisect_finding",
            Self::SafeModeProfileFinding => "safe_mode_profile_finding",
            Self::CrashEnvelopeFinding => "crash_envelope_finding",
            Self::RecordsGovernanceFinding => "records_governance_finding",
            Self::RuntimeReplayFinding => "runtime_replay_finding",
        }
    }
}

/// Closed evidence-artifact-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceArtifactClass {
    /// Local support-bundle preview ref.
    SupportBundlePreview,
    /// Crash envelope ref.
    CrashEnvelopeRef,
    /// Doctor probe pack record.
    DoctorProbePackRecord,
    /// Extension-bisect session record.
    ExtensionBisectSessionRecord,
    /// Safe-mode profile record.
    SafeModeProfileRecord,
    /// Repair-preview skeleton record.
    RepairPreviewSkeletonRecord,
    /// Records-governance packet record.
    RecordsGovernancePacketRecord,
    /// Runtime replay pack record.
    RuntimeReplayPackRecord,
    /// Alpha incident-workspace packet ref.
    IncidentWorkspaceAlphaPacketRef,
    /// Runbook packet ref.
    RunbookPacketRef,
}

impl EvidenceArtifactClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportBundlePreview => "support_bundle_preview",
            Self::CrashEnvelopeRef => "crash_envelope_ref",
            Self::DoctorProbePackRecord => "doctor_probe_pack_record",
            Self::ExtensionBisectSessionRecord => "extension_bisect_session_record",
            Self::SafeModeProfileRecord => "safe_mode_profile_record",
            Self::RepairPreviewSkeletonRecord => "repair_preview_skeleton_record",
            Self::RecordsGovernancePacketRecord => "records_governance_packet_record",
            Self::RuntimeReplayPackRecord => "runtime_replay_pack_record",
            Self::IncidentWorkspaceAlphaPacketRef => "incident_workspace_alpha_packet_ref",
            Self::RunbookPacketRef => "runbook_packet_ref",
        }
    }
}

/// Closed evidence-custody vocabulary. Distinguishes local-only
/// artifacts (still owned by the blocked user), managed copies
/// (available through the managed admin lane), and held records (under
/// legal or security hold).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCustodyClass {
    /// Artifact stays on the user's host; no managed copy exists.
    LocalOnlyArtifact,
    /// A managed copy is reachable through the managed admin lane.
    ManagedCopyAvailable,
    /// Record is under legal hold; export is governed.
    HeldRecordUnderLegalHold,
    /// Record is under a security hold; export waits on the security
    /// triage lane.
    HeldRecordUnderSecurityHold,
    /// Artifact has already been exported to support intake.
    ExportedToSupportIntake,
    /// Artifact is withheld pending an explicit user review.
    WithheldPendingUserReview,
}

impl EvidenceCustodyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyArtifact => "local_only_artifact",
            Self::ManagedCopyAvailable => "managed_copy_available",
            Self::HeldRecordUnderLegalHold => "held_record_under_legal_hold",
            Self::HeldRecordUnderSecurityHold => "held_record_under_security_hold",
            Self::ExportedToSupportIntake => "exported_to_support_intake",
            Self::WithheldPendingUserReview => "withheld_pending_user_review",
        }
    }

    /// True when the custody class names a held record (legal or
    /// security hold). The validator uses this to refuse claim-state
    /// blocks that drop the `held_record_blocks_export` token while a
    /// held record is attached.
    pub const fn is_held_record(self) -> bool {
        matches!(
            self,
            Self::HeldRecordUnderLegalHold | Self::HeldRecordUnderSecurityHold
        )
    }
}

/// Closed recovery-option vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryOptionClass {
    /// Enter a typed safe-mode profile.
    EnterSafeMode,
    /// Start an extension-bisect session.
    StartExtensionBisect,
    /// Open a repair-preview skeleton.
    OpenRepairPreview,
    /// Locate a missing target.
    LocateMissingTarget,
    /// Hand off to support.
    HandoffToSupport,
    /// Open a private security triage lane.
    OpenSecurityPrivateTriage,
    /// Rebuild disposable state (caches, indices).
    RebuildDisposableState,
    /// Export a support bundle.
    ExportSupportBundle,
}

impl RecoveryOptionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnterSafeMode => "enter_safe_mode",
            Self::StartExtensionBisect => "start_extension_bisect",
            Self::OpenRepairPreview => "open_repair_preview",
            Self::LocateMissingTarget => "locate_missing_target",
            Self::HandoffToSupport => "handoff_to_support",
            Self::OpenSecurityPrivateTriage => "open_security_private_triage",
            Self::RebuildDisposableState => "rebuild_disposable_state",
            Self::ExportSupportBundle => "export_support_bundle",
        }
    }
}

/// Closed claim-state downgrade tokens. Mirrors the scenario corpus's
/// downgrade triggers so review surfaces share the same vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimDowngradeToken {
    /// A primary fixture is missing.
    FixtureMissing,
    /// A drill step has no proving artifact ref.
    DrillStepUnproven,
    /// Drill replay produced a record the evaluator refused.
    DrillProvesRegression,
    /// A recovery action was proposed that the safety baseline refuses.
    RecoveryActionUnsafe,
    /// Raw private material was detected on an artifact row.
    RawPrivateMaterialPresent,
    /// Support bundle still needs a redaction pass before export.
    SupportBundleRedactionRequired,
    /// A managed copy is pending admin review.
    ManagedCopyPendingAdminReview,
    /// A held record blocks export until the hold lifts.
    HeldRecordBlocksExport,
}

impl ClaimDowngradeToken {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixtureMissing => "fixture_missing",
            Self::DrillStepUnproven => "drill_step_unproven",
            Self::DrillProvesRegression => "drill_proves_regression",
            Self::RecoveryActionUnsafe => "recovery_action_unsafe",
            Self::RawPrivateMaterialPresent => "raw_private_material_present",
            Self::SupportBundleRedactionRequired => "support_bundle_redaction_required",
            Self::ManagedCopyPendingAdminReview => "managed_copy_pending_admin_review",
            Self::HeldRecordBlocksExport => "held_record_blocks_export",
        }
    }
}

/// Workspace identity block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceIdentity {
    /// Stable workspace id.
    pub workspace_id: String,
    /// Reviewer-safe summary of the workspace profile.
    pub workspace_profile_summary: String,
    /// Always true; the validator refuses the packet when false.
    pub preserves_user_authored_files: bool,
}

/// Target block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetBlock {
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Deployment profile class.
    pub deployment_profile_class: DeploymentProfileClass,
    /// Reviewer-safe summary.
    pub target_summary: String,
    /// True when the deployment profile requires managed admin sign-off.
    #[serde(default)]
    pub managed_admin_required: bool,
}

/// Degraded-state block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedStateBlock {
    /// Closed degraded-state class.
    pub degraded_state_class: DegradedStateClass,
    /// Reviewer-safe summary.
    pub degraded_summary: String,
    /// UTC timestamp observed at.
    pub observed_at: String,
}

/// One finding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Closed finding class.
    pub finding_class: FindingClass,
    /// Reviewer-safe summary.
    pub finding_summary: String,
    /// Source ref the finding came from.
    pub source_ref: String,
}

/// One evidence-artifact row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceArtifactRow {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Closed artifact class.
    pub artifact_class: EvidenceArtifactClass,
    /// Closed custody class.
    pub custody_class: EvidenceCustodyClass,
    /// Reviewer-safe summary.
    pub artifact_summary: String,
    /// Stable artifact ref.
    pub artifact_ref: String,
}

/// One recovery-option row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryOptionRow {
    /// Closed recovery-option class.
    pub recovery_option_class: RecoveryOptionClass,
    /// Reviewer-safe summary.
    pub option_summary: String,
}

/// Claim-state block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimStateBlock {
    /// Downgrade tokens that apply.
    pub downgrade_tokens: Vec<ClaimDowngradeToken>,
    /// Reviewer-safe summary.
    pub claim_summary: String,
}

/// Privacy baseline block. Pinned on every packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyBaseline {
    /// Always true; the validator refuses the packet when false.
    pub raw_private_material_excluded: bool,
    /// Always true; the validator refuses the packet when false.
    pub ambient_authority_excluded: bool,
    /// Reviewer-safe summary.
    pub redaction_baseline_summary: String,
}

/// References block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencesBlock {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Scenario corpus doc ref.
    pub scenario_corpus_doc_ref: String,
    /// Incident-workspace alpha doc ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incident_workspace_alpha_doc_ref: Option<String>,
}

/// One M3 beta incident-workspace handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceBetaPacket {
    /// Frozen schema version (1).
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Closed handoff-consumer classes.
    pub handoff_consumer_classes: Vec<HandoffConsumerClass>,
    /// Workspace identity.
    pub workspace_identity: WorkspaceIdentity,
    /// Target.
    pub target: TargetBlock,
    /// Degraded state.
    pub degraded_state: DegradedStateBlock,
    /// Findings.
    pub findings: Vec<FindingRow>,
    /// Evidence artifacts with custody class.
    pub evidence_artifacts: Vec<EvidenceArtifactRow>,
    /// Recovery options.
    pub recovery_options: Vec<RecoveryOptionRow>,
    /// Claim state.
    pub claim_state: ClaimStateBlock,
    /// Privacy baseline.
    pub privacy_baseline: PrivacyBaseline,
    /// References.
    pub references: ReferencesBlock,
    /// UTC emit timestamp.
    pub emitted_at: String,
}

/// One fixture-bound entry in the protected corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceBetaPacketEntry {
    /// Repo-relative fixture ref.
    pub fixture_ref: String,
    /// Parsed packet record.
    pub packet: IncidentWorkspaceBetaPacket,
}

/// Validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketViolation {
    /// Stable check id.
    pub check_id: String,
    /// Packet id or fixture ref that failed.
    pub target_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// The protected fixture corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceBetaPacketCorpus {
    /// Fixture-bound entries.
    pub entries: Vec<IncidentWorkspaceBetaPacketEntry>,
}

impl IncidentWorkspaceBetaPacketCorpus {
    /// Returns the parsed packets without their fixture-path wrapper.
    pub fn packets(&self) -> impl Iterator<Item = &IncidentWorkspaceBetaPacket> {
        self.entries.iter().map(|entry| &entry.packet)
    }

    /// Validates the corpus against the beta packet contract.
    pub fn validate(&self) -> Vec<PacketViolation> {
        let mut violations = Vec::new();
        if self.entries.is_empty() {
            push_violation(
                &mut violations,
                "corpus.empty",
                INCIDENT_WORKSPACE_BETA_FIXTURE_DIR,
                "corpus must contain at least one packet",
            );
            return violations;
        }
        let mut packet_ids = BTreeSet::new();
        let mut fixture_refs = BTreeSet::new();
        for entry in &self.entries {
            if !fixture_refs.insert(entry.fixture_ref.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_fixture_ref",
                    &entry.fixture_ref,
                    "fixture_ref must be unique within the corpus",
                );
            }
            if !packet_ids.insert(entry.packet.packet_id.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_packet_id",
                    &entry.packet.packet_id,
                    "packet_id must be unique within the corpus",
                );
            }
            validate_packet(&mut violations, &entry.packet);
        }
        violations
    }
}

/// Validates one packet in isolation. Returns the collected violations.
pub fn validate_packet_record(packet: &IncidentWorkspaceBetaPacket) -> Vec<PacketViolation> {
    let mut violations = Vec::new();
    validate_packet(&mut violations, packet);
    violations
}

fn validate_packet(violations: &mut Vec<PacketViolation>, packet: &IncidentWorkspaceBetaPacket) {
    let target = packet.packet_id.as_str();

    if packet.schema_version != INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_VERSION {
        push_violation(
            violations,
            "packet.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if packet.record_kind != INCIDENT_WORKSPACE_BETA_PACKET_RECORD_KIND {
        push_violation(
            violations,
            "packet.record_kind",
            target,
            "record_kind must be incident_workspace_beta_packet_record",
        );
    }
    for (field, value) in [
        ("packet_id", packet.packet_id.as_str()),
        ("title", packet.title.as_str()),
        ("summary", packet.summary.as_str()),
        ("emitted_at", packet.emitted_at.as_str()),
    ] {
        if value.trim().is_empty() {
            push_violation(
                violations,
                "packet.required_field_empty",
                target,
                format!("{field} must be non-empty"),
            );
        }
    }
    if packet.handoff_consumer_classes.is_empty() {
        push_violation(
            violations,
            "packet.handoff_consumer_classes.empty",
            target,
            "handoff_consumer_classes must declare at least one consumer",
        );
    }
    let unique_consumers: BTreeSet<_> = packet.handoff_consumer_classes.iter().collect();
    if unique_consumers.len() != packet.handoff_consumer_classes.len() {
        push_violation(
            violations,
            "packet.handoff_consumer_classes.duplicate",
            target,
            "handoff_consumer_classes must be unique",
        );
    }
    validate_workspace_identity(violations, target, &packet.workspace_identity);
    validate_target_block(violations, target, &packet.target);
    validate_degraded_state(violations, target, &packet.degraded_state);
    validate_findings(violations, target, &packet.findings);
    let custody_summary =
        validate_evidence_artifacts(violations, target, &packet.evidence_artifacts);
    validate_recovery_options(
        violations,
        target,
        &packet.recovery_options,
        &packet.handoff_consumer_classes,
    );
    validate_claim_state(violations, target, &packet.claim_state, &custody_summary);
    validate_privacy_baseline(violations, target, &packet.privacy_baseline);
    validate_references(violations, target, &packet.references);
}

#[derive(Debug, Default)]
struct CustodySummary {
    has_held_record: bool,
    has_managed_copy: bool,
}

fn validate_workspace_identity(
    violations: &mut Vec<PacketViolation>,
    target: &str,
    identity: &WorkspaceIdentity,
) {
    if identity.workspace_id.trim().is_empty()
        || identity.workspace_profile_summary.trim().is_empty()
    {
        push_violation(
            violations,
            "packet.workspace_identity.empty",
            target,
            "workspace_identity must declare workspace_id and workspace_profile_summary",
        );
    }
    if !identity.preserves_user_authored_files {
        push_violation(
            violations,
            "packet.workspace_identity.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_target_block(violations: &mut Vec<PacketViolation>, target: &str, block: &TargetBlock) {
    if block.exact_build_identity_ref.trim().is_empty() || block.target_summary.trim().is_empty() {
        push_violation(
            violations,
            "packet.target.empty",
            target,
            "target must declare exact_build_identity_ref and target_summary",
        );
    }
}

fn validate_degraded_state(
    violations: &mut Vec<PacketViolation>,
    target: &str,
    state: &DegradedStateBlock,
) {
    if state.degraded_summary.trim().is_empty() || state.observed_at.trim().is_empty() {
        push_violation(
            violations,
            "packet.degraded_state.empty",
            target,
            "degraded_state must declare degraded_summary and observed_at",
        );
    }
}

fn validate_findings(violations: &mut Vec<PacketViolation>, target: &str, findings: &[FindingRow]) {
    if findings.is_empty() {
        push_violation(
            violations,
            "packet.findings.empty",
            target,
            "findings must declare at least one row",
        );
        return;
    }
    let mut seen = BTreeSet::new();
    for finding in findings {
        if !seen.insert(finding.finding_id.as_str()) {
            push_violation(
                violations,
                "packet.findings.duplicate_finding_id",
                target,
                format!("duplicate finding_id {}", finding.finding_id),
            );
        }
        if finding.finding_id.trim().is_empty()
            || finding.finding_summary.trim().is_empty()
            || finding.source_ref.trim().is_empty()
        {
            push_violation(
                violations,
                "packet.findings.row_empty",
                target,
                "finding rows must declare finding_id, finding_summary, and source_ref",
            );
        }
    }
}

fn validate_evidence_artifacts(
    violations: &mut Vec<PacketViolation>,
    target: &str,
    artifacts: &[EvidenceArtifactRow],
) -> CustodySummary {
    let mut summary = CustodySummary::default();
    if artifacts.is_empty() {
        push_violation(
            violations,
            "packet.evidence_artifacts.empty",
            target,
            "evidence_artifacts must declare at least one row",
        );
        return summary;
    }
    let mut seen = BTreeSet::new();
    for artifact in artifacts {
        if !seen.insert(artifact.artifact_id.as_str()) {
            push_violation(
                violations,
                "packet.evidence_artifacts.duplicate_artifact_id",
                target,
                format!("duplicate artifact_id {}", artifact.artifact_id),
            );
        }
        if artifact.artifact_id.trim().is_empty()
            || artifact.artifact_summary.trim().is_empty()
            || artifact.artifact_ref.trim().is_empty()
        {
            push_violation(
                violations,
                "packet.evidence_artifacts.row_empty",
                target,
                "evidence_artifacts rows must declare artifact_id, artifact_summary, and artifact_ref",
            );
        }
        if artifact.custody_class.is_held_record() {
            summary.has_held_record = true;
        }
        if matches!(
            artifact.custody_class,
            EvidenceCustodyClass::ManagedCopyAvailable
        ) {
            summary.has_managed_copy = true;
        }
    }
    summary
}

fn validate_recovery_options(
    violations: &mut Vec<PacketViolation>,
    target: &str,
    options: &[RecoveryOptionRow],
    consumers: &[HandoffConsumerClass],
) {
    if options.is_empty() {
        push_violation(
            violations,
            "packet.recovery_options.empty",
            target,
            "recovery_options must declare at least one row",
        );
        return;
    }
    let mut seen = BTreeSet::new();
    for option in options {
        if !seen.insert(option.recovery_option_class) {
            push_violation(
                violations,
                "packet.recovery_options.duplicate_class",
                target,
                format!(
                    "duplicate recovery_option_class {}",
                    option.recovery_option_class.as_str()
                ),
            );
        }
        if option.option_summary.trim().is_empty() {
            push_violation(
                violations,
                "packet.recovery_options.empty_summary",
                target,
                format!(
                    "recovery_option {} must declare a non-empty option_summary",
                    option.recovery_option_class.as_str()
                ),
            );
        }
    }
    let needs_security_route = consumers.iter().any(|c| {
        matches!(
            c,
            HandoffConsumerClass::SecurityTriageOnly
                | HandoffConsumerClass::SupportIntakeAndSecurityTriage
        )
    });
    if needs_security_route
        && !options
            .iter()
            .any(|opt| opt.recovery_option_class == RecoveryOptionClass::OpenSecurityPrivateTriage)
    {
        push_violation(
            violations,
            "packet.recovery_options.security_route_missing",
            target,
            "handoff_consumer_classes names a security route; recovery_options must include open_security_private_triage",
        );
    }
}

fn validate_claim_state(
    violations: &mut Vec<PacketViolation>,
    target: &str,
    claim_state: &ClaimStateBlock,
    custody: &CustodySummary,
) {
    if claim_state.claim_summary.trim().is_empty() {
        push_violation(
            violations,
            "packet.claim_state.empty_summary",
            target,
            "claim_state must declare a non-empty claim_summary",
        );
    }
    let seen: BTreeSet<_> = claim_state.downgrade_tokens.iter().collect();
    if seen.len() != claim_state.downgrade_tokens.len() {
        push_violation(
            violations,
            "packet.claim_state.duplicate_token",
            target,
            "claim_state.downgrade_tokens must be unique",
        );
    }
    if custody.has_held_record && !seen.contains(&ClaimDowngradeToken::HeldRecordBlocksExport) {
        push_violation(
            violations,
            "packet.claim_state.held_record_token_required",
            target,
            "an artifact is under hold; claim_state must include held_record_blocks_export",
        );
    }
    if custody.has_managed_copy
        && !seen.contains(&ClaimDowngradeToken::ManagedCopyPendingAdminReview)
        && !seen.contains(&ClaimDowngradeToken::HeldRecordBlocksExport)
    {
        // managed copy is admissible without a downgrade only when no
        // hold is in effect; we surface the gentler downgrade instead
        // of failing closed so a reviewer sees the right signal.
        push_violation(
            violations,
            "packet.claim_state.managed_copy_token_required",
            target,
            "a managed copy is attached; claim_state must include managed_copy_pending_admin_review",
        );
    }
}

fn validate_privacy_baseline(
    violations: &mut Vec<PacketViolation>,
    target: &str,
    baseline: &PrivacyBaseline,
) {
    if !baseline.raw_private_material_excluded {
        push_violation(
            violations,
            "packet.privacy_baseline.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !baseline.ambient_authority_excluded {
        push_violation(
            violations,
            "packet.privacy_baseline.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if baseline.redaction_baseline_summary.trim().is_empty() {
        push_violation(
            violations,
            "packet.privacy_baseline.empty_summary",
            target,
            "redaction_baseline_summary must be non-empty",
        );
    }
}

fn validate_references(
    violations: &mut Vec<PacketViolation>,
    target: &str,
    references: &ReferencesBlock,
) {
    if references.doc_ref != INCIDENT_WORKSPACE_BETA_PACKET_DOC_REF {
        push_violation(
            violations,
            "packet.references.doc_ref",
            target,
            format!(
                "doc_ref must pin {}",
                INCIDENT_WORKSPACE_BETA_PACKET_DOC_REF
            ),
        );
    }
    if references.schema_ref != INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_REF {
        push_violation(
            violations,
            "packet.references.schema_ref",
            target,
            format!(
                "schema_ref must pin {}",
                INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_REF
            ),
        );
    }
    if references.scenario_corpus_doc_ref != INCIDENT_WORKSPACE_BETA_SCENARIO_CORPUS_DOC_REF {
        push_violation(
            violations,
            "packet.references.scenario_corpus_doc_ref",
            target,
            format!(
                "scenario_corpus_doc_ref must pin {}",
                INCIDENT_WORKSPACE_BETA_SCENARIO_CORPUS_DOC_REF
            ),
        );
    }
    if let Some(alpha_ref) = references.incident_workspace_alpha_doc_ref.as_deref() {
        if alpha_ref != INCIDENT_WORKSPACE_ALPHA_DOC_REF {
            push_violation(
                violations,
                "packet.references.incident_workspace_alpha_doc_ref",
                target,
                format!(
                    "incident_workspace_alpha_doc_ref must pin {}",
                    INCIDENT_WORKSPACE_ALPHA_DOC_REF
                ),
            );
        }
    }
}

fn push_violation(
    violations: &mut Vec<PacketViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(PacketViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}

/// Strongly typed load error.
#[derive(Debug)]
pub enum IncidentWorkspaceBetaPacketLoadError {
    /// YAML parse error.
    Yaml(serde_yaml::Error),
}

impl fmt::Display for IncidentWorkspaceBetaPacketLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "packet yaml parse error: {err}"),
        }
    }
}

impl Error for IncidentWorkspaceBetaPacketLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml(err) => Some(err),
        }
    }
}

impl From<serde_yaml::Error> for IncidentWorkspaceBetaPacketLoadError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

/// Parses one YAML packet record.
///
/// # Errors
///
/// Returns [`IncidentWorkspaceBetaPacketLoadError::Yaml`] when the YAML
/// does not match [`IncidentWorkspaceBetaPacket`].
pub fn load_incident_workspace_beta_packet(
    yaml: &str,
) -> Result<IncidentWorkspaceBetaPacket, IncidentWorkspaceBetaPacketLoadError> {
    serde_yaml::from_str::<IncidentWorkspaceBetaPacket>(yaml)
        .map_err(IncidentWorkspaceBetaPacketLoadError::from)
}

/// Loads the protected checked-in fixture corpus.
///
/// # Errors
///
/// Returns a YAML parse error when a fixture does not match
/// [`IncidentWorkspaceBetaPacket`].
pub fn current_incident_workspace_beta_packet_corpus(
) -> Result<IncidentWorkspaceBetaPacketCorpus, serde_yaml::Error> {
    let entries = FIXTURE_SOURCES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<IncidentWorkspaceBetaPacket>(yaml).map(|packet| {
                IncidentWorkspaceBetaPacketEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    packet,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(IncidentWorkspaceBetaPacketCorpus { entries })
}
