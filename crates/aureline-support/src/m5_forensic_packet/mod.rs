//! M5 support-side forensic packet for host-failure drills and exports.
//!
//! This module projects the runtime forensic packet, crash-store packet, and
//! support-export rules into one metadata-safe packet for M5 host families.
//! The packet keeps exact-build identity, checkpoint lineage, artifact
//! locality, redaction posture, and reviewed share actions explicit so
//! drill/export flows can distinguish local-only, imported, mirrored, and
//! uploaded states without silently broadening retention or egress scope.

use serde::{Deserialize, Serialize};

use crate::m5_fault_crash_governance::{FaultDomainClass, RedactionProfileClass, RetentionClass};

/// Stable record-kind tag carried by the M5 forensic packet.
pub const M5_FORENSIC_PACKET_RECORD_KIND: &str = "m5_forensic_packet";

/// Frozen schema version for the M5 forensic packet.
pub const M5_FORENSIC_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the M5 forensic packet schema.
pub const M5_FORENSIC_PACKET_SCHEMA_REF: &str = "schemas/support/m5-forensic-packet.schema.json";

/// Repository-relative path of the reviewer-facing help document.
pub const M5_FORENSIC_PACKET_DOC_REF: &str = "docs/help/support/m5-host-failure-drills.md";

/// Repository-relative path of the checked review artifact.
pub const M5_FORENSIC_PACKET_ARTIFACT_REF: &str = "artifacts/support/m5/host-failure-drills.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_FORENSIC_PACKET_FIXTURE_DIR: &str = "fixtures/support/m5/forensic_packets";

const REQUIRED_HOST_FAMILY_IDS: &[&str] = &[
    "notebook_kernel_host",
    "data_api_connector_host",
    "preview_dev_server_host",
    "provider_run_session_host",
    "profiler_replay_session_host",
    "pipeline_viewer_host",
    "query_runtime_host",
    "docs_browser_bridge_host",
    "registry_database_connector_host",
    "infra_helper_job",
];

/// Closed trigger vocabulary for support-side M5 forensics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForensicTriggerClass {
    /// A crash packet tied to an abnormal exit.
    Crash,
    /// A stall packet tied to missed progress or heartbeat.
    StallDetected,
    /// A restart budget was consumed and the host was quarantined or disabled.
    RestartBudgetExhausted,
    /// Connector/session metadata drift invalidated the current host.
    ConnectorDrift,
    /// A provider or tool breaker opened and narrowed the lane.
    CircuitBreakerOpen,
    /// Target or host identity no longer matched the admitted session.
    HostMismatch,
    /// A reviewed recovery transition closed the drill.
    RecoveryTransition,
}

impl ForensicTriggerClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Crash => "crash",
            Self::StallDetected => "stall_detected",
            Self::RestartBudgetExhausted => "restart_budget_exhausted",
            Self::ConnectorDrift => "connector_drift",
            Self::CircuitBreakerOpen => "circuit_breaker_open",
            Self::HostMismatch => "host_mismatch",
            Self::RecoveryTransition => "recovery_transition",
        }
    }
}

/// Closed artifact vocabulary carried by M5 forensic rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForensicArtifactClass {
    /// Runtime-owned bounded forensic packet.
    RuntimeForensicPacket,
    /// Metadata-safe crash envelope.
    CrashEnvelope,
    /// Raw dump metadata or bounded dump manifest.
    DumpManifest,
    /// Exact-build symbol or source-map manifest.
    SymbolManifest,
    /// Local or mirrored symbolication report.
    SymbolicationReport,
    /// Support-bundle manifest or local preview snapshot.
    SupportBundleManifest,
    /// Reviewed upload receipt or managed handoff receipt.
    UploadReceipt,
}

impl ForensicArtifactClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeForensicPacket => "runtime_forensic_packet",
            Self::CrashEnvelope => "crash_envelope",
            Self::DumpManifest => "dump_manifest",
            Self::SymbolManifest => "symbol_manifest",
            Self::SymbolicationReport => "symbolication_report",
            Self::SupportBundleManifest => "support_bundle_manifest",
            Self::UploadReceipt => "upload_receipt",
        }
    }
}

/// Closed locality/state vocabulary for forensic artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForensicArtifactStateClass {
    /// Artifact exists only on the local device or workspace store.
    LocalOnly,
    /// Artifact was imported from another bundle or replay source.
    Imported,
    /// Artifact is mirrored to a declared mirror or symbol service.
    Mirrored,
    /// Artifact was uploaded or attached through an explicit reviewed step.
    Uploaded,
}

impl ForensicArtifactStateClass {
    /// All artifact states in canonical order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::Imported,
        Self::Mirrored,
        Self::Uploaded,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Imported => "imported",
            Self::Mirrored => "mirrored",
            Self::Uploaded => "uploaded",
        }
    }
}

/// Closed reviewed-share destinations surfaced by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForensicShareDestinationClass {
    /// Review locally without egress.
    LocalPreview,
    /// Save or export locally without network egress.
    LocalExport,
    /// Use a declared mirror path instead of a vendor endpoint.
    MirrorCopy,
    /// Attach or upload through an explicit reviewed path.
    ManagedUpload,
    /// Import a previously exported packet or support handoff.
    ImportedHandoff,
}

impl ForensicShareDestinationClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPreview => "local_preview",
            Self::LocalExport => "local_export",
            Self::MirrorCopy => "mirror_copy",
            Self::ManagedUpload => "managed_upload",
            Self::ImportedHandoff => "imported_handoff",
        }
    }
}

/// One artifact-state row carried by an M5 forensic row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForensicArtifactStateRow {
    /// Closed artifact-class token.
    pub artifact_class: ForensicArtifactClass,
    /// Stable artifact reference.
    pub artifact_ref: String,
    /// Closed locality/state token.
    pub state_class: ForensicArtifactStateClass,
    /// Retention class disclosed for this artifact.
    pub retention_class: RetentionClass,
    /// Default redaction posture for this artifact.
    pub redaction_profile: RedactionProfileClass,
    /// True when broader egress requires explicit user or policy action.
    pub explicit_user_or_policy_action_required: bool,
    /// True when scope widening is blocked without review.
    pub scope_widening_without_review_forbidden: bool,
    /// Reviewable summary safe for support export.
    pub summary: String,
}

/// One reviewed share action surfaced by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForensicShareActionRow {
    /// Stable action id.
    pub action_id: String,
    /// Human-readable action label.
    pub label: String,
    /// Destination class for the action.
    pub destination_class: ForensicShareDestinationClass,
    /// Whether the action would cause network egress.
    pub network_egress: bool,
    /// True when the action remains gated on explicit user or policy action.
    pub explicit_user_or_policy_action_required: bool,
    /// True when the action is available on the seeded drill row.
    pub enabled: bool,
    /// True when the action would change retention or export scope.
    pub retention_or_scope_change_requires_review: bool,
    /// Reviewable summary safe for support export.
    pub note: String,
}

/// One forensic row tied to an M5 host-failure scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ForensicRow {
    /// Stable forensic packet id for the row.
    pub forensic_packet_id: String,
    /// Stable scenario id used by the drill corpus.
    pub scenario_id: String,
    /// Stable host-family id.
    pub host_family_id: String,
    /// Human-readable host-family label.
    pub host_family_label: String,
    /// Governing fault-domain token.
    pub fault_domain_class: FaultDomainClass,
    /// Trigger that produced the forensic row.
    pub trigger_class: ForensicTriggerClass,
    /// Stable runtime forensic packet reference.
    pub runtime_forensic_packet_ref: String,
    /// Stable exact-build identity reference.
    pub exact_build_identity_ref: String,
    /// Stable restart-lineage or quarantine reference.
    pub restart_lineage_ref: String,
    /// Stable checkpoint or clean-state reference.
    pub checkpoint_ref: String,
    /// Reviewable redaction profile for the row.
    pub redaction_profile: RedactionProfileClass,
    /// Reviewable retention class for the row.
    pub retention_class: RetentionClass,
    /// Artifact-state rows the forensic packet exposes.
    pub artifact_states: Vec<ForensicArtifactStateRow>,
    /// Reviewed share actions surfaced for the row.
    pub share_actions: Vec<ForensicShareActionRow>,
    /// Explicit guard assertions preventing silent upload or scope widening.
    pub no_silent_upload_guards: Vec<String>,
    /// Reviewable export-safe summary.
    pub export_safe_summary: String,
}

/// One validation error emitted by [`M5ForensicPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ForensicPacketViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 support-side forensic packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ForensicPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing help document reference.
    pub doc_ref: String,
    /// Boundary schema reference.
    pub schema_ref: String,
    /// Checked review artifact reference.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing packets and contracts this packet composes.
    pub supporting_contract_refs: Vec<String>,
    /// Visible forensic rows.
    pub rows: Vec<M5ForensicRow>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5ForensicPacket {
    /// Validates the packet's locality, share, and redaction invariants.
    pub fn validate(&self) -> Vec<M5ForensicPacketViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FORENSIC_PACKET_RECORD_KIND {
            violations.push(M5ForensicPacketViolation {
                path: "record_kind".to_owned(),
                message: "unexpected record_kind".to_owned(),
            });
        }
        if self.schema_version != M5_FORENSIC_PACKET_SCHEMA_VERSION {
            violations.push(M5ForensicPacketViolation {
                path: "schema_version".to_owned(),
                message: "unexpected schema_version".to_owned(),
            });
        }
        if self.doc_ref != M5_FORENSIC_PACKET_DOC_REF {
            violations.push(M5ForensicPacketViolation {
                path: "doc_ref".to_owned(),
                message: "packet must quote the canonical reviewer doc".to_owned(),
            });
        }
        if self.schema_ref != M5_FORENSIC_PACKET_SCHEMA_REF {
            violations.push(M5ForensicPacketViolation {
                path: "schema_ref".to_owned(),
                message: "packet must quote the canonical schema ref".to_owned(),
            });
        }
        if self.artifact_ref != M5_FORENSIC_PACKET_ARTIFACT_REF {
            violations.push(M5ForensicPacketViolation {
                path: "artifact_ref".to_owned(),
                message: "packet must quote the checked review artifact ref".to_owned(),
            });
        }

        for required in REQUIRED_HOST_FAMILY_IDS {
            if !self.rows.iter().any(|row| row.host_family_id == *required) {
                violations.push(M5ForensicPacketViolation {
                    path: "rows".to_owned(),
                    message: format!("missing required forensic host family {required}"),
                });
            }
        }

        let mut state_coverage = Vec::new();
        for row in &self.rows {
            let path = format!("rows.{}", row.forensic_packet_id);
            for (field, value) in [
                ("forensic_packet_id", row.forensic_packet_id.as_str()),
                ("scenario_id", row.scenario_id.as_str()),
                ("host_family_id", row.host_family_id.as_str()),
                ("host_family_label", row.host_family_label.as_str()),
                (
                    "runtime_forensic_packet_ref",
                    row.runtime_forensic_packet_ref.as_str(),
                ),
                (
                    "exact_build_identity_ref",
                    row.exact_build_identity_ref.as_str(),
                ),
                ("restart_lineage_ref", row.restart_lineage_ref.as_str()),
                ("checkpoint_ref", row.checkpoint_ref.as_str()),
                ("export_safe_summary", row.export_safe_summary.as_str()),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ForensicPacketViolation {
                        path: format!("{path}.{field}"),
                        message: "field must not be empty".to_owned(),
                    });
                }
            }

            if row.artifact_states.is_empty() {
                violations.push(M5ForensicPacketViolation {
                    path: format!("{path}.artifact_states"),
                    message: "forensic row must carry artifact state rows".to_owned(),
                });
            }
            if row.share_actions.is_empty() {
                violations.push(M5ForensicPacketViolation {
                    path: format!("{path}.share_actions"),
                    message: "forensic row must carry reviewed share actions".to_owned(),
                });
            }
            if row.no_silent_upload_guards.is_empty() {
                violations.push(M5ForensicPacketViolation {
                    path: format!("{path}.no_silent_upload_guards"),
                    message: "forensic row must list no-silent-upload guard assertions".to_owned(),
                });
            }

            let has_local_preview = row.share_actions.iter().any(|action| {
                action.destination_class == ForensicShareDestinationClass::LocalPreview
                    && action.enabled
                    && !action.network_egress
            });
            if !has_local_preview {
                violations.push(M5ForensicPacketViolation {
                    path: format!("{path}.share_actions"),
                    message: "forensic row must expose a local preview before any egress path"
                        .to_owned(),
                });
            }

            for (index, artifact) in row.artifact_states.iter().enumerate() {
                let artifact_path = format!("{path}.artifact_states.{index}");
                if artifact.artifact_ref.trim().is_empty() || artifact.summary.trim().is_empty() {
                    violations.push(M5ForensicPacketViolation {
                        path: artifact_path.clone(),
                        message: "artifact rows must carry a stable ref and summary".to_owned(),
                    });
                }
                state_coverage.push(artifact.state_class);
                if matches!(artifact.state_class, ForensicArtifactStateClass::Uploaded)
                    && !artifact.explicit_user_or_policy_action_required
                {
                    violations.push(M5ForensicPacketViolation {
                        path: artifact_path,
                        message: "uploaded artifact rows must prove explicit user or policy action"
                            .to_owned(),
                    });
                }
                if !artifact.scope_widening_without_review_forbidden {
                    violations.push(M5ForensicPacketViolation {
                        path: format!("{path}.artifact_states.{index}.scope_widening_without_review_forbidden"),
                        message: "forensic artifact rows must forbid silent scope widening"
                            .to_owned(),
                    });
                }
            }

            for (index, action) in row.share_actions.iter().enumerate() {
                let action_path = format!("{path}.share_actions.{index}");
                if action.action_id.trim().is_empty()
                    || action.label.trim().is_empty()
                    || action.note.trim().is_empty()
                {
                    violations.push(M5ForensicPacketViolation {
                        path: action_path.clone(),
                        message: "share actions must carry stable ids, labels, and notes"
                            .to_owned(),
                    });
                }
                if action.network_egress && !action.explicit_user_or_policy_action_required {
                    violations.push(M5ForensicPacketViolation {
                        path: action_path.clone(),
                        message: "network-egress actions must remain behind explicit user or policy action".to_owned(),
                    });
                }
                if matches!(
                    action.destination_class,
                    ForensicShareDestinationClass::ManagedUpload
                        | ForensicShareDestinationClass::MirrorCopy
                ) && !action.retention_or_scope_change_requires_review
                {
                    violations.push(M5ForensicPacketViolation {
                        path: action_path,
                        message: "egress or mirror actions must require retention/scope review"
                            .to_owned(),
                    });
                }
            }
        }

        for required in ForensicArtifactStateClass::ALL {
            if !state_coverage.contains(&required) {
                violations.push(M5ForensicPacketViolation {
                    path: "rows.artifact_states.state_class".to_owned(),
                    message: format!(
                        "packet must distinguish {} artifact state(s)",
                        required.as_str()
                    ),
                });
            }
        }

        violations
    }

    /// Returns true when the packet remains metadata-safe and redaction-aware.
    pub fn is_export_safe(&self) -> bool {
        self.validate().is_empty()
    }
}

/// Returns the canonical seeded M5 support-side forensic packet.
pub fn seeded_m5_forensic_packet() -> M5ForensicPacket {
    M5ForensicPacket {
        record_kind: M5_FORENSIC_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_FORENSIC_PACKET_SCHEMA_VERSION,
        packet_id: "support.m5.forensic_packet.v1".to_owned(),
        generated_at: "2026-06-12T23:59:00Z".to_owned(),
        doc_ref: M5_FORENSIC_PACKET_DOC_REF.to_owned(),
        schema_ref: M5_FORENSIC_PACKET_SCHEMA_REF.to_owned(),
        artifact_ref: M5_FORENSIC_PACKET_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".plans/M05-248.md".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#1092".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#4800".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#4799".to_owned(),
            "docs/runtime/fault_domains_and_restart_policy.md#forensic-packet-requirements"
                .to_owned(),
        ],
        supporting_contract_refs: vec![
            "schemas/runtime/forensic_packet.schema.json".to_owned(),
            "schemas/support/m5-fault-crash-governance.schema.json".to_owned(),
            "schemas/support/crash_store_viewer.schema.json".to_owned(),
            "schemas/support/depth-surface-schema-registry.schema.json".to_owned(),
            "schemas/support/recovery_review.schema.json".to_owned(),
        ],
        rows: vec![
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:notebook-kernel-crash-stall",
                scenario_id: "scenario:notebook_kernel_crash_stall",
                host_family_id: "notebook_kernel_host",
                host_family_label: "Notebook kernel session",
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                trigger_class: ForensicTriggerClass::StallDetected,
                redaction_profile: RedactionProfileClass::LocalOnlyReviewRequired,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:notebook-kernel:stall:0001",
                exact_build_identity_ref: "build-id:aureline:preview:notebook:001",
                restart_lineage_ref: "restart-lineage:notebook-kernel:0001",
                checkpoint_ref: "checkpoint:notebook-kernel:cell-run:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::RuntimeForensicPacket,
                        "runtime.forensic:notebook-kernel:stall:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Bounded supervisor packet remains on-device until reviewed export.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::CrashEnvelope,
                        "crash-envelope:notebook-kernel:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::EnvironmentSummaryOnly,
                        true,
                        "Crash envelope stays local and metadata-safe by default.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SupportBundleManifest,
                        "support.bundle.manifest:notebook-kernel:0001",
                        ForensicArtifactStateClass::Uploaded,
                        RetentionClass::ManagedContractWindow,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Reviewed manifest was attached to support only after local preview confirmed the scoped export.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:notebook-kernel:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Local preview renders the exact metadata-safe packet before any share path.",
                    ),
                    seed_action(
                        "action:notebook-kernel:export",
                        "Export locally",
                        ForensicShareDestinationClass::LocalExport,
                        false,
                        true,
                        true,
                        "Local export writes the reviewed packet without network egress.",
                    ),
                    seed_action(
                        "action:notebook-kernel:upload",
                        "Upload reviewed packet",
                        ForensicShareDestinationClass::ManagedUpload,
                        true,
                        true,
                        true,
                        "Upload remains blocked until the reviewed export scope and retention note are accepted.",
                    ),
                ],
                export_safe_summary: "Notebook stall packet keeps checkpoint, restart lineage, and reviewed upload scope visible; no kernel bytes leave the device silently.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:data-api-connector-drift",
                scenario_id: "scenario:remote_connector_drift",
                host_family_id: "data_api_connector_host",
                host_family_label: "Data/API connector and query runtime",
                fault_domain_class: FaultDomainClass::RemoteConnector,
                trigger_class: ForensicTriggerClass::ConnectorDrift,
                redaction_profile: RedactionProfileClass::OperatorOnlyRestricted,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:data-api:drift:0001",
                exact_build_identity_ref: "build-id:aureline:preview:data-api:001",
                restart_lineage_ref: "restart-lineage:data-api:0001",
                checkpoint_ref: "checkpoint:data-api:reviewed-request-metadata:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::RuntimeForensicPacket,
                        "runtime.forensic:data-api:drift:0001",
                        ForensicArtifactStateClass::Imported,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::OperatorOnlyRestricted,
                        true,
                        "Imported replay packet preserves drift lineage without claiming a live connector state.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SupportBundleManifest,
                        "support.bundle.manifest:data-api:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalManifestUntilSent,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Support export remains local until the operator approves broader share.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:data-api:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Imported drift evidence stays inspectable before any new export.",
                    ),
                    seed_action(
                        "action:data-api:handoff",
                        "Import reviewed handoff",
                        ForensicShareDestinationClass::ImportedHandoff,
                        false,
                        true,
                        true,
                        "Imported handoff keeps source class explicit and does not widen retention.",
                    ),
                ],
                export_safe_summary: "Remote connector drift stays attributable to imported evidence and reviewed export posture; stale imported context never masquerades as live truth.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:preview-dev-server-restart",
                scenario_id: "scenario:preview_server_restart",
                host_family_id: "preview_dev_server_host",
                host_family_label: "Preview dev server",
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                trigger_class: ForensicTriggerClass::RestartBudgetExhausted,
                redaction_profile: RedactionProfileClass::LocalOnlyReviewRequired,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:preview-server:restart:0001",
                exact_build_identity_ref: "build-id:aureline:preview:preview-server:001",
                restart_lineage_ref: "restart-lineage:preview-server:0001",
                checkpoint_ref: "checkpoint:preview-server:route-metadata:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::CrashEnvelope,
                        "crash-envelope:preview-server:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::EnvironmentSummaryOnly,
                        true,
                        "Envelope preserves exact build and port-binding loop without raw response bodies.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SymbolManifest,
                        "symbol-manifest:preview-server:0001",
                        ForensicArtifactStateClass::Mirrored,
                        RetentionClass::ManagedContractWindow,
                        RedactionProfileClass::MetadataSafeDefault,
                        true,
                        "Mirrored symbol manifest is declared explicitly instead of implied by current connectivity.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:preview-server:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "The reviewed preview shows checkpoint preservation before restart lineage is shared.",
                    ),
                    seed_action(
                        "action:preview-server:mirror",
                        "Use mirrored symbols",
                        ForensicShareDestinationClass::MirrorCopy,
                        true,
                        true,
                        true,
                        "Mirrored symbol lookup is a separate reviewed path with visible destination and retention.",
                    ),
                ],
                export_safe_summary: "Preview-server restart forensics remain local-first and exact-build-aware even when mirrored symbols are used.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:provider-run-failure",
                scenario_id: "scenario:provider_run_failure",
                host_family_id: "provider_run_session_host",
                host_family_label: "Provider-backed run session",
                fault_domain_class: FaultDomainClass::AiToolBroker,
                trigger_class: ForensicTriggerClass::Crash,
                redaction_profile: RedactionProfileClass::OperatorOnlyRestricted,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:provider-run:crash:0001",
                exact_build_identity_ref: "build-id:aureline:beta:provider-run:001",
                restart_lineage_ref: "restart-lineage:provider-run:0001",
                checkpoint_ref: "checkpoint:provider-run:tool-ticket-metadata:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::RuntimeForensicPacket,
                        "runtime.forensic:provider-run:crash:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::OperatorOnlyRestricted,
                        true,
                        "Provider-run crash packet keeps ticket lineage local until reviewed export.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::UploadReceipt,
                        "upload-receipt:provider-run:0001",
                        ForensicArtifactStateClass::Uploaded,
                        RetentionClass::ManagedContractWindow,
                        RedactionProfileClass::MetadataSafeDefault,
                        true,
                        "Uploaded receipt proves that egress happened only after explicit support attachment.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:provider-run:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview keeps tool ids and ticket lineage visible without exporting them broadly.",
                    ),
                    seed_action(
                        "action:provider-run:upload",
                        "Upload reviewed packet",
                        ForensicShareDestinationClass::ManagedUpload,
                        true,
                        true,
                        true,
                        "Managed upload requires a new reviewed packet because authority was revoked on failure.",
                    ),
                ],
                export_safe_summary: "Provider-run crash forensics keep authority revocation and explicit reviewed upload in the same packet.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:profiler-replay-imported-gap",
                scenario_id: "scenario:profiler_replay_imported_gap",
                host_family_id: "profiler_replay_session_host",
                host_family_label: "Profiler and replay session",
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                trigger_class: ForensicTriggerClass::RecoveryTransition,
                redaction_profile: RedactionProfileClass::OperatorOnlyRestricted,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:profiler-replay:gap:0001",
                exact_build_identity_ref: "build-id:aureline:beta:profiler-replay:001",
                restart_lineage_ref: "restart-lineage:profiler-replay:0001",
                checkpoint_ref: "checkpoint:profiler-replay:capture-metadata:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::SymbolicationReport,
                        "symbolication-report:profiler-replay:0001",
                        ForensicArtifactStateClass::Imported,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::OperatorOnlyRestricted,
                        true,
                        "Imported profile mapping report stays labeled as imported and partial.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SupportBundleManifest,
                        "support.bundle.manifest:profiler-replay:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalManifestUntilSent,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Replay export remains local-only until the imported mapping gap is reviewed.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:profiler-replay:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview preserves imported-vs-exact mapping quality before any share.",
                    ),
                    seed_action(
                        "action:profiler-replay:export",
                        "Export locally",
                        ForensicShareDestinationClass::LocalExport,
                        false,
                        true,
                        true,
                        "Local export captures mapping quality without forcing upload of profile bytes.",
                    ),
                ],
                export_safe_summary: "Profiler/replay packet keeps imported mapping quality explicit and export-safe.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:pipeline-viewer-fault",
                scenario_id: "scenario:pipeline_viewer_fault",
                host_family_id: "pipeline_viewer_host",
                host_family_label: "Pipeline viewer session",
                fault_domain_class: FaultDomainClass::RemoteConnector,
                trigger_class: ForensicTriggerClass::ConnectorDrift,
                redaction_profile: RedactionProfileClass::LocalOnlyReviewRequired,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:pipeline-viewer:drift:0001",
                exact_build_identity_ref: "build-id:aureline:preview:pipeline-viewer:001",
                restart_lineage_ref: "restart-lineage:pipeline-viewer:0001",
                checkpoint_ref: "checkpoint:pipeline-viewer:event-cursor:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::RuntimeForensicPacket,
                        "runtime.forensic:pipeline-viewer:drift:0001",
                        ForensicArtifactStateClass::Imported,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::MetadataSafeDefault,
                        true,
                        "Pipeline packet cites imported event-stream lineage instead of pretending the stream is live.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SupportBundleManifest,
                        "support.bundle.manifest:pipeline-viewer:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalManifestUntilSent,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Support export remains metadata-only and local-first.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:pipeline-viewer:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview keeps imported event lineage visible before any export.",
                    ),
                    seed_action(
                        "action:pipeline-viewer:export",
                        "Export locally",
                        ForensicShareDestinationClass::LocalExport,
                        false,
                        true,
                        true,
                        "Local export preserves reconnect lineage and partial-truth status.",
                    ),
                ],
                export_safe_summary: "Pipeline-viewer fault packet stays export-safe and explicit about imported event lineage.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:query-runtime-crash",
                scenario_id: "scenario:query_runtime_crash",
                host_family_id: "query_runtime_host",
                host_family_label: "Query/request runtime",
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                trigger_class: ForensicTriggerClass::Crash,
                redaction_profile: RedactionProfileClass::LocalOnlyReviewRequired,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:query-runtime:crash:0001",
                exact_build_identity_ref: "build-id:aureline:preview:query-runtime:001",
                restart_lineage_ref: "restart-lineage:query-runtime:0001",
                checkpoint_ref: "checkpoint:query-runtime:request-metadata:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::DumpManifest,
                        "dump-manifest:query-runtime:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Query-runtime dump metadata remains local and review-required by default.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SymbolicationReport,
                        "symbolication-report:query-runtime:0001",
                        ForensicArtifactStateClass::Mirrored,
                        RetentionClass::ManagedContractWindow,
                        RedactionProfileClass::MetadataSafeDefault,
                        true,
                        "Mirrored symbolication remains explicit and exact-build scoped.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:query-runtime:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview shows request-family metadata and no-hidden-rerun posture first.",
                    ),
                    seed_action(
                        "action:query-runtime:mirror",
                        "Use mirrored symbols",
                        ForensicShareDestinationClass::MirrorCopy,
                        true,
                        true,
                        true,
                        "Mirrored symbol lookup is reviewed and does not auto-upload dumps.",
                    ),
                ],
                export_safe_summary: "Query/runtime crash packet preserves request metadata locally and keeps mirrored symbol use explicit.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:docs-browser-bridge-drift",
                scenario_id: "scenario:docs_browser_bridge_route_drift",
                host_family_id: "docs_browser_bridge_host",
                host_family_label: "Docs and browser bridge",
                fault_domain_class: FaultDomainClass::RemoteConnector,
                trigger_class: ForensicTriggerClass::ConnectorDrift,
                redaction_profile: RedactionProfileClass::OperatorOnlyRestricted,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:docs-bridge:drift:0001",
                exact_build_identity_ref: "build-id:aureline:preview:docs-bridge:001",
                restart_lineage_ref: "restart-lineage:docs-bridge:0001",
                checkpoint_ref: "checkpoint:docs-bridge:route-note:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::RuntimeForensicPacket,
                        "runtime.forensic:docs-bridge:drift:0001",
                        ForensicArtifactStateClass::Imported,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::OperatorOnlyRestricted,
                        true,
                        "Bridge packet cites imported route facts and keeps origin mismatch explicit.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SupportBundleManifest,
                        "support.bundle.manifest:docs-bridge:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalManifestUntilSent,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Docs/browser export remains a local reviewed handoff path.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:docs-bridge:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview keeps bridge route drift visible and scoped before share.",
                    ),
                    seed_action(
                        "action:docs-bridge:handoff",
                        "Import reviewed handoff",
                        ForensicShareDestinationClass::ImportedHandoff,
                        false,
                        true,
                        true,
                        "Imported docs handoff remains labeled and does not broaden retention.",
                    ),
                ],
                export_safe_summary: "Docs/browser bridge packet distinguishes imported route facts from local reviewed export state.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:registry-connector-host-mismatch",
                scenario_id: "scenario:connector_host_mismatch",
                host_family_id: "registry_database_connector_host",
                host_family_label: "Registry or database connector",
                fault_domain_class: FaultDomainClass::RemoteConnector,
                trigger_class: ForensicTriggerClass::HostMismatch,
                redaction_profile: RedactionProfileClass::OperatorOnlyRestricted,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:registry-connector:mismatch:0001",
                exact_build_identity_ref: "build-id:aureline:beta:registry-connector:001",
                restart_lineage_ref: "restart-lineage:registry-connector:0001",
                checkpoint_ref: "checkpoint:registry-connector:target-metadata:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::CrashEnvelope,
                        "crash-envelope:registry-connector:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::OperatorOnlyRestricted,
                        true,
                        "Connector mismatch envelope stays local and target-scoped.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::UploadReceipt,
                        "upload-receipt:registry-connector:0001",
                        ForensicArtifactStateClass::Uploaded,
                        RetentionClass::ManagedContractWindow,
                        RedactionProfileClass::MetadataSafeDefault,
                        true,
                        "Receipt proves that external handoff happened only after reviewed export.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:registry-connector:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview names the mismatched host and approval scope before export.",
                    ),
                    seed_action(
                        "action:registry-connector:upload",
                        "Upload reviewed packet",
                        ForensicShareDestinationClass::ManagedUpload,
                        true,
                        true,
                        true,
                        "Upload remains explicit because target identity and approval scope changed.",
                    ),
                ],
                export_safe_summary: "Connector mismatch packet keeps target identity, reviewed export scope, and upload receipt visible together.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:infra-helper-signature-failure",
                scenario_id: "scenario:infra_helper_signature_failure",
                host_family_id: "infra_helper_job",
                host_family_label: "Infrastructure helper",
                fault_domain_class: FaultDomainClass::PolicyVerifierHelper,
                trigger_class: ForensicTriggerClass::RestartBudgetExhausted,
                redaction_profile: RedactionProfileClass::MetadataSafeDefault,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:infra-helper:signature-failure:0001",
                exact_build_identity_ref: "build-id:aureline:stable:infra-helper:001",
                restart_lineage_ref: "restart-lineage:infra-helper:0001",
                checkpoint_ref: "checkpoint:infra-helper:signed-bundle-cache:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::RuntimeForensicPacket,
                        "runtime.forensic:infra-helper:signature-failure:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::MetadataSafeDefault,
                        true,
                        "Verifier helper packet names signature failure and cache snapshot locally.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SymbolManifest,
                        "symbol-manifest:infra-helper:0001",
                        ForensicArtifactStateClass::Mirrored,
                        RetentionClass::ManagedContractWindow,
                        RedactionProfileClass::MetadataSafeDefault,
                        true,
                        "Mirrored symbol manifest remains an explicit artifact-state row.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:infra-helper:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview shows signature failure and checkpoint lineage locally first.",
                    ),
                    seed_action(
                        "action:infra-helper:mirror",
                        "Use mirrored symbols",
                        ForensicShareDestinationClass::MirrorCopy,
                        true,
                        true,
                        true,
                        "Mirror use is explicit and separate from crash/support upload.",
                    ),
                ],
                export_safe_summary: "Infrastructure helper packet keeps signature-failure forensics local-first and mirror-aware.",
            }),
            seed_row(SeededForensicRowInput {
                forensic_packet_id: "forensic:provider-broker-circuit-breaker",
                scenario_id: "scenario:ai_broker_circuit_breaker",
                host_family_id: "provider_run_session_host",
                host_family_label: "Provider-backed run session",
                fault_domain_class: FaultDomainClass::AiToolBroker,
                trigger_class: ForensicTriggerClass::CircuitBreakerOpen,
                redaction_profile: RedactionProfileClass::OperatorOnlyRestricted,
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                runtime_forensic_packet_ref: "runtime.forensic:provider-broker:circuit-breaker:0001",
                exact_build_identity_ref: "build-id:aureline:beta:provider-broker:001",
                restart_lineage_ref: "restart-lineage:provider-broker:0001",
                checkpoint_ref: "checkpoint:provider-broker:ticket-metadata:0001",
                artifact_states: vec![
                    seed_artifact(
                        ForensicArtifactClass::RuntimeForensicPacket,
                        "runtime.forensic:provider-broker:circuit-breaker:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                        RedactionProfileClass::OperatorOnlyRestricted,
                        true,
                        "Circuit-breaker packet keeps breaker state, ticket lineage, and replay scope local-first.",
                    ),
                    seed_artifact(
                        ForensicArtifactClass::SupportBundleManifest,
                        "support.bundle.manifest:provider-broker:0001",
                        ForensicArtifactStateClass::LocalOnly,
                        RetentionClass::LocalManifestUntilSent,
                        RedactionProfileClass::LocalOnlyReviewRequired,
                        true,
                        "Support export remains separate from ambient AI telemetry.",
                    ),
                ],
                share_actions: vec![
                    seed_action(
                        "action:provider-broker:preview",
                        "Preview locally",
                        ForensicShareDestinationClass::LocalPreview,
                        false,
                        true,
                        true,
                        "Preview shows breaker state and revoked authority before any share.",
                    ),
                    seed_action(
                        "action:provider-broker:export",
                        "Export locally",
                        ForensicShareDestinationClass::LocalExport,
                        false,
                        true,
                        true,
                        "Local export preserves ticket lineage without re-enabling provider egress.",
                    ),
                ],
                export_safe_summary: "AI broker circuit-breaker packet keeps authority revocation, breaker state, and local-first export on one surface.",
            }),
        ],
        export_safe_summary: "M5 forensic packets remain metadata-safe and redaction-aware; locality and egress state stay explicit across local-only, imported, mirrored, and uploaded artifacts.".to_owned(),
    }
}

struct SeededForensicRowInput<'a> {
    forensic_packet_id: &'a str,
    scenario_id: &'a str,
    host_family_id: &'a str,
    host_family_label: &'a str,
    fault_domain_class: FaultDomainClass,
    trigger_class: ForensicTriggerClass,
    redaction_profile: RedactionProfileClass,
    retention_class: RetentionClass,
    runtime_forensic_packet_ref: &'a str,
    exact_build_identity_ref: &'a str,
    restart_lineage_ref: &'a str,
    checkpoint_ref: &'a str,
    artifact_states: Vec<ForensicArtifactStateRow>,
    share_actions: Vec<ForensicShareActionRow>,
    export_safe_summary: &'a str,
}

fn seed_row(input: SeededForensicRowInput<'_>) -> M5ForensicRow {
    M5ForensicRow {
        forensic_packet_id: input.forensic_packet_id.to_owned(),
        scenario_id: input.scenario_id.to_owned(),
        host_family_id: input.host_family_id.to_owned(),
        host_family_label: input.host_family_label.to_owned(),
        fault_domain_class: input.fault_domain_class,
        trigger_class: input.trigger_class,
        runtime_forensic_packet_ref: input.runtime_forensic_packet_ref.to_owned(),
        exact_build_identity_ref: input.exact_build_identity_ref.to_owned(),
        restart_lineage_ref: input.restart_lineage_ref.to_owned(),
        checkpoint_ref: input.checkpoint_ref.to_owned(),
        redaction_profile: input.redaction_profile,
        retention_class: input.retention_class,
        artifact_states: input.artifact_states,
        share_actions: input.share_actions,
        no_silent_upload_guards: vec![
            "local_preview_required_before_egress".to_owned(),
            "explicit_user_or_policy_action_required_for_upload".to_owned(),
            "retention_or_export_scope_change_requires_review".to_owned(),
        ],
        export_safe_summary: input.export_safe_summary.to_owned(),
    }
}

fn seed_artifact(
    artifact_class: ForensicArtifactClass,
    artifact_ref: &str,
    state_class: ForensicArtifactStateClass,
    retention_class: RetentionClass,
    redaction_profile: RedactionProfileClass,
    explicit_user_or_policy_action_required: bool,
    summary: &str,
) -> ForensicArtifactStateRow {
    ForensicArtifactStateRow {
        artifact_class,
        artifact_ref: artifact_ref.to_owned(),
        state_class,
        retention_class,
        redaction_profile,
        explicit_user_or_policy_action_required,
        scope_widening_without_review_forbidden: true,
        summary: summary.to_owned(),
    }
}

fn seed_action(
    action_id: &str,
    label: &str,
    destination_class: ForensicShareDestinationClass,
    network_egress: bool,
    explicit_user_or_policy_action_required: bool,
    retention_or_scope_change_requires_review: bool,
    note: &str,
) -> ForensicShareActionRow {
    ForensicShareActionRow {
        action_id: action_id.to_owned(),
        label: label.to_owned(),
        destination_class,
        network_egress,
        explicit_user_or_policy_action_required,
        enabled: true,
        retention_or_scope_change_requires_review,
        note: note.to_owned(),
    }
}
