//! Local crash-store viewer and export-safe crash-entry packet for M5 hosts.
//!
//! This module turns the crash-envelope, dump-manifest, exact-build, and
//! support-export contracts into one local-first crash-store surface. The
//! packet is intentionally metadata-safe: users can inspect build identity,
//! fault-domain attribution, preservation class, redaction posture, and the
//! actions available before any export or upload path is taken.

use serde::{Deserialize, Serialize};

use aureline_crash::SymbolicationState;

/// Stable record-kind tag carried on crash-store viewer packets.
pub const CRASH_STORE_VIEWER_PACKET_RECORD_KIND: &str = "crash_store_viewer_packet";

/// Stable record-kind tag carried on crash-store viewer rows.
pub const CRASH_STORE_VIEWER_ROW_RECORD_KIND: &str = "crash_store_viewer_row";

/// Frozen schema version shared by crash-store viewer records.
pub const CRASH_STORE_VIEWER_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema.
pub const CRASH_STORE_VIEWER_SCHEMA_REF: &str = "schemas/support/crash_store_viewer.schema.json";

/// Repository-relative path of the reviewer-facing help document.
pub const CRASH_STORE_VIEWER_DOC_REF: &str = "docs/support/m5/crash_store.md";

/// Repository-relative path of the checked review artifact.
pub const CRASH_STORE_VIEWER_ARTIFACT_REF: &str = "artifacts/support/m5/crash-store.md";

/// Repository-relative path of the protected fixture directory.
pub const CRASH_STORE_VIEWER_FIXTURE_DIR: &str = "fixtures/support/m5/crash_store";

const REQUIRED_HOST_FAMILY_IDS: &[&str] = &[
    "notebook_kernel_host",
    "preview_dev_server_host",
    "provider_run_session_host",
    "profiler_replay_session_host",
    "pipeline_viewer_host",
    "query_runtime_host",
    "data_api_connector_host",
    "docs_browser_bridge_host",
    "registry_database_connector_host",
    "infra_helper_job",
];

/// Closed preservation-class vocabulary shown in the crash-store viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashPreservationClass {
    /// Envelope, dump metadata, and local dump body are still available.
    EnvelopeAndDumpRetainedLocal,
    /// Envelope and symbolication metadata remain available; raw dump
    /// attachment still requires explicit opt-in.
    EnvelopeAndSymbolicationRetainedDumpOptInOnly,
    /// The raw dump body expired or was cleared, but envelope metadata remains
    /// locally inspectable and export-safe.
    EnvelopeOnlyDumpExpiredMetadataPreserved,
}

impl CrashPreservationClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvelopeAndDumpRetainedLocal => "envelope_and_dump_retained_local",
            Self::EnvelopeAndSymbolicationRetainedDumpOptInOnly => {
                "envelope_and_symbolication_retained_dump_opt_in_only"
            }
            Self::EnvelopeOnlyDumpExpiredMetadataPreserved => {
                "envelope_only_dump_expired_metadata_preserved"
            }
        }
    }
}

/// Closed redaction-posture vocabulary shown in the crash-store viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashStoreRedactionPostureClass {
    /// Metadata-safe crash metadata may export after local review.
    MetadataSafeDefault,
    /// Code-adjacent fields remain narrowed to operator-only review.
    OperatorOnlyRestricted,
    /// Review must remain local until the user explicitly broadens scope.
    LocalOnlyReviewRequired,
}

impl CrashStoreRedactionPostureClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::LocalOnlyReviewRequired => "local_only_review_required",
        }
    }
}

/// Closed action vocabulary surfaced by the local crash-store viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashStoreActionClass {
    /// Inspect crash metadata locally.
    InspectMetadataLocal,
    /// Inspect dump/core metadata locally.
    InspectDumpMetadataLocal,
    /// Preview the export-safe support packet locally.
    PreviewSupportExportLocal,
    /// Export the metadata-safe crash packet locally.
    ExportMetadataBundle,
    /// Attach the raw dump only after an explicit opt-in step.
    AttachRawDumpOptIn,
    /// Upload the reviewed packet only after local review.
    UploadReviewedPacket,
    /// Open restart lineage, quarantine, and recovery truth.
    OpenRestartLineage,
}

impl CrashStoreActionClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectMetadataLocal => "inspect_metadata_local",
            Self::InspectDumpMetadataLocal => "inspect_dump_metadata_local",
            Self::PreviewSupportExportLocal => "preview_support_export_local",
            Self::ExportMetadataBundle => "export_metadata_bundle",
            Self::AttachRawDumpOptIn => "attach_raw_dump_opt_in",
            Self::UploadReviewedPacket => "upload_reviewed_packet",
            Self::OpenRestartLineage => "open_restart_lineage",
        }
    }
}

/// One visible action row inside the local crash-store viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashStoreActionRow {
    /// Closed action token.
    pub action_class: CrashStoreActionClass,
    /// Reviewer-visible label.
    pub label: String,
    /// Whether the action is currently available.
    pub enabled: bool,
    /// Whether the action requires the user to remain in a local review step
    /// before it may continue.
    pub requires_local_review: bool,
    /// Whether the action requires an explicit raw-dump opt-in.
    pub raw_dump_opt_in_required: bool,
    /// Whether this action would initiate network egress.
    pub network_egress: bool,
    /// Redaction-safe reviewer note.
    pub notes: String,
}

/// One crash row shown by the local crash-store viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashStoreViewerRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable crash-store entry id.
    pub crash_store_entry_id: String,
    /// Stable crash id carried by the envelope and dump manifest.
    pub crash_id: String,
    /// Stable host-family id.
    pub host_family_id: String,
    /// Human-readable host-family label.
    pub host_family_label: String,
    /// Stable session-type id.
    pub session_type_id: String,
    /// Stable fault-domain id.
    pub fault_domain_id: String,
    /// Stable crash-envelope ref.
    pub crash_envelope_ref: String,
    /// Stable dump/core ref.
    pub crash_dump_ref: String,
    /// Optional symbolication report ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbolication_report_ref: Option<String>,
    /// Optional support-bundle manifest ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_bundle_manifest_ref: Option<String>,
    /// Optional local preview snapshot ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_preview_snapshot_ref: Option<String>,
    /// Stable restart-lineage or quarantine packet ref.
    pub restart_lineage_ref: String,
    /// Stable exact-build identity ref.
    pub primary_exact_build_identity_ref: String,
    /// Stable build id copied into the viewer row.
    pub build_id: String,
    /// Release channel class such as `preview` or `stable`.
    pub release_channel_class: String,
    /// Trace ids overlapping the bounded crash window.
    pub trace_ids: Vec<String>,
    /// Stable extension-set or host-set hash.
    pub extension_or_host_set_hash: String,
    /// Stable policy fingerprint.
    pub policy_fingerprint: String,
    /// Sandbox profile active at crash time.
    pub sandbox_profile: String,
    /// RFC 3339 UTC start of the bounded crash window.
    pub crash_window_started_at: String,
    /// RFC 3339 UTC end of the bounded crash window.
    pub crash_window_ended_at: String,
    /// Architecture copied from the dump/core manifest.
    pub architecture: String,
    /// Signal, exception, or panic class copied from the dump/core manifest.
    pub signal_or_exception_class: String,
    /// Dump format class such as `minidump`.
    pub dump_format_class: String,
    /// Stable dump-format identity.
    pub dump_format_identity: String,
    /// Stable module ids observed in the dump metadata.
    pub module_ids: Vec<String>,
    /// Build ids or exact-build identities retained for the modules.
    pub module_build_ids: Vec<String>,
    /// Overall symbolication state for the crash.
    pub symbolication_state: SymbolicationState,
    /// Local preservation class shown to the user.
    pub preservation_class: CrashPreservationClass,
    /// Redaction posture shown to the user.
    pub redaction_posture: CrashStoreRedactionPostureClass,
    /// Reviewable support-export redaction profile ref.
    pub support_export_review_ref: String,
    /// Optional reviewed upload target ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_target_ref: Option<String>,
    /// True when the crash row is local-first by default.
    pub local_first_by_default: bool,
    /// False by construction; raw dump bytes do not leave the device silently.
    pub raw_dump_exported: bool,
    /// Visible actions the crash-store viewer offers.
    pub available_actions: Vec<CrashStoreActionRow>,
    /// Redaction-safe reviewer note.
    pub notes: String,
}

/// One validation error emitted by [`CrashStoreViewerPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashStoreViewerViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical local crash-store viewer packet for M5 host families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashStoreViewerPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing help doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing contracts this packet composes.
    pub supporting_contract_refs: Vec<String>,
    /// Visible crash-store rows.
    pub rows: Vec<CrashStoreViewerRow>,
    /// Metadata-safe summary rendered by support/export surfaces.
    pub export_safe_summary: String,
}

impl CrashStoreViewerPacket {
    /// Validates the crash-store packet and its local-first review posture.
    pub fn validate(&self) -> Vec<CrashStoreViewerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CRASH_STORE_VIEWER_PACKET_RECORD_KIND {
            violations.push(CrashStoreViewerViolation {
                path: "record_kind".to_owned(),
                message: "unexpected record_kind".to_owned(),
            });
        }
        if self.schema_version != CRASH_STORE_VIEWER_SCHEMA_VERSION {
            violations.push(CrashStoreViewerViolation {
                path: "schema_version".to_owned(),
                message: "unexpected schema_version".to_owned(),
            });
        }
        if self.doc_ref != CRASH_STORE_VIEWER_DOC_REF {
            violations.push(CrashStoreViewerViolation {
                path: "doc_ref".to_owned(),
                message: "packet must quote the canonical reviewer doc".to_owned(),
            });
        }
        if self.schema_ref != CRASH_STORE_VIEWER_SCHEMA_REF {
            violations.push(CrashStoreViewerViolation {
                path: "schema_ref".to_owned(),
                message: "packet must quote the canonical schema ref".to_owned(),
            });
        }
        if self.artifact_ref != CRASH_STORE_VIEWER_ARTIFACT_REF {
            violations.push(CrashStoreViewerViolation {
                path: "artifact_ref".to_owned(),
                message: "packet must quote the checked crash-store artifact ref".to_owned(),
            });
        }

        for required in REQUIRED_HOST_FAMILY_IDS {
            if !self.rows.iter().any(|row| row.host_family_id == *required) {
                violations.push(CrashStoreViewerViolation {
                    path: "rows".to_owned(),
                    message: format!("missing required crash-store host family {required}"),
                });
            }
        }

        for row in &self.rows {
            let path = format!("rows.{}", row.crash_store_entry_id);
            if row.record_kind != CRASH_STORE_VIEWER_ROW_RECORD_KIND {
                violations.push(CrashStoreViewerViolation {
                    path: path.clone(),
                    message: "row must quote the canonical row record kind".to_owned(),
                });
            }
            for (field, value) in [
                ("crash_id", row.crash_id.as_str()),
                ("host_family_id", row.host_family_id.as_str()),
                ("session_type_id", row.session_type_id.as_str()),
                ("fault_domain_id", row.fault_domain_id.as_str()),
                ("crash_envelope_ref", row.crash_envelope_ref.as_str()),
                ("crash_dump_ref", row.crash_dump_ref.as_str()),
                (
                    "primary_exact_build_identity_ref",
                    row.primary_exact_build_identity_ref.as_str(),
                ),
                ("build_id", row.build_id.as_str()),
                ("release_channel_class", row.release_channel_class.as_str()),
                (
                    "extension_or_host_set_hash",
                    row.extension_or_host_set_hash.as_str(),
                ),
                ("policy_fingerprint", row.policy_fingerprint.as_str()),
                ("sandbox_profile", row.sandbox_profile.as_str()),
                (
                    "crash_window_started_at",
                    row.crash_window_started_at.as_str(),
                ),
                ("crash_window_ended_at", row.crash_window_ended_at.as_str()),
                ("architecture", row.architecture.as_str()),
                (
                    "signal_or_exception_class",
                    row.signal_or_exception_class.as_str(),
                ),
                ("dump_format_class", row.dump_format_class.as_str()),
                ("dump_format_identity", row.dump_format_identity.as_str()),
                (
                    "support_export_review_ref",
                    row.support_export_review_ref.as_str(),
                ),
            ] {
                if value.trim().is_empty() {
                    violations.push(CrashStoreViewerViolation {
                        path: format!("{path}.{field}"),
                        message: "field must be non-empty".to_owned(),
                    });
                }
            }
            if row.trace_ids.is_empty() {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.trace_ids"),
                    message: "crash-store row must preserve trace ids".to_owned(),
                });
            }
            if row.module_ids.is_empty() || row.module_build_ids.is_empty() {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.module_ids"),
                    message: "crash-store row must preserve module ids and build ids".to_owned(),
                });
            }
            if row.module_ids.len() != row.module_build_ids.len() {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.module_build_ids"),
                    message: "module ids and module build ids must align".to_owned(),
                });
            }
            if !row.local_first_by_default {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.local_first_by_default"),
                    message: "crash-store rows must remain local-first".to_owned(),
                });
            }
            if row.raw_dump_exported {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.raw_dump_exported"),
                    message: "raw dump bytes may not leave the device silently".to_owned(),
                });
            }
            if row.available_actions.is_empty() {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.available_actions"),
                    message: "crash-store row must list visible actions".to_owned(),
                });
            }

            let has_preview = row.available_actions.iter().any(|action| {
                action.enabled
                    && action.action_class == CrashStoreActionClass::PreviewSupportExportLocal
                    && !action.network_egress
            });
            let has_export = row.available_actions.iter().any(|action| {
                action.enabled
                    && action.action_class == CrashStoreActionClass::ExportMetadataBundle
                    && !action.network_egress
                    && action.requires_local_review
            });
            if !has_preview || !has_export {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.available_actions"),
                    message:
                        "crash-store row must expose local preview and local export before upload"
                            .to_owned(),
                });
            }

            let upload_action = row
                .available_actions
                .iter()
                .find(|action| action.action_class == CrashStoreActionClass::UploadReviewedPacket);
            if let Some(upload) = upload_action {
                if upload.enabled && (!upload.requires_local_review || !upload.network_egress) {
                    violations.push(CrashStoreViewerViolation {
                        path: format!("{path}.available_actions.upload_reviewed_packet"),
                        message:
                            "reviewed upload must remain behind local review and explicit egress"
                                .to_owned(),
                    });
                }
                if upload.enabled && row.upload_target_ref.is_none() {
                    violations.push(CrashStoreViewerViolation {
                        path: format!("{path}.upload_target_ref"),
                        message: "enabled upload action must name an upload target ref".to_owned(),
                    });
                }
            }

            let dump_attach_enabled = row.available_actions.iter().any(|action| {
                action.action_class == CrashStoreActionClass::AttachRawDumpOptIn && action.enabled
            });
            if matches!(
                row.preservation_class,
                CrashPreservationClass::EnvelopeOnlyDumpExpiredMetadataPreserved
            ) && dump_attach_enabled
            {
                violations.push(CrashStoreViewerViolation {
                    path: format!("{path}.available_actions.attach_raw_dump_opt_in"),
                    message: "expired dumps may not offer raw-dump attachment".to_owned(),
                });
            }
        }

        violations
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        !self.rows.iter().any(|row| row.raw_dump_exported)
            && self.export_safe_summary.contains("metadata-safe")
    }

    /// Renders a short plaintext summary for review surfaces.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::from("Crash store viewer packet\n");
        out.push_str(&format!("packet_id: {}\n", self.packet_id));
        out.push_str("host_families:");
        for row in &self.rows {
            out.push_str(&format!(" {}", row.host_family_id));
        }
        out.push('\n');
        out.push_str("build_ids:");
        for row in &self.rows {
            out.push_str(&format!(" {}", row.build_id));
        }
        out.push('\n');
        out
    }
}

/// Returns the seeded crash-store viewer packet used by tests and docs.
pub fn seeded_crash_store_viewer_packet() -> CrashStoreViewerPacket {
    CrashStoreViewerPacket {
        record_kind: CRASH_STORE_VIEWER_PACKET_RECORD_KIND.to_owned(),
        schema_version: CRASH_STORE_VIEWER_SCHEMA_VERSION,
        packet_id: "support.m5.crash_store_viewer.v1".to_owned(),
        generated_at: "2026-06-12T23:50:00Z".to_owned(),
        doc_ref: CRASH_STORE_VIEWER_DOC_REF.to_owned(),
        schema_ref: CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
        artifact_ref: CRASH_STORE_VIEWER_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".plans/M05-244.md".to_owned(),
            ".t2/docs/Aureline_PRD.md#541-debug-artifact-symbol-and-source-map-architecture"
                .to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#7127-crash-capture-minidump-symbolication-and-symbol-service".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#appendix-ai--crash-capture-symbolication-and-field-readiness-matrix".to_owned(),
        ],
        supporting_contract_refs: vec![
            "schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json"
                .to_owned(),
            "schemas/support/support_bundle_manifest.schema.json".to_owned(),
            "schemas/support/export_redaction_profile.schema.json".to_owned(),
            "schemas/support/m5-fault-crash-governance.schema.json".to_owned(),
        ],
        rows: vec![
            seed_row(
                "notebook_kernel_host",
                "Notebook kernel session",
                "notebook_kernel_session",
                "fd.notebook.kernel.001",
                "crash:notebook-kernel:0001",
                "preview",
                "build-id:aureline:preview:0.9.0:x86_64-apple-darwin:release:0011aa22",
                "trace:notebook:run:0001",
                "trace:notebook:kernel:0001",
                "hostset:notebook-kernel:sha256:0011223344556677",
                "policy:notebook-kernel:sha256:aa550011",
                "sandbox_profile.notebook_kernel_local",
                "arm64-apple-darwin",
                "signal.sigabrt.kernel_protocol_fault",
                "minidump",
                "minidump.breakpad.v1",
                SymbolicationState::Exact,
                CrashPreservationClass::EnvelopeAndDumpRetainedLocal,
                CrashStoreRedactionPostureClass::LocalOnlyReviewRequired,
                true,
            ),
            seed_row(
                "data_api_connector_host",
                "Data/API connector and query runtime",
                "data_api_connector_session",
                "fd.data.api.connector.001",
                "crash:data-api-connector:0001",
                "preview",
                "build-id:aureline:preview:0.9.0:x86_64-unknown-linux-gnu:release:2211bb33",
                "trace:data-api:request:0001",
                "trace:data-api:auth:0001",
                "hostset:data-api:sha256:8899aabbccddeeff",
                "policy:data-api:sha256:bb660022",
                "sandbox_profile.connector_remote_narrow",
                "x86_64-unknown-linux-gnu",
                "exception.route_scope_revoked",
                "minidump",
                "minidump.breakpad.v1",
                SymbolicationState::Partial,
                CrashPreservationClass::EnvelopeAndSymbolicationRetainedDumpOptInOnly,
                CrashStoreRedactionPostureClass::OperatorOnlyRestricted,
                false,
            ),
            seed_row(
                "preview_dev_server_host",
                "Preview dev server",
                "preview_dev_server_session",
                "fd.preview.dev-server.001",
                "crash:preview-dev-server:0001",
                "preview",
                "build-id:aureline:preview:0.9.0:x86_64-unknown-linux-gnu:release:3311cc44",
                "trace:preview:session:0001",
                "trace:preview:route:0001",
                "hostset:preview-server:sha256:7766554433221100",
                "policy:preview-server:sha256:cc770033",
                "sandbox_profile.preview_server_local",
                "x86_64-unknown-linux-gnu",
                "signal.sigterm.port_bind_loop",
                "minidump",
                "minidump.breakpad.v1",
                SymbolicationState::Exact,
                CrashPreservationClass::EnvelopeAndDumpRetainedLocal,
                CrashStoreRedactionPostureClass::LocalOnlyReviewRequired,
                true,
            ),
            seed_row(
                "provider_run_session_host",
                "Provider-backed run session",
                "provider_run_session",
                "fd.provider.run-session.001",
                "crash:provider-run-session:0001",
                "beta",
                "build-id:aureline:beta:0.9.0:x86_64-unknown-linux-gnu:release:4411dd55",
                "trace:provider:run:0001",
                "trace:provider:ticket:0001",
                "hostset:provider-run:sha256:aa33bb44cc55dd66",
                "policy:provider-run:sha256:dd880044",
                "sandbox_profile.provider_run_external_mutation",
                "x86_64-unknown-linux-gnu",
                "exception.ticket_scope_revoked",
                "core",
                "elf.core.v1",
                SymbolicationState::Exact,
                CrashPreservationClass::EnvelopeAndSymbolicationRetainedDumpOptInOnly,
                CrashStoreRedactionPostureClass::OperatorOnlyRestricted,
                true,
            ),
            seed_row(
                "profiler_replay_session_host",
                "Profiler and replay session",
                "profiler_replay_session",
                "fd.profiler.replay.001",
                "crash:profiler-replay:0001",
                "beta",
                "build-id:aureline:beta:0.9.0:x86_64-unknown-linux-gnu:release:5511ee66",
                "trace:profiler:replay:0001",
                "trace:profiler:map:0001",
                "hostset:profiler-replay:sha256:8899cc00dd11ee22",
                "policy:profiler-replay:sha256:ee990055",
                "sandbox_profile.profiler_replay_local",
                "x86_64-unknown-linux-gnu",
                "signal.sigbus.trace_mapping_fault",
                "minidump",
                "minidump.breakpad.v1",
                SymbolicationState::Partial,
                CrashPreservationClass::EnvelopeAndSymbolicationRetainedDumpOptInOnly,
                CrashStoreRedactionPostureClass::OperatorOnlyRestricted,
                true,
            ),
            seed_row(
                "pipeline_viewer_host",
                "Pipeline viewer session",
                "pipeline_viewer_session",
                "fd.pipeline.viewer.001",
                "crash:pipeline-viewer:0001",
                "preview",
                "build-id:aureline:preview:0.9.0:x86_64-unknown-linux-gnu:release:6611ff77",
                "trace:pipeline:event-stream:0001",
                "trace:pipeline:viewer:0001",
                "hostset:pipeline-viewer:sha256:0123ab45cd67ef89",
                "policy:pipeline-viewer:sha256:ffaa0066",
                "sandbox_profile.pipeline_viewer_remote",
                "x86_64-unknown-linux-gnu",
                "exception.capability_manifest_drift",
                "minidump",
                "minidump.breakpad.v1",
                SymbolicationState::Missing,
                CrashPreservationClass::EnvelopeAndSymbolicationRetainedDumpOptInOnly,
                CrashStoreRedactionPostureClass::LocalOnlyReviewRequired,
                false,
            ),
            seed_row(
                "query_runtime_host",
                "Query/request runtime",
                "query_runtime_session",
                "fd.query.runtime.001",
                "crash:query-runtime:0001",
                "preview",
                "build-id:aureline:preview:0.9.0:x86_64-unknown-linux-gnu:release:77110088",
                "trace:query:runtime:0001",
                "trace:query:result:0001",
                "hostset:query-runtime:sha256:10ab32cd54ef76ab",
                "policy:query-runtime:sha256:abbb0077",
                "sandbox_profile.query_runtime_local",
                "x86_64-unknown-linux-gnu",
                "signal.sigsegv.request_executor_fault",
                "core",
                "elf.core.v1",
                SymbolicationState::Exact,
                CrashPreservationClass::EnvelopeAndDumpRetainedLocal,
                CrashStoreRedactionPostureClass::LocalOnlyReviewRequired,
                false,
            ),
            seed_row(
                "docs_browser_bridge_host",
                "Docs and browser bridge",
                "docs_browser_bridge_session",
                "fd.docs.browser.bridge.001",
                "crash:docs-browser-bridge:0001",
                "preview",
                "build-id:aureline:preview:0.9.0:aarch64-apple-darwin:release:88110099",
                "trace:docs-bridge:session:0001",
                "trace:docs-bridge:origin:0001",
                "hostset:docs-browser-bridge:sha256:cc2211dd4433ee55",
                "policy:docs-browser-bridge:sha256:bcdd0088",
                "sandbox_profile.docs_browser_bridge_remote",
                "arm64-apple-darwin",
                "exception.target_mismatch",
                "minidump",
                "minidump.breakpad.v1",
                SymbolicationState::Partial,
                CrashPreservationClass::EnvelopeAndSymbolicationRetainedDumpOptInOnly,
                CrashStoreRedactionPostureClass::OperatorOnlyRestricted,
                false,
            ),
            seed_row(
                "registry_database_connector_host",
                "Registry or database connector",
                "registry_database_connector_session",
                "fd.registry.database.connector.001",
                "crash:registry-database-connector:0001",
                "beta",
                "build-id:aureline:beta:0.9.0:x86_64-unknown-linux-gnu:release:991100aa",
                "trace:registry-db:session:0001",
                "trace:registry-db:target:0001",
                "hostset:registry-db-connector:sha256:9988776655443322",
                "policy:registry-db-connector:sha256:cdee0099",
                "sandbox_profile.registry_database_connector",
                "x86_64-unknown-linux-gnu",
                "exception.credential_scope_revoked",
                "core",
                "elf.core.v1",
                SymbolicationState::Exact,
                CrashPreservationClass::EnvelopeAndSymbolicationRetainedDumpOptInOnly,
                CrashStoreRedactionPostureClass::OperatorOnlyRestricted,
                true,
            ),
            seed_row(
                "infra_helper_job",
                "Infrastructure helper",
                "infra_helper_job_session",
                "fd.infra.helper.job.001",
                "crash:infra-helper-job:0001",
                "stable",
                "build-id:aureline:stable:0.9.0:x86_64-unknown-linux-gnu:release:aa1100bb",
                "trace:infra-helper:job:0001",
                "trace:infra-helper:verify:0001",
                "hostset:infra-helper:sha256:1234567890abcdef",
                "policy:infra-helper:sha256:def000aa",
                "sandbox_profile.infra_helper_restricted",
                "x86_64-unknown-linux-gnu",
                "exception.signature_failure",
                "minidump",
                "minidump.breakpad.v1",
                SymbolicationState::Missing,
                CrashPreservationClass::EnvelopeAndDumpRetainedLocal,
                CrashStoreRedactionPostureClass::MetadataSafeDefault,
                false,
            ),
        ],
        export_safe_summary: "Crash-store viewers remain metadata-safe and local-first; preview and export happen on-device before any reviewed upload path may proceed.".to_owned(),
    }
}

/// Returns a fixture variant where a dump expired but metadata remains visible.
pub fn seeded_expired_dump_crash_store_viewer_packet() -> CrashStoreViewerPacket {
    let mut packet = seeded_crash_store_viewer_packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|row| row.host_family_id == "query_runtime_host")
    {
        row.preservation_class = CrashPreservationClass::EnvelopeOnlyDumpExpiredMetadataPreserved;
        row.redaction_posture = CrashStoreRedactionPostureClass::MetadataSafeDefault;
        row.upload_target_ref = None;
        for action in &mut row.available_actions {
            if action.action_class == CrashStoreActionClass::AttachRawDumpOptIn
                || action.action_class == CrashStoreActionClass::UploadReviewedPacket
            {
                action.enabled = false;
                action.notes =
                    "Dump body expired or was cleared locally; metadata review remains available."
                        .to_owned();
            }
        }
        row.notes =
            "Raw dump body expired under bounded local retention; crash metadata, build identity, and restart lineage remain locally inspectable."
                .to_owned();
    }
    packet.packet_id = "support.m5.crash_store_viewer.expired_dump".to_owned();
    packet
}

fn seed_row(
    host_family_id: &str,
    host_family_label: &str,
    session_type_id: &str,
    fault_domain_id: &str,
    crash_id: &str,
    release_channel_class: &str,
    build_id: &str,
    trace_id_one: &str,
    trace_id_two: &str,
    extension_or_host_set_hash: &str,
    policy_fingerprint: &str,
    sandbox_profile: &str,
    architecture: &str,
    signal_or_exception_class: &str,
    dump_format_class: &str,
    dump_format_identity: &str,
    symbolication_state: SymbolicationState,
    preservation_class: CrashPreservationClass,
    redaction_posture: CrashStoreRedactionPostureClass,
    upload_ready: bool,
) -> CrashStoreViewerRow {
    let crash_store_entry_id = format!("crash-store-entry:{host_family_id}:{crash_id}");
    let crash_envelope_ref = format!("crash-envelope:{host_family_id}:{crash_id}");
    let crash_dump_ref = format!("crash-dump:{host_family_id}:{crash_id}");
    let symbolication_report_ref =
        Some(format!("symbolication-report:{host_family_id}:{crash_id}"));
    let support_bundle_manifest_ref = Some(format!(
        "support.bundle.manifest:{host_family_id}:{crash_id}:local-review"
    ));
    let support_preview_snapshot_ref = Some(format!(
        "preview-snapshot:support-bundle:{host_family_id}:{crash_id}"
    ));
    let restart_lineage_ref = format!("restart-lineage:{host_family_id}:{crash_id}");
    let module_ids = vec![
        format!("{host_family_id}.host"),
        format!("{host_family_id}.renderer_or_worker"),
    ];
    let module_build_ids = vec![build_id.to_owned(), format!("{build_id}:source-map")];
    let raw_dump_available = !matches!(
        preservation_class,
        CrashPreservationClass::EnvelopeOnlyDumpExpiredMetadataPreserved
    );

    CrashStoreViewerRow {
        record_kind: CRASH_STORE_VIEWER_ROW_RECORD_KIND.to_owned(),
        crash_store_entry_id,
        crash_id: crash_id.to_owned(),
        host_family_id: host_family_id.to_owned(),
        host_family_label: host_family_label.to_owned(),
        session_type_id: session_type_id.to_owned(),
        fault_domain_id: fault_domain_id.to_owned(),
        crash_envelope_ref,
        crash_dump_ref,
        symbolication_report_ref,
        support_bundle_manifest_ref,
        support_preview_snapshot_ref,
        restart_lineage_ref,
        primary_exact_build_identity_ref: build_id.to_owned(),
        build_id: build_id.to_owned(),
        release_channel_class: release_channel_class.to_owned(),
        trace_ids: vec![trace_id_one.to_owned(), trace_id_two.to_owned()],
        extension_or_host_set_hash: extension_or_host_set_hash.to_owned(),
        policy_fingerprint: policy_fingerprint.to_owned(),
        sandbox_profile: sandbox_profile.to_owned(),
        crash_window_started_at: "2026-06-12T23:39:12Z".to_owned(),
        crash_window_ended_at: "2026-06-12T23:39:29Z".to_owned(),
        architecture: architecture.to_owned(),
        signal_or_exception_class: signal_or_exception_class.to_owned(),
        dump_format_class: dump_format_class.to_owned(),
        dump_format_identity: dump_format_identity.to_owned(),
        module_ids,
        module_build_ids,
        symbolication_state,
        preservation_class,
        redaction_posture,
        support_export_review_ref:
            "fixtures/support/m3/redaction_and_escalation/default_redacted_profile.yaml"
                .to_owned(),
        upload_target_ref: upload_ready.then(|| format!("upload-target:{host_family_id}:reviewed")),
        local_first_by_default: true,
        raw_dump_exported: false,
        available_actions: seed_actions(raw_dump_available, upload_ready),
        notes: format!(
            "{host_family_label} crash remains attributable to exact build, fault domain, session type, trace ids, and local crash-store actions before any export."
        ),
    }
}

fn seed_actions(raw_dump_available: bool, upload_ready: bool) -> Vec<CrashStoreActionRow> {
    vec![
        CrashStoreActionRow {
            action_class: CrashStoreActionClass::InspectMetadataLocal,
            label: "Inspect metadata".to_owned(),
            enabled: true,
            requires_local_review: false,
            raw_dump_opt_in_required: false,
            network_egress: false,
            notes: "Opens the local crash-store metadata sheet only.".to_owned(),
        },
        CrashStoreActionRow {
            action_class: CrashStoreActionClass::InspectDumpMetadataLocal,
            label: "Inspect dump metadata".to_owned(),
            enabled: true,
            requires_local_review: false,
            raw_dump_opt_in_required: false,
            network_egress: false,
            notes: "Shows dump format, architecture, signal class, and module identities without opening raw bytes.".to_owned(),
        },
        CrashStoreActionRow {
            action_class: CrashStoreActionClass::PreviewSupportExportLocal,
            label: "Preview support export".to_owned(),
            enabled: true,
            requires_local_review: false,
            raw_dump_opt_in_required: false,
            network_egress: false,
            notes: "Renders the local metadata-safe preview before any export or upload.".to_owned(),
        },
        CrashStoreActionRow {
            action_class: CrashStoreActionClass::ExportMetadataBundle,
            label: "Export metadata bundle".to_owned(),
            enabled: true,
            requires_local_review: true,
            raw_dump_opt_in_required: false,
            network_egress: false,
            notes: "Writes the metadata-safe crash packet locally after review.".to_owned(),
        },
        CrashStoreActionRow {
            action_class: CrashStoreActionClass::AttachRawDumpOptIn,
            label: "Attach raw dump".to_owned(),
            enabled: raw_dump_available,
            requires_local_review: true,
            raw_dump_opt_in_required: true,
            network_egress: false,
            notes: if raw_dump_available {
                "Raw dump stays local until the user explicitly opts in from the reviewed sheet."
                    .to_owned()
            } else {
                "Raw dump body is no longer locally attachable in this row.".to_owned()
            },
        },
        CrashStoreActionRow {
            action_class: CrashStoreActionClass::UploadReviewedPacket,
            label: "Upload reviewed packet".to_owned(),
            enabled: upload_ready,
            requires_local_review: true,
            raw_dump_opt_in_required: raw_dump_available,
            network_egress: true,
            notes: if upload_ready {
                "Upload remains disabled until the local review sheet confirms the exact export scope.".to_owned()
            } else {
                "No reviewed upload target is configured for this crash row.".to_owned()
            },
        },
        CrashStoreActionRow {
            action_class: CrashStoreActionClass::OpenRestartLineage,
            label: "Open restart lineage".to_owned(),
            enabled: true,
            requires_local_review: false,
            raw_dump_opt_in_required: false,
            network_egress: false,
            notes: "Shows restart-budget, quarantine, and recovery lineage without rerunning the crashed session.".to_owned(),
        },
    ]
}
