//! M5 depth-surface telemetry, diagnostics, consent, and export registry.
//!
//! This module freezes the support-side packet that binds M5 depth surfaces to
//! declared schema rows, consent-ledger inheritance, endpoint truth, retention
//! posture, and redaction-default packet classes. It gives support, release,
//! diagnostics, and CLI/headless consumers one inspectable object for:
//!
//! - notebook, provider, profiler, pipeline, preview, and data surfaces;
//! - crash, performance, feature-usage, and support-export signal families;
//! - schema id/version, purpose, allowed fields, and prohibited content
//!   classes for each declared signal;
//! - visible consent and endpoint state that stays local-first for ordinary
//!   diagnostics on open-source and local builds; and
//! - redaction-default packet classes that keep support export explicit rather
//!   than collapsing it into ambient telemetry.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::m5_fault_crash_governance::{
    DiagnosticDataClass, DiagnosticOptInScope, RedactionProfileClass, RetentionClass,
};

const CONSENT_LEDGER_REF: &str = "artifacts/governance/consent_ledger_seed.yaml";
const TELEMETRY_SUPPORT_STABLE_REGISTRY_REF: &str =
    "artifacts/governance/telemetry_support_usage_schema_registry.json";
const ENDPOINT_POLICY_DOC_REF: &str = "docs/privacy/endpoint_policy_alpha.md";
const PACKET_CLASS_REGISTRY_REF: &str = "artifacts/governance/packet_class_registry.yaml";
const CRASH_SCHEMA_REF: &str =
    "schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json";
const PERFORMANCE_SCHEMA_REF: &str =
    "schemas/perf/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.schema.json";
const FEATURE_USAGE_SCHEMA_REF: &str = "schemas/telemetry/event_catalog.schema.json";
const SUPPORT_EXPORT_SCHEMA_REF: &str = "schemas/support/support_bundle_manifest.schema.json";

/// Frozen schema version for the depth-surface schema registry packet.
pub const DEPTH_SCHEMA_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the depth-surface schema registry packet.
pub const DEPTH_SCHEMA_REGISTRY_PACKET_RECORD_KIND: &str = "depth_surface_schema_registry_packet";

/// Repository-relative path of the boundary schema.
pub const DEPTH_SCHEMA_REGISTRY_SCHEMA_REF: &str =
    "schemas/support/depth-surface-schema-registry.schema.json";

/// Repository-relative path of the reviewer-facing help document.
pub const DEPTH_SCHEMA_REGISTRY_DOC_REF: &str =
    "docs/help/support/depth-surface-schema-registry.md";

/// Repository-relative path of the review artifact.
pub const DEPTH_SCHEMA_REGISTRY_ARTIFACT_REF: &str =
    "artifacts/support/m5/depth-surface-schema-registry.md";

/// Repository-relative path of the fixture corpus.
pub const DEPTH_SCHEMA_REGISTRY_FIXTURE_DIR: &str =
    "fixtures/support/m5/depth_surface_schema_registry";

const REQUIRED_GUARDRAIL_CLASSES: &[&str] = &[
    "source_code_bodies",
    "filenames_and_paths",
    "prompt_bodies",
    "terminal_contents",
    "secret_material",
    "clipboard_contents",
];

/// Closed vocabulary for M5 depth surfaces covered by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepthSurfaceClass {
    /// Notebook kernel and execution surface.
    NotebookKernel,
    /// Provider-run session and AI-tool broker surface.
    ProviderRunSession,
    /// Profiler capture, replay, and analysis surface.
    ProfilerReplaySession,
    /// Pipeline and CI-viewer surface.
    PipelineViewer,
    /// Preview dev-server and preview runtime surface.
    PreviewDevServer,
    /// Data, query, and registry connector surface.
    DataApiConnector,
}

impl DepthSurfaceClass {
    /// All required depth surfaces in canonical order.
    pub const ALL: [Self; 6] = [
        Self::NotebookKernel,
        Self::ProviderRunSession,
        Self::ProfilerReplaySession,
        Self::PipelineViewer,
        Self::PreviewDevServer,
        Self::DataApiConnector,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookKernel => "notebook_kernel",
            Self::ProviderRunSession => "provider_run_session",
            Self::ProfilerReplaySession => "profiler_replay_session",
            Self::PipelineViewer => "pipeline_viewer",
            Self::PreviewDevServer => "preview_dev_server",
            Self::DataApiConnector => "data_api_connector",
        }
    }

    /// Returns a short review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotebookKernel => "Notebook kernel",
            Self::ProviderRunSession => "Provider run session",
            Self::ProfilerReplaySession => "Profiler/replay session",
            Self::PipelineViewer => "Pipeline viewer",
            Self::PreviewDevServer => "Preview dev server",
            Self::DataApiConnector => "Data/API connector",
        }
    }
}

/// Closed signal vocabulary frozen by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepthSignalClass {
    /// Crash, panic, and restart-adjacent diagnostics.
    CrashDiagnostic,
    /// Performance, profile, and trace summaries.
    PerformanceDiagnostic,
    /// Feature-usage and coarse workflow telemetry.
    FeatureUsageTelemetry,
    /// Explicit user/admin support export.
    SupportExport,
}

impl DepthSignalClass {
    /// All required signal classes in canonical order.
    pub const ALL: [Self; 4] = [
        Self::CrashDiagnostic,
        Self::PerformanceDiagnostic,
        Self::FeatureUsageTelemetry,
        Self::SupportExport,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashDiagnostic => "crash_diagnostic",
            Self::PerformanceDiagnostic => "performance_diagnostic",
            Self::FeatureUsageTelemetry => "feature_usage_telemetry",
            Self::SupportExport => "support_export",
        }
    }
}

/// Closed consent-state vocabulary projected into diagnostics and CLI output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStateClass {
    /// Local capture exists, but any egress waits for explicit submission.
    LocalCapturePendingSubmission,
    /// The signal is disabled until the user explicitly opts in.
    DisabledPendingOptIn,
    /// The signal exists only after the user starts an export flow.
    ExplicitUserRequestRequired,
    /// Admin policy disabled the signal on the active lane.
    AdminLockedDisabled,
}

impl ConsentStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCapturePendingSubmission => "local_capture_pending_submission",
            Self::DisabledPendingOptIn => "disabled_pending_opt_in",
            Self::ExplicitUserRequestRequired => "explicit_user_request_required",
            Self::AdminLockedDisabled => "admin_locked_disabled",
        }
    }
}

/// Closed endpoint-state vocabulary projected into diagnostics and CLI output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointStateClass {
    /// The signal remains local until an explicit submission step.
    LocalOnlyUntilExplicitSubmission,
    /// Upload is possible only after explicit opt-in.
    OptionalUploadAfterOptIn,
    /// The signal is available only through a manual export flow.
    ManualExportOnly,
    /// Policy has disabled any active endpoint.
    DisabledByPolicy,
}

impl EndpointStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyUntilExplicitSubmission => "local_only_until_explicit_submission",
            Self::OptionalUploadAfterOptIn => "optional_upload_after_opt_in",
            Self::ManualExportOnly => "manual_export_only",
            Self::DisabledByPolicy => "disabled_by_policy",
        }
    }
}

/// One declared depth-surface schema row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaDeclarationRow {
    /// Stable schema identifier quoted by diagnostics and CLI/headless output.
    pub schema_id: String,
    /// Frozen descriptor version for this schema declaration.
    pub schema_version: u32,
    /// Boundary schema path the declaration inherits.
    pub schema_ref: String,
    /// Covered depth surface.
    pub surface: DepthSurfaceClass,
    /// Covered signal family.
    pub signal_class: DepthSignalClass,
    /// Reviewable purpose sentence.
    pub purpose: String,
    /// Shared data-class label.
    pub data_class: DiagnosticDataClass,
    /// Collection and opt-in posture for the family.
    pub opt_in_scope: DiagnosticOptInScope,
    /// Inspectable consent state for the active lane.
    pub active_consent_state: ConsentStateClass,
    /// Inspectable endpoint state for the active lane.
    pub active_endpoint_state: EndpointStateClass,
    /// Allowed machine-readable fields for the family.
    pub allowed_fields: Vec<String>,
    /// Content classes forbidden by default.
    pub prohibited_content_classes: Vec<String>,
    /// Retention class for the family.
    pub retention_class: RetentionClass,
    /// Default redaction profile for the family.
    pub redaction_profile: RedactionProfileClass,
    /// Stable owner reference.
    pub owner_ref: String,
    /// Stable reviewer references.
    pub reviewer_refs: Vec<String>,
    /// Repo-relative evidence and design references.
    pub evidence_refs: Vec<String>,
}

/// One consent-ledger inheritance row quoted by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentLedgerBindingRow {
    /// Covered signal family.
    pub signal_class: DepthSignalClass,
    /// Canonical consent-ledger row inherited by this signal.
    pub consent_ledger_entry_ref: String,
    /// Stable registry row that exposes endpoint truth by context.
    pub stable_registry_entry_ref: String,
    /// Endpoint inspector doc that surfaces the row.
    pub endpoint_policy_ref: String,
    /// Reviewable explanation of the inheritance choice.
    pub inheritance_note: String,
    /// True when open-source and local builds stay local-first by default.
    pub open_source_local_default: bool,
    /// True when managed builds may narrow but never silently broaden fields.
    pub managed_builds_may_narrow_but_not_broaden: bool,
    /// True when the row remains an explicit export, not ambient telemetry.
    pub explicit_export_not_ambient_telemetry: bool,
}

/// One per-surface inspection summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceInspectionRow {
    /// Covered depth surface.
    pub surface: DepthSurfaceClass,
    /// Declared signal classes present for the surface.
    pub declared_signal_classes: Vec<DepthSignalClass>,
    /// Signal classes disabled by default on local/open-source lanes.
    pub disabled_by_default_signal_classes: Vec<DepthSignalClass>,
    /// Signal classes available only through explicit export.
    pub export_only_signal_classes: Vec<DepthSignalClass>,
    /// Packet classes the surface may export under default redaction.
    pub redaction_default_packet_class_refs: Vec<String>,
    /// Retention classes the surface may emit under this packet.
    pub retention_classes: Vec<RetentionClass>,
    /// Local-only support note visible to users and support.
    pub inspection_note: String,
}

/// One redaction-default packet-class row for M5 depth surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketClassManifestRow {
    /// Stable packet-class identifier.
    pub packet_class_id: String,
    /// Review-safe label.
    pub packet_class_label: String,
    /// Covered depth surfaces.
    pub surfaces: Vec<DepthSurfaceClass>,
    /// Covered signal classes.
    pub signal_classes: Vec<DepthSignalClass>,
    /// Default redaction profile applied to the packet.
    pub default_redaction_profile: RedactionProfileClass,
    /// True when the packet stays local by default.
    pub local_only_by_default: bool,
    /// True when explicit user or admin review is required before share.
    pub explicit_review_required_before_share: bool,
    /// True when raw source code is forbidden by default.
    pub raw_source_code_forbidden: bool,
    /// True when filenames and paths are forbidden by default.
    pub filenames_and_paths_forbidden: bool,
    /// True when prompt bodies are forbidden by default.
    pub prompt_bodies_forbidden: bool,
    /// True when terminal contents are forbidden by default.
    pub terminal_contents_forbidden: bool,
    /// True when secret-bearing content is forbidden by default.
    pub secrets_forbidden: bool,
    /// True when clipboard content is forbidden by default.
    pub clipboard_contents_forbidden: bool,
    /// Repo-relative evidence and policy references.
    pub evidence_refs: Vec<String>,
}

/// One validation error reported by [`DepthSurfaceSchemaRegistryPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthSurfaceSchemaRegistryViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical support-side schema registry packet for M5 depth surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthSurfaceSchemaRegistryPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// RFC 3339 UTC generation timestamp.
    pub generated_at: String,
    /// Reviewer-facing help doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing contracts this packet composes.
    pub supporting_contract_refs: Vec<String>,
    /// High-level invariants consumers must preserve.
    pub normative_rules: Vec<String>,
    /// Declared schema rows for every required surface/signal pair.
    pub schema_declarations: Vec<SchemaDeclarationRow>,
    /// Consent-ledger inheritance rows that keep the packet tied to canonical policy.
    pub consent_ledger_bindings: Vec<ConsentLedgerBindingRow>,
    /// Inspectable surface summaries for diagnostics and CLI/headless output.
    pub surface_inspections: Vec<SurfaceInspectionRow>,
    /// Redaction-default packet classes available to the covered surfaces.
    pub packet_class_manifest: Vec<PacketClassManifestRow>,
    /// Metadata-safe summary for support and release surfaces.
    pub export_safe_summary: String,
}

impl DepthSurfaceSchemaRegistryPacket {
    /// Validates closed-vocabulary coverage and the task's guardrails.
    pub fn validate(&self) -> Vec<DepthSurfaceSchemaRegistryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DEPTH_SCHEMA_REGISTRY_PACKET_RECORD_KIND {
            violations.push(DepthSurfaceSchemaRegistryViolation {
                path: "record_kind".to_owned(),
                message: "unexpected record_kind".to_owned(),
            });
        }
        if self.schema_version != DEPTH_SCHEMA_REGISTRY_SCHEMA_VERSION {
            violations.push(DepthSurfaceSchemaRegistryViolation {
                path: "schema_version".to_owned(),
                message: "unexpected schema_version".to_owned(),
            });
        }

        let mut declaration_coverage = BTreeSet::new();
        for row in &self.schema_declarations {
            declaration_coverage.insert((row.surface, row.signal_class));
            if row.schema_version == 0 {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("schema_declarations.{}", row.schema_id),
                    message: "schema_version must be >= 1".to_owned(),
                });
            }
            if row.allowed_fields.is_empty() {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("schema_declarations.{}.allowed_fields", row.schema_id),
                    message: "allowed_fields must not be empty".to_owned(),
                });
            }
            if row.owner_ref.trim().is_empty()
                || row.reviewer_refs.is_empty()
                || row.evidence_refs.is_empty()
            {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("schema_declarations.{}", row.schema_id),
                    message: "owner, reviewers, and evidence refs are required".to_owned(),
                });
            }
            if row.signal_class != DepthSignalClass::SupportExport {
                for guardrail in REQUIRED_GUARDRAIL_CLASSES {
                    if !row
                        .prohibited_content_classes
                        .iter()
                        .any(|item| item == guardrail)
                    {
                        violations.push(DepthSurfaceSchemaRegistryViolation {
                            path: format!(
                                "schema_declarations.{}.prohibited_content_classes",
                                row.schema_id
                            ),
                            message: format!(
                                "ordinary diagnostic/telemetry schema must forbid {guardrail}"
                            ),
                        });
                    }
                }
            }
            if row.signal_class == DepthSignalClass::SupportExport
                && (row.opt_in_scope != DiagnosticOptInScope::UserInitiatedExportOnly
                    || row.active_consent_state != ConsentStateClass::ExplicitUserRequestRequired
                    || row.active_endpoint_state != EndpointStateClass::ManualExportOnly)
            {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("schema_declarations.{}", row.schema_id),
                    message: "support export must remain an explicit user/admin export flow"
                        .to_owned(),
                });
            }
        }

        for surface in DepthSurfaceClass::ALL {
            for signal in DepthSignalClass::ALL {
                if !declaration_coverage.contains(&(surface, signal)) {
                    violations.push(DepthSurfaceSchemaRegistryViolation {
                        path: "schema_declarations".to_owned(),
                        message: format!(
                            "missing schema declaration for {} / {}",
                            surface.as_str(),
                            signal.as_str()
                        ),
                    });
                }
            }
        }

        let binding_signals = self
            .consent_ledger_bindings
            .iter()
            .map(|row| row.signal_class)
            .collect::<BTreeSet<_>>();
        for signal in DepthSignalClass::ALL {
            if !binding_signals.contains(&signal) {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: "consent_ledger_bindings".to_owned(),
                    message: format!("missing consent-ledger binding for {}", signal.as_str()),
                });
            }
        }

        for row in &self.consent_ledger_bindings {
            if !row.open_source_local_default && row.signal_class != DepthSignalClass::SupportExport
            {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("consent_ledger_bindings.{}", row.signal_class.as_str()),
                    message: "ordinary crash/performance/usage signals must stay local-first on open-source and local builds".to_owned(),
                });
            }
            if !row.managed_builds_may_narrow_but_not_broaden {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("consent_ledger_bindings.{}", row.signal_class.as_str()),
                    message:
                        "managed builds may narrow posture but must not silently broaden fields"
                            .to_owned(),
                });
            }
            if row.signal_class == DepthSignalClass::SupportExport
                && !row.explicit_export_not_ambient_telemetry
            {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("consent_ledger_bindings.{}", row.signal_class.as_str()),
                    message: "support export must not be treated as ambient telemetry".to_owned(),
                });
            }
        }

        let packet_class_ids = self
            .packet_class_manifest
            .iter()
            .map(|row| row.packet_class_id.as_str())
            .collect::<BTreeSet<_>>();
        for row in &self.packet_class_manifest {
            if !(row.raw_source_code_forbidden
                && row.filenames_and_paths_forbidden
                && row.prompt_bodies_forbidden
                && row.terminal_contents_forbidden
                && row.secrets_forbidden
                && row.clipboard_contents_forbidden)
            {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("packet_class_manifest.{}", row.packet_class_id),
                    message: "redaction-default packet classes must forbid raw code, filenames, prompts, terminal contents, secrets, and clipboard by default".to_owned(),
                });
            }
            if row.evidence_refs.is_empty() {
                violations.push(DepthSurfaceSchemaRegistryViolation {
                    path: format!("packet_class_manifest.{}", row.packet_class_id),
                    message: "packet class rows require evidence refs".to_owned(),
                });
            }
        }

        for row in &self.surface_inspections {
            let declared = row
                .declared_signal_classes
                .iter()
                .copied()
                .collect::<BTreeSet<_>>();
            for signal in DepthSignalClass::ALL {
                if !declared.contains(&signal) {
                    violations.push(DepthSurfaceSchemaRegistryViolation {
                        path: format!("surface_inspections.{}", row.surface.as_str()),
                        message: format!(
                            "surface inspection missing declared signal {}",
                            signal.as_str()
                        ),
                    });
                }
            }
            for packet_class in &row.redaction_default_packet_class_refs {
                if !packet_class_ids.contains(packet_class.as_str()) {
                    violations.push(DepthSurfaceSchemaRegistryViolation {
                        path: format!("surface_inspections.{}", row.surface.as_str()),
                        message: format!("unknown packet class reference {}", packet_class),
                    });
                }
            }
        }

        violations
    }

    /// Returns true when the packet remains metadata-safe for ordinary export.
    pub fn is_export_safe(&self) -> bool {
        let json = serde_json::to_string(self).unwrap_or_default();
        !json.contains("/Users/")
            && !json.contains("BEGIN PRIVATE KEY")
            && !json.contains("terminal history")
            && !json.contains("prompt body")
    }
}

/// Returns the canonical depth-surface schema registry packet.
pub fn seeded_depth_surface_schema_registry_packet() -> DepthSurfaceSchemaRegistryPacket {
    DepthSurfaceSchemaRegistryPacket {
        record_kind: DEPTH_SCHEMA_REGISTRY_PACKET_RECORD_KIND.to_owned(),
        schema_version: DEPTH_SCHEMA_REGISTRY_SCHEMA_VERSION,
        packet_id: "depth-surface-schema-registry".to_owned(),
        generated_at: "2026-06-12T00:00:00Z".to_owned(),
        doc_ref: DEPTH_SCHEMA_REGISTRY_DOC_REF.to_owned(),
        schema_ref: DEPTH_SCHEMA_REGISTRY_SCHEMA_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Technical_Design_Document.md#7126-telemetry-schema-registry-and-consent-ledger".to_owned(),
            ".t2/docs/Aureline_PRD.md#1022-diagnostic-data-classes-and-support-bundle-redaction"
                .to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#185-diagnostics-and-support-center"
                .to_owned(),
        ],
        supporting_contract_refs: vec![
            CONSENT_LEDGER_REF.to_owned(),
            TELEMETRY_SUPPORT_STABLE_REGISTRY_REF.to_owned(),
            ENDPOINT_POLICY_DOC_REF.to_owned(),
            PACKET_CLASS_REGISTRY_REF.to_owned(),
            "docs/governance/telemetry_and_support_schema_registry.md".to_owned(),
            "docs/support/support_bundle_contract.md".to_owned(),
        ],
        normative_rules: vec![
            "Every M5 depth-surface telemetry or diagnostic signal must bind to a declared schema row before it ships.".to_owned(),
            "Open-source and local builds stay opt-in and local-first for crash, performance, and feature-usage signals.".to_owned(),
            "Managed builds may narrow consent, endpoint, retention, or packet contents, but they may not silently broaden fields beyond the shipped registry vocabulary.".to_owned(),
            "Support bundles reuse the same registry for classification and redaction, but they remain explicit export flows rather than ambient telemetry.".to_owned(),
        ],
        schema_declarations: seed_schema_declarations(),
        consent_ledger_bindings: seed_consent_ledger_bindings(),
        surface_inspections: seed_surface_inspections(),
        packet_class_manifest: seed_packet_class_manifest(),
        export_safe_summary: "M5 depth surfaces declare crash, performance, feature-usage, and support-export schemas with local-first consent posture, inspectable endpoint truth, and redaction-default packet classes.".to_owned(),
    }
}

fn seed_schema_declarations() -> Vec<SchemaDeclarationRow> {
    let mut rows = Vec::new();
    for surface in DepthSurfaceClass::ALL {
        rows.push(schema_row(
            surface,
            DepthSignalClass::CrashDiagnostic,
            CRASH_SCHEMA_REF,
            DiagnosticDataClass::EnvironmentAdjacent,
            DiagnosticOptInScope::ExplicitSubmissionOrPolicy,
            ConsentStateClass::LocalCapturePendingSubmission,
            EndpointStateClass::LocalOnlyUntilExplicitSubmission,
            crash_allowed_fields(),
            "diagnostics_governance",
            &["support_export", "privacy_review"],
            RedactionProfileClass::EnvironmentSummaryOnly,
            RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
            "Local crash evidence for the depth surface stays on-device until the user or policy explicitly submits it.",
        ));
        rows.push(schema_row(
            surface,
            DepthSignalClass::PerformanceDiagnostic,
            PERFORMANCE_SCHEMA_REF,
            DiagnosticDataClass::MetadataOnly,
            DiagnosticOptInScope::OptInOnly,
            ConsentStateClass::DisabledPendingOptIn,
            EndpointStateClass::OptionalUploadAfterOptIn,
            performance_allowed_fields(),
            "performance_governance",
            &["support_export", "privacy_review"],
            RedactionProfileClass::MetadataSafeDefault,
            RetentionClass::LocalSamplingWindow,
            "Coarse performance summaries stay sampled, redaction-aware, and opt-in only.",
        ));
        rows.push(schema_row(
            surface,
            DepthSignalClass::FeatureUsageTelemetry,
            FEATURE_USAGE_SCHEMA_REF,
            DiagnosticDataClass::MetadataOnly,
            DiagnosticOptInScope::OptInOnly,
            ConsentStateClass::DisabledPendingOptIn,
            EndpointStateClass::OptionalUploadAfterOptIn,
            feature_usage_allowed_fields(),
            "telemetry_governance",
            &["support_export", "privacy_review"],
            RedactionProfileClass::MetadataSafeDefault,
            RetentionClass::LocalSamplingWindow,
            "Feature-usage counters remain coarse, stable-named, and opt-in only.",
        ));
        rows.push(schema_row(
            surface,
            DepthSignalClass::SupportExport,
            SUPPORT_EXPORT_SCHEMA_REF,
            DiagnosticDataClass::MetadataOnly,
            DiagnosticOptInScope::UserInitiatedExportOnly,
            ConsentStateClass::ExplicitUserRequestRequired,
            EndpointStateClass::ManualExportOnly,
            support_export_allowed_fields(),
            "support_export",
            &["support_center", "privacy_review"],
            RedactionProfileClass::LocalOnlyReviewRequired,
            RetentionClass::LocalManifestUntilSent,
            "Support export remains a reviewed local-first bundle manifest rather than always-on collection.",
        ));
    }
    rows
}

fn schema_row(
    surface: DepthSurfaceClass,
    signal_class: DepthSignalClass,
    schema_ref: &str,
    data_class: DiagnosticDataClass,
    opt_in_scope: DiagnosticOptInScope,
    active_consent_state: ConsentStateClass,
    active_endpoint_state: EndpointStateClass,
    allowed_fields: Vec<String>,
    owner_ref: &str,
    reviewer_refs: &[&str],
    redaction_profile: RedactionProfileClass,
    retention_class: RetentionClass,
    purpose: &str,
) -> SchemaDeclarationRow {
    SchemaDeclarationRow {
        schema_id: format!("m5.{}.{}", surface.as_str(), signal_class.as_str()),
        schema_version: 1,
        schema_ref: schema_ref.to_owned(),
        surface,
        signal_class,
        purpose: purpose.to_owned(),
        data_class,
        opt_in_scope,
        active_consent_state,
        active_endpoint_state,
        allowed_fields,
        prohibited_content_classes: REQUIRED_GUARDRAIL_CLASSES
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        retention_class,
        redaction_profile,
        owner_ref: owner_ref.to_owned(),
        reviewer_refs: reviewer_refs
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        evidence_refs: vec![
            CONSENT_LEDGER_REF.to_owned(),
            TELEMETRY_SUPPORT_STABLE_REGISTRY_REF.to_owned(),
            ENDPOINT_POLICY_DOC_REF.to_owned(),
        ],
    }
}

fn seed_consent_ledger_bindings() -> Vec<ConsentLedgerBindingRow> {
    vec![
        ConsentLedgerBindingRow {
            signal_class: DepthSignalClass::CrashDiagnostic,
            consent_ledger_entry_ref: format!("{CONSENT_LEDGER_REF}#diagnostics.crash_payload"),
            stable_registry_entry_ref: format!(
                "{TELEMETRY_SUPPORT_STABLE_REGISTRY_REF}#diagnostics.crash_payload"
            ),
            endpoint_policy_ref: ENDPOINT_POLICY_DOC_REF.to_owned(),
            inheritance_note: "Depth-surface crash diagnostics inherit the canonical crash-payload consent posture: local capture first, explicit submission later, never automatic upload.".to_owned(),
            open_source_local_default: true,
            managed_builds_may_narrow_but_not_broaden: true,
            explicit_export_not_ambient_telemetry: false,
        },
        ConsentLedgerBindingRow {
            signal_class: DepthSignalClass::PerformanceDiagnostic,
            consent_ledger_entry_ref: format!("{CONSENT_LEDGER_REF}#telemetry.ux_product_event"),
            stable_registry_entry_ref: format!(
                "{TELEMETRY_SUPPORT_STABLE_REGISTRY_REF}#telemetry.ux_product_event"
            ),
            endpoint_policy_ref: ENDPOINT_POLICY_DOC_REF.to_owned(),
            inheritance_note: "The current repo-wide consent ledger has no dedicated performance family, so M5 performance summaries inherit the strict opt-in local-first telemetry posture while staying narrower than the coarse product telemetry field set.".to_owned(),
            open_source_local_default: true,
            managed_builds_may_narrow_but_not_broaden: true,
            explicit_export_not_ambient_telemetry: false,
        },
        ConsentLedgerBindingRow {
            signal_class: DepthSignalClass::FeatureUsageTelemetry,
            consent_ledger_entry_ref: format!("{CONSENT_LEDGER_REF}#telemetry.ux_product_event"),
            stable_registry_entry_ref: format!(
                "{TELEMETRY_SUPPORT_STABLE_REGISTRY_REF}#telemetry.ux_product_event"
            ),
            endpoint_policy_ref: ENDPOINT_POLICY_DOC_REF.to_owned(),
            inheritance_note: "Depth-surface feature-usage counters inherit the canonical UX telemetry row so coarse capability counters stay opt-in, sampled, and non-coercive on local-only lanes.".to_owned(),
            open_source_local_default: true,
            managed_builds_may_narrow_but_not_broaden: true,
            explicit_export_not_ambient_telemetry: false,
        },
        ConsentLedgerBindingRow {
            signal_class: DepthSignalClass::SupportExport,
            consent_ledger_entry_ref: format!("{CONSENT_LEDGER_REF}#support.bundle_manifest"),
            stable_registry_entry_ref: format!(
                "{TELEMETRY_SUPPORT_STABLE_REGISTRY_REF}#support.bundle_manifest"
            ),
            endpoint_policy_ref: ENDPOINT_POLICY_DOC_REF.to_owned(),
            inheritance_note: "Depth-surface support export inherits the canonical support-bundle manifest row: explicit preview and user/admin initiation only, with no always-on telemetry interpretation.".to_owned(),
            open_source_local_default: true,
            managed_builds_may_narrow_but_not_broaden: true,
            explicit_export_not_ambient_telemetry: true,
        },
    ]
}

fn seed_surface_inspections() -> Vec<SurfaceInspectionRow> {
    DepthSurfaceClass::ALL
        .iter()
        .copied()
        .map(|surface| SurfaceInspectionRow {
            surface,
            declared_signal_classes: DepthSignalClass::ALL.to_vec(),
            disabled_by_default_signal_classes: vec![
                DepthSignalClass::PerformanceDiagnostic,
                DepthSignalClass::FeatureUsageTelemetry,
            ],
            export_only_signal_classes: vec![DepthSignalClass::SupportExport],
            redaction_default_packet_class_refs: vec![
                "runtime_health_snapshot".to_owned(),
                "crash_manifest_reference_packet".to_owned(),
                "performance_summary_packet".to_owned(),
                "support_bundle_manifest_packet".to_owned(),
            ],
            retention_classes: vec![
                RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                RetentionClass::LocalSamplingWindow,
                RetentionClass::LocalManifestUntilSent,
            ],
            inspection_note: format!(
                "{} exposes crash, performance, usage, and support-export posture with local-only alternatives visible in diagnostics and CLI/headless output.",
                surface.label()
            ),
        })
        .collect()
}

fn seed_packet_class_manifest() -> Vec<PacketClassManifestRow> {
    vec![
        packet_class_row(
            "runtime_health_snapshot",
            "Runtime health snapshot",
            vec![
                DepthSignalClass::CrashDiagnostic,
                DepthSignalClass::FeatureUsageTelemetry,
            ],
            RedactionProfileClass::MetadataSafeDefault,
            true,
        ),
        packet_class_row(
            "crash_manifest_reference_packet",
            "Crash manifest reference packet",
            vec![DepthSignalClass::CrashDiagnostic],
            RedactionProfileClass::EnvironmentSummaryOnly,
            true,
        ),
        packet_class_row(
            "performance_summary_packet",
            "Performance summary packet",
            vec![DepthSignalClass::PerformanceDiagnostic],
            RedactionProfileClass::MetadataSafeDefault,
            true,
        ),
        packet_class_row(
            "support_bundle_manifest_packet",
            "Support bundle manifest packet",
            vec![DepthSignalClass::SupportExport],
            RedactionProfileClass::LocalOnlyReviewRequired,
            true,
        ),
    ]
}

fn packet_class_row(
    packet_class_id: &str,
    packet_class_label: &str,
    signal_classes: Vec<DepthSignalClass>,
    default_redaction_profile: RedactionProfileClass,
    local_only_by_default: bool,
) -> PacketClassManifestRow {
    PacketClassManifestRow {
        packet_class_id: packet_class_id.to_owned(),
        packet_class_label: packet_class_label.to_owned(),
        surfaces: DepthSurfaceClass::ALL.to_vec(),
        signal_classes,
        default_redaction_profile,
        local_only_by_default,
        explicit_review_required_before_share: true,
        raw_source_code_forbidden: true,
        filenames_and_paths_forbidden: true,
        prompt_bodies_forbidden: true,
        terminal_contents_forbidden: true,
        secrets_forbidden: true,
        clipboard_contents_forbidden: true,
        evidence_refs: vec![
            PACKET_CLASS_REGISTRY_REF.to_owned(),
            "docs/support/support_bundle_contract.md".to_owned(),
            ENDPOINT_POLICY_DOC_REF.to_owned(),
        ],
    }
}

fn crash_allowed_fields() -> Vec<String> {
    vec![
        "crash_id",
        "build_id",
        "fault_domain_id",
        "session_family",
        "policy_fingerprint",
        "enabled_extension_hash",
        "coarse_environment_summary",
        "symbolication_report_ref",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn performance_allowed_fields() -> Vec<String> {
    vec![
        "capture_id",
        "surface_class",
        "feature_path_id",
        "duration_bucket_ms",
        "memory_bucket_mb",
        "device_class",
        "sample_window",
        "trace_or_profile_ref",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn feature_usage_allowed_fields() -> Vec<String> {
    vec![
        "event_name",
        "surface_class",
        "workflow_class",
        "counter_value",
        "sampling_bucket",
        "build_flavor",
        "platform_class",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn support_export_allowed_fields() -> Vec<String> {
    vec![
        "bundle_id",
        "manifest_id",
        "included_class_counts",
        "excluded_class_counts",
        "redaction_profile",
        "destination_class",
        "case_ref_or_null",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
