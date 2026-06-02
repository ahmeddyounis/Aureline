//! Stabilized support-bundle generation with redaction-default manifests and
//! chain-of-custody fields for the M4 stable lane.
//!
//! This module owns the typed [`StabilizedSupportBundleManifest`] record that
//! makes support-bundle generation truthful, versioned, and redacted-by-default.
//! It guarantees:
//!
//! - Every manifest carries schema version, included/excluded classes,
//!   destination class, retention note, redaction profile, and chain-of-custody.
//! - Ordinary redaction-default export is clearly distinguished from
//!   higher-fidelity incident capture, which requires explicit consent or policy.
//! - Support-bundle schema governance stays separate from analytics or usage
//!   payloads; the export remains inspectable offline.
//! - Recovery-ladder hooks are present for every seeded scenario so blocked
//!   users can recover without wiping unrelated durable state.
//!
//! The [`StabilizedSupportBundleEvaluator`] validates manifests and projects
//! a metadata-safe [`StabilizedSupportBundleSupportPacket`].
//!
//! The boundary schema is at [`STABILIZED_SUPPORT_BUNDLE_SCHEMA_REF`] and the
//! reviewer doc is at [`STABILIZED_SUPPORT_BUNDLE_DOC_REF`].

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Frozen schema version for the stabilized support-bundle contract.
pub const STABILIZED_SUPPORT_BUNDLE_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for a stabilized support-bundle manifest.
pub const STABILIZED_SUPPORT_BUNDLE_MANIFEST_RECORD_KIND: &str =
    "stabilized_support_bundle_manifest_record";

/// Record-kind tag for a stabilized support-bundle manifest fixture seed.
pub const STABILIZED_SUPPORT_BUNDLE_MANIFEST_SEED_CASE_RECORD_KIND: &str =
    "stabilized_support_bundle_manifest_seed_case_record";

/// Record-kind tag for a stabilized support-bundle support packet.
pub const STABILIZED_SUPPORT_BUNDLE_SUPPORT_PACKET_RECORD_KIND: &str =
    "stabilized_support_bundle_support_packet_record";

/// Repo-relative path of the boundary schema.
pub const STABILIZED_SUPPORT_BUNDLE_SCHEMA_REF: &str =
    "schemas/support/stabilize_support_bundle_generation_with_redaction_default_manifests.schema.json";

/// Reviewer doc ref quoted verbatim by every emitted record.
pub const STABILIZED_SUPPORT_BUNDLE_DOC_REF: &str =
    "docs/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const STABILIZED_SUPPORT_BUNDLE_ARTIFACT_REF: &str =
    "artifacts/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const STABILIZED_SUPPORT_BUNDLE_FIXTURE_DIR: &str =
    "fixtures/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed support-bundle generation-mode vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportBundleGenerationMode {
    /// Ordinary redaction-default support export.
    OrdinaryRedactionDefault,
    /// Higher-fidelity incident capture that requires explicit user consent
    /// or policy override.
    HighFidelityIncidentCapture,
}

impl SupportBundleGenerationMode {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryRedactionDefault => "ordinary_redaction_default",
            Self::HighFidelityIncidentCapture => "high_fidelity_incident_capture",
        }
    }
}

/// Closed consent-escalation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentEscalationClass {
    /// No consent escalation required; default redaction applies.
    NotRequired,
    /// Explicit user consent is required before broadening evidence.
    ExplicitUserConsent,
    /// Admin policy override permits broader evidence collection.
    AdminPolicyOverride,
}

impl ConsentEscalationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::ExplicitUserConsent => "explicit_user_consent",
            Self::AdminPolicyOverride => "admin_policy_override",
        }
    }
}

/// Closed destination-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationClass {
    /// Evidence stays on the local device for review.
    LocalOnlyReview,
    /// Handed off to a vendor support case.
    VendorCaseHandoff,
    /// User explicitly chose to upload.
    UserInitiatedUpload,
    /// Managed admin handoff path.
    ManagedAdminHandoff,
    /// Private security channel.
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

/// Closed retention-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Short-term retention (e.g., 30 days).
    ShortTerm,
    /// Medium-term retention (e.g., 90 days).
    MediumTerm,
    /// Long-term retention (e.g., 1 year).
    LongTerm,
    /// Legal hold — retention governed by active legal process.
    LegalHold,
}

impl RetentionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShortTerm => "short_term",
            Self::MediumTerm => "medium_term",
            Self::LongTerm => "long_term",
            Self::LegalHold => "legal_hold",
        }
    }
}

/// Closed incident-capture scenario vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentCaptureScenarioClass {
    /// Export triggered after a crash-loop recovery flow.
    PostCrashLoopExport,
    /// User explicitly opened diagnostics and chose export.
    UserInitiatedDiagnostics,
    /// Policy-mandated audit capture.
    PolicyMandatedAudit,
    /// Support agent directed the user to capture evidence.
    SupportAgentDirected,
}

impl IncidentCaptureScenarioClass {
    /// Every scenario in catalog order.
    pub const fn all() -> [Self; 4] {
        [
            Self::PostCrashLoopExport,
            Self::UserInitiatedDiagnostics,
            Self::PolicyMandatedAudit,
            Self::SupportAgentDirected,
        ]
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostCrashLoopExport => "post_crash_loop_export",
            Self::UserInitiatedDiagnostics => "user_initiated_diagnostics",
            Self::PolicyMandatedAudit => "policy_mandated_audit",
            Self::SupportAgentDirected => "support_agent_directed",
        }
    }
}

/// Closed recovery-ladder hook vocabulary for narrow reset/repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderHookClass {
    /// Open in safe mode with a minimal runtime profile.
    SafeModeMinimalProfile,
    /// Open without restoring the prior session.
    OpenWithoutRestore,
    /// Export support evidence for review.
    ExportEvidence,
    /// Retry only the affected fault domain.
    RetryFaultDomain,
    /// Disable the most recently changed extension.
    DisableRecentExtension,
    /// Reset ephemeral cache without touching user files.
    ResetEphemeralCache,
    /// Run Project Doctor probes.
    RunProjectDoctor,
    /// Perform a bounded repair with preview.
    BoundedRepairPreview,
}

impl RecoveryLadderHookClass {
    /// Every required hook in catalog order.
    pub const REQUIRED: [Self; 8] = [
        Self::SafeModeMinimalProfile,
        Self::OpenWithoutRestore,
        Self::ExportEvidence,
        Self::RetryFaultDomain,
        Self::DisableRecentExtension,
        Self::ResetEphemeralCache,
        Self::RunProjectDoctor,
        Self::BoundedRepairPreview,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeModeMinimalProfile => "safe_mode_minimal_profile",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::ExportEvidence => "export_evidence",
            Self::RetryFaultDomain => "retry_fault_domain",
            Self::DisableRecentExtension => "disable_recent_extension",
            Self::ResetEphemeralCache => "reset_ephemeral_cache",
            Self::RunProjectDoctor => "run_project_doctor",
            Self::BoundedRepairPreview => "bounded_repair_preview",
        }
    }
}

/// Closed diagnostic-data-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDataClass {
    /// Metadata-only rows (build id, version, channel).
    MetadataOnly,
    /// Environment-adjacent rows (paths, env vars, platform info).
    EnvironmentAdjacent,
    /// Code-adjacent rows (snippets, notebook cells, mutation journals).
    CodeAdjacent,
    /// High-risk rows (secrets, dumps, full shell history).
    HighRisk,
}

impl DiagnosticDataClass {
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

/// Closed high-risk-content vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighRiskContentClass {
    /// Not applicable to this row.
    NotApplicable,
    /// Contains secrets or tokens.
    SecretBearing,
    /// Contains raw dump or memory contents.
    RawDumpOrMemory,
    /// Contains full shell history.
    FullShellHistory,
    /// Contains raw trace or transcript.
    RawTraceOrTranscript,
    /// Prohibited by policy for an unknown reason.
    PolicyProhibitedUnknown,
}

impl HighRiskContentClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::SecretBearing => "secret_bearing",
            Self::RawDumpOrMemory => "raw_dump_or_memory",
            Self::FullShellHistory => "full_shell_history",
            Self::RawTraceOrTranscript => "raw_trace_or_transcript",
            Self::PolicyProhibitedUnknown => "policy_prohibited_unknown",
        }
    }
}

/// Closed custody-actor vocabulary for chain-of-custody events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyActorClass {
    LocalUser,
    AdminInitiated,
    SupportAgent,
    AutomatedRetentionJob,
    ExportPipeline,
    GovernancePackets,
}

impl CustodyActorClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::AdminInitiated => "admin_initiated",
            Self::SupportAgent => "support_agent",
            Self::AutomatedRetentionJob => "automated_retention_job",
            Self::ExportPipeline => "export_pipeline",
            Self::GovernancePackets => "governance_packets",
        }
    }
}

/// Closed custody-action vocabulary for chain-of-custody events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyActionClass {
    Created,
    PackagedForExport,
    ExportedLocally,
    MirroredToManaged,
    PlacedOnHold,
    HoldReleased,
    DeleteRequested,
    DeleteCompleted,
    ReceiptIssued,
}

impl CustodyActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::PackagedForExport => "packaged_for_export",
            Self::ExportedLocally => "exported_locally",
            Self::MirroredToManaged => "mirrored_to_managed",
            Self::PlacedOnHold => "placed_on_hold",
            Self::HoldReleased => "hold_released",
            Self::DeleteRequested => "delete_requested",
            Self::DeleteCompleted => "delete_completed",
            Self::ReceiptIssued => "receipt_issued",
        }
    }
}

/// Closed custody-location vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyLocationClass {
    LocalDeviceOnly,
    LocalExportCopy,
    ManagedArchiveActive,
    ManagedArchiveHeld,
    ManagedArchivePolicyRetained,
    DestructionReceiptOnly,
    NoRemainingLocation,
}

impl CustodyLocationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::LocalExportCopy => "local_export_copy",
            Self::ManagedArchiveActive => "managed_archive_active",
            Self::ManagedArchiveHeld => "managed_archive_held",
            Self::ManagedArchivePolicyRetained => "managed_archive_policy_retained",
            Self::DestructionReceiptOnly => "destruction_receipt_only",
            Self::NoRemainingLocation => "no_remaining_location",
        }
    }
}

// ---------------------------------------------------------------------------
// Row types
// ---------------------------------------------------------------------------

/// Build identity block carried on every stabilized manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedBuildIdentity {
    pub build_id: String,
    pub producer_build_id: String,
    pub product_version: String,
    pub release_channel_class: String,
    pub exact_build_refs: Vec<String>,
}

/// One included class entry with its inclusion reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncludedClassEntry {
    pub data_class: DiagnosticDataClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_pack_item_id: Option<String>,
    pub inclusion_reason: String,
    pub redaction_class: String,
}

/// One excluded class entry with its exclusion reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExcludedClassEntry {
    pub data_class: DiagnosticDataClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_pack_item_id: Option<String>,
    pub exclusion_reason: String,
    pub policy_ref: Option<String>,
}

/// One chain-of-custody event for a support-bundle manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainOfCustodyEntry {
    pub sequence: u32,
    pub actor_class: CustodyActorClass,
    pub actor_ref: String,
    pub action_class: CustodyActionClass,
    pub occurred_at: String,
    pub location_class: CustodyLocationClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    pub note: String,
}

/// One recovery-ladder hook binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderHookBinding {
    pub hook_class: RecoveryLadderHookClass,
    pub hook_ref: String,
    pub label: String,
    pub enabled: bool,
    pub blast_radius: String,
    pub preserves_user_state: bool,
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Main records
// ---------------------------------------------------------------------------

/// Stabilized support-bundle manifest record.
///
/// Mirrors `stabilized_support_bundle_manifest_record` in the boundary schema.
/// This is the canonical truth model for what a stable support bundle contains
/// on export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedSupportBundleManifest {
    pub schema_version: u32,
    pub record_kind: String,
    pub manifest_id: String,
    pub title: String,
    pub generation_mode: SupportBundleGenerationMode,
    pub build_identity: StabilizedBuildIdentity,
    pub included_classes: Vec<IncludedClassEntry>,
    pub excluded_classes: Vec<ExcludedClassEntry>,
    pub destination_class: DestinationClass,
    pub retention_class: RetentionClass,
    pub retention_note: String,
    pub redaction_profile_ref: String,
    pub chain_of_custody: Vec<ChainOfCustodyEntry>,
    pub consent_escalation_class: ConsentEscalationClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_escalation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incident_capture_scenario: Option<IncidentCaptureScenarioClass>,
    pub recovery_ladder_hooks: Vec<RecoveryLadderHookBinding>,
    pub supports_offline_inspection: bool,
    pub emitted_at: String,
    pub notes: String,
}

impl StabilizedSupportBundleManifest {
    /// Stable id for this manifest.
    pub fn id(&self) -> &str {
        &self.manifest_id
    }

    /// True when this manifest represents ordinary redaction-default export.
    pub fn is_ordinary_redaction_default(&self) -> bool {
        matches!(
            self.generation_mode,
            SupportBundleGenerationMode::OrdinaryRedactionDefault
        )
    }

    /// True when this manifest represents high-fidelity incident capture.
    pub fn is_high_fidelity_incident_capture(&self) -> bool {
        matches!(
            self.generation_mode,
            SupportBundleGenerationMode::HighFidelityIncidentCapture
        )
    }

    /// True when the manifest supports offline inspection without upload.
    pub fn supports_offline_inspection(&self) -> bool {
        self.supports_offline_inspection
    }

    /// Returns the count of included classes.
    pub fn included_count(&self) -> usize {
        self.included_classes.len()
    }

    /// Returns the count of excluded classes.
    pub fn excluded_count(&self) -> usize {
        self.excluded_classes.len()
    }

    /// Returns the count of chain-of-custody entries.
    pub fn custody_event_count(&self) -> usize {
        self.chain_of_custody.len()
    }

    /// Returns every enabled recovery-ladder hook class.
    pub fn enabled_recovery_hooks(&self) -> Vec<RecoveryLadderHookClass> {
        self.recovery_ladder_hooks
            .iter()
            .filter(|h| h.enabled)
            .map(|h| h.hook_class)
            .collect()
    }
}

/// Stabilized support-bundle support packet.
///
/// Mirrors `stabilized_support_bundle_support_packet_record` in the boundary
/// schema. This is the metadata-safe projection emitted for support intake.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedSupportBundleSupportPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub manifest_ref: String,
    pub generation_mode: SupportBundleGenerationMode,
    pub destination_class: DestinationClass,
    pub retention_class: RetentionClass,
    pub redaction_profile_ref: String,
    pub included_class_count: usize,
    pub excluded_class_count: usize,
    pub custody_event_count: usize,
    pub consent_escalation_class: ConsentEscalationClass,
    pub supports_offline_inspection: bool,
    pub schema_ref: String,
    pub doc_ref: String,
    pub artifact_ref: String,
    pub emitted_at: String,
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors raised by the [`StabilizedSupportBundleEvaluator`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabilizedSupportBundleError {
    /// Schema version does not match the frozen contract version.
    UnsupportedSchemaVersion { actual: u32, expected: u32 },
    /// Record kind is not recognized.
    UnknownRecordKind(String),
    /// Manifest id is empty or malformed.
    InvalidManifestId,
    /// Build id is empty.
    EmptyBuildId,
    /// Exact-build refs list is empty.
    EmptyExactBuildRefs,
    /// Generation mode is inconsistent with the manifest content.
    GenerationModeInconsistent {
        mode: SupportBundleGenerationMode,
        reason: &'static str,
    },
    /// High-fidelity incident capture lacks an incident-capture scenario.
    MissingIncidentCaptureScenario,
    /// High-fidelity incident capture lacks a consent escalation ref.
    MissingConsentEscalationRef,
    /// Ordinary redaction-default export must not carry a consent escalation ref.
    UnexpectedConsentEscalationRef,
    /// Ordinary redaction-default export must not carry an incident scenario.
    UnexpectedIncidentCaptureScenario,
    /// Included classes list is empty.
    EmptyIncludedClasses,
    /// Excluded classes list is empty.
    EmptyExcludedClasses,
    /// An included class entry has an empty reason.
    EmptyIncludedReason,
    /// An excluded class entry has an empty reason.
    EmptyExcludedReason,
    /// Redaction profile ref is empty.
    EmptyRedactionProfileRef,
    /// Retention note is empty.
    EmptyRetentionNote,
    /// Chain of custody is empty.
    EmptyChainOfCustody,
    /// Chain-of-custody sequence is not strictly increasing.
    NonMonotonicCustodySequence { sequence: u32 },
    /// Chain-of-custody event has a duplicate or empty event id.
    InvalidCustodyEventId,
    /// Chain-of-custody event has an empty actor_ref.
    EmptyCustodyActorRef,
    /// Chain-of-custody event has an empty note.
    EmptyCustodyNote,
    /// Recovery-ladder hooks list is empty.
    EmptyRecoveryLadderHooks,
    /// A required recovery-ladder hook class is missing.
    MissingRecoveryLadderHook(RecoveryLadderHookClass),
    /// A recovery-ladder hook has an empty ref.
    EmptyRecoveryHookRef,
    /// A recovery-ladder hook has an empty label.
    EmptyRecoveryHookLabel,
    /// A recovery-ladder hook claims to preserve user state but the blast-radius
    /// description suggests otherwise.
    SuspectedDestructiveHook {
        hook_class: RecoveryLadderHookClass,
        blast_radius: String,
    },
    /// Destination class is local-only but supports_offline_inspection is false.
    OfflineInspectionRequiredForLocalOnly,
    /// High-risk or code-adjacent data class is included without proper consent.
    ImproperlyIncludedHighRiskClass {
        data_class: DiagnosticDataClass,
        support_pack_item_id: Option<String>,
    },
}

impl fmt::Display for StabilizedSupportBundleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual, expected } => {
                write!(
                    f,
                    "unsupported schema version {actual}, expected {expected}"
                )
            }
            Self::UnknownRecordKind(kind) => write!(f, "unknown record kind: {kind}"),
            Self::InvalidManifestId => write!(f, "manifest_id must not be empty"),
            Self::EmptyBuildId => write!(f, "build_id must not be empty"),
            Self::EmptyExactBuildRefs => {
                write!(f, "exact_build_refs must contain at least one entry")
            }
            Self::GenerationModeInconsistent { mode, reason } => {
                write!(
                    f,
                    "generation mode {} is inconsistent: {reason}",
                    mode.as_str()
                )
            }
            Self::MissingIncidentCaptureScenario => {
                write!(
                    f,
                    "high_fidelity_incident_capture requires an incident_capture_scenario"
                )
            }
            Self::MissingConsentEscalationRef => {
                write!(
                    f,
                    "high_fidelity_incident_capture requires a consent_escalation_ref"
                )
            }
            Self::UnexpectedConsentEscalationRef => {
                write!(
                    f,
                    "ordinary_redaction_default must not carry a consent_escalation_ref"
                )
            }
            Self::UnexpectedIncidentCaptureScenario => {
                write!(
                    f,
                    "ordinary_redaction_default must not carry an incident_capture_scenario"
                )
            }
            Self::EmptyIncludedClasses => write!(f, "included_classes must not be empty"),
            Self::EmptyExcludedClasses => write!(f, "excluded_classes must not be empty"),
            Self::EmptyIncludedReason => {
                write!(
                    f,
                    "every included_classes entry must have a non-empty inclusion_reason"
                )
            }
            Self::EmptyExcludedReason => {
                write!(
                    f,
                    "every excluded_classes entry must have a non-empty exclusion_reason"
                )
            }
            Self::EmptyRedactionProfileRef => {
                write!(f, "redaction_profile_ref must not be empty")
            }
            Self::EmptyRetentionNote => write!(f, "retention_note must not be empty"),
            Self::EmptyChainOfCustody => write!(f, "chain_of_custody must not be empty"),
            Self::NonMonotonicCustodySequence { sequence } => {
                write!(f, "chain_of_custody sequence {sequence} is not monotonic")
            }
            Self::InvalidCustodyEventId => {
                write!(f, "chain_of_custody event_id must be non-empty and unique")
            }
            Self::EmptyCustodyActorRef => {
                write!(f, "chain_of_custody actor_ref must not be empty")
            }
            Self::EmptyCustodyNote => write!(f, "chain_of_custody note must not be empty"),
            Self::EmptyRecoveryLadderHooks => {
                write!(f, "recovery_ladder_hooks must not be empty")
            }
            Self::MissingRecoveryLadderHook(hook) => {
                write!(
                    f,
                    "missing required recovery_ladder_hook: {}",
                    hook.as_str()
                )
            }
            Self::EmptyRecoveryHookRef => {
                write!(
                    f,
                    "every recovery_ladder_hook must have a non-empty hook_ref"
                )
            }
            Self::EmptyRecoveryHookLabel => {
                write!(f, "every recovery_ladder_hook must have a non-empty label")
            }
            Self::SuspectedDestructiveHook {
                hook_class,
                blast_radius,
            } => {
                write!(
                    f,
                    "recovery hook {} claims preserves_user_state=true but blast_radius '{}' suggests destructive behavior",
                    hook_class.as_str(),
                    blast_radius
                )
            }
            Self::OfflineInspectionRequiredForLocalOnly => {
                write!(
                    f,
                    "destination_class=local_only_review requires supports_offline_inspection=true"
                )
            }
            Self::ImproperlyIncludedHighRiskClass {
                data_class,
                support_pack_item_id,
            } => {
                write!(
                    f,
                    "high-risk data class {} is included without proper consent escalation (item {:?})",
                    data_class.as_str(),
                    support_pack_item_id
                )
            }
        }
    }
}

impl Error for StabilizedSupportBundleError {}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Evaluates [`StabilizedSupportBundleManifest`] records and projects
/// metadata-safe [`StabilizedSupportBundleSupportPacket`] rows.
#[derive(Debug, Clone)]
pub struct StabilizedSupportBundleEvaluator;

impl StabilizedSupportBundleEvaluator {
    /// Create a new evaluator.
    pub fn new() -> Self {
        Self
    }

    /// Validate a stabilized manifest against the M4 stable contract.
    pub fn validate_manifest(
        &self,
        manifest: &StabilizedSupportBundleManifest,
    ) -> Result<(), StabilizedSupportBundleError> {
        if manifest.schema_version != STABILIZED_SUPPORT_BUNDLE_SCHEMA_VERSION {
            return Err(StabilizedSupportBundleError::UnsupportedSchemaVersion {
                actual: manifest.schema_version,
                expected: STABILIZED_SUPPORT_BUNDLE_SCHEMA_VERSION,
            });
        }
        if manifest.record_kind != STABILIZED_SUPPORT_BUNDLE_MANIFEST_RECORD_KIND
            && manifest.record_kind != STABILIZED_SUPPORT_BUNDLE_MANIFEST_SEED_CASE_RECORD_KIND
        {
            return Err(StabilizedSupportBundleError::UnknownRecordKind(
                manifest.record_kind.clone(),
            ));
        }
        if manifest.manifest_id.trim().is_empty() {
            return Err(StabilizedSupportBundleError::InvalidManifestId);
        }
        if manifest.build_identity.build_id.trim().is_empty() {
            return Err(StabilizedSupportBundleError::EmptyBuildId);
        }
        if manifest.build_identity.exact_build_refs.is_empty() {
            return Err(StabilizedSupportBundleError::EmptyExactBuildRefs);
        }

        // Generation-mode consistency checks.
        match manifest.generation_mode {
            SupportBundleGenerationMode::OrdinaryRedactionDefault => {
                if manifest.consent_escalation_ref.is_some() {
                    return Err(StabilizedSupportBundleError::UnexpectedConsentEscalationRef);
                }
                if manifest.incident_capture_scenario.is_some() {
                    return Err(StabilizedSupportBundleError::UnexpectedIncidentCaptureScenario);
                }
                if manifest.consent_escalation_class != ConsentEscalationClass::NotRequired {
                    return Err(StabilizedSupportBundleError::GenerationModeInconsistent {
                        mode: manifest.generation_mode,
                        reason: "ordinary_redaction_default must have consent_escalation_class=not_required",
                    });
                }
            }
            SupportBundleGenerationMode::HighFidelityIncidentCapture => {
                if manifest.incident_capture_scenario.is_none() {
                    return Err(StabilizedSupportBundleError::MissingIncidentCaptureScenario);
                }
                if manifest.consent_escalation_ref.is_none() {
                    return Err(StabilizedSupportBundleError::MissingConsentEscalationRef);
                }
                if manifest.consent_escalation_class == ConsentEscalationClass::NotRequired {
                    return Err(StabilizedSupportBundleError::GenerationModeInconsistent {
                        mode: manifest.generation_mode,
                        reason: "high_fidelity_incident_capture must have consent_escalation_class != not_required",
                    });
                }
            }
        }

        if manifest.included_classes.is_empty() {
            return Err(StabilizedSupportBundleError::EmptyIncludedClasses);
        }
        if manifest.excluded_classes.is_empty() {
            return Err(StabilizedSupportBundleError::EmptyExcludedClasses);
        }
        for entry in &manifest.included_classes {
            if entry.inclusion_reason.trim().is_empty() {
                return Err(StabilizedSupportBundleError::EmptyIncludedReason);
            }
            if matches!(
                entry.data_class,
                DiagnosticDataClass::HighRisk | DiagnosticDataClass::CodeAdjacent
            ) && manifest.consent_escalation_class != ConsentEscalationClass::ExplicitUserConsent
                && manifest.consent_escalation_class != ConsentEscalationClass::AdminPolicyOverride
            {
                return Err(
                    StabilizedSupportBundleError::ImproperlyIncludedHighRiskClass {
                        data_class: entry.data_class,
                        support_pack_item_id: entry.support_pack_item_id.clone(),
                    },
                );
            }
        }
        for entry in &manifest.excluded_classes {
            if entry.exclusion_reason.trim().is_empty() {
                return Err(StabilizedSupportBundleError::EmptyExcludedReason);
            }
        }

        if manifest.redaction_profile_ref.trim().is_empty() {
            return Err(StabilizedSupportBundleError::EmptyRedactionProfileRef);
        }
        if manifest.retention_note.trim().is_empty() {
            return Err(StabilizedSupportBundleError::EmptyRetentionNote);
        }

        // Chain-of-custody validation.
        if manifest.chain_of_custody.is_empty() {
            return Err(StabilizedSupportBundleError::EmptyChainOfCustody);
        }
        let mut seen_ids = BTreeSet::new();
        let mut last_sequence: Option<u32> = None;
        for entry in &manifest.chain_of_custody {
            if let Some(prev) = last_sequence {
                if entry.sequence <= prev {
                    return Err(StabilizedSupportBundleError::NonMonotonicCustodySequence {
                        sequence: entry.sequence,
                    });
                }
            }
            last_sequence = Some(entry.sequence);
            if entry.actor_ref.trim().is_empty() {
                return Err(StabilizedSupportBundleError::EmptyCustodyActorRef);
            }
            if entry.note.trim().is_empty() {
                return Err(StabilizedSupportBundleError::EmptyCustodyNote);
            }
            let event_id = format!("{}.{}", manifest.manifest_id, entry.sequence);
            if !seen_ids.insert(event_id.clone()) {
                return Err(StabilizedSupportBundleError::InvalidCustodyEventId);
            }
        }

        // Recovery-ladder hooks validation.
        if manifest.recovery_ladder_hooks.is_empty() {
            return Err(StabilizedSupportBundleError::EmptyRecoveryLadderHooks);
        }
        let mut covered_hooks = BTreeSet::new();
        for hook in &manifest.recovery_ladder_hooks {
            if hook.hook_ref.trim().is_empty() {
                return Err(StabilizedSupportBundleError::EmptyRecoveryHookRef);
            }
            if hook.label.trim().is_empty() {
                return Err(StabilizedSupportBundleError::EmptyRecoveryHookLabel);
            }
            covered_hooks.insert(hook.hook_class);
            let destructive_keywords = ["wipe", "delete", "erase", "destroy", "clear all"];
            if hook.preserves_user_state {
                let lower = hook.blast_radius.to_lowercase();
                for kw in &destructive_keywords {
                    if lower.contains(kw) {
                        return Err(StabilizedSupportBundleError::SuspectedDestructiveHook {
                            hook_class: hook.hook_class,
                            blast_radius: hook.blast_radius.clone(),
                        });
                    }
                }
            }
        }
        for required in &RecoveryLadderHookClass::REQUIRED {
            if !covered_hooks.contains(required) {
                return Err(StabilizedSupportBundleError::MissingRecoveryLadderHook(
                    *required,
                ));
            }
        }

        // Local-only destination must support offline inspection.
        if matches!(
            manifest.destination_class,
            DestinationClass::LocalOnlyReview
        ) && !manifest.supports_offline_inspection
        {
            return Err(StabilizedSupportBundleError::OfflineInspectionRequiredForLocalOnly);
        }

        Ok(())
    }

    /// Project a validated manifest into a metadata-safe support packet.
    pub fn project_support_packet(
        &self,
        manifest: &StabilizedSupportBundleManifest,
        packet_id: impl Into<String>,
    ) -> Result<StabilizedSupportBundleSupportPacket, StabilizedSupportBundleError> {
        self.validate_manifest(manifest)?;
        Ok(StabilizedSupportBundleSupportPacket {
            schema_version: STABILIZED_SUPPORT_BUNDLE_SCHEMA_VERSION,
            record_kind: STABILIZED_SUPPORT_BUNDLE_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: packet_id.into(),
            manifest_ref: manifest.manifest_id.clone(),
            generation_mode: manifest.generation_mode,
            destination_class: manifest.destination_class,
            retention_class: manifest.retention_class,
            redaction_profile_ref: manifest.redaction_profile_ref.clone(),
            included_class_count: manifest.included_classes.len(),
            excluded_class_count: manifest.excluded_classes.len(),
            custody_event_count: manifest.chain_of_custody.len(),
            consent_escalation_class: manifest.consent_escalation_class,
            supports_offline_inspection: manifest.supports_offline_inspection,
            schema_ref: STABILIZED_SUPPORT_BUNDLE_SCHEMA_REF.to_owned(),
            doc_ref: STABILIZED_SUPPORT_BUNDLE_DOC_REF.to_owned(),
            artifact_ref: STABILIZED_SUPPORT_BUNDLE_ARTIFACT_REF.to_owned(),
            emitted_at: manifest.emitted_at.clone(),
            notes: manifest.notes.clone(),
        })
    }
}

impl Default for StabilizedSupportBundleEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

/// Deserialize a stabilized support-bundle manifest from a YAML string.
pub fn load_stabilized_support_bundle_manifest(
    yaml: &str,
) -> Result<StabilizedSupportBundleManifest, Box<dyn Error + Send + Sync>> {
    let manifest: StabilizedSupportBundleManifest = serde_yaml::from_str(yaml)?;
    Ok(manifest)
}

/// Deserialize a stabilized support-bundle support packet from a YAML string.
pub fn load_stabilized_support_bundle_support_packet(
    yaml: &str,
) -> Result<StabilizedSupportBundleSupportPacket, Box<dyn Error + Send + Sync>> {
    let packet: StabilizedSupportBundleSupportPacket = serde_yaml::from_str(yaml)?;
    Ok(packet)
}
