//! Default-redacted support and incident export profile review.
//!
//! This module owns the typed [`SupportExportRedactionProfile`] and the
//! [`SupportExportReopenManifest`] records that pin the protected
//! default-redacted posture used by stable-facing support and incident
//! exports. The seed corpus lives at
//! `/fixtures/support/m3/redaction_and_escalation/` and the boundary
//! schema lives at
//! `/schemas/support/export_redaction_profile.schema.json`.
//!
//! The seed is intentionally narrow:
//!
//! - the profile MUST preserve exact-build identity, scenario family,
//!   channel/platform identity, and doctor finding codes;
//! - crash manifest refs, symbolication report refs, repair history
//!   refs, support-bundle refs, and incident-workspace packet refs cross
//!   by stable id only;
//! - raw dumps, raw traces, raw logs, raw transcripts, code-adjacent
//!   attachments, full shell history, secret-bearing material, and
//!   ambient credentials never embed by default;
//! - widening any code-adjacent or high-risk evidence class requires a
//!   recorded `broaden_evidence_review_ref`;
//! - the local-only save/copy path is always available and equal in
//!   prominence to any upload/handoff path;
//! - the reopen manifest preserves included/excluded class lists, build
//!   identity, destination class, and whether the export ever left the
//!   machine.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a default-redacted export profile.
pub const SUPPORT_EXPORT_REDACTION_PROFILE_RECORD_KIND: &str =
    "support_export_redaction_profile_record";

/// Stable record-kind tag for a default-redacted export profile fixture case.
pub const SUPPORT_EXPORT_REDACTION_PROFILE_SEED_CASE_RECORD_KIND: &str =
    "support_export_redaction_profile_seed_case_record";

/// Stable record-kind tag for a reopen manifest.
pub const SUPPORT_EXPORT_REOPEN_MANIFEST_RECORD_KIND: &str =
    "support_export_reopen_manifest_record";

/// Stable record-kind tag for a reopen manifest fixture case.
pub const SUPPORT_EXPORT_REOPEN_MANIFEST_SEED_CASE_RECORD_KIND: &str =
    "support_export_reopen_manifest_seed_case_record";

/// Frozen collection-schema version emitted by this seed.
pub const SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path to the boundary schema.
pub const SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path to the seed-corpus directory.
pub const SUPPORT_EXPORT_REDACTION_PROFILE_CORPUS_DIR: &str =
    "fixtures/support/m3/redaction_and_escalation";

/// Repo-relative path to the reviewer doc.
pub const SUPPORT_EXPORT_REDACTION_PROFILE_DOC_REF: &str =
    "docs/support/m3/redaction_default_exports.md";

const PROFILE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/support/m3/redaction_and_escalation/default_redacted_profile.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/redaction_and_escalation/default_redacted_profile.yaml"
        )),
    ),
    (
        "fixtures/support/m3/redaction_and_escalation/vendor_handoff_profile.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/redaction_and_escalation/vendor_handoff_profile.yaml"
        )),
    ),
    (
        "fixtures/support/m3/redaction_and_escalation/broaden_evidence_review_required.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/redaction_and_escalation/broaden_evidence_review_required.yaml"
        )),
    ),
];

const REOPEN_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/support/m3/redaction_and_escalation/reopen_manifest_local_only.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/redaction_and_escalation/reopen_manifest_local_only.yaml"
        )),
    ),
    (
        "fixtures/support/m3/redaction_and_escalation/reopen_manifest_vendor_handoff.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/redaction_and_escalation/reopen_manifest_vendor_handoff.yaml"
        )),
    ),
];

/// Closed scenario-family vocabulary mirroring
/// `schemas/support/scenario_picker.schema.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioFamilyClass {
    ExecutionContextMismatch,
    TrustPolicyIdentityApprovalBlock,
    NetworkCaProxyMirrorFailure,
    ExtensionOrHostRegression,
    StateCorruptionSchemaDriftLowDiskRecovery,
    RemoteRouteCollaborationMismatch,
}

impl ScenarioFamilyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionContextMismatch => "execution_context_mismatch",
            Self::TrustPolicyIdentityApprovalBlock => "trust_policy_identity_approval_block",
            Self::NetworkCaProxyMirrorFailure => "network_ca_proxy_mirror_failure",
            Self::ExtensionOrHostRegression => "extension_or_host_regression",
            Self::StateCorruptionSchemaDriftLowDiskRecovery => {
                "state_corruption_schema_drift_low_disk_recovery"
            }
            Self::RemoteRouteCollaborationMismatch => "remote_route_collaboration_mismatch",
        }
    }
}

/// Closed release-channel vocabulary mirroring the export-profile schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannelClass {
    Stable,
    Preview,
    Beta,
    Lts,
    PortableStable,
    PortablePreview,
    DevLocal,
}

impl ReleaseChannelClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Lts => "lts",
            Self::PortableStable => "portable_stable",
            Self::PortablePreview => "portable_preview",
            Self::DevLocal => "dev_local",
        }
    }
}

/// Closed platform vocabulary mirroring the export-profile schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlatformClass {
    #[serde(rename = "macos_arm64")]
    MacosArm64,
    #[serde(rename = "macos_x86_64")]
    MacosX8664,
    #[serde(rename = "linux_x86_64")]
    LinuxX8664,
    #[serde(rename = "linux_arm64")]
    LinuxArm64,
    #[serde(rename = "windows_x86_64")]
    WindowsX8664,
    #[serde(rename = "windows_arm64")]
    WindowsArm64,
    #[serde(rename = "portable_unspecified")]
    PortableUnspecified,
}

impl PlatformClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacosArm64 => "macos_arm64",
            Self::MacosX8664 => "macos_x86_64",
            Self::LinuxX8664 => "linux_x86_64",
            Self::LinuxArm64 => "linux_arm64",
            Self::WindowsX8664 => "windows_x86_64",
            Self::WindowsArm64 => "windows_arm64",
            Self::PortableUnspecified => "portable_unspecified",
        }
    }
}

/// Closed destination-class vocabulary mirroring
/// `schemas/support/escalation_packet.schema.json` delivery_path_class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationClass {
    LocalOnlyReview,
    VendorCaseHandoff,
    UserInitiatedUpload,
    ManagedAdminHandoff,
    PrivateSecurityChannel,
}

impl DestinationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyReview => "local_only_review",
            Self::VendorCaseHandoff => "vendor_case_handoff",
            Self::UserInitiatedUpload => "user_initiated_upload",
            Self::ManagedAdminHandoff => "managed_admin_handoff",
            Self::PrivateSecurityChannel => "private_security_channel",
        }
    }
}

/// Closed data-class-boundary vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataClassBoundaryClass {
    MetadataOnly,
    EnvironmentAdjacent,
    CodeAdjacent,
    HighRisk,
}

impl DataClassBoundaryClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::EnvironmentAdjacent => "environment_adjacent",
            Self::CodeAdjacent => "code_adjacent",
            Self::HighRisk => "high_risk",
        }
    }
}

/// Closed evidence-class vocabulary the export-profile schema declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    BuildIdentity,
    ChannelAndPlatformIdentity,
    ScenarioFamily,
    DoctorFindingCodes,
    RepairHistoryRefs,
    CrashManifestRefs,
    SymbolicationReportRefs,
    SupportBundleRefs,
    IncidentWorkspacePacketRefs,
    PolicyAndTrustState,
    RawCrashDumpPayload,
    RawTraceCapture,
    RawLogExcerpt,
    RawTranscriptExcerpt,
    CodeSnippetAttachment,
    NotebookCellAttachment,
    MutationJournalExcerpt,
    FullShellHistoryCapture,
    SecretBearingMaterial,
    AmbientCredentialMaterial,
}

impl EvidenceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIdentity => "build_identity",
            Self::ChannelAndPlatformIdentity => "channel_and_platform_identity",
            Self::ScenarioFamily => "scenario_family",
            Self::DoctorFindingCodes => "doctor_finding_codes",
            Self::RepairHistoryRefs => "repair_history_refs",
            Self::CrashManifestRefs => "crash_manifest_refs",
            Self::SymbolicationReportRefs => "symbolication_report_refs",
            Self::SupportBundleRefs => "support_bundle_refs",
            Self::IncidentWorkspacePacketRefs => "incident_workspace_packet_refs",
            Self::PolicyAndTrustState => "policy_and_trust_state",
            Self::RawCrashDumpPayload => "raw_crash_dump_payload",
            Self::RawTraceCapture => "raw_trace_capture",
            Self::RawLogExcerpt => "raw_log_excerpt",
            Self::RawTranscriptExcerpt => "raw_transcript_excerpt",
            Self::CodeSnippetAttachment => "code_snippet_attachment",
            Self::NotebookCellAttachment => "notebook_cell_attachment",
            Self::MutationJournalExcerpt => "mutation_journal_excerpt",
            Self::FullShellHistoryCapture => "full_shell_history_capture",
            Self::SecretBearingMaterial => "secret_bearing_material",
            Self::AmbientCredentialMaterial => "ambient_credential_material",
        }
    }

    /// True when this class is code-adjacent — broadening any of these
    /// past their default handling requires a recorded broaden-evidence
    /// review marker.
    pub const fn is_code_adjacent(self) -> bool {
        matches!(
            self,
            Self::CodeSnippetAttachment
                | Self::NotebookCellAttachment
                | Self::MutationJournalExcerpt
        )
    }

    /// True when this class is prohibited from crossing the boundary
    /// under any inclusion choice.
    pub const fn is_always_prohibited(self) -> bool {
        matches!(
            self,
            Self::FullShellHistoryCapture
                | Self::SecretBearingMaterial
                | Self::AmbientCredentialMaterial
        )
    }
}

/// Closed inclusion-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceInclusionClass {
    EmbeddedMetadataOnly,
    EmbeddedByReference,
    RetainedLocalOnly,
    ExcludedByDefault,
    ExcludedAlways,
}

impl EvidenceInclusionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedMetadataOnly => "embedded_metadata_only",
            Self::EmbeddedByReference => "embedded_by_reference",
            Self::RetainedLocalOnly => "retained_local_only",
            Self::ExcludedByDefault => "excluded_by_default",
            Self::ExcludedAlways => "excluded_always",
        }
    }

    /// True when the export-side projection includes this class either as
    /// metadata or by reference.
    pub const fn is_included(self) -> bool {
        matches!(self, Self::EmbeddedMetadataOnly | Self::EmbeddedByReference)
    }
}

/// Closed broaden-evidence-review vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BroadenEvidenceReviewClass {
    NotRequired,
    RequiresExplicitReview,
    Prohibited,
}

impl BroadenEvidenceReviewClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::RequiresExplicitReview => "requires_explicit_review",
            Self::Prohibited => "prohibited",
        }
    }
}

/// Default-required evidence class — must be preserved by every
/// default-redacted profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultRequiredEvidenceClass {
    BuildIdentity,
    ChannelAndPlatformIdentity,
    ScenarioFamily,
    DoctorFindingCodes,
}

impl DefaultRequiredEvidenceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIdentity => "build_identity",
            Self::ChannelAndPlatformIdentity => "channel_and_platform_identity",
            Self::ScenarioFamily => "scenario_family",
            Self::DoctorFindingCodes => "doctor_finding_codes",
        }
    }

    /// The matching [`EvidenceClass`] token.
    pub const fn as_evidence_class(self) -> EvidenceClass {
        match self {
            Self::BuildIdentity => EvidenceClass::BuildIdentity,
            Self::ChannelAndPlatformIdentity => EvidenceClass::ChannelAndPlatformIdentity,
            Self::ScenarioFamily => EvidenceClass::ScenarioFamily,
            Self::DoctorFindingCodes => EvidenceClass::DoctorFindingCodes,
        }
    }
}

/// Closed list of evidence classes that MUST be preserved on every
/// default-redacted profile.
pub const REQUIRED_EVIDENCE_CLASSES: [DefaultRequiredEvidenceClass; 4] = [
    DefaultRequiredEvidenceClass::BuildIdentity,
    DefaultRequiredEvidenceClass::ChannelAndPlatformIdentity,
    DefaultRequiredEvidenceClass::ScenarioFamily,
    DefaultRequiredEvidenceClass::DoctorFindingCodes,
];

/// Exact-build identity block carried on every profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIdentityBlock {
    pub exact_build_identity_ref: String,
    pub release_channel_class: ReleaseChannelClass,
    pub platform_class: PlatformClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_build_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
    pub build_identity_summary: String,
}

/// Evidence-class rule row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceClassRuleRow {
    pub evidence_class: EvidenceClass,
    pub inclusion_class: EvidenceInclusionClass,
    pub default_data_class_boundary: DataClassBoundaryClass,
    pub broaden_review_class: BroadenEvidenceReviewClass,
    #[serde(default)]
    pub carries_raw_body_by_default: bool,
    pub rule_summary: String,
}

/// Crash linkage block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLinkageBlock {
    pub crash_manifest_refs: Vec<String>,
    pub symbolication_report_refs: Vec<String>,
    pub raw_dump_attached: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkage_summary: Option<String>,
}

/// Destination-posture block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestinationPostureBlock {
    pub selected_destination_class: DestinationClass,
    pub local_only_path_available: bool,
    pub local_only_equal_prominence: bool,
    pub destination_summary: String,
}

/// Reopen-path block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenPathBlock {
    pub reopen_supported: bool,
    pub manifest_ref: String,
    pub shows_included_classes: bool,
    pub shows_excluded_classes: bool,
    pub shows_build_identity: bool,
    pub shows_destination_class: bool,
    pub reopen_summary: String,
}

/// Governance bindings block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceBindingsBlock {
    pub support_bundle_schema_ref: String,
    pub escalation_packet_schema_ref: String,
    pub scenario_picker_schema_ref: String,
    pub doctor_finding_schema_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_default_doc_ref: Option<String>,
}

/// Default-redacted export profile record. Mirrors
/// `support_export_redaction_profile_record` in the boundary schema.
///
/// The same shape backs both the live record (`record_kind =
/// support_export_redaction_profile_record` with `profile_id`) and the
/// fixture seed (`record_kind = support_export_redaction_profile_seed_case_record`
/// with `case_id`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportRedactionProfile {
    pub schema_version: u32,
    pub record_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    pub title: String,
    pub summary: String,
    pub scenario_family_class: ScenarioFamilyClass,
    pub build_identity: BuildIdentityBlock,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub doctor_finding_codes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repair_history_refs: Vec<String>,
    pub evidence_class_rules: Vec<EvidenceClassRuleRow>,
    pub default_required_evidence_classes: Vec<DefaultRequiredEvidenceClass>,
    pub crash_linkage: CrashLinkageBlock,
    pub destination_posture: DestinationPostureBlock,
    pub reopen_path: ReopenPathBlock,
    pub raw_dumps_attached: bool,
    pub raw_transcripts_attached: bool,
    pub code_adjacent_attached: bool,
    pub secret_bearing_attached: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broaden_evidence_review_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub governance_bindings: GovernanceBindingsBlock,
    pub emitted_at: String,
}

impl SupportExportRedactionProfile {
    /// Stable id for this profile — uses `profile_id` for live records and
    /// `case_id` for fixture seeds.
    pub fn id(&self) -> &str {
        self.profile_id
            .as_deref()
            .or(self.case_id.as_deref())
            .unwrap_or("")
    }

    /// True when no raw or code-adjacent class is attached and no
    /// prohibited class is widened.
    pub fn is_default_redacted(&self) -> bool {
        !self.raw_dumps_attached
            && !self.raw_transcripts_attached
            && !self.code_adjacent_attached
            && !self.secret_bearing_attached
    }

    /// True when the local-only path is preserved at equal prominence.
    pub fn preserves_local_only_path(&self) -> bool {
        self.destination_posture.local_only_path_available
            && self.destination_posture.local_only_equal_prominence
    }

    /// True when every default-required evidence class is included with
    /// either metadata or by-reference inclusion.
    pub fn preserves_default_required_classes(&self) -> bool {
        REQUIRED_EVIDENCE_CLASSES.iter().all(|required| {
            self.default_required_evidence_classes.contains(required)
                && self.evidence_class_rules.iter().any(|row| {
                    row.evidence_class == required.as_evidence_class()
                        && row.inclusion_class.is_included()
                })
        })
    }

    /// Returns every included evidence class in the order it appears in
    /// `evidence_class_rules`.
    pub fn included_evidence_classes(&self) -> Vec<EvidenceClass> {
        self.evidence_class_rules
            .iter()
            .filter(|row| row.inclusion_class.is_included())
            .map(|row| row.evidence_class)
            .collect()
    }

    /// Returns every excluded or retained-local-only evidence class in
    /// the order it appears in `evidence_class_rules`.
    pub fn excluded_evidence_classes(&self) -> Vec<EvidenceClass> {
        self.evidence_class_rules
            .iter()
            .filter(|row| !row.inclusion_class.is_included())
            .map(|row| row.evidence_class)
            .collect()
    }

    /// Validate the profile against the protected default-redacted
    /// posture. Returns a [`SupportExportRedactionError`] describing the
    /// first violation found.
    pub fn validate(&self) -> Result<(), SupportExportRedactionError> {
        if self.schema_version != SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_VERSION {
            return Err(SupportExportRedactionError::UnsupportedSchemaVersion {
                actual: self.schema_version,
                expected: SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_VERSION,
            });
        }
        if self.record_kind != SUPPORT_EXPORT_REDACTION_PROFILE_RECORD_KIND
            && self.record_kind != SUPPORT_EXPORT_REDACTION_PROFILE_SEED_CASE_RECORD_KIND
        {
            return Err(SupportExportRedactionError::UnknownRecordKind(
                self.record_kind.clone(),
            ));
        }
        if self
            .build_identity
            .exact_build_identity_ref
            .trim()
            .is_empty()
        {
            return Err(SupportExportRedactionError::MissingExactBuildIdentity);
        }
        if self.raw_dumps_attached {
            return Err(SupportExportRedactionError::RawDumpAttached);
        }
        if self.raw_transcripts_attached {
            return Err(SupportExportRedactionError::RawTranscriptAttached);
        }
        if self.code_adjacent_attached {
            return Err(SupportExportRedactionError::CodeAdjacentAttached);
        }
        if self.secret_bearing_attached {
            return Err(SupportExportRedactionError::SecretBearingAttached);
        }
        if self.crash_linkage.raw_dump_attached {
            return Err(SupportExportRedactionError::RawDumpAttached);
        }
        if !self.preserves_local_only_path() {
            return Err(SupportExportRedactionError::LocalOnlyPathHidden);
        }
        for required in &REQUIRED_EVIDENCE_CLASSES {
            if !self.default_required_evidence_classes.contains(required) {
                return Err(SupportExportRedactionError::MissingRequiredEvidenceClass(
                    *required,
                ));
            }
        }
        for row in &self.evidence_class_rules {
            if row.evidence_class.is_always_prohibited()
                && row.inclusion_class != EvidenceInclusionClass::ExcludedAlways
            {
                return Err(SupportExportRedactionError::ProhibitedClassNotExcluded(
                    row.evidence_class,
                ));
            }
            if row.evidence_class.is_always_prohibited()
                && row.broaden_review_class != BroadenEvidenceReviewClass::Prohibited
            {
                return Err(SupportExportRedactionError::ProhibitedClassWidenable(
                    row.evidence_class,
                ));
            }
            if row.evidence_class.is_code_adjacent()
                && row.inclusion_class.is_included()
                && self.broaden_evidence_review_ref.is_none()
            {
                return Err(
                    SupportExportRedactionError::CodeAdjacentWidenedWithoutReview(
                        row.evidence_class,
                    ),
                );
            }
            if row.carries_raw_body_by_default
                && matches!(
                    row.default_data_class_boundary,
                    DataClassBoundaryClass::CodeAdjacent | DataClassBoundaryClass::HighRisk,
                )
            {
                return Err(SupportExportRedactionError::RawBodyOnHighRiskClass(
                    row.evidence_class,
                ));
            }
        }
        for required in &REQUIRED_EVIDENCE_CLASSES {
            let evidence_class = required.as_evidence_class();
            let included = self.evidence_class_rules.iter().any(|row| {
                row.evidence_class == evidence_class && row.inclusion_class.is_included()
            });
            if !included {
                return Err(
                    SupportExportRedactionError::RequiredEvidenceClassNotIncluded(*required),
                );
            }
        }
        Ok(())
    }
}

/// Reopen-after-export manifest. Mirrors
/// `support_export_reopen_manifest_record` in the boundary schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportReopenManifest {
    pub schema_version: u32,
    pub record_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    pub profile_ref: String,
    pub scenario_family_class: ScenarioFamilyClass,
    pub build_identity: BuildIdentityBlock,
    pub included_evidence_classes: Vec<EvidenceClass>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_evidence_classes: Vec<EvidenceClass>,
    pub destination_class: DestinationClass,
    #[serde(default)]
    pub exported_at_or_null: Option<String>,
    pub local_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broaden_evidence_review_ref: Option<String>,
    pub summary: String,
    pub emitted_at: String,
}

impl SupportExportReopenManifest {
    /// Stable id for this manifest — uses `manifest_id` for live records
    /// and `case_id` for fixture seeds.
    pub fn id(&self) -> &str {
        self.manifest_id
            .as_deref()
            .or(self.case_id.as_deref())
            .unwrap_or("")
    }

    /// True when the manifest reports a local-only export.
    pub fn is_local_only(&self) -> bool {
        self.local_only
            && self.destination_class == DestinationClass::LocalOnlyReview
            && self.exported_at_or_null.is_none()
    }

    /// True when the manifest preserves every default-required evidence
    /// class on the `included` side.
    pub fn preserves_default_required_classes(&self) -> bool {
        REQUIRED_EVIDENCE_CLASSES.iter().all(|required| {
            self.included_evidence_classes
                .contains(&required.as_evidence_class())
        })
    }

    /// Validate the manifest against the reopen-truth posture. The
    /// reopen manifest is the truthful inspectable evidence of what was
    /// shared and why.
    pub fn validate(&self) -> Result<(), SupportExportRedactionError> {
        if self.schema_version != SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_VERSION {
            return Err(SupportExportRedactionError::UnsupportedSchemaVersion {
                actual: self.schema_version,
                expected: SUPPORT_EXPORT_REDACTION_PROFILE_SCHEMA_VERSION,
            });
        }
        if self.record_kind != SUPPORT_EXPORT_REOPEN_MANIFEST_RECORD_KIND
            && self.record_kind != SUPPORT_EXPORT_REOPEN_MANIFEST_SEED_CASE_RECORD_KIND
        {
            return Err(SupportExportRedactionError::UnknownRecordKind(
                self.record_kind.clone(),
            ));
        }
        if self.profile_ref.trim().is_empty() {
            return Err(SupportExportRedactionError::ReopenMissingProfileRef);
        }
        if self
            .build_identity
            .exact_build_identity_ref
            .trim()
            .is_empty()
        {
            return Err(SupportExportRedactionError::MissingExactBuildIdentity);
        }
        for class in &self.included_evidence_classes {
            if class.is_always_prohibited() {
                return Err(SupportExportRedactionError::ProhibitedClassNotExcluded(
                    *class,
                ));
            }
            if class.is_code_adjacent() && self.broaden_evidence_review_ref.is_none() {
                return Err(SupportExportRedactionError::CodeAdjacentWidenedWithoutReview(*class));
            }
        }
        if !self.preserves_default_required_classes() {
            for required in &REQUIRED_EVIDENCE_CLASSES {
                if !self
                    .included_evidence_classes
                    .contains(&required.as_evidence_class())
                {
                    return Err(
                        SupportExportRedactionError::RequiredEvidenceClassNotIncluded(*required),
                    );
                }
            }
        }
        if self.destination_class == DestinationClass::LocalOnlyReview
            && self.exported_at_or_null.is_some()
        {
            return Err(SupportExportRedactionError::LocalOnlyManifestExported);
        }
        if self.destination_class != DestinationClass::LocalOnlyReview && self.local_only {
            return Err(SupportExportRedactionError::LocalOnlyDestinationMismatch);
        }
        Ok(())
    }
}

/// Inspectable review projection for the chrome and the export writer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscalationPacketReview {
    pub profile_id: String,
    pub scenario_family_class: ScenarioFamilyClass,
    pub exact_build_identity_ref: String,
    pub release_channel_class: ReleaseChannelClass,
    pub platform_class: PlatformClass,
    pub doctor_finding_codes: Vec<String>,
    pub repair_history_refs: Vec<String>,
    pub crash_manifest_refs: Vec<String>,
    pub symbolication_report_refs: Vec<String>,
    pub included_evidence_classes: Vec<EvidenceClass>,
    pub excluded_evidence_classes: Vec<EvidenceClass>,
    pub destination_class: DestinationClass,
    pub local_only_equal_prominence: bool,
    pub raw_dump_attached: bool,
    pub raw_transcripts_attached: bool,
    pub code_adjacent_attached: bool,
    pub secret_bearing_attached: bool,
    pub broaden_evidence_review_ref: Option<String>,
    pub reopen_manifest_ref: String,
    pub summary: String,
}

impl EscalationPacketReview {
    /// Project the inspectable review surface from a validated profile.
    pub fn from_profile(profile: &SupportExportRedactionProfile) -> Self {
        Self {
            profile_id: profile.id().to_owned(),
            scenario_family_class: profile.scenario_family_class,
            exact_build_identity_ref: profile.build_identity.exact_build_identity_ref.clone(),
            release_channel_class: profile.build_identity.release_channel_class,
            platform_class: profile.build_identity.platform_class,
            doctor_finding_codes: profile.doctor_finding_codes.clone(),
            repair_history_refs: profile.repair_history_refs.clone(),
            crash_manifest_refs: profile.crash_linkage.crash_manifest_refs.clone(),
            symbolication_report_refs: profile.crash_linkage.symbolication_report_refs.clone(),
            included_evidence_classes: profile.included_evidence_classes(),
            excluded_evidence_classes: profile.excluded_evidence_classes(),
            destination_class: profile.destination_posture.selected_destination_class,
            local_only_equal_prominence: profile.destination_posture.local_only_equal_prominence,
            raw_dump_attached: profile.crash_linkage.raw_dump_attached
                || profile.raw_dumps_attached,
            raw_transcripts_attached: profile.raw_transcripts_attached,
            code_adjacent_attached: profile.code_adjacent_attached,
            secret_bearing_attached: profile.secret_bearing_attached,
            broaden_evidence_review_ref: profile.broaden_evidence_review_ref.clone(),
            reopen_manifest_ref: profile.reopen_path.manifest_ref.clone(),
            summary: profile.summary.clone(),
        }
    }
}

/// Validation error returned by the profile and reopen-manifest evaluators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportExportRedactionError {
    /// Profile schema version is not the one supported by this seed.
    UnsupportedSchemaVersion { actual: u32, expected: u32 },
    /// Record-kind tag is not recognized.
    UnknownRecordKind(String),
    /// Exact-build identity ref is empty.
    MissingExactBuildIdentity,
    /// Profile claims a raw dump is attached to the export body.
    RawDumpAttached,
    /// Profile claims a raw transcript is attached to the export body.
    RawTranscriptAttached,
    /// Profile claims a code-adjacent body is embedded.
    CodeAdjacentAttached,
    /// Profile claims a secret-bearing body is embedded.
    SecretBearingAttached,
    /// Local-only save/copy path is hidden or rendered below equal prominence.
    LocalOnlyPathHidden,
    /// A default-required evidence class is missing from the profile.
    MissingRequiredEvidenceClass(DefaultRequiredEvidenceClass),
    /// A default-required evidence class is declared but not included by
    /// any evidence-class rule.
    RequiredEvidenceClassNotIncluded(DefaultRequiredEvidenceClass),
    /// A class that must always be excluded is widened past
    /// `excluded_always`.
    ProhibitedClassNotExcluded(EvidenceClass),
    /// A class that must always be excluded carries a non-prohibited
    /// broaden-review class.
    ProhibitedClassWidenable(EvidenceClass),
    /// A code-adjacent class is widened past its default handling
    /// without a recorded broaden-evidence review marker.
    CodeAdjacentWidenedWithoutReview(EvidenceClass),
    /// A rule declares the class carries a raw body by default but the
    /// class is code-adjacent or high-risk.
    RawBodyOnHighRiskClass(EvidenceClass),
    /// Could not parse YAML/JSON fixture content.
    InvalidFixture {
        fixture: &'static str,
        message: String,
    },
    /// Reopen manifest is missing its profile ref.
    ReopenMissingProfileRef,
    /// Reopen manifest claims `local_only_review` destination but also
    /// records an export timestamp.
    LocalOnlyManifestExported,
    /// Reopen manifest claims `local_only = true` but does not target
    /// the `local_only_review` destination.
    LocalOnlyDestinationMismatch,
}

impl fmt::Display for SupportExportRedactionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual, expected } => write!(
                formatter,
                "support export redaction profile schema version mismatch: actual {actual}, expected {expected}",
            ),
            Self::UnknownRecordKind(kind) => write!(
                formatter,
                "support export redaction profile record_kind not recognized: {kind}",
            ),
            Self::MissingExactBuildIdentity => formatter
                .write_str("support export redaction profile is missing the exact-build identity ref"),
            Self::RawDumpAttached => formatter
                .write_str("support export redaction profile attaches a raw dump payload; raw_dump_attached must be false"),
            Self::RawTranscriptAttached => formatter
                .write_str("support export redaction profile attaches raw transcripts; raw_transcripts_attached must be false"),
            Self::CodeAdjacentAttached => formatter
                .write_str("support export redaction profile attaches code-adjacent bodies; code_adjacent_attached must be false"),
            Self::SecretBearingAttached => formatter
                .write_str("support export redaction profile attaches secret-bearing material; secret_bearing_attached must be false"),
            Self::LocalOnlyPathHidden => formatter
                .write_str("support export redaction profile does not preserve the local-only path at equal prominence"),
            Self::MissingRequiredEvidenceClass(class) => write!(
                formatter,
                "support export redaction profile drops the required evidence class {}",
                class.as_str(),
            ),
            Self::RequiredEvidenceClassNotIncluded(class) => write!(
                formatter,
                "support export redaction profile declares but does not include the required evidence class {}",
                class.as_str(),
            ),
            Self::ProhibitedClassNotExcluded(class) => write!(
                formatter,
                "support export redaction profile must always exclude the prohibited class {}",
                class.as_str(),
            ),
            Self::ProhibitedClassWidenable(class) => write!(
                formatter,
                "support export redaction profile must mark the prohibited class {} as broaden_review_class = prohibited",
                class.as_str(),
            ),
            Self::CodeAdjacentWidenedWithoutReview(class) => write!(
                formatter,
                "support export redaction profile widens the code-adjacent class {} without an explicit broaden-evidence review marker",
                class.as_str(),
            ),
            Self::RawBodyOnHighRiskClass(class) => write!(
                formatter,
                "support export redaction profile claims the high-risk or code-adjacent class {} carries a raw body by default",
                class.as_str(),
            ),
            Self::InvalidFixture { fixture, message } => write!(
                formatter,
                "could not parse support export redaction fixture {fixture}: {message}",
            ),
            Self::ReopenMissingProfileRef => formatter
                .write_str("support export reopen manifest is missing its profile_ref"),
            Self::LocalOnlyManifestExported => formatter
                .write_str("support export reopen manifest claims a local-only destination but also records an export timestamp"),
            Self::LocalOnlyDestinationMismatch => formatter
                .write_str("support export reopen manifest claims local_only = true but selects a non-local destination class"),
        }
    }
}

impl Error for SupportExportRedactionError {}

/// Load every checked-in default-redacted profile fixture.
///
/// # Errors
///
/// Returns [`SupportExportRedactionError`] when any fixture fails to
/// parse or fails the default-redacted posture check.
pub fn load_profile_corpus(
) -> Result<Vec<SupportExportRedactionProfile>, SupportExportRedactionError> {
    let mut profiles = Vec::with_capacity(PROFILE_FIXTURES.len());
    for (path, contents) in PROFILE_FIXTURES {
        let profile: SupportExportRedactionProfile =
            serde_yaml::from_str(contents).map_err(|err| {
                SupportExportRedactionError::InvalidFixture {
                    fixture: path,
                    message: err.to_string(),
                }
            })?;
        profile.validate()?;
        profiles.push(profile);
    }
    Ok(profiles)
}

/// Load every checked-in reopen-manifest fixture.
///
/// # Errors
///
/// Returns [`SupportExportRedactionError`] when any fixture fails to
/// parse or fails the reopen-truth posture check.
pub fn load_reopen_corpus() -> Result<Vec<SupportExportReopenManifest>, SupportExportRedactionError>
{
    let mut manifests = Vec::with_capacity(REOPEN_FIXTURES.len());
    for (path, contents) in REOPEN_FIXTURES {
        let manifest: SupportExportReopenManifest =
            serde_yaml::from_str(contents).map_err(|err| {
                SupportExportRedactionError::InvalidFixture {
                    fixture: path,
                    message: err.to_string(),
                }
            })?;
        manifest.validate()?;
        manifests.push(manifest);
    }
    Ok(manifests)
}

/// Project every checked-in profile into an
/// [`EscalationPacketReview`] for the shell preview surface.
///
/// # Errors
///
/// Returns [`SupportExportRedactionError`] when any fixture fails to
/// parse or fails the default-redacted posture check.
pub fn current_escalation_packet_reviews(
) -> Result<Vec<EscalationPacketReview>, SupportExportRedactionError> {
    Ok(load_profile_corpus()?
        .iter()
        .map(EscalationPacketReview::from_profile)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loaded_profiles_are_default_redacted() {
        let profiles = load_profile_corpus().expect("profiles load");
        assert!(!profiles.is_empty());
        for profile in &profiles {
            assert!(profile.is_default_redacted());
            assert!(profile.preserves_local_only_path());
            assert!(profile.preserves_default_required_classes());
        }
    }

    #[test]
    fn loaded_reopen_manifests_match_profile_ids() {
        let profiles = load_profile_corpus().expect("profiles load");
        let manifests = load_reopen_corpus().expect("manifests load");
        for manifest in &manifests {
            assert!(profiles
                .iter()
                .any(|profile| profile.id() == manifest.profile_ref));
            assert!(manifest.preserves_default_required_classes());
        }
    }

    #[test]
    fn local_only_manifest_never_records_export_timestamp() {
        let manifests = load_reopen_corpus().expect("manifests load");
        let local_only = manifests
            .iter()
            .find(|manifest| manifest.is_local_only())
            .expect("at least one local-only manifest");
        assert!(local_only.exported_at_or_null.is_none());
    }

    #[test]
    fn escalation_packet_review_quotes_exact_build_identity() {
        let reviews = current_escalation_packet_reviews().expect("reviews compile");
        for review in &reviews {
            assert!(!review.exact_build_identity_ref.is_empty());
            assert!(!review.raw_dump_attached);
            assert!(!review.code_adjacent_attached);
            assert!(!review.secret_bearing_attached);
        }
    }
}
