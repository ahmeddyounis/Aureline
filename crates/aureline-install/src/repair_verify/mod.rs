//! Repair, verify, and uninstall diagnostics for install profiles.
//!
//! The records in this module make silent and enterprise install operations
//! supportable without relying on platform-specific installer logs. They carry
//! copyable install ids, timestamps, state-root refs, rollback evidence, and
//! failure summaries as structured diagnostics.

use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use crate::profile_cards::INSTALL_PROFILE_BETA_SCHEMA_VERSION;
use crate::topology::{InstallModeClass, UpdaterOwnerClass};

/// Stable record-kind tag for [`RepairVerifyPacket`].
pub const REPAIR_VERIFY_PACKET_RECORD_KIND: &str = "repair_verify_uninstall_packet";

/// Stable record-kind tag for [`RepairVerifySupportExport`].
pub const REPAIR_VERIFY_SUPPORT_EXPORT_RECORD_KIND: &str = "repair_verify_uninstall_support_export";

/// Install operation kind emitted by repair, verify, and uninstall flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallOperationKind {
    /// Install operation.
    Install,
    /// Update operation.
    Update,
    /// Repair operation.
    Repair,
    /// Verify operation.
    Verify,
    /// Rollback operation.
    Rollback,
    /// Uninstall operation.
    Uninstall,
}

/// Deployment profile class for operation coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationProfileClass {
    /// Interactive self-serve profile.
    Interactive,
    /// Silent install or unattended deployment profile.
    SilentInstall,
    /// Enterprise managed profile.
    EnterpriseManaged,
    /// Portable profile.
    Portable,
}

/// Operation status class for install diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatusClass {
    /// Operation succeeded.
    Success,
    /// Operation partially succeeded.
    PartialSuccess,
    /// Operation failed.
    Failed,
    /// Operation rolled back.
    RolledBack,
    /// Verification failed.
    VerifyFailed,
    /// Operation requires reboot.
    RebootRequired,
}

impl OperationStatusClass {
    /// Returns true when the status requires a human-readable failure summary.
    pub const fn requires_failure_summary(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::RolledBack | Self::VerifyFailed | Self::RebootRequired
        )
    }
}

/// Return-code family for unattended and managed install outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnCodeFamily {
    /// Successful operation.
    Success,
    /// Partial success.
    PartialSuccess,
    /// User configuration error.
    UserConfigError,
    /// Trust or policy denial.
    TrustPolicyDenial,
    /// Missing dependency.
    MissingDependency,
    /// Network transport failure.
    NetworkTransport,
    /// Internal failure.
    InternalFailure,
    /// Rollback is required.
    RollbackRequired,
    /// Verification failed.
    VerificationFailed,
    /// Administrator action is required.
    AdminRequired,
}

impl ReturnCodeFamily {
    /// Returns the stable numeric return code for the family.
    pub const fn numeric_code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::PartialSuccess => 2,
            Self::UserConfigError => 3,
            Self::TrustPolicyDenial => 4,
            Self::MissingDependency => 5,
            Self::NetworkTransport => 6,
            Self::InternalFailure => 7,
            Self::RollbackRequired => 8,
            Self::VerificationFailed => 9,
            Self::AdminRequired => 10,
        }
    }
}

/// Failure reason class for install operation diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureReasonClass {
    /// Signature was invalid.
    SignatureInvalid,
    /// Signing material was revoked.
    SigningMaterialRevoked,
    /// Mirror metadata was stale.
    MirrorMetadataStale,
    /// Policy denied the operation.
    PolicyDenied,
    /// Disk space was exhausted.
    DiskSpaceExhausted,
    /// Path permission was denied.
    PathPermissionDenied,
    /// State roots collided.
    StateRootCollision,
    /// Side-by-side marker was corrupt.
    SideBySideMarkerCorruption,
    /// Portable mode spill was detected.
    PortableSpillDetected,
    /// Network was unreachable where offline operation was required.
    NetworkUnreachableOfflineRequired,
    /// Bootstrap bundle was missing.
    MissingBootstrapBundle,
    /// Toolchain dependency was missing.
    ToolchainMissing,
    /// Platform is unsupported.
    UnsupportedPlatform,
    /// Internal implementation bug.
    InternalBug,
    /// User cancelled the operation.
    UserCancelled,
}

/// Remediation pointer class for operation diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemediationPointerClass {
    /// Run Project Doctor.
    RunProjectDoctor,
    /// Inspect support bundle.
    InspectSupportBundle,
    /// Refresh mirror metadata.
    RefreshMirrorMetadata,
    /// Contact administrator.
    ContactAdmin,
    /// Reissue policy bundle.
    ReissuePolicyBundle,
    /// Retry from an offline bundle.
    RetryFromOfflineBundle,
    /// Reinstall from clean declared install state.
    ReinstallFromCleanState,
    /// Open release notes.
    OpenReleaseNotes,
    /// No remediation required.
    None,
}

/// Redaction class for operation diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationRedactionClass {
    /// No secrets are present.
    NoSecretsPresent,
    /// Secrets were redacted before emit.
    SecretsRedactedBeforeEmit,
    /// Full details are support-bundle only.
    SupportBundleOnly,
}

/// Upstream source refs consumed by a repair/verify packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairVerifySourceRefs {
    /// Install-profile beta packet ref.
    pub install_profile_packet_ref: String,
    /// Install diagnostics packet ref.
    pub install_diagnostics_packet_ref: String,
    /// Silent deployment results ref.
    pub silent_deployment_results_ref: String,
    /// Ring rollout packet ref.
    pub ring_rollout_packet_ref: String,
    /// Repair transaction schema ref.
    pub repair_transaction_schema_ref: String,
    /// Support bundle schema ref.
    pub support_bundle_schema_ref: String,
}

/// One repair, verify, rollback, or uninstall operation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallOperationDiagnostic {
    /// Stable operation id.
    pub operation_id: String,
    /// Operation kind.
    pub operation_kind: InstallOperationKind,
    /// Deployment profile classes covered by this operation.
    pub profile_classes: Vec<OperationProfileClass>,
    /// Operation status.
    pub status: OperationStatusClass,
    /// Return-code family.
    pub return_code_family: ReturnCodeFamily,
    /// Numeric return code.
    pub return_code_numeric: i32,
    /// Copyable install id ref.
    pub install_id_ref: String,
    /// Install-profile card ref.
    pub install_profile_card_ref: String,
    /// Install-topology row id.
    pub topology_row_id: String,
    /// Install diagnostics row ref.
    pub diagnostic_row_ref: String,
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// UTC start timestamp.
    pub started_at: String,
    /// UTC finish timestamp.
    pub finished_at: String,
    /// True when the install id is copyable in human-readable diagnostics.
    pub copyable_install_id: bool,
    /// Human-readable summary.
    pub human_summary: String,
    /// Failure summary when status is failed, verify-failed, rolled-back, or reboot-required.
    pub failure_summary: Option<String>,
    /// Failure reason class.
    pub failure_reason_class: Option<FailureReasonClass>,
    /// Remediation pointer class.
    pub remediation_pointer_class: RemediationPointerClass,
    /// State-root refs associated with the operation.
    pub state_root_refs: Vec<String>,
    /// State-root refs preserved by the operation.
    pub preserved_state_root_refs: Vec<String>,
    /// Install-state refs removed by uninstall or cleanup.
    pub removed_install_state_refs: Vec<String>,
    /// Rollback evidence ref.
    pub rollback_evidence_ref: Option<String>,
    /// Repair transaction ref.
    pub repair_transaction_ref: Option<String>,
    /// Support bundle ref.
    pub support_bundle_ref: String,
    /// Managed package report ref.
    pub managed_package_report_ref: Option<String>,
    /// Fleet ring context ref.
    pub fleet_ring_context_ref: Option<String>,
    /// Redaction class.
    pub redaction_class: OperationRedactionClass,
}

/// Expected uninstall behavior shared by operation diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UninstallBehaviorExpectation {
    /// Stable expectation id.
    pub expectation_id: String,
    /// Install-profile card ref.
    pub install_profile_card_ref: String,
    /// State or marker refs removed by uninstall.
    pub removes: Vec<String>,
    /// User state refs preserved by uninstall.
    pub preserves: Vec<String>,
    /// Human-readable summary.
    pub summary: String,
}

/// Repair, verify, and uninstall diagnostic packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairVerifyPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Source refs consumed by the packet.
    pub source_refs: RepairVerifySourceRefs,
    /// Operation diagnostics.
    pub operations: Vec<InstallOperationDiagnostic>,
    /// Expected uninstall behavior rows.
    pub uninstall_expectations: Vec<UninstallBehaviorExpectation>,
}

impl RepairVerifyPacket {
    /// Validates repair, verify, uninstall, failure-summary, and preservation truth.
    pub fn validate(&self) -> RepairVerifyValidationReport {
        let mut validator = RepairVerifyValidator::new(self);
        validator.validate();
        validator.finish()
    }

    /// Returns a metadata-safe support-export projection.
    pub fn support_export_projection(&self) -> RepairVerifySupportExport {
        RepairVerifySupportExport {
            record_kind: REPAIR_VERIFY_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: INSTALL_PROFILE_BETA_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            source_packet_ref:
                "fixtures/install/m3/profile_cards_and_repair/repair_verify_uninstall_packet.json"
                    .to_string(),
            operations: self
                .operations
                .iter()
                .map(RepairVerifySupportOperationRow::from)
                .collect(),
            uninstall_expectations: self.uninstall_expectations.clone(),
            redaction_class: "metadata_only_no_paths_or_secrets".to_string(),
        }
    }
}

/// Support-export projection for repair, verify, and uninstall diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairVerifySupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source packet id.
    pub packet_id: String,
    /// Source packet ref.
    pub source_packet_ref: String,
    /// Operation support rows.
    pub operations: Vec<RepairVerifySupportOperationRow>,
    /// Uninstall expectations.
    pub uninstall_expectations: Vec<UninstallBehaviorExpectation>,
    /// Redaction class.
    pub redaction_class: String,
}

/// Support-export row for one repair, verify, rollback, or uninstall operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairVerifySupportOperationRow {
    /// Stable operation id.
    pub operation_id: String,
    /// Operation kind.
    pub operation_kind: InstallOperationKind,
    /// Profile classes covered by this operation.
    pub profile_classes: Vec<OperationProfileClass>,
    /// Operation status.
    pub status: OperationStatusClass,
    /// Return-code family.
    pub return_code_family: ReturnCodeFamily,
    /// Copyable install id ref.
    pub install_id_ref: String,
    /// Install-profile card ref.
    pub install_profile_card_ref: String,
    /// Operation finish timestamp.
    pub finished_at: String,
    /// Human-readable summary.
    pub human_summary: String,
    /// Failure summary.
    pub failure_summary: Option<String>,
    /// Preserved state-root refs.
    pub preserved_state_root_refs: Vec<String>,
    /// Removed install-state refs.
    pub removed_install_state_refs: Vec<String>,
    /// Support bundle ref.
    pub support_bundle_ref: String,
}

impl From<&InstallOperationDiagnostic> for RepairVerifySupportOperationRow {
    fn from(operation: &InstallOperationDiagnostic) -> Self {
        Self {
            operation_id: operation.operation_id.clone(),
            operation_kind: operation.operation_kind,
            profile_classes: operation.profile_classes.clone(),
            status: operation.status,
            return_code_family: operation.return_code_family,
            install_id_ref: operation.install_id_ref.clone(),
            install_profile_card_ref: operation.install_profile_card_ref.clone(),
            finished_at: operation.finished_at.clone(),
            human_summary: operation.human_summary.clone(),
            failure_summary: operation.failure_summary.clone(),
            preserved_state_root_refs: operation.preserved_state_root_refs.clone(),
            removed_install_state_refs: operation.removed_install_state_refs.clone(),
            support_bundle_ref: operation.support_bundle_ref.clone(),
        }
    }
}

/// Validation coverage for repair, verify, and uninstall packets.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairVerifyCoverage {
    /// Operation kinds covered.
    pub operation_kinds: BTreeSet<InstallOperationKind>,
    /// Profile classes covered.
    pub profile_classes: BTreeSet<OperationProfileClass>,
    /// Failure reason classes covered.
    pub failure_reason_classes: BTreeSet<FailureReasonClass>,
}

/// One repair/verify validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairVerifyValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable message.
    pub message: String,
    /// Row or packet ref associated with the finding.
    pub ref_id: String,
}

/// Validation report for repair, verify, and uninstall packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairVerifyValidationReport {
    /// True when validation found no errors.
    pub passed: bool,
    /// Coverage collected during validation.
    pub coverage: RepairVerifyCoverage,
    /// Validation findings.
    pub findings: Vec<RepairVerifyValidationFinding>,
}

struct RepairVerifyValidator<'a> {
    packet: &'a RepairVerifyPacket,
    coverage: RepairVerifyCoverage,
    findings: Vec<RepairVerifyValidationFinding>,
    seen_operations: BTreeSet<String>,
}

impl<'a> RepairVerifyValidator<'a> {
    fn new(packet: &'a RepairVerifyPacket) -> Self {
        Self {
            packet,
            coverage: RepairVerifyCoverage::default(),
            findings: Vec::new(),
            seen_operations: BTreeSet::new(),
        }
    }

    fn validate(&mut self) {
        self.validate_header();
        for operation in &self.packet.operations {
            self.validate_operation(operation);
        }
        for expectation in &self.packet.uninstall_expectations {
            self.validate_uninstall_expectation(expectation);
        }
        self.validate_required_coverage();
    }

    fn finish(self) -> RepairVerifyValidationReport {
        RepairVerifyValidationReport {
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn push(&mut self, check_id: &str, message: impl Into<String>, ref_id: impl Into<String>) {
        self.findings.push(RepairVerifyValidationFinding {
            check_id: check_id.to_string(),
            message: message.into(),
            ref_id: ref_id.into(),
        });
    }

    fn validate_header(&mut self) {
        if self.packet.record_kind != REPAIR_VERIFY_PACKET_RECORD_KIND {
            self.push(
                "repair_verify.packet.record_kind",
                "packet record_kind must be repair_verify_uninstall_packet",
                self.packet.packet_id.clone(),
            );
        }
        if self.packet.schema_version != INSTALL_PROFILE_BETA_SCHEMA_VERSION {
            self.push(
                "repair_verify.packet.schema_version",
                "packet schema_version must be 1",
                self.packet.packet_id.clone(),
            );
        }
        if self.packet.operations.is_empty() {
            self.push(
                "repair_verify.packet.operations_empty",
                "packet must contain operation diagnostics",
                self.packet.packet_id.clone(),
            );
        }
        if self.packet.uninstall_expectations.is_empty() {
            self.push(
                "repair_verify.packet.uninstall_expectations_empty",
                "packet must contain uninstall behavior expectations",
                self.packet.packet_id.clone(),
            );
        }
    }

    fn validate_operation(&mut self, operation: &InstallOperationDiagnostic) {
        if !self.seen_operations.insert(operation.operation_id.clone()) {
            self.push(
                "repair_verify.operation.duplicate",
                "operation ids must be unique",
                operation.operation_id.clone(),
            );
        }
        self.coverage
            .operation_kinds
            .insert(operation.operation_kind);
        for profile_class in &operation.profile_classes {
            self.coverage.profile_classes.insert(*profile_class);
        }
        if let Some(reason) = operation.failure_reason_class {
            self.coverage.failure_reason_classes.insert(reason);
        }

        if operation.return_code_numeric != operation.return_code_family.numeric_code() {
            self.push(
                "repair_verify.operation.return_code_mismatch",
                "return_code_numeric must match return_code_family",
                operation.operation_id.clone(),
            );
        }
        if operation.install_id_ref.trim().is_empty()
            || operation.install_profile_card_ref.trim().is_empty()
            || operation.diagnostic_row_ref.trim().is_empty()
            || operation.started_at.trim().is_empty()
            || operation.finished_at.trim().is_empty()
        {
            self.push(
                "repair_verify.operation.identity_missing",
                "operation must carry install id, card ref, diagnostic ref, and timestamps",
                operation.operation_id.clone(),
            );
        }
        if !operation.copyable_install_id || operation.human_summary.trim().is_empty() {
            self.push(
                "repair_verify.operation.human_summary_missing",
                "operation must expose a copyable install id and human-readable summary",
                operation.operation_id.clone(),
            );
        }
        if operation.state_root_refs.is_empty() {
            self.push(
                "repair_verify.operation.state_roots_missing",
                "operation must carry state-root refs",
                operation.operation_id.clone(),
            );
        }
        if operation.status.requires_failure_summary() {
            if operation
                .failure_summary
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
                || operation.failure_reason_class.is_none()
                || operation.remediation_pointer_class == RemediationPointerClass::None
            {
                self.push(
                    "repair_verify.operation.failure_summary_missing",
                    "failed or rolled-back operations must carry failure summary, reason, and remediation",
                    operation.operation_id.clone(),
                );
            }
        } else if operation.failure_reason_class.is_some() {
            self.push(
                "repair_verify.operation.success_has_failure_reason",
                "successful operations must not carry a failure reason class",
                operation.operation_id.clone(),
            );
        }
        if operation.support_bundle_ref.trim().is_empty() {
            self.push(
                "repair_verify.operation.support_ref_missing",
                "operation diagnostics must carry a support bundle ref",
                operation.operation_id.clone(),
            );
        }
        if operation.operation_kind == InstallOperationKind::Repair
            && operation.repair_transaction_ref.is_none()
        {
            self.push(
                "repair_verify.operation.repair_ref_missing",
                "repair operations must reference a repair transaction",
                operation.operation_id.clone(),
            );
        }
        if operation.operation_kind == InstallOperationKind::Uninstall {
            self.validate_uninstall_operation(operation);
        }
    }

    fn validate_uninstall_operation(&mut self, operation: &InstallOperationDiagnostic) {
        if operation.preserved_state_root_refs.is_empty()
            || operation.removed_install_state_refs.is_empty()
        {
            self.push(
                "repair_verify.uninstall.preservation_missing",
                "uninstall operations must declare preserved user state and removed install state",
                operation.operation_id.clone(),
            );
        }
        if !operation
            .preserved_state_root_refs
            .iter()
            .any(|root| root.contains("configuration_root") || root.contains("recovery_root"))
        {
            self.push(
                "repair_verify.uninstall.user_state_not_preserved",
                "uninstall operations must preserve user configuration or recovery roots",
                operation.operation_id.clone(),
            );
        }
    }

    fn validate_uninstall_expectation(&mut self, expectation: &UninstallBehaviorExpectation) {
        if expectation.removes.is_empty()
            || expectation.preserves.is_empty()
            || expectation.summary.trim().is_empty()
        {
            self.push(
                "repair_verify.uninstall_expectation.incomplete",
                "uninstall expectations must declare removed and preserved state",
                expectation.expectation_id.clone(),
            );
        }
    }

    fn validate_required_coverage(&mut self) {
        let mut coverage_by_profile: HashMap<
            OperationProfileClass,
            BTreeSet<InstallOperationKind>,
        > = HashMap::new();
        for operation in &self.packet.operations {
            for profile_class in &operation.profile_classes {
                coverage_by_profile
                    .entry(*profile_class)
                    .or_default()
                    .insert(operation.operation_kind);
            }
        }
        for profile_class in [
            OperationProfileClass::EnterpriseManaged,
            OperationProfileClass::SilentInstall,
        ] {
            let kinds = coverage_by_profile.get(&profile_class);
            for required in [
                InstallOperationKind::Repair,
                InstallOperationKind::Verify,
                InstallOperationKind::Uninstall,
            ] {
                if !kinds.is_some_and(|covered| covered.contains(&required)) {
                    self.push(
                        "repair_verify.coverage.required_operation_missing",
                        format!("{profile_class:?} must cover {required:?}"),
                        self.packet.packet_id.clone(),
                    );
                }
            }
        }
        if self.coverage.failure_reason_classes.is_empty() {
            self.push(
                "repair_verify.coverage.failure_summary_missing",
                "packet must include at least one failed diagnostic with failure summary",
                self.packet.packet_id.clone(),
            );
        }
    }
}
