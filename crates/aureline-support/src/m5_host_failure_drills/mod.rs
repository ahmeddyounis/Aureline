//! M5 host-failure drill corpus and readiness packet.
//!
//! This module binds the seeded M5 host-failure corpus to the existing
//! governance, crash-store, schema-registry, and recovery-review packets.
//! Each drill row proves restart-budget enforcement, scoped failure,
//! checkpoint preservation, no-hidden-rerun behavior, and explicit no-silent-
//! upload posture for a claimed M5 host family.

use serde::{Deserialize, Serialize};

use crate::m5_forensic_packet::{
    seeded_m5_forensic_packet, M5ForensicPacket, M5ForensicRow, M5_FORENSIC_PACKET_SCHEMA_REF,
};

/// Stable record-kind tag carried by the M5 host-failure drill packet.
pub const M5_HOST_FAILURE_DRILL_PACKET_RECORD_KIND: &str = "m5_host_failure_drill_packet";

/// Frozen schema version for the M5 host-failure drill packet.
pub const M5_HOST_FAILURE_DRILL_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the M5 host-failure drill schema.
pub const M5_HOST_FAILURE_DRILL_SCHEMA_REF: &str =
    "schemas/support/m5-host-failure-drills.schema.json";

/// Repository-relative path of the reviewer-facing help document.
pub const M5_HOST_FAILURE_DRILL_DOC_REF: &str = "docs/help/support/m5-host-failure-drills.md";

/// Repository-relative path of the checked review artifact.
pub const M5_HOST_FAILURE_DRILL_ARTIFACT_REF: &str = "artifacts/support/m5/host-failure-drills.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_HOST_FAILURE_DRILL_FIXTURE_DIR: &str = "fixtures/support/m5/host_failure_drills";

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

/// Closed scenario vocabulary covered by the M5 host-failure corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostFailureScenarioClass {
    /// Notebook kernel crash/stall with preserved cell-run checkpoint.
    NotebookKernelCrashStall,
    /// Provider-run session failure with revoked authority.
    ProviderRunFailure,
    /// Preview-server restart loop within a bounded strike window.
    PreviewServerRestart,
    /// Data/API connector drift that narrows the current route.
    RemoteConnectorDrift,
    /// AI broker breaker open with no silent replay of side effects.
    AiBrokerCircuitBreaker,
    /// Query/request runtime crash with explicit rerun boundary.
    QueryRuntimeCrash,
    /// Pipeline viewer fault with imported/live truth separation.
    PipelineViewerFault,
    /// Registry or database connector host mismatch.
    ConnectorHostMismatch,
    /// Docs/browser bridge route drift and import-safe handoff.
    DocsBrowserBridgeRouteDrift,
    /// Profiler/replay imported mapping gap and local export fallback.
    ProfilerReplayImportedGap,
    /// Infrastructure helper signature failure and fail-closed review.
    InfraHelperSignatureFailure,
}

impl HostFailureScenarioClass {
    /// All scenario classes in canonical order.
    pub const ALL: [Self; 11] = [
        Self::NotebookKernelCrashStall,
        Self::ProviderRunFailure,
        Self::PreviewServerRestart,
        Self::RemoteConnectorDrift,
        Self::AiBrokerCircuitBreaker,
        Self::QueryRuntimeCrash,
        Self::PipelineViewerFault,
        Self::ConnectorHostMismatch,
        Self::DocsBrowserBridgeRouteDrift,
        Self::ProfilerReplayImportedGap,
        Self::InfraHelperSignatureFailure,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookKernelCrashStall => "notebook_kernel_crash_stall",
            Self::ProviderRunFailure => "provider_run_failure",
            Self::PreviewServerRestart => "preview_server_restart",
            Self::RemoteConnectorDrift => "remote_connector_drift",
            Self::AiBrokerCircuitBreaker => "ai_broker_circuit_breaker",
            Self::QueryRuntimeCrash => "query_runtime_crash",
            Self::PipelineViewerFault => "pipeline_viewer_fault",
            Self::ConnectorHostMismatch => "connector_host_mismatch",
            Self::DocsBrowserBridgeRouteDrift => "docs_browser_bridge_route_drift",
            Self::ProfilerReplayImportedGap => "profiler_replay_imported_gap",
            Self::InfraHelperSignatureFailure => "infra_helper_signature_failure",
        }
    }
}

/// One seeded drill row proving an M5 host-failure scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostFailureDrillRow {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Closed scenario-class token.
    pub scenario_class: HostFailureScenarioClass,
    /// Stable host-family id.
    pub host_family_id: String,
    /// Human-readable host-family label.
    pub host_family_label: String,
    /// Stable restart-budget or lifecycle reference used by the row.
    pub restart_budget_ref: String,
    /// Whether the drill proves bounded restart-budget enforcement.
    pub restart_budget_enforced: bool,
    /// Whether the drill proves failure stayed inside the claimed host scope.
    pub scoped_failure_only: bool,
    /// Whether the drill preserves a checkpoint, reviewed metadata restore, or clean-state boundary explicitly.
    pub checkpoint_preserved: bool,
    /// Whether the drill blocks hidden rerun or replay.
    pub no_hidden_rerun: bool,
    /// Whether quarantine, disablement, or circuit-open state stays visible.
    pub visible_fail_closed_state: bool,
    /// Stable forensic packet id for the drill row.
    pub forensic_packet_ref: String,
    /// Existing support packets this drill reuses.
    pub support_packet_refs: Vec<String>,
    /// Explicit guard assertions proving no silent upload or scope widening.
    pub upload_guard_assertions: Vec<String>,
    /// Protected fixture or evidence reference used by the row.
    pub evidence_fixture_ref: String,
    /// Reviewable export-safe summary.
    pub summary: String,
}

/// One validation error emitted by [`M5HostFailureDrillPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostFailureDrillViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 host-failure drill packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostFailureDrillPacket {
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
    /// Existing packets and contracts the drill packet composes.
    pub supporting_contract_refs: Vec<String>,
    /// Support-side forensic packet quoted by the drill packet.
    pub forensic_packet: M5ForensicPacket,
    /// Seeded drill rows.
    pub drills: Vec<HostFailureDrillRow>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5HostFailureDrillPacket {
    /// Validates drill coverage and bounded-failure/export invariants.
    pub fn validate(&self) -> Vec<M5HostFailureDrillViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_HOST_FAILURE_DRILL_PACKET_RECORD_KIND {
            violations.push(M5HostFailureDrillViolation {
                path: "record_kind".to_owned(),
                message: "unexpected record_kind".to_owned(),
            });
        }
        if self.schema_version != M5_HOST_FAILURE_DRILL_SCHEMA_VERSION {
            violations.push(M5HostFailureDrillViolation {
                path: "schema_version".to_owned(),
                message: "unexpected schema_version".to_owned(),
            });
        }
        if self.doc_ref != M5_HOST_FAILURE_DRILL_DOC_REF {
            violations.push(M5HostFailureDrillViolation {
                path: "doc_ref".to_owned(),
                message: "packet must quote the canonical reviewer doc".to_owned(),
            });
        }
        if self.schema_ref != M5_HOST_FAILURE_DRILL_SCHEMA_REF {
            violations.push(M5HostFailureDrillViolation {
                path: "schema_ref".to_owned(),
                message: "packet must quote the canonical schema ref".to_owned(),
            });
        }
        if self.artifact_ref != M5_HOST_FAILURE_DRILL_ARTIFACT_REF {
            violations.push(M5HostFailureDrillViolation {
                path: "artifact_ref".to_owned(),
                message: "packet must quote the checked review artifact ref".to_owned(),
            });
        }

        for required in HostFailureScenarioClass::ALL {
            if !self.drills.iter().any(|row| row.scenario_class == required) {
                violations.push(M5HostFailureDrillViolation {
                    path: "drills".to_owned(),
                    message: format!("missing required scenario {}", required.as_str()),
                });
            }
        }

        for required in REQUIRED_HOST_FAMILY_IDS {
            if !self
                .drills
                .iter()
                .any(|row| row.host_family_id == *required)
            {
                violations.push(M5HostFailureDrillViolation {
                    path: "drills.host_family_id".to_owned(),
                    message: format!("missing required drill host family {required}"),
                });
            }
        }

        if !self
            .supporting_contract_refs
            .iter()
            .any(|item| item == M5_FORENSIC_PACKET_SCHEMA_REF)
        {
            violations.push(M5HostFailureDrillViolation {
                path: "supporting_contract_refs".to_owned(),
                message: "drill packet must cite the M5 forensic packet schema".to_owned(),
            });
        }

        for row in &self.drills {
            let path = format!("drills.{}", row.scenario_id);
            for (field, value) in [
                ("scenario_id", row.scenario_id.as_str()),
                ("host_family_id", row.host_family_id.as_str()),
                ("host_family_label", row.host_family_label.as_str()),
                ("restart_budget_ref", row.restart_budget_ref.as_str()),
                ("forensic_packet_ref", row.forensic_packet_ref.as_str()),
                ("evidence_fixture_ref", row.evidence_fixture_ref.as_str()),
                ("summary", row.summary.as_str()),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5HostFailureDrillViolation {
                        path: format!("{path}.{field}"),
                        message: "field must not be empty".to_owned(),
                    });
                }
            }
            if !row.restart_budget_enforced
                || !row.scoped_failure_only
                || !row.checkpoint_preserved
                || !row.no_hidden_rerun
                || !row.visible_fail_closed_state
            {
                violations.push(M5HostFailureDrillViolation {
                    path,
                    message: "drill rows must prove restart-budget, scope, checkpoint, no-hidden-rerun, and visible fail-closed posture".to_owned(),
                });
            }
            if row.support_packet_refs.is_empty() {
                violations.push(M5HostFailureDrillViolation {
                    path: format!("drills.{}.support_packet_refs", row.scenario_id),
                    message: "drill rows must cite reused support packets".to_owned(),
                });
            }
            if row.upload_guard_assertions.len() < 3 {
                violations.push(M5HostFailureDrillViolation {
                    path: format!("drills.{}.upload_guard_assertions", row.scenario_id),
                    message: "drill rows must carry explicit no-silent-upload assertions"
                        .to_owned(),
                });
            }
            if !self
                .forensic_packet
                .rows
                .iter()
                .any(|forensic| forensic.forensic_packet_id == row.forensic_packet_ref)
            {
                violations.push(M5HostFailureDrillViolation {
                    path: format!("drills.{}.forensic_packet_ref", row.scenario_id),
                    message: "drill row must reference a forensic row in the same packet set"
                        .to_owned(),
                });
            }
        }

        for violation in self.forensic_packet.validate() {
            violations.push(M5HostFailureDrillViolation {
                path: format!("forensic_packet.{}", violation.path),
                message: violation.message,
            });
        }

        violations
    }

    /// Returns true when the packet remains metadata-safe and bounded.
    pub fn is_export_safe(&self) -> bool {
        self.validate().is_empty() && self.forensic_packet.is_export_safe()
    }
}

/// Returns the canonical seeded M5 host-failure drill packet.
pub fn seeded_m5_host_failure_drill_packet() -> M5HostFailureDrillPacket {
    let forensic_packet = seeded_m5_forensic_packet();
    M5HostFailureDrillPacket {
        record_kind: M5_HOST_FAILURE_DRILL_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_HOST_FAILURE_DRILL_SCHEMA_VERSION,
        packet_id: "support.m5.host_failure_drills.v1".to_owned(),
        generated_at: "2026-06-13T00:05:00Z".to_owned(),
        doc_ref: M5_HOST_FAILURE_DRILL_DOC_REF.to_owned(),
        schema_ref: M5_HOST_FAILURE_DRILL_SCHEMA_REF.to_owned(),
        artifact_ref: M5_HOST_FAILURE_DRILL_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".plans/M05-248.md".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#5210".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#5454".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#4811".to_owned(),
            "docs/runtime/fault_domains_and_restart_policy.md#forensic-packet-requirements"
                .to_owned(),
        ],
        supporting_contract_refs: vec![
            "schemas/support/m5-fault-crash-governance.schema.json".to_owned(),
            "schemas/support/crash_store_viewer.schema.json".to_owned(),
            "schemas/support/depth-surface-schema-registry.schema.json".to_owned(),
            "schemas/support/recovery_review.schema.json".to_owned(),
            M5_FORENSIC_PACKET_SCHEMA_REF.to_owned(),
        ],
        forensic_packet,
        drills: seed_drills(),
        export_safe_summary: "Seeded M5 host-failure drills cover restart-budget enforcement, scoped failure, checkpoint preservation, export-safe forensics, and no-silent-upload rules across every claimed host family.".to_owned(),
    }
}

fn seed_drills() -> Vec<HostFailureDrillRow> {
    vec![
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::NotebookKernelCrashStall,
            scenario_id: "scenario:notebook_kernel_crash_stall",
            host_family_id: "notebook_kernel_host",
            host_family_label: "Notebook kernel session",
            restart_budget_ref: "restart-budget:notebook-kernel:session-scoped",
            forensic_packet_ref: "forensic:notebook-kernel-crash-stall",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/notebook_kernel_crash_stall.json",
            summary: "Notebook kernel stall preserves the last cell-run checkpoint, narrows only the kernel lane, forbids hidden rerun, and keeps crash/support export local-first.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::ProviderRunFailure,
            scenario_id: "scenario:provider_run_failure",
            host_family_id: "provider_run_session_host",
            host_family_label: "Provider-backed run session",
            restart_budget_ref: "restart-budget:provider-run:privileged-external",
            forensic_packet_ref: "forensic:provider-run-failure",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/provider_run_failure.json",
            summary: "Provider-run failure revokes authority, scopes failure to the provider lane, preserves ticket metadata, and requires explicit reviewed upload for any shared packet.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::PreviewServerRestart,
            scenario_id: "scenario:preview_server_restart",
            host_family_id: "preview_dev_server_host",
            host_family_label: "Preview dev server",
            restart_budget_ref: "restart-budget:preview-server:session-scoped",
            forensic_packet_ref: "forensic:preview-dev-server-restart",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/preview_server_restart.json",
            summary: "Preview-server restart exhausts only the dev-server budget, preserves route metadata, and keeps mirrored-symbol lookup explicit.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::RemoteConnectorDrift,
            scenario_id: "scenario:remote_connector_drift",
            host_family_id: "data_api_connector_host",
            host_family_label: "Data/API connector and query runtime",
            restart_budget_ref: "restart-budget:data-api:remote-connector",
            forensic_packet_ref: "forensic:data-api-connector-drift",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/remote_connector_drift.json",
            summary: "Remote connector drift narrows only the connector lane, keeps request metadata checkpointed, and preserves imported-vs-live truth in export.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::AiBrokerCircuitBreaker,
            scenario_id: "scenario:ai_broker_circuit_breaker",
            host_family_id: "provider_run_session_host",
            host_family_label: "Provider-backed run session",
            restart_budget_ref: "restart-budget:provider-broker:circuit-breaker",
            forensic_packet_ref: "forensic:provider-broker-circuit-breaker",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/ai_broker_circuit_breaker.json",
            summary: "AI broker breaker-open drills prove scoped failure, explicit replay boundaries, and no silent export broadening when provider egress narrows.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::QueryRuntimeCrash,
            scenario_id: "scenario:query_runtime_crash",
            host_family_id: "query_runtime_host",
            host_family_label: "Query/request runtime",
            restart_budget_ref: "restart-budget:query-runtime:session-scoped",
            forensic_packet_ref: "forensic:query-runtime-crash",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/query_runtime_crash.json",
            summary: "Query/runtime crash isolates to the request lane, preserves reviewed request metadata, and blocks hidden rerun behind an explicit restart path.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::PipelineViewerFault,
            scenario_id: "scenario:pipeline_viewer_fault",
            host_family_id: "pipeline_viewer_host",
            host_family_label: "Pipeline viewer session",
            restart_budget_ref: "restart-budget:pipeline-viewer:remote-connector",
            forensic_packet_ref: "forensic:pipeline-viewer-fault",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/pipeline_viewer_fault.json",
            summary: "Pipeline-viewer fault keeps imported event lineage explicit and proves reconnect loops do not silently rerun or widen export scope.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::ConnectorHostMismatch,
            scenario_id: "scenario:connector_host_mismatch",
            host_family_id: "registry_database_connector_host",
            host_family_label: "Registry or database connector",
            restart_budget_ref: "restart-budget:registry-connector:privileged-external",
            forensic_packet_ref: "forensic:registry-connector-host-mismatch",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/connector_host_mismatch.json",
            summary: "Connector host mismatch forces visible fail-closed state, preserves target metadata, and keeps upload review explicit.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::DocsBrowserBridgeRouteDrift,
            scenario_id: "scenario:docs_browser_bridge_route_drift",
            host_family_id: "docs_browser_bridge_host",
            host_family_label: "Docs and browser bridge",
            restart_budget_ref: "restart-budget:docs-browser-bridge:remote-connector",
            forensic_packet_ref: "forensic:docs-browser-bridge-drift",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/docs_browser_bridge_route_drift.json",
            summary: "Docs/browser bridge drift keeps imported route facts explicit and preserves a local-only handoff path.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::ProfilerReplayImportedGap,
            scenario_id: "scenario:profiler_replay_imported_gap",
            host_family_id: "profiler_replay_session_host",
            host_family_label: "Profiler and replay session",
            restart_budget_ref: "restart-budget:profiler-replay:session-scoped",
            forensic_packet_ref: "forensic:profiler-replay-imported-gap",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/profiler_replay_imported_gap.json",
            summary: "Profiler/replay imported-gap drill keeps partial mapping quality visible and export-safe without forced upload.",
        }),
        seed_drill(SeededDrillInput {
            scenario_class: HostFailureScenarioClass::InfraHelperSignatureFailure,
            scenario_id: "scenario:infra_helper_signature_failure",
            host_family_id: "infra_helper_job",
            host_family_label: "Infrastructure helper",
            restart_budget_ref: "restart-budget:infra-helper:authority-verifier",
            forensic_packet_ref: "forensic:infra-helper-signature-failure",
            evidence_fixture_ref:
                "fixtures/support/m5/host_failure_drills/infra_helper_signature_failure.json",
            summary: "Infrastructure helper signature failure proves fail-closed restart policy, cache-snapshot checkpoint preservation, and explicit mirror review.",
        }),
    ]
}

struct SeededDrillInput<'a> {
    scenario_class: HostFailureScenarioClass,
    scenario_id: &'a str,
    host_family_id: &'a str,
    host_family_label: &'a str,
    restart_budget_ref: &'a str,
    forensic_packet_ref: &'a str,
    evidence_fixture_ref: &'a str,
    summary: &'a str,
}

fn seed_drill(input: SeededDrillInput<'_>) -> HostFailureDrillRow {
    HostFailureDrillRow {
        scenario_id: input.scenario_id.to_owned(),
        scenario_class: input.scenario_class,
        host_family_id: input.host_family_id.to_owned(),
        host_family_label: input.host_family_label.to_owned(),
        restart_budget_ref: input.restart_budget_ref.to_owned(),
        restart_budget_enforced: true,
        scoped_failure_only: true,
        checkpoint_preserved: true,
        no_hidden_rerun: true,
        visible_fail_closed_state: true,
        forensic_packet_ref: input.forensic_packet_ref.to_owned(),
        support_packet_refs: vec![
            "schemas/support/m5-fault-crash-governance.schema.json".to_owned(),
            "schemas/support/crash_store_viewer.schema.json".to_owned(),
            "schemas/support/depth-surface-schema-registry.schema.json".to_owned(),
            "schemas/support/recovery_review.schema.json".to_owned(),
        ],
        upload_guard_assertions: vec![
            "local_preview_precedes_export".to_owned(),
            "explicit_user_or_policy_action_required_for_egress".to_owned(),
            "retention_scope_change_requires_review".to_owned(),
        ],
        evidence_fixture_ref: input.evidence_fixture_ref.to_owned(),
        summary: input.summary.to_owned(),
    }
}

/// Returns the forensic row for a given scenario id, if present in the seeded packet.
pub fn seeded_forensic_row(scenario_id: &str) -> Option<M5ForensicRow> {
    seeded_m5_forensic_packet()
        .rows
        .into_iter()
        .find(|row| row.scenario_id == scenario_id)
}
