//! Schema, endpoint-policy, and operational-signal inspector for alpha trust surfaces.
//!
//! The inspector reads the checked-in telemetry/support payload-family
//! register and the alpha schema register, then projects claimed rows into
//! desktop, support-export, and runbook/help records. It does not mint
//! alternate policy truth: endpoint labels, consent state, local-only
//! alternatives, and support/export posture are derived from the registry
//! artifacts that own those rows.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

const DEFAULT_CONSENT_LEDGER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/consent_ledger_seed.yaml"
));
const DEFAULT_ALPHA_SCHEMA_REGISTRY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/schema_registry_alpha.yaml"
));

/// Stable record-kind tag for endpoint-policy inspector snapshots.
pub const ENDPOINT_POLICY_INSPECTION_RECORD_KIND: &str =
    "schema_registry_endpoint_policy_inspection_record";

/// Schema version for [`EndpointPolicyInspectionSnapshot`].
pub const ENDPOINT_POLICY_INSPECTION_SCHEMA_VERSION: u32 = 1;

/// Stable support-export record kind emitted by this inspector.
pub const ENDPOINT_POLICY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "schema_registry_endpoint_policy_support_export";

/// Stable runbook/help handoff record kind emitted by this inspector.
pub const ENDPOINT_POLICY_RUNBOOK_HANDOFF_RECORD_KIND: &str =
    "schema_registry_endpoint_policy_runbook_handoff";

/// The checked-in payload-family register consumed by the inspector.
pub const CONSENT_LEDGER_SOURCE_REF: &str = "artifacts/governance/consent_ledger_seed.yaml";

/// The checked-in alpha schema register consumed by the inspector.
pub const ALPHA_SCHEMA_REGISTRY_SOURCE_REF: &str =
    "artifacts/governance/schema_registry_alpha.yaml";

/// Closed destination classes exposed by endpoint and signal inspection rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationClass {
    /// No off-device destination is active for the row.
    LocalOnly,
    /// Upload may happen only after explicit telemetry consent.
    OptionalTelemetryUpload,
    /// A user/admin initiated support export path is available.
    ManualSupportExport,
    /// The destination is a support bundle or support-export packet.
    SupportBundle,
    /// The destination is a release or benchmark evidence packet.
    ReleaseEvidencePacket,
    /// The destination is a local review packet or imported review artifact.
    LocalReview,
    /// The destination is a post-incident or incident-review packet.
    IncidentPostmortem,
    /// A managed admin broker or policy service controls the path.
    ManagedAdminBroker,
    /// The path is reserved for deletion or offboarding export.
    OffboardingExportChannel,
    /// The row can be emitted only to local CLI stdout.
    CliLocalStdout,
    /// The registry cannot resolve a safe destination without review.
    UnknownReviewRequired,
}

impl DestinationClass {
    /// Stable token used in serialized inspector rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::OptionalTelemetryUpload => "optional_telemetry_upload",
            Self::ManualSupportExport => "manual_support_export",
            Self::SupportBundle => "support_bundle",
            Self::ReleaseEvidencePacket => "release_evidence_packet",
            Self::LocalReview => "local_review",
            Self::IncidentPostmortem => "incident_postmortem",
            Self::ManagedAdminBroker => "managed_admin_broker",
            Self::OffboardingExportChannel => "offboarding_export_channel",
            Self::CliLocalStdout => "cli_local_stdout",
            Self::UnknownReviewRequired => "unknown_review_required",
        }
    }
}

/// Closed consent and policy states quoted by endpoint-policy rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentPolicyState {
    /// The row is inactive until explicit opt-in consent exists.
    ExplicitOptInRequired,
    /// Export is available only after a user-initiated request.
    ExportOnlyUserRequest,
    /// An administrator or signed policy bundle gates the row.
    AdminPolicyGated,
    /// Local authoritative mechanics do not require off-device consent.
    ImpliedLocalAuthoritative,
    /// The row is limited to delete/offboarding governed channels.
    DeleteGovernedOnly,
    /// The path denies by default until a policy explicitly widens it.
    DenyByDefault,
    /// No consent class applies to this inspection row.
    NotApplicable,
}

impl ConsentPolicyState {
    /// Stable token used in serialized inspector rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitOptInRequired => "explicit_opt_in_required",
            Self::ExportOnlyUserRequest => "export_only_user_request",
            Self::AdminPolicyGated => "admin_policy_gated",
            Self::ImpliedLocalAuthoritative => "implied_local_authoritative",
            Self::DeleteGovernedOnly => "delete_governed_only",
            Self::DenyByDefault => "deny_by_default",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed egress classes shown by endpoint-policy rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressClass {
    /// The inspected row stays on the local target.
    TargetLocal,
    /// External egress is allowed only through the approved opt-in route.
    OrgApprovedExternalAfterOptIn,
    /// The destination is chosen during explicit export review.
    UserSelectedExportDestination,
    /// A managed control-plane route is required.
    ManagedControlPlane,
    /// The route is denied by policy.
    DenyAll,
    /// The only egress is local CLI stdout.
    CliStdoutLocalOnly,
}

/// Closed endpoint decision vocabulary reused by desktop and support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointOutcomeClass {
    /// Local-only work is allowed with no off-device route.
    AllowLocalOnly,
    /// The route is allowed only after consent is present.
    AllowAfterConsent,
    /// The route is allowed as an explicit manual export.
    AllowManualExport,
    /// The route depends on an admin policy gate.
    AdminPolicyGated,
    /// The route is denied by policy.
    DenyPolicy,
    /// The route needs review before it can be treated as allowed.
    ReviewRequired,
}

/// Operational signal kinds covered by the inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalSignalKind {
    /// Log stream or bounded log window.
    Logs,
    /// Metric query or aggregate window.
    Metrics,
    /// Trace or span-set window.
    Traces,
    /// Incident timeline or post-incident chronology slice.
    IncidentTimeline,
}

impl OperationalSignalKind {
    /// Stable token used in serialized inspector rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Logs => "logs",
            Self::Metrics => "metrics",
            Self::Traces => "traces",
            Self::IncidentTimeline => "incident_timeline",
        }
    }
}

/// Shared freshness vocabulary for desktop, support export, and runbook/help.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalFreshnessClass {
    /// The source is reachable and actively refreshing.
    Live,
    /// A live path is active but the current slice is not complete yet.
    Buffering,
    /// The slice is a cached or refreshed snapshot inside its accepted window.
    Cached,
    /// The slice is outside its accepted freshness window.
    Stale,
    /// Coverage is incomplete because of permissions, retention, or omissions.
    Partial,
    /// No live refresh route is available; local/offline evidence is being inspected.
    Offline,
}

impl SignalFreshnessClass {
    /// Stable token used in serialized inspector rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Buffering => "buffering",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Offline => "offline",
        }
    }

    /// All freshness tokens this inspector is allowed to emit.
    pub fn vocabulary() -> Vec<Self> {
        vec![
            Self::Live,
            Self::Buffering,
            Self::Cached,
            Self::Stale,
            Self::Partial,
            Self::Offline,
        ]
    }
}

/// Shared redaction vocabulary for signal-slice projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalRedactionClass {
    /// Metadata only; no raw payload is embedded.
    MetadataOnly,
    /// Payload summary is redacted before inspection or export.
    RedactedPayload,
    /// Raw payload remains behind an opaque reference.
    ByReferenceOnly,
    /// Policy withholds the raw payload.
    WithheldByPolicy,
    /// The slice remains local and is not exported.
    RetainedLocalOnly,
}

impl SignalRedactionClass {
    /// Stable token used in serialized inspector rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::RedactedPayload => "redacted_payload",
            Self::ByReferenceOnly => "by_reference_only",
            Self::WithheldByPolicy => "withheld_by_policy",
            Self::RetainedLocalOnly => "retained_local_only",
        }
    }

    /// All redaction tokens this inspector is allowed to emit.
    pub fn vocabulary() -> Vec<Self> {
        vec![
            Self::MetadataOnly,
            Self::RedactedPayload,
            Self::ByReferenceOnly,
            Self::WithheldByPolicy,
            Self::RetainedLocalOnly,
        ]
    }
}

/// Truncation or reduction state visible on an operational signal slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalTruncationState {
    /// The declared window is not truncated.
    NotTruncated,
    /// Records were truncated by size, time, retention, or export boundary.
    Truncated,
    /// The visible slice was clipped at one or both ends.
    Clipped,
    /// Resolution was reduced before rendering or export.
    Downsampled,
    /// Policy redaction removed records or payload fields.
    PolicyRedacted,
    /// The truncation state needs review.
    Unknown,
}

/// Time window and render timezone shown for a signal slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalTimeWindow {
    /// UTC timestamp at the start of the signal window.
    pub window_start_utc: String,
    /// UTC timestamp at the end of the signal window.
    pub window_end_utc: String,
    /// IANA timezone used by the human-facing surface.
    pub display_time_zone_iana: String,
    /// UTC offset used by the rendered surface.
    pub display_utc_offset: String,
}

/// One operational signal slice admitted into the inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalSignalSlice {
    /// Stable slice id shared by all projections.
    pub signal_slice_id: String,
    /// Signal family covered by the row.
    pub signal_kind: OperationalSignalKind,
    /// Review-safe backend label such as local process, provider, or imported artifact.
    pub source_backend: String,
    /// Opaque backend/source ref with no raw URL, hostname, or payload.
    pub source_backend_ref: String,
    /// Time window and timezone context for the slice.
    pub time_window: SignalTimeWindow,
    /// Shared freshness token.
    pub freshness: SignalFreshnessClass,
    /// Shared truncation/reduction token.
    pub truncation_state: SignalTruncationState,
    /// Shared redaction token.
    pub redaction_class: SignalRedactionClass,
    /// Human-readable retention/export posture copied into projections.
    pub retention_export_posture: String,
    /// Destination class for export/share posture.
    pub destination_class: DestinationClass,
}

/// Input claims for a schema/endpoint-policy inspection pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicyInspectionInput {
    /// Stable inspection id.
    pub inspection_id: String,
    /// RFC 3339 timestamp supplied by the caller or fixture.
    pub generated_at: String,
    /// Registry row ids that must resolve through checked-in artifacts.
    pub claimed_schema_refs: Vec<String>,
    /// Signal slices projected through desktop, support, and runbook/help views.
    pub operational_signal_slices: Vec<OperationalSignalSlice>,
    /// Support-export id to assign to the generated support packet.
    pub support_export_id: String,
    /// Runbook/help handoff id to assign to the generated handoff.
    pub runbook_handoff_id: String,
}

/// Schema row resolved from a checked-in register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaInspectionRow {
    /// Claimed row id supplied to the inspector.
    pub claim_ref: String,
    /// Registry row that satisfied the claim.
    pub source_register_row_id: String,
    /// Register artifact that owns this row.
    pub source_register_ref: String,
    /// Review-facing title from the register.
    pub title: String,
    /// Schema path from the owning artifact.
    pub schema_ref: String,
    /// Schema version pin from the owning artifact.
    pub schema_version: u32,
    /// Owner from the owning artifact.
    pub owner_ref: String,
    /// Payload family class when the row came from the consent ledger.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_class: Option<String>,
    /// Alpha schema role when the row came from the alpha schema registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_role: Option<String>,
    /// Schema status when the row came from the alpha schema registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_status: Option<String>,
    /// Consent class from the payload-family register, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_class: Option<String>,
    /// Endpoint class from the payload-family register, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_class: Option<String>,
    /// Record classes bound to the schema row.
    pub record_class_refs: Vec<String>,
    /// Local-only posture from the payload-family register, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_lane_posture: Option<String>,
    /// Local-only alternative from the payload-family register, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_alternative: Option<String>,
}

/// Endpoint-policy row derived from a payload-family register entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicyRow {
    /// Stable endpoint-policy row id.
    pub endpoint_policy_row_id: String,
    /// Claimed schema ref this endpoint policy explains.
    pub claimed_schema_ref: String,
    /// Source register row that owns the policy vocabulary.
    pub source_register_row_id: String,
    /// Source register artifact that owns the policy vocabulary.
    pub source_register_ref: String,
    /// Schema path the policy applies to.
    pub schema_ref: String,
    /// Destination class visible to desktop and support export.
    pub destination_class: DestinationClass,
    /// Consent or policy state visible to desktop and support export.
    pub consent_or_policy_state: ConsentPolicyState,
    /// Egress class visible to desktop and support export.
    pub egress_class: EgressClass,
    /// Endpoint outcome visible to desktop and support export.
    pub outcome_class: EndpointOutcomeClass,
    /// Endpoint class token from the payload-family register.
    pub endpoint_class: String,
    /// Local-only alternative or fallback from the payload-family register.
    pub local_only_alternative: String,
    /// Redaction posture inferred from the register's payload exclusions.
    pub redaction_class: SignalRedactionClass,
    /// Retention/export posture quoted from the owning register row.
    pub retention_export_posture: String,
    /// Whether the row is quoted from another policy row for an alpha schema ref.
    pub quoted_policy_row: bool,
}

/// Desktop-facing signal row that keeps the shared freshness/redaction vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSignalRow {
    /// Stable signal slice id.
    pub signal_slice_id: String,
    /// Signal family covered by the row.
    pub signal_kind: OperationalSignalKind,
    /// Source backend label.
    pub source_backend: String,
    /// Opaque source backend ref.
    pub source_backend_ref: String,
    /// Human-readable window label.
    pub window_label: String,
    /// Shared freshness token.
    pub freshness: SignalFreshnessClass,
    /// Shared redaction token.
    pub redaction_class: SignalRedactionClass,
    /// Truncation/reduction state.
    pub truncation_state: SignalTruncationState,
    /// Retention/export posture.
    pub retention_export_posture: String,
}

/// Desktop-facing projection for the inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicyDesktopProjection {
    /// Stable projection kind.
    pub projection_kind: String,
    /// Source inspection id.
    pub inspection_id: String,
    /// Resolved schema rows.
    pub schema_rows: Vec<SchemaInspectionRow>,
    /// Endpoint-policy rows.
    pub endpoint_policy_rows: Vec<EndpointPolicyRow>,
    /// Operational signal rows.
    pub signal_rows: Vec<DesktopSignalRow>,
}

/// Support-export signal row that keeps the shared freshness/redaction vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicySupportSignalRow {
    /// Stable signal slice id.
    pub signal_slice_id: String,
    /// Signal family covered by the row.
    pub signal_kind: OperationalSignalKind,
    /// Source backend label.
    pub source_backend: String,
    /// Opaque source backend ref.
    pub source_backend_ref: String,
    /// Human-readable window label.
    pub window_label: String,
    /// Shared freshness token.
    pub freshness: SignalFreshnessClass,
    /// Shared redaction token.
    pub redaction_class: SignalRedactionClass,
    /// Truncation/reduction state.
    pub truncation_state: SignalTruncationState,
    /// Destination class for the support/export packet.
    pub destination_class: DestinationClass,
    /// Retention/export posture.
    pub retention_export_posture: String,
}

/// Support/export projection emitted by this inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicySupportExport {
    /// Stable support-export record kind.
    pub record_kind: String,
    /// Source inspection id.
    pub source_inspection_id: String,
    /// Stable support-export id.
    pub export_id: String,
    /// RFC 3339 timestamp supplied by the caller or fixture.
    pub generated_at: String,
    /// Resolved schema rows.
    pub schema_rows: Vec<SchemaInspectionRow>,
    /// Endpoint-policy rows.
    pub endpoint_policy_rows: Vec<EndpointPolicyRow>,
    /// Operational signal rows.
    pub signal_rows: Vec<EndpointPolicySupportSignalRow>,
    /// Freshness tokens allowed by the export.
    pub freshness_vocabulary: Vec<SignalFreshnessClass>,
    /// Redaction tokens allowed by the export.
    pub redaction_vocabulary: Vec<SignalRedactionClass>,
    /// True when raw payload bytes are excluded from this projection.
    pub raw_payloads_excluded: bool,
}

impl EndpointPolicySupportExport {
    /// Number of resolved schema rows.
    pub fn schema_row_count(&self) -> usize {
        self.schema_rows.len()
    }

    /// Number of endpoint-policy rows.
    pub fn endpoint_policy_row_count(&self) -> usize {
        self.endpoint_policy_rows.len()
    }

    /// Number of operational signal rows.
    pub fn operational_signal_slice_count(&self) -> usize {
        self.signal_rows.len()
    }
}

/// Runbook/help signal row that keeps the shared freshness/redaction vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookHelpSignalRow {
    /// Stable signal slice id.
    pub signal_slice_id: String,
    /// Signal family covered by the row.
    pub signal_kind: OperationalSignalKind,
    /// Source backend label.
    pub source_backend: String,
    /// Human-readable window label.
    pub window_label: String,
    /// Shared freshness token.
    pub freshness: SignalFreshnessClass,
    /// Shared redaction token.
    pub redaction_class: SignalRedactionClass,
    /// Destination class for follow-up handoff.
    pub destination_class: DestinationClass,
    /// Help/runbook cue preserving local-only and export posture.
    pub handoff_cue: String,
}

/// Runbook/help handoff projection emitted by this inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicyRunbookHandoff {
    /// Stable runbook/help handoff record kind.
    pub record_kind: String,
    /// Stable handoff id.
    pub handoff_id: String,
    /// Source inspection id.
    pub source_inspection_id: String,
    /// Operational signal rows.
    pub signal_rows: Vec<RunbookHelpSignalRow>,
    /// Freshness tokens allowed by the handoff.
    pub freshness_vocabulary: Vec<SignalFreshnessClass>,
    /// Redaction tokens allowed by the handoff.
    pub redaction_vocabulary: Vec<SignalRedactionClass>,
}

/// Full schema and endpoint-policy inspection snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicyInspectionSnapshot {
    /// Stable snapshot record kind.
    pub record_kind: String,
    /// Snapshot schema version.
    pub schema_version: u32,
    /// Stable inspection id.
    pub inspection_id: String,
    /// RFC 3339 timestamp supplied by the caller or fixture.
    pub generated_at: String,
    /// Resolved schema rows from checked-in register artifacts.
    pub schema_rows: Vec<SchemaInspectionRow>,
    /// Endpoint-policy rows derived from checked-in register artifacts.
    pub endpoint_policy_rows: Vec<EndpointPolicyRow>,
    /// Operational signal slices admitted into the inspection.
    pub operational_signal_slices: Vec<OperationalSignalSlice>,
    /// Desktop projection.
    pub desktop: EndpointPolicyDesktopProjection,
    /// Support/export projection.
    pub support_export: EndpointPolicySupportExport,
    /// Runbook/help handoff projection.
    pub runbook_help_handoff: EndpointPolicyRunbookHandoff,
}

impl EndpointPolicyInspectionSnapshot {
    /// True when desktop, support export, and runbook/help rows reuse the
    /// same freshness and redaction tokens for every signal slice.
    pub fn has_cross_surface_signal_vocabulary_parity(&self) -> bool {
        let desktop = signal_vocab_pairs(
            self.desktop
                .signal_rows
                .iter()
                .map(|row| (&row.signal_slice_id, row.freshness, row.redaction_class)),
        );
        let support = signal_vocab_pairs(
            self.support_export
                .signal_rows
                .iter()
                .map(|row| (&row.signal_slice_id, row.freshness, row.redaction_class)),
        );
        let runbook = signal_vocab_pairs(
            self.runbook_help_handoff
                .signal_rows
                .iter()
                .map(|row| (&row.signal_slice_id, row.freshness, row.redaction_class)),
        );
        desktop == support && support == runbook
    }
}

/// Errors returned while loading artifact registers or resolving claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaRegistryInspectorError {
    /// YAML parsing failed for one of the checked-in register artifacts.
    ParseYaml {
        /// Source artifact that failed to parse.
        source_ref: String,
        /// Parser error message.
        message: String,
    },
    /// A claimed row id was not found in any loaded register.
    UnknownClaim {
        /// Claimed row id that could not be resolved.
        claim_ref: String,
    },
    /// A fixture or caller supplied an incomplete signal slice.
    InvalidSignalSlice {
        /// Signal slice id with the invalid field.
        signal_slice_id: String,
        /// Name of the missing or invalid field.
        field_name: String,
    },
}

impl fmt::Display for SchemaRegistryInspectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseYaml {
                source_ref,
                message,
            } => write!(f, "failed to parse {source_ref}: {message}"),
            Self::UnknownClaim { claim_ref } => {
                write!(f, "schema registry claim did not resolve: {claim_ref}")
            }
            Self::InvalidSignalSlice {
                signal_slice_id,
                field_name,
            } => write!(
                f,
                "signal slice {signal_slice_id} is missing required field {field_name}"
            ),
        }
    }
}

impl Error for SchemaRegistryInspectorError {}

/// Inspector backed by the checked-in schema and consent registers.
#[derive(Debug, Clone)]
pub struct SchemaRegistryInspector {
    consent_entries: BTreeMap<String, ConsentLedgerEntry>,
    alpha_schema_rows: BTreeMap<String, AlphaSchemaRow>,
}

impl SchemaRegistryInspector {
    /// Builds an inspector from the repository's checked-in register artifacts.
    pub fn from_default_artifact_registers() -> Result<Self, SchemaRegistryInspectorError> {
        Self::from_artifact_registers(
            DEFAULT_CONSENT_LEDGER_YAML,
            DEFAULT_ALPHA_SCHEMA_REGISTRY_YAML,
        )
    }

    /// Builds an inspector from supplied register YAML strings.
    pub fn from_artifact_registers(
        consent_ledger_yaml: &str,
        alpha_schema_registry_yaml: &str,
    ) -> Result<Self, SchemaRegistryInspectorError> {
        let consent_ledger: ConsentLedger =
            serde_yaml::from_str(consent_ledger_yaml).map_err(|err| {
                SchemaRegistryInspectorError::ParseYaml {
                    source_ref: CONSENT_LEDGER_SOURCE_REF.to_owned(),
                    message: err.to_string(),
                }
            })?;
        let alpha_schema_registry: AlphaSchemaRegistry =
            serde_yaml::from_str(alpha_schema_registry_yaml).map_err(|err| {
                SchemaRegistryInspectorError::ParseYaml {
                    source_ref: ALPHA_SCHEMA_REGISTRY_SOURCE_REF.to_owned(),
                    message: err.to_string(),
                }
            })?;

        Ok(Self {
            consent_entries: consent_ledger
                .rows
                .into_iter()
                .map(|row| (row.entry_id.clone(), row))
                .collect(),
            alpha_schema_rows: alpha_schema_registry
                .schema_rows
                .into_iter()
                .map(|row| (row.row_id.clone(), row))
                .collect(),
        })
    }

    /// Resolves the supplied claims and materializes desktop, support, and
    /// runbook/help projections.
    pub fn inspect(
        &self,
        input: EndpointPolicyInspectionInput,
    ) -> Result<EndpointPolicyInspectionSnapshot, SchemaRegistryInspectorError> {
        validate_signal_slices(&input.operational_signal_slices)?;

        let mut schema_rows = Vec::new();
        let mut endpoint_policy_rows = Vec::new();

        for claim_ref in &input.claimed_schema_refs {
            if let Some(entry) = self.consent_entries.get(claim_ref) {
                schema_rows.push(schema_row_from_consent_entry(claim_ref, entry));
                endpoint_policy_rows.push(endpoint_policy_from_entry(claim_ref, entry, false));
                continue;
            }

            if let Some(row) = self.alpha_schema_rows.get(claim_ref) {
                schema_rows.push(schema_row_from_alpha_schema(row));
                if let Some(entry) = self.paired_consent_entry_for_alpha_row(row) {
                    endpoint_policy_rows.push(endpoint_policy_from_entry(claim_ref, entry, true));
                }
                continue;
            }

            return Err(SchemaRegistryInspectorError::UnknownClaim {
                claim_ref: claim_ref.clone(),
            });
        }

        let desktop = EndpointPolicyDesktopProjection {
            projection_kind: "desktop_schema_endpoint_policy_inspector".to_owned(),
            inspection_id: input.inspection_id.clone(),
            schema_rows: schema_rows.clone(),
            endpoint_policy_rows: endpoint_policy_rows.clone(),
            signal_rows: input
                .operational_signal_slices
                .iter()
                .map(desktop_signal_row)
                .collect(),
        };

        let support_export = EndpointPolicySupportExport {
            record_kind: ENDPOINT_POLICY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            source_inspection_id: input.inspection_id.clone(),
            export_id: input.support_export_id.clone(),
            generated_at: input.generated_at.clone(),
            schema_rows: schema_rows.clone(),
            endpoint_policy_rows: endpoint_policy_rows.clone(),
            signal_rows: input
                .operational_signal_slices
                .iter()
                .map(support_signal_row)
                .collect(),
            freshness_vocabulary: SignalFreshnessClass::vocabulary(),
            redaction_vocabulary: SignalRedactionClass::vocabulary(),
            raw_payloads_excluded: true,
        };

        let runbook_help_handoff = EndpointPolicyRunbookHandoff {
            record_kind: ENDPOINT_POLICY_RUNBOOK_HANDOFF_RECORD_KIND.to_owned(),
            handoff_id: input.runbook_handoff_id.clone(),
            source_inspection_id: input.inspection_id.clone(),
            signal_rows: input
                .operational_signal_slices
                .iter()
                .map(runbook_signal_row)
                .collect(),
            freshness_vocabulary: SignalFreshnessClass::vocabulary(),
            redaction_vocabulary: SignalRedactionClass::vocabulary(),
        };

        Ok(EndpointPolicyInspectionSnapshot {
            record_kind: ENDPOINT_POLICY_INSPECTION_RECORD_KIND.to_owned(),
            schema_version: ENDPOINT_POLICY_INSPECTION_SCHEMA_VERSION,
            inspection_id: input.inspection_id,
            generated_at: input.generated_at,
            schema_rows,
            endpoint_policy_rows,
            operational_signal_slices: input.operational_signal_slices,
            desktop,
            support_export,
            runbook_help_handoff,
        })
    }

    fn paired_consent_entry_for_alpha_row(
        &self,
        row: &AlphaSchemaRow,
    ) -> Option<&ConsentLedgerEntry> {
        if row.schema_role == "support_export_packet_schema" {
            self.consent_entries.get("support.bundle_manifest")
        } else {
            None
        }
    }
}

fn signal_vocab_pairs<'a>(
    rows: impl Iterator<Item = (&'a String, SignalFreshnessClass, SignalRedactionClass)>,
) -> BTreeSet<(String, SignalFreshnessClass, SignalRedactionClass)> {
    rows.map(|(id, freshness, redaction)| (id.clone(), freshness, redaction))
        .collect()
}

fn validate_signal_slices(
    slices: &[OperationalSignalSlice],
) -> Result<(), SchemaRegistryInspectorError> {
    for slice in slices {
        for (field_name, value) in [
            ("source_backend", slice.source_backend.as_str()),
            ("source_backend_ref", slice.source_backend_ref.as_str()),
            (
                "window_start_utc",
                slice.time_window.window_start_utc.as_str(),
            ),
            ("window_end_utc", slice.time_window.window_end_utc.as_str()),
            (
                "display_time_zone_iana",
                slice.time_window.display_time_zone_iana.as_str(),
            ),
            (
                "display_utc_offset",
                slice.time_window.display_utc_offset.as_str(),
            ),
            (
                "retention_export_posture",
                slice.retention_export_posture.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                return Err(SchemaRegistryInspectorError::InvalidSignalSlice {
                    signal_slice_id: slice.signal_slice_id.clone(),
                    field_name: field_name.to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn schema_row_from_consent_entry(
    claim_ref: &str,
    entry: &ConsentLedgerEntry,
) -> SchemaInspectionRow {
    SchemaInspectionRow {
        claim_ref: claim_ref.to_owned(),
        source_register_row_id: entry.entry_id.clone(),
        source_register_ref: CONSENT_LEDGER_SOURCE_REF.to_owned(),
        title: entry.title.clone(),
        schema_ref: entry.schema_family_binding.schema_ref.clone(),
        schema_version: entry.schema_family_binding.schema_version,
        owner_ref: entry.owner_ref.clone(),
        family_class: Some(entry.family_class.clone()),
        schema_role: None,
        schema_status: None,
        consent_class: Some(entry.consent_class.clone()),
        endpoint_class: Some(entry.endpoint_class.clone()),
        record_class_refs: entry.schema_family_binding.record_class_id_refs.clone(),
        local_only_lane_posture: Some(entry.local_only_lane_posture.clone()),
        local_only_alternative: Some(normalize_ws(&entry.local_only_lane_note)),
    }
}

fn schema_row_from_alpha_schema(row: &AlphaSchemaRow) -> SchemaInspectionRow {
    SchemaInspectionRow {
        claim_ref: row.row_id.clone(),
        source_register_row_id: row.row_id.clone(),
        source_register_ref: ALPHA_SCHEMA_REGISTRY_SOURCE_REF.to_owned(),
        title: row.title.clone(),
        schema_ref: row.schema_ref.clone(),
        schema_version: row.schema_version_pin,
        owner_ref: row.owner_dri.clone(),
        family_class: None,
        schema_role: Some(row.schema_role.clone()),
        schema_status: Some(row.schema_status.clone()),
        consent_class: None,
        endpoint_class: None,
        record_class_refs: row.record_class_refs.clone(),
        local_only_lane_posture: None,
        local_only_alternative: None,
    }
}

fn endpoint_policy_from_entry(
    claim_ref: &str,
    entry: &ConsentLedgerEntry,
    quoted_policy_row: bool,
) -> EndpointPolicyRow {
    let destination_class = destination_for_endpoint(&entry.endpoint_class);
    let consent_or_policy_state = consent_policy_state(&entry.consent_class);
    let outcome_class = endpoint_outcome(destination_class, consent_or_policy_state);
    EndpointPolicyRow {
        endpoint_policy_row_id: format!("endpoint_policy:{}", sanitize_id(claim_ref)),
        claimed_schema_ref: claim_ref.to_owned(),
        source_register_row_id: entry.entry_id.clone(),
        source_register_ref: CONSENT_LEDGER_SOURCE_REF.to_owned(),
        schema_ref: entry.schema_family_binding.schema_ref.clone(),
        destination_class,
        consent_or_policy_state,
        egress_class: egress_for_destination(destination_class),
        outcome_class,
        endpoint_class: entry.endpoint_class.clone(),
        local_only_alternative: normalize_ws(&entry.local_only_lane_note),
        redaction_class: redaction_for_entry(entry),
        retention_export_posture: normalize_ws(&entry.retention_note),
        quoted_policy_row,
    }
}

fn destination_for_endpoint(endpoint_class: &str) -> DestinationClass {
    match endpoint_class {
        "local_device_only" | "local_only" => DestinationClass::LocalOnly,
        "local_authoritative_with_optional_upload" => DestinationClass::OptionalTelemetryUpload,
        "export_only_user_initiated" => DestinationClass::ManualSupportExport,
        "managed_mirror_when_enabled" | "managed_authoritative_when_enabled" => {
            DestinationClass::ManagedAdminBroker
        }
        "admin_broker_gated" => DestinationClass::ManagedAdminBroker,
        "deletion_or_offboarding_export_channel_only" => DestinationClass::OffboardingExportChannel,
        "cli_stdio_local_only" => DestinationClass::CliLocalStdout,
        _ => DestinationClass::UnknownReviewRequired,
    }
}

fn consent_policy_state(consent_class: &str) -> ConsentPolicyState {
    match consent_class {
        "explicit_opt_in_required" | "explicit_opt_in_default_on_under_review" => {
            ConsentPolicyState::ExplicitOptInRequired
        }
        "export_only_on_user_request" => ConsentPolicyState::ExportOnlyUserRequest,
        "admin_policy_gated" => ConsentPolicyState::AdminPolicyGated,
        "implied_for_local_authoritative" | "local_mechanics_not_applicable" => {
            ConsentPolicyState::ImpliedLocalAuthoritative
        }
        "delete_governed_only" => ConsentPolicyState::DeleteGovernedOnly,
        "deny_by_default" => ConsentPolicyState::DenyByDefault,
        _ => ConsentPolicyState::NotApplicable,
    }
}

fn endpoint_outcome(
    destination: DestinationClass,
    consent_policy_state: ConsentPolicyState,
) -> EndpointOutcomeClass {
    match (destination, consent_policy_state) {
        (DestinationClass::LocalOnly, _) | (DestinationClass::CliLocalStdout, _) => {
            EndpointOutcomeClass::AllowLocalOnly
        }
        (_, ConsentPolicyState::ExplicitOptInRequired) => EndpointOutcomeClass::AllowAfterConsent,
        (_, ConsentPolicyState::ExportOnlyUserRequest) => EndpointOutcomeClass::AllowManualExport,
        (_, ConsentPolicyState::AdminPolicyGated) => EndpointOutcomeClass::AdminPolicyGated,
        (_, ConsentPolicyState::DenyByDefault) => EndpointOutcomeClass::DenyPolicy,
        (DestinationClass::UnknownReviewRequired, _) => EndpointOutcomeClass::ReviewRequired,
        _ => EndpointOutcomeClass::ReviewRequired,
    }
}

fn egress_for_destination(destination: DestinationClass) -> EgressClass {
    match destination {
        DestinationClass::LocalOnly | DestinationClass::LocalReview => EgressClass::TargetLocal,
        DestinationClass::OptionalTelemetryUpload => EgressClass::OrgApprovedExternalAfterOptIn,
        DestinationClass::ManualSupportExport
        | DestinationClass::SupportBundle
        | DestinationClass::ReleaseEvidencePacket
        | DestinationClass::IncidentPostmortem
        | DestinationClass::OffboardingExportChannel => EgressClass::UserSelectedExportDestination,
        DestinationClass::ManagedAdminBroker => EgressClass::ManagedControlPlane,
        DestinationClass::CliLocalStdout => EgressClass::CliStdoutLocalOnly,
        DestinationClass::UnknownReviewRequired => EgressClass::DenyAll,
    }
}

fn redaction_for_entry(entry: &ConsentLedgerEntry) -> SignalRedactionClass {
    let has_reviewed_opt_in = entry
        .default_ux_payload_exclusions
        .iter()
        .any(|exclusion| exclusion.override_policy.contains("opt_in"));
    if entry.endpoint_class == "export_only_user_initiated" || has_reviewed_opt_in {
        SignalRedactionClass::RedactedPayload
    } else {
        SignalRedactionClass::MetadataOnly
    }
}

fn desktop_signal_row(slice: &OperationalSignalSlice) -> DesktopSignalRow {
    DesktopSignalRow {
        signal_slice_id: slice.signal_slice_id.clone(),
        signal_kind: slice.signal_kind,
        source_backend: slice.source_backend.clone(),
        source_backend_ref: slice.source_backend_ref.clone(),
        window_label: signal_window_label(slice),
        freshness: slice.freshness,
        redaction_class: slice.redaction_class,
        truncation_state: slice.truncation_state,
        retention_export_posture: slice.retention_export_posture.clone(),
    }
}

fn support_signal_row(slice: &OperationalSignalSlice) -> EndpointPolicySupportSignalRow {
    EndpointPolicySupportSignalRow {
        signal_slice_id: slice.signal_slice_id.clone(),
        signal_kind: slice.signal_kind,
        source_backend: slice.source_backend.clone(),
        source_backend_ref: slice.source_backend_ref.clone(),
        window_label: signal_window_label(slice),
        freshness: slice.freshness,
        redaction_class: slice.redaction_class,
        truncation_state: slice.truncation_state,
        destination_class: slice.destination_class,
        retention_export_posture: slice.retention_export_posture.clone(),
    }
}

fn runbook_signal_row(slice: &OperationalSignalSlice) -> RunbookHelpSignalRow {
    RunbookHelpSignalRow {
        signal_slice_id: slice.signal_slice_id.clone(),
        signal_kind: slice.signal_kind,
        source_backend: slice.source_backend.clone(),
        window_label: signal_window_label(slice),
        freshness: slice.freshness,
        redaction_class: slice.redaction_class,
        destination_class: slice.destination_class,
        handoff_cue: format!(
            "{} evidence from {} is {} with {} redaction; {}",
            slice.signal_kind.as_str(),
            slice.source_backend,
            slice.freshness.as_str(),
            slice.redaction_class.as_str(),
            slice.retention_export_posture
        ),
    }
}

fn signal_window_label(slice: &OperationalSignalSlice) -> String {
    format!(
        "{} to {} ({}, {})",
        slice.time_window.window_start_utc,
        slice.time_window.window_end_utc,
        slice.time_window.display_time_zone_iana,
        slice.time_window.display_utc_offset
    )
}

fn normalize_ws(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

#[derive(Debug, Deserialize)]
struct ConsentLedger {
    rows: Vec<ConsentLedgerEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct ConsentLedgerEntry {
    entry_id: String,
    family_class: String,
    title: String,
    owner_ref: String,
    schema_family_binding: SchemaFamilyBinding,
    consent_class: String,
    endpoint_class: String,
    retention_note: String,
    #[serde(default)]
    default_ux_payload_exclusions: Vec<PayloadExclusion>,
    local_only_lane_posture: String,
    local_only_lane_note: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SchemaFamilyBinding {
    schema_ref: String,
    schema_version: u32,
    #[serde(default)]
    record_class_id_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct PayloadExclusion {
    override_policy: String,
}

#[derive(Debug, Deserialize)]
struct AlphaSchemaRegistry {
    schema_rows: Vec<AlphaSchemaRow>,
}

#[derive(Debug, Clone, Deserialize)]
struct AlphaSchemaRow {
    row_id: String,
    title: String,
    schema_role: String,
    schema_ref: String,
    schema_status: String,
    owner_dri: String,
    schema_version_pin: u32,
    #[serde(default)]
    record_class_refs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signal_slices() -> Vec<OperationalSignalSlice> {
        vec![
            signal_slice(
                "signal.slice.log.live",
                OperationalSignalKind::Logs,
                "local structured log stream",
                SignalFreshnessClass::Live,
                SignalTruncationState::Clipped,
                SignalRedactionClass::RedactedPayload,
                DestinationClass::SupportBundle,
            ),
            signal_slice(
                "signal.slice.metric.cached",
                OperationalSignalKind::Metrics,
                "managed metrics provider",
                SignalFreshnessClass::Cached,
                SignalTruncationState::Downsampled,
                SignalRedactionClass::MetadataOnly,
                DestinationClass::ReleaseEvidencePacket,
            ),
            signal_slice(
                "signal.slice.trace.partial",
                OperationalSignalKind::Traces,
                "imported trace archive",
                SignalFreshnessClass::Partial,
                SignalTruncationState::NotTruncated,
                SignalRedactionClass::ByReferenceOnly,
                DestinationClass::LocalReview,
            ),
            signal_slice(
                "signal.slice.incident.offline",
                OperationalSignalKind::IncidentTimeline,
                "post-incident evidence bundle",
                SignalFreshnessClass::Offline,
                SignalTruncationState::PolicyRedacted,
                SignalRedactionClass::WithheldByPolicy,
                DestinationClass::IncidentPostmortem,
            ),
        ]
    }

    fn signal_slice(
        signal_slice_id: &str,
        signal_kind: OperationalSignalKind,
        source_backend: &str,
        freshness: SignalFreshnessClass,
        truncation_state: SignalTruncationState,
        redaction_class: SignalRedactionClass,
        destination_class: DestinationClass,
    ) -> OperationalSignalSlice {
        OperationalSignalSlice {
            signal_slice_id: signal_slice_id.to_owned(),
            signal_kind,
            source_backend: source_backend.to_owned(),
            source_backend_ref: format!("source.ref.{signal_slice_id}"),
            time_window: SignalTimeWindow {
                window_start_utc: "2026-05-14T00:00:00Z".to_owned(),
                window_end_utc: "2026-05-14T00:05:00Z".to_owned(),
                display_time_zone_iana: "UTC".to_owned(),
                display_utc_offset: "+00:00".to_owned(),
            },
            freshness,
            truncation_state,
            redaction_class,
            retention_export_posture: "metadata retained locally; export preserves omission notes"
                .to_owned(),
            destination_class,
        }
    }

    fn inspection_input() -> EndpointPolicyInspectionInput {
        EndpointPolicyInspectionInput {
            inspection_id: "inspection.schema_endpoint_policy.test".to_owned(),
            generated_at: "2026-05-14T00:05:00Z".to_owned(),
            claimed_schema_refs: vec![
                "telemetry.ux_product_event".to_owned(),
                "support.bundle_manifest".to_owned(),
                "schema_alpha:support_export.bundle_manifest".to_owned(),
            ],
            operational_signal_slices: signal_slices(),
            support_export_id: "support.export.schema_endpoint_policy.test".to_owned(),
            runbook_handoff_id: "runbook.handoff.schema_endpoint_policy.test".to_owned(),
        }
    }

    #[test]
    fn schema_registry_endpoint_policy_default_registers_resolve_telemetry_support_and_alpha_schema_rows(
    ) {
        let inspector = SchemaRegistryInspector::from_default_artifact_registers()
            .expect("load checked-in registers");
        let snapshot = inspector
            .inspect(inspection_input())
            .expect("inspect claims");

        assert!(snapshot.schema_rows.iter().any(|row| {
            row.claim_ref == "telemetry.ux_product_event"
                && row.title == "UX product-usage telemetry event"
                && row.source_register_ref == CONSENT_LEDGER_SOURCE_REF
        }));
        assert!(snapshot.schema_rows.iter().any(|row| {
            row.claim_ref == "support.bundle_manifest"
                && row.endpoint_class.as_deref() == Some("export_only_user_initiated")
        }));
        assert!(snapshot.schema_rows.iter().any(|row| {
            row.claim_ref == "schema_alpha:support_export.bundle_manifest"
                && row.schema_role.as_deref() == Some("support_export_packet_schema")
                && row.source_register_ref == ALPHA_SCHEMA_REGISTRY_SOURCE_REF
        }));
        assert_eq!(snapshot.endpoint_policy_rows.len(), 3);
        assert!(snapshot.endpoint_policy_rows.iter().any(|row| {
            row.claimed_schema_ref == "telemetry.ux_product_event"
                && row.destination_class == DestinationClass::OptionalTelemetryUpload
                && row.consent_or_policy_state == ConsentPolicyState::ExplicitOptInRequired
                && row.local_only_alternative.contains("fully functional")
        }));
        assert!(snapshot.endpoint_policy_rows.iter().any(|row| {
            row.claimed_schema_ref == "schema_alpha:support_export.bundle_manifest"
                && row.quoted_policy_row
        }));
    }

    #[test]
    fn schema_registry_endpoint_policy_signal_rows_share_freshness_and_redaction_across_projections(
    ) {
        let inspector = SchemaRegistryInspector::from_default_artifact_registers()
            .expect("load checked-in registers");
        let snapshot = inspector
            .inspect(inspection_input())
            .expect("inspect claims");

        assert!(snapshot.has_cross_surface_signal_vocabulary_parity());
        assert_eq!(snapshot.operational_signal_slices.len(), 4);
        let observed: BTreeSet<_> = snapshot
            .operational_signal_slices
            .iter()
            .map(|slice| slice.signal_kind)
            .collect();
        assert_eq!(
            observed,
            BTreeSet::from([
                OperationalSignalKind::Logs,
                OperationalSignalKind::Metrics,
                OperationalSignalKind::Traces,
                OperationalSignalKind::IncidentTimeline,
            ])
        );
        assert_eq!(
            snapshot.support_export.freshness_vocabulary,
            SignalFreshnessClass::vocabulary()
        );
        assert_eq!(
            snapshot.runbook_help_handoff.redaction_vocabulary,
            SignalRedactionClass::vocabulary()
        );
    }

    #[test]
    fn schema_registry_endpoint_policy_unknown_claim_fails_closed() {
        let inspector = SchemaRegistryInspector::from_default_artifact_registers()
            .expect("load checked-in registers");
        let mut input = inspection_input();
        input.claimed_schema_refs = vec!["telemetry.unknown_payload".to_owned()];

        let err = inspector.inspect(input).expect_err("unknown claim denied");
        assert_eq!(
            err,
            SchemaRegistryInspectorError::UnknownClaim {
                claim_ref: "telemetry.unknown_payload".to_owned()
            }
        );
    }
}
