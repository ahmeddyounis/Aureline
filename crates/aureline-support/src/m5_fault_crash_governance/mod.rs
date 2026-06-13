//! M5 fault-domain, crash-forensics, and diagnostics-schema governance.
//!
//! This module freezes the canonical M5 host-failure contract so notebook,
//! data/API, preview, provider, profiler/replay, pipeline, browser/docs, and
//! infrastructure helper hosts all consume one typed vocabulary for:
//!
//! - fault-domain classes and restart classes,
//! - checkpoint and rehydrate sources,
//! - quarantine triggers and minimum diagnostic exports,
//! - crash-artifact and exact-build symbolication requirements,
//! - diagnostics-schema and consent posture for crash, performance, usage, and
//!   support signals, and
//! - downgrade rules that narrow claims when restart, crash, or schema proof
//!   goes stale.
//!
//! The packet composes existing supportability and crash contracts rather than
//! inventing parallel names. It quotes the supervised-restart evidence packet,
//! the hardened crash-capture packet, and the telemetry/support schema
//! registry by repository ref so downstream consumers can reuse one governance
//! surface.

use serde::{Deserialize, Serialize};

use crate::crash_store::{CRASH_STORE_VIEWER_DOC_REF, CRASH_STORE_VIEWER_SCHEMA_REF};
use crate::supervised_restart_evidence_pipeline::{
    SUPERVISED_RESTART_EVIDENCE_PIPELINE_DOC_REF, SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF,
};
use aureline_crash::{HARDEN_CRASH_CAPTURE_DOC_REF, HARDEN_CRASH_CAPTURE_SCHEMA_REF};

/// Frozen schema version for the M5 governance packet.
pub const M5_FAULT_CRASH_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the M5 governance packet.
pub const M5_FAULT_CRASH_GOVERNANCE_PACKET_RECORD_KIND: &str = "m5_fault_crash_governance_packet";

/// Repository-relative path of the boundary schema.
pub const M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/support/m5-fault-crash-governance.schema.json";

/// Repository-relative path of the reviewer-facing help document.
pub const M5_FAULT_CRASH_GOVERNANCE_DOC_REF: &str =
    "docs/help/support/m5-fault-crash-governance.md";

/// Repository-relative path of the review artifact.
pub const M5_FAULT_CRASH_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/support/m5/fault-crash-governance.md";

/// Repository-relative path of the fixture corpus.
pub const M5_FAULT_CRASH_GOVERNANCE_FIXTURE_DIR: &str =
    "fixtures/support/m5/fault_crash_governance";

const TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF: &str =
    "artifacts/governance/telemetry_support_usage_schema_registry.json";
const CONSENT_LEDGER_REF: &str = "artifacts/governance/consent_ledger_seed.yaml";
const PERFORMANCE_SUPPORT_SCHEMA_REF: &str =
    "schemas/perf/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.schema.json";

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

/// Closed vocabulary for the M5 fault-domain classes frozen by Appendix CD.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultDomainClass {
    /// One shell process per app instance.
    ShellInteractionCore,
    /// Per-workspace watcher, search, graph, and language cohort.
    WorkspaceKnowledgeGroup,
    /// Per task, test, debug, PTY, notebook, or provider-run session.
    SessionExecutionHost,
    /// Per extension host pool, package, or external adapter family.
    ExtensionOrExternalToolHost,
    /// Per provider route, model worker, or gateway host.
    AiToolBroker,
    /// Per target, session, relay, or connector path.
    RemoteConnector,
    /// Per fetch, verify, or local-authority helper job.
    PolicyVerifierHelper,
}

impl FaultDomainClass {
    /// All fault-domain classes in canonical order.
    pub const ALL: [Self; 7] = [
        Self::ShellInteractionCore,
        Self::WorkspaceKnowledgeGroup,
        Self::SessionExecutionHost,
        Self::ExtensionOrExternalToolHost,
        Self::AiToolBroker,
        Self::RemoteConnector,
        Self::PolicyVerifierHelper,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellInteractionCore => "shell_interaction_core",
            Self::WorkspaceKnowledgeGroup => "workspace_knowledge_group",
            Self::SessionExecutionHost => "session_execution_host",
            Self::ExtensionOrExternalToolHost => "extension_or_external_tool_host",
            Self::AiToolBroker => "ai_tool_broker",
            Self::RemoteConnector => "remote_connector",
            Self::PolicyVerifierHelper => "policy_verifier_helper",
        }
    }
}

/// Closed vocabulary for the restart classes frozen by Appendix CD.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartClass {
    /// Small helper with an open-circuit downgrade posture.
    StatelessHelper,
    /// Workspace graph/search/language cohort with rebuild from source truth.
    WorkspaceKnowledge,
    /// Task, debug, notebook, or PTY session that fails locally.
    SessionScoped,
    /// Privileged or externally mutating host requiring explicit reapproval.
    PrivilegedExternallyMutating,
    /// Authority or verifier helper that fails closed.
    AuthorityVerifier,
}

impl RestartClass {
    /// All restart classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::StatelessHelper,
        Self::WorkspaceKnowledge,
        Self::SessionScoped,
        Self::PrivilegedExternallyMutating,
        Self::AuthorityVerifier,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatelessHelper => "stateless_helper",
            Self::WorkspaceKnowledge => "workspace_knowledge",
            Self::SessionScoped => "session_scoped",
            Self::PrivilegedExternallyMutating => "privileged_externally_mutating",
            Self::AuthorityVerifier => "authority_verifier",
        }
    }
}

/// Closed vocabulary for checkpoint and rehydrate sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointSourceClass {
    /// Restore from autosave, session-restore, and last-known-good binary.
    SessionRestoreCheckpoint,
    /// Rebuild from VFS truth, graph shards, and query epochs.
    VfsTruthAndDisposableCaches,
    /// Restore session metadata only; never silently rerun commands.
    ExplicitRerunOrMetadataRestore,
    /// Extension manifest plus restart-clean state only.
    ManifestRestartCleanState,
    /// Provider/model registry entry plus allowed local cache.
    ProviderRegistryAndAllowedLocalCache,
    /// Reconnect token plus target metadata and safe session-state replay.
    ReconnectTokenAndTargetMetadata,
    /// Signed bundle cache or last-known-good authority snapshot.
    SignedBundleCacheSnapshot,
}

impl CheckpointSourceClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionRestoreCheckpoint => "session_restore_checkpoint",
            Self::VfsTruthAndDisposableCaches => "vfs_truth_and_disposable_caches",
            Self::ExplicitRerunOrMetadataRestore => "explicit_rerun_or_metadata_restore",
            Self::ManifestRestartCleanState => "manifest_restart_clean_state",
            Self::ProviderRegistryAndAllowedLocalCache => {
                "provider_registry_and_allowed_local_cache"
            }
            Self::ReconnectTokenAndTargetMetadata => "reconnect_token_and_target_metadata",
            Self::SignedBundleCacheSnapshot => "signed_bundle_cache_snapshot",
        }
    }
}

/// Closed vocabulary for crash-artifact classes frozen by Appendix CK.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashArtifactClass {
    /// Metadata-only crash envelope naming build, session, and fault domain.
    CrashEnvelope,
    /// Raw crash bytes retained locally by default.
    MinidumpOrCoreArtifact,
    /// Exact-build symbol or source-map manifest.
    SymbolOrSourceMapManifest,
    /// Local symbolication report tied to one build identity.
    LocalSymbolicationReport,
    /// Optional mirrored symbol service with explicit access policy.
    MirroredSymbolService,
}

impl CrashArtifactClass {
    /// All crash-artifact classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::CrashEnvelope,
        Self::MinidumpOrCoreArtifact,
        Self::SymbolOrSourceMapManifest,
        Self::LocalSymbolicationReport,
        Self::MirroredSymbolService,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashEnvelope => "crash_envelope",
            Self::MinidumpOrCoreArtifact => "minidump_or_core_artifact",
            Self::SymbolOrSourceMapManifest => "symbol_or_source_map_manifest",
            Self::LocalSymbolicationReport => "local_symbolication_report",
            Self::MirroredSymbolService => "mirrored_symbol_service",
        }
    }
}

/// Closed vocabulary for governed diagnostic signal classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSignalClass {
    /// Crash and panic capture.
    Crash,
    /// Profile, trace, and performance capture.
    Performance,
    /// Metering and usage export.
    Usage,
    /// User-initiated support export.
    Support,
}

impl DiagnosticSignalClass {
    /// All diagnostic signal classes in canonical order.
    pub const ALL: [Self; 4] = [Self::Crash, Self::Performance, Self::Usage, Self::Support];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Crash => "crash",
            Self::Performance => "performance",
            Self::Usage => "usage",
            Self::Support => "support",
        }
    }
}

/// Closed diagnostic data-class vocabulary aligned to the shared UX contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDataClass {
    /// Build IDs, counters, feature flags, and policy fingerprints.
    MetadataOnly,
    /// Toolchain and environment summaries without raw machine state.
    EnvironmentAdjacent,
    /// Stack traces, snippets, cell excerpts, or source-linked profiles.
    CodeAdjacent,
    /// Secret-bearing or otherwise high-risk content.
    HighRisk,
}

impl DiagnosticDataClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::EnvironmentAdjacent => "environment_adjacent",
            Self::CodeAdjacent => "code_adjacent",
            Self::HighRisk => "high_risk",
        }
    }
}

/// Closed diagnostic opt-in and consent vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticOptInScope {
    /// Local by default until explicit submission or policy-gated upload.
    ExplicitSubmissionOrPolicy,
    /// Disabled until the user opts in to sampled collection.
    OptInOnly,
    /// Produced only when the user explicitly starts an export.
    UserInitiatedExportOnly,
    /// Managed export controlled by admin policy.
    AdminPolicyGated,
}

impl DiagnosticOptInScope {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitSubmissionOrPolicy => "explicit_submission_or_policy",
            Self::OptInOnly => "opt_in_only",
            Self::UserInitiatedExportOnly => "user_initiated_export_only",
            Self::AdminPolicyGated => "admin_policy_gated",
        }
    }
}

/// Closed retention vocabulary for the governed diagnostic families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Local evidence owned by the user until deletion or case attachment.
    LocalUserOwnedUntilDeleteOrAttach,
    /// Local sampled aggregate retained for a bounded metrics window.
    LocalSamplingWindow,
    /// Managed or contractual export retention window.
    ManagedContractWindow,
    /// Local manifest retained until the user sends or clears it.
    LocalManifestUntilSent,
}

impl RetentionClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUserOwnedUntilDeleteOrAttach => "local_user_owned_until_delete_or_attach",
            Self::LocalSamplingWindow => "local_sampling_window",
            Self::ManagedContractWindow => "managed_contract_window",
            Self::LocalManifestUntilSent => "local_manifest_until_sent",
        }
    }
}

/// Closed redaction-profile vocabulary for the governed diagnostic families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionProfileClass {
    /// Metadata-only default export posture.
    MetadataSafeDefault,
    /// Environment facts summarized without raw machine state.
    EnvironmentSummaryOnly,
    /// Restricted operator view with extra code-adjacent narrowing.
    OperatorOnlyRestricted,
    /// Local-only reviewed step required before any broader export.
    LocalOnlyReviewRequired,
}

impl RedactionProfileClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::EnvironmentSummaryOnly => "environment_summary_only",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::LocalOnlyReviewRequired => "local_only_review_required",
        }
    }
}

/// Closed claim-state vocabulary for governed M5 host families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimStateClass {
    /// Fresh host-failure, crash, and schema-governance proof is present.
    Qualified,
    /// The row remains available but narrows below stable depth.
    NarrowedPreview,
    /// Only local-only or inspect-only truth may be claimed.
    NarrowedLocalOnly,
    /// The host row is blocked from claiming readiness.
    BlockedUnverified,
}

impl ClaimStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::NarrowedPreview => "narrowed_preview",
            Self::NarrowedLocalOnly => "narrowed_local_only",
            Self::BlockedUnverified => "blocked_unverified",
        }
    }
}

/// Closed downgrade-trigger vocabulary for stale or missing proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTriggerClass {
    /// Restart lineage, strike-window, or quarantine evidence is stale.
    RestartEvidenceStale,
    /// Quarantine cause or minimum-export proof is missing.
    QuarantineProofMissing,
    /// Crash artifact lineage or local capture proof is stale.
    CrashArtifactProofStale,
    /// Symbolication is absent or not exact-build.
    SymbolicationNotExactBuild,
    /// Schema, consent, or endpoint posture proof is stale.
    DiagnosticSchemaStale,
    /// Redaction review or consent review no longer matches shipped posture.
    ConsentOrRedactionReviewStale,
}

impl DowngradeTriggerClass {
    /// All downgrade triggers in canonical order.
    pub const ALL: [Self; 6] = [
        Self::RestartEvidenceStale,
        Self::QuarantineProofMissing,
        Self::CrashArtifactProofStale,
        Self::SymbolicationNotExactBuild,
        Self::DiagnosticSchemaStale,
        Self::ConsentOrRedactionReviewStale,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestartEvidenceStale => "restart_evidence_stale",
            Self::QuarantineProofMissing => "quarantine_proof_missing",
            Self::CrashArtifactProofStale => "crash_artifact_proof_stale",
            Self::SymbolicationNotExactBuild => "symbolication_not_exact_build",
            Self::DiagnosticSchemaStale => "diagnostic_schema_stale",
            Self::ConsentOrRedactionReviewStale => "consent_or_redaction_review_stale",
        }
    }
}

/// One canonical fault-domain row in the M5 governance matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultDomainMatrixRow {
    /// Closed fault-domain class token.
    pub fault_domain_class: FaultDomainClass,
    /// Human-readable isolation unit frozen for the domain.
    pub isolation_unit: String,
    /// Restart class inherited by the domain.
    pub restart_class: RestartClass,
    /// Checkpoint or rehydrate source for the domain.
    pub checkpoint_source: CheckpointSourceClass,
    /// Typed triggers that quarantine or hard-stop the domain.
    pub quarantine_or_hard_stop_triggers: Vec<String>,
    /// Minimum metadata-safe exports downstream consumers must preserve.
    pub minimum_diagnostic_exports: Vec<String>,
    /// Repository refs that currently substantiate the row.
    pub supporting_contract_refs: Vec<String>,
}

/// One canonical restart-class row in the M5 governance matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartClassRow {
    /// Closed restart-class token.
    pub restart_class: RestartClass,
    /// Fault-domain classes that commonly use this restart class.
    pub typical_fault_domains: Vec<FaultDomainClass>,
    /// Default strike window in minutes.
    pub default_strike_window_minutes: u32,
    /// Default automatic restarts permitted in that window.
    pub default_automatic_restarts: u32,
    /// Reviewable escalation posture quoted in docs and exports.
    pub escalation_posture: String,
    /// True when retries may never silently broaden authority.
    pub hidden_authority_widening_forbidden: bool,
}

/// One crash-artifact governance row frozen by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashArtifactGovernanceRow {
    /// Closed crash-artifact class token.
    pub artifact_class: CrashArtifactClass,
    /// Default storage or publication location.
    pub default_location: String,
    /// Highest-risk data class this artifact may carry.
    pub high_risk_content_class: DiagnosticDataClass,
    /// Companion metadata fields downstream readers require.
    pub required_companion_metadata: Vec<String>,
    /// True when the artifact stays local until reviewed export.
    pub local_first_by_default: bool,
    /// True when exact-build identity is mandatory for use.
    pub exact_build_only: bool,
    /// True when raw or derived bytes may not upload automatically.
    pub auto_upload_forbidden: bool,
    /// True when enterprise mirrors may host the artifact family explicitly.
    pub mirrorable_when_claimed: bool,
    /// Repository refs substantiating the row.
    pub supporting_contract_refs: Vec<String>,
}

/// One governed diagnostic-schema row for crash, perf, usage, or support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSchemaGovernanceRow {
    /// Closed signal class token.
    pub signal_class: DiagnosticSignalClass,
    /// Stable schema identifier quoted in docs, exports, and review packets.
    pub schema_id: String,
    /// Repository schema path consumed by the family.
    pub schema_ref: String,
    /// Reviewable purpose sentence.
    pub purpose: String,
    /// Shared diagnostic data-class label for the family.
    pub data_class: DiagnosticDataClass,
    /// Consent or opt-in posture.
    pub opt_in_scope: DiagnosticOptInScope,
    /// Content classes forbidden by default.
    pub prohibited_content_classes: Vec<String>,
    /// Retention class for the family.
    pub retention_class: RetentionClass,
    /// Redaction profile applied by default.
    pub redaction_profile: RedactionProfileClass,
    /// Repository refs substantiating the row.
    pub evidence_source_refs: Vec<String>,
}

/// One host-family row covered by the M5 governance freeze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostFamilyGovernanceRow {
    /// Stable machine-readable host-family id.
    pub host_family_id: String,
    /// Human-readable host-family label.
    pub host_family_label: String,
    /// Governing fault-domain class.
    pub fault_domain_class: FaultDomainClass,
    /// Restart class applied to the family.
    pub restart_class: RestartClass,
    /// Default strike window in minutes for the family.
    pub default_strike_window_minutes: u32,
    /// Default automatic restart budget for the family.
    pub default_automatic_restarts: u32,
    /// Checkpoint or rehydrate source used by the family.
    pub checkpoint_source: CheckpointSourceClass,
    /// Typed quarantine triggers the family must surface.
    pub quarantine_trigger_classes: Vec<String>,
    /// Minimum metadata-safe exports the family must preserve.
    pub minimum_diagnostic_exports: Vec<String>,
    /// Crash artifacts the family must bind or reference.
    pub required_crash_artifacts: Vec<CrashArtifactClass>,
    /// Diagnostic signal families the host must remain compatible with.
    pub required_diagnostic_signal_classes: Vec<DiagnosticSignalClass>,
    /// Existing supportability and crash packets this host reuses.
    pub canonical_support_packet_refs: Vec<String>,
    /// Current maturity state after applying any downgrade rules.
    pub claim_state: ClaimStateClass,
    /// Proof tokens explaining why a row narrowed below qualified.
    pub stale_proof_tokens: Vec<String>,
    /// Downgrade rules applicable to this host family.
    pub downgrade_rule_ids: Vec<String>,
}

/// One downgrade rule narrowing stale host, crash, or schema proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeRuleRow {
    /// Stable rule id.
    pub rule_id: String,
    /// Scope or lane family the rule applies to.
    pub applies_to: String,
    /// Closed trigger token.
    pub trigger_class: DowngradeTriggerClass,
    /// State before the downgrade applies.
    pub source_claim_state: ClaimStateClass,
    /// Narrowed state after the downgrade applies.
    pub downgraded_claim_state: ClaimStateClass,
    /// Required user-visible effect after the downgrade.
    pub required_effect: String,
    /// Reviewable rationale for the rule.
    pub rationale: String,
    /// Refs reviewers use to inspect the rule's inputs.
    pub evidence_refs: Vec<String>,
}

/// One validation error reported by [`M5FaultCrashGovernancePacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FaultCrashGovernanceViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 host-failure and crash-forensics governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FaultCrashGovernancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id.
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
    /// Canonical fault-domain rows.
    pub fault_domains: Vec<FaultDomainMatrixRow>,
    /// Canonical restart-class rows.
    pub restart_classes: Vec<RestartClassRow>,
    /// Canonical crash-artifact rows.
    pub crash_artifacts: Vec<CrashArtifactGovernanceRow>,
    /// Canonical diagnostic-schema rows.
    pub diagnostic_schemas: Vec<DiagnosticSchemaGovernanceRow>,
    /// Canonical M5 host-family rows.
    pub host_families: Vec<HostFamilyGovernanceRow>,
    /// Canonical downgrade rules.
    pub downgrade_rules: Vec<DowngradeRuleRow>,
    /// Metadata-safe summary for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5FaultCrashGovernancePacket {
    /// Validates the packet's closed-vocabulary coverage and invariants.
    pub fn validate(&self) -> Vec<M5FaultCrashGovernanceViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_FAULT_CRASH_GOVERNANCE_PACKET_RECORD_KIND {
            violations.push(M5FaultCrashGovernanceViolation {
                path: "record_kind".to_owned(),
                message: "unexpected record_kind".to_owned(),
            });
        }
        if self.schema_version != M5_FAULT_CRASH_GOVERNANCE_SCHEMA_VERSION {
            violations.push(M5FaultCrashGovernanceViolation {
                path: "schema_version".to_owned(),
                message: "unexpected schema_version".to_owned(),
            });
        }
        if self.doc_ref != M5_FAULT_CRASH_GOVERNANCE_DOC_REF {
            violations.push(M5FaultCrashGovernanceViolation {
                path: "doc_ref".to_owned(),
                message: "packet must quote the canonical reviewer doc".to_owned(),
            });
        }
        if self.schema_ref != M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF {
            violations.push(M5FaultCrashGovernanceViolation {
                path: "schema_ref".to_owned(),
                message: "packet must quote the canonical schema ref".to_owned(),
            });
        }

        let fault_domain_tokens = self
            .fault_domains
            .iter()
            .map(|row| row.fault_domain_class.as_str())
            .collect::<Vec<_>>();
        for required in FaultDomainClass::ALL {
            if !fault_domain_tokens.contains(&required.as_str()) {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "fault_domains".to_owned(),
                    message: format!("missing fault-domain class {}", required.as_str()),
                });
            }
        }

        let restart_class_tokens = self
            .restart_classes
            .iter()
            .map(|row| row.restart_class.as_str())
            .collect::<Vec<_>>();
        for required in RestartClass::ALL {
            if !restart_class_tokens.contains(&required.as_str()) {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "restart_classes".to_owned(),
                    message: format!("missing restart class {}", required.as_str()),
                });
            }
        }

        let crash_artifact_tokens = self
            .crash_artifacts
            .iter()
            .map(|row| row.artifact_class.as_str())
            .collect::<Vec<_>>();
        for required in CrashArtifactClass::ALL {
            if !crash_artifact_tokens.contains(&required.as_str()) {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "crash_artifacts".to_owned(),
                    message: format!("missing crash artifact {}", required.as_str()),
                });
            }
        }

        let signal_tokens = self
            .diagnostic_schemas
            .iter()
            .map(|row| row.signal_class.as_str())
            .collect::<Vec<_>>();
        for required in DiagnosticSignalClass::ALL {
            if !signal_tokens.contains(&required.as_str()) {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "diagnostic_schemas".to_owned(),
                    message: format!("missing diagnostic signal class {}", required.as_str()),
                });
            }
        }

        for required in REQUIRED_HOST_FAMILY_IDS {
            if !self
                .host_families
                .iter()
                .any(|row| row.host_family_id == *required)
            {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "host_families".to_owned(),
                    message: format!("missing required M5 host family {required}"),
                });
            }
        }

        for row in &self.crash_artifacts {
            if !row.auto_upload_forbidden {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!("crash_artifacts.{}", row.artifact_class.as_str()),
                    message: "crash artifacts may not auto-upload".to_owned(),
                });
            }
            if row.required_companion_metadata.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!("crash_artifacts.{}", row.artifact_class.as_str()),
                    message: "crash artifact row must declare companion metadata".to_owned(),
                });
            }
        }

        for row in &self.diagnostic_schemas {
            if row.prohibited_content_classes.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!("diagnostic_schemas.{}", row.signal_class.as_str()),
                    message: "diagnostic schema row must list prohibited content classes"
                        .to_owned(),
                });
            }
            if matches!(row.signal_class, DiagnosticSignalClass::Support)
                && row.opt_in_scope != DiagnosticOptInScope::UserInitiatedExportOnly
            {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "diagnostic_schemas.support".to_owned(),
                    message: "support exports must remain user-initiated".to_owned(),
                });
            }
            if matches!(row.signal_class, DiagnosticSignalClass::Usage)
                && row.opt_in_scope != DiagnosticOptInScope::AdminPolicyGated
            {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "diagnostic_schemas.usage".to_owned(),
                    message: "usage export must remain admin-policy-gated".to_owned(),
                });
            }
        }

        for row in &self.host_families {
            if row.minimum_diagnostic_exports.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!(
                        "host_families.{}.minimum_diagnostic_exports",
                        row.host_family_id
                    ),
                    message: "host family must preserve minimum diagnostic exports".to_owned(),
                });
            }
            if row.required_crash_artifacts.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!(
                        "host_families.{}.required_crash_artifacts",
                        row.host_family_id
                    ),
                    message: "host family must reference crash artifacts".to_owned(),
                });
            }
            if row.required_diagnostic_signal_classes.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!(
                        "host_families.{}.required_diagnostic_signal_classes",
                        row.host_family_id
                    ),
                    message: "host family must reference governed diagnostic signal classes"
                        .to_owned(),
                });
            }
            if row.canonical_support_packet_refs.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!(
                        "host_families.{}.canonical_support_packet_refs",
                        row.host_family_id
                    ),
                    message: "host family must cite canonical supportability packets".to_owned(),
                });
            }
            if row.claim_state == ClaimStateClass::Qualified && !row.stale_proof_tokens.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!("host_families.{}.claim_state", row.host_family_id),
                    message: "qualified rows may not carry stale proof tokens".to_owned(),
                });
            }
            if row.claim_state != ClaimStateClass::Qualified && row.downgrade_rule_ids.is_empty() {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: format!("host_families.{}.downgrade_rule_ids", row.host_family_id),
                    message: "narrowed rows must cite downgrade rules".to_owned(),
                });
            }
        }

        let downgrade_tokens = self
            .downgrade_rules
            .iter()
            .map(|row| row.trigger_class.as_str())
            .collect::<Vec<_>>();
        for required in [
            DowngradeTriggerClass::RestartEvidenceStale,
            DowngradeTriggerClass::CrashArtifactProofStale,
            DowngradeTriggerClass::SymbolicationNotExactBuild,
            DowngradeTriggerClass::DiagnosticSchemaStale,
        ] {
            if !downgrade_tokens.contains(&required.as_str()) {
                violations.push(M5FaultCrashGovernanceViolation {
                    path: "downgrade_rules".to_owned(),
                    message: format!("missing downgrade trigger {}", required.as_str()),
                });
            }
        }

        violations
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self
                .diagnostic_schemas
                .iter()
                .all(|row| !matches!(row.data_class, DiagnosticDataClass::HighRisk))
    }

    /// Renders a short plaintext summary for support and release review.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::from("M5 fault/crash governance packet\n");
        out.push_str(&format!("packet_id: {}\n", self.packet_id));
        out.push_str(&format!("generated_at: {}\n", self.generated_at));
        out.push_str("fault_domains:");
        for row in &self.fault_domains {
            out.push_str(&format!(" {}", row.fault_domain_class.as_str()));
        }
        out.push('\n');
        out.push_str("diagnostic_schemas:");
        for row in &self.diagnostic_schemas {
            out.push_str(&format!(" {}", row.schema_id));
        }
        out.push('\n');
        out.push_str("host_families:");
        for row in &self.host_families {
            out.push_str(&format!(" {}", row.host_family_id));
        }
        out.push('\n');
        out
    }
}

/// Returns the seeded M5 fault/crash governance packet used by tests and docs.
pub fn seeded_m5_fault_crash_governance_packet() -> M5FaultCrashGovernancePacket {
    M5FaultCrashGovernancePacket {
        record_kind: M5_FAULT_CRASH_GOVERNANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_FAULT_CRASH_GOVERNANCE_SCHEMA_VERSION,
        packet_id: "support.m5.fault_crash_governance.v1".to_owned(),
        generated_at: "2026-06-12T23:40:00Z".to_owned(),
        doc_ref: M5_FAULT_CRASH_GOVERNANCE_DOC_REF.to_owned(),
        schema_ref: M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Technical_Architecture_Document.md#appendix-cd".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#appendix-ck".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#7126-telemetry-schema-registry-and-consent-ledger".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#7127-crash-capture-minidump-symbolication-and-symbol-service".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md#2729-diagnostic-data-classes-redaction-policy-and-collection-schema-ux".to_owned(),
        ],
        supporting_contract_refs: vec![
            SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
            SUPERVISED_RESTART_EVIDENCE_PIPELINE_DOC_REF.to_owned(),
            HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
            HARDEN_CRASH_CAPTURE_DOC_REF.to_owned(),
            CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
            CRASH_STORE_VIEWER_DOC_REF.to_owned(),
            TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
            CONSENT_LEDGER_REF.to_owned(),
        ],
        normative_rules: vec![
            "worker failure stays local and legible; no host may inherit hidden shell-wide failure semantics".to_owned(),
            "restart budgets and strike windows remain typed, inspectable, and never silently widen authority on retry".to_owned(),
            "crash capture and symbolication stay local-first and exact-build; same-version guessing is forbidden".to_owned(),
            "diagnostic and telemetry flows are schema-governed, redaction-aware, and never silently upload protected content".to_owned(),
            "stale host-failure, crash-forensics, or diagnostics-schema proof narrows the claim instead of inheriting adjacent maturity".to_owned(),
        ],
        fault_domains: vec![
            FaultDomainMatrixRow {
                fault_domain_class: FaultDomainClass::ShellInteractionCore,
                isolation_unit: "one shell process per app instance".to_owned(),
                restart_class: RestartClass::AuthorityVerifier,
                checkpoint_source: CheckpointSourceClass::SessionRestoreCheckpoint,
                quarantine_or_hard_stop_triggers: vec![
                    "repeated_launch_crash".to_owned(),
                    "binary_integrity_failure".to_owned(),
                    "user_chosen_safe_mode".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "shell_version_build_id".to_owned(),
                    "crash_marker".to_owned(),
                    "window_session_lineage".to_owned(),
                    "last_restore_checkpoint".to_owned(),
                ],
                supporting_contract_refs: vec![SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned()],
            },
            FaultDomainMatrixRow {
                fault_domain_class: FaultDomainClass::WorkspaceKnowledgeGroup,
                isolation_unit: "per workspace/workset watcher-search-graph-language cohort".to_owned(),
                restart_class: RestartClass::WorkspaceKnowledge,
                checkpoint_source: CheckpointSourceClass::VfsTruthAndDisposableCaches,
                quarantine_or_hard_stop_triggers: vec![
                    "restart_budget_exceeded".to_owned(),
                    "repeated_corrupt_cache_detection".to_owned(),
                    "contract_version_mismatch".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "workspace_workset_id".to_owned(),
                    "worker_lineage".to_owned(),
                    "stale_rebuild_state".to_owned(),
                    "epoch_change_journal_refs".to_owned(),
                ],
                supporting_contract_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                ],
            },
            FaultDomainMatrixRow {
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                isolation_unit: "per task/test/debug/pty/notebook/provider-run session".to_owned(),
                restart_class: RestartClass::SessionScoped,
                checkpoint_source: CheckpointSourceClass::ExplicitRerunOrMetadataRestore,
                quarantine_or_hard_stop_triggers: vec![
                    "repeated_crash_or_stall".to_owned(),
                    "revoked_target_authority".to_owned(),
                    "lost_backend_contract".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "session_id".to_owned(),
                    "target_identity".to_owned(),
                    "command_family".to_owned(),
                    "envelope_profile_id".to_owned(),
                    "last_artifact_log_refs".to_owned(),
                ],
                supporting_contract_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
                ],
            },
            FaultDomainMatrixRow {
                fault_domain_class: FaultDomainClass::ExtensionOrExternalToolHost,
                isolation_unit: "per host pool, package, or adapter family".to_owned(),
                restart_class: RestartClass::StatelessHelper,
                checkpoint_source: CheckpointSourceClass::ManifestRestartCleanState,
                quarantine_or_hard_stop_triggers: vec![
                    "repeated_crash".to_owned(),
                    "ui_thread_abuse".to_owned(),
                    "quota_breach".to_owned(),
                    "manifest_abi_violation".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "package_id_version".to_owned(),
                    "host_lineage".to_owned(),
                    "budget_counters".to_owned(),
                    "quarantine_reason".to_owned(),
                    "manifest_refs".to_owned(),
                ],
                supporting_contract_refs: vec![SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned()],
            },
            FaultDomainMatrixRow {
                fault_domain_class: FaultDomainClass::AiToolBroker,
                isolation_unit: "per provider route, model worker, or gateway host".to_owned(),
                restart_class: RestartClass::PrivilegedExternallyMutating,
                checkpoint_source: CheckpointSourceClass::ProviderRegistryAndAllowedLocalCache,
                quarantine_or_hard_stop_triggers: vec![
                    "quota_exhaustion".to_owned(),
                    "repeated_side_effect_failure".to_owned(),
                    "ticket_revoke".to_owned(),
                    "trust_policy_deny".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "provider_model_id".to_owned(),
                    "tool_ids".to_owned(),
                    "breaker_state".to_owned(),
                    "ticket_lineage".to_owned(),
                    "evidence_packet_refs".to_owned(),
                ],
                supporting_contract_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                ],
            },
            FaultDomainMatrixRow {
                fault_domain_class: FaultDomainClass::RemoteConnector,
                isolation_unit: "per target/session/relay path".to_owned(),
                restart_class: RestartClass::StatelessHelper,
                checkpoint_source: CheckpointSourceClass::ReconnectTokenAndTargetMetadata,
                quarantine_or_hard_stop_triggers: vec![
                    "repeated_auth_failure".to_owned(),
                    "target_mismatch".to_owned(),
                    "incompatible_capability_window".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "target_uri_identity".to_owned(),
                    "transport_class".to_owned(),
                    "reconnect_lineage".to_owned(),
                    "last_good_capability_manifest".to_owned(),
                ],
                supporting_contract_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                ],
            },
            FaultDomainMatrixRow {
                fault_domain_class: FaultDomainClass::PolicyVerifierHelper,
                isolation_unit: "per fetch/verify/local-authority job".to_owned(),
                restart_class: RestartClass::AuthorityVerifier,
                checkpoint_source: CheckpointSourceClass::SignedBundleCacheSnapshot,
                quarantine_or_hard_stop_triggers: vec![
                    "signature_failure".to_owned(),
                    "epoch_rollback".to_owned(),
                    "signer_mismatch".to_owned(),
                    "replay_nonce_breach".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "bundle_id".to_owned(),
                    "signer_set".to_owned(),
                    "epoch_numbers".to_owned(),
                    "verification_failure_code".to_owned(),
                ],
                supporting_contract_refs: vec![CONSENT_LEDGER_REF.to_owned()],
            },
        ],
        restart_classes: vec![
            RestartClassRow {
                restart_class: RestartClass::StatelessHelper,
                typical_fault_domains: vec![
                    FaultDomainClass::ExtensionOrExternalToolHost,
                    FaultDomainClass::RemoteConnector,
                ],
                default_strike_window_minutes: 5,
                default_automatic_restarts: 5,
                escalation_posture: "Open circuit or disable the helper while keeping dependent surfaces explicit.".to_owned(),
                hidden_authority_widening_forbidden: true,
            },
            RestartClassRow {
                restart_class: RestartClass::WorkspaceKnowledge,
                typical_fault_domains: vec![FaultDomainClass::WorkspaceKnowledgeGroup],
                default_strike_window_minutes: 10,
                default_automatic_restarts: 3,
                escalation_posture: "Quarantine the workspace knowledge group and rebuild from source truth.".to_owned(),
                hidden_authority_widening_forbidden: true,
            },
            RestartClassRow {
                restart_class: RestartClass::SessionScoped,
                typical_fault_domains: vec![FaultDomainClass::SessionExecutionHost],
                default_strike_window_minutes: 10,
                default_automatic_restarts: 2,
                escalation_posture: "Fail the session only, preserve logs and artifacts, and require explicit rerun.".to_owned(),
                hidden_authority_widening_forbidden: true,
            },
            RestartClassRow {
                restart_class: RestartClass::PrivilegedExternallyMutating,
                typical_fault_domains: vec![
                    FaultDomainClass::AiToolBroker,
                    FaultDomainClass::RemoteConnector,
                ],
                default_strike_window_minutes: 10,
                default_automatic_restarts: 1,
                escalation_posture: "Revoke authority, require a fresh ticket, and force explicit retry or reapproval.".to_owned(),
                hidden_authority_widening_forbidden: true,
            },
            RestartClassRow {
                restart_class: RestartClass::AuthorityVerifier,
                typical_fault_domains: vec![
                    FaultDomainClass::ShellInteractionCore,
                    FaultDomainClass::PolicyVerifierHelper,
                ],
                default_strike_window_minutes: 10,
                default_automatic_restarts: 0,
                escalation_posture: "Fail closed for managed-only actions and keep the local core available.".to_owned(),
                hidden_authority_widening_forbidden: true,
            },
        ],
        crash_artifacts: vec![
            CrashArtifactGovernanceRow {
                artifact_class: CrashArtifactClass::CrashEnvelope,
                default_location: "local crash store".to_owned(),
                high_risk_content_class: DiagnosticDataClass::MetadataOnly,
                required_companion_metadata: vec![
                    "crash_id".to_owned(),
                    "build_id".to_owned(),
                    "fault_domain".to_owned(),
                    "policy_fingerprint".to_owned(),
                ],
                local_first_by_default: true,
                exact_build_only: true,
                auto_upload_forbidden: true,
                mirrorable_when_claimed: true,
                supporting_contract_refs: vec![HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned()],
            },
            CrashArtifactGovernanceRow {
                artifact_class: CrashArtifactClass::MinidumpOrCoreArtifact,
                default_location: "local crash store".to_owned(),
                high_risk_content_class: DiagnosticDataClass::CodeAdjacent,
                required_companion_metadata: vec![
                    "architecture".to_owned(),
                    "signal_or_exception_class".to_owned(),
                    "module_build_ids".to_owned(),
                    "dump_format".to_owned(),
                ],
                local_first_by_default: true,
                exact_build_only: true,
                auto_upload_forbidden: true,
                mirrorable_when_claimed: false,
                supporting_contract_refs: vec![HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned()],
            },
            CrashArtifactGovernanceRow {
                artifact_class: CrashArtifactClass::SymbolOrSourceMapManifest,
                default_location: "artifact graph, mirror, or symbol store".to_owned(),
                high_risk_content_class: DiagnosticDataClass::MetadataOnly,
                required_companion_metadata: vec![
                    "build_id".to_owned(),
                    "target_triple".to_owned(),
                    "debug_format".to_owned(),
                    "digest".to_owned(),
                    "retention_policy".to_owned(),
                ],
                local_first_by_default: true,
                exact_build_only: true,
                auto_upload_forbidden: true,
                mirrorable_when_claimed: true,
                supporting_contract_refs: vec![HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned()],
            },
            CrashArtifactGovernanceRow {
                artifact_class: CrashArtifactClass::LocalSymbolicationReport,
                default_location: "local diagnostics store".to_owned(),
                high_risk_content_class: DiagnosticDataClass::CodeAdjacent,
                required_companion_metadata: vec![
                    "symbol_sources_used".to_owned(),
                    "match_quality".to_owned(),
                    "unresolved_frames".to_owned(),
                    "redaction_profile".to_owned(),
                ],
                local_first_by_default: true,
                exact_build_only: true,
                auto_upload_forbidden: true,
                mirrorable_when_claimed: true,
                supporting_contract_refs: vec![HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned()],
            },
            CrashArtifactGovernanceRow {
                artifact_class: CrashArtifactClass::MirroredSymbolService,
                default_location: "regional or tenant-scoped symbol store".to_owned(),
                high_risk_content_class: DiagnosticDataClass::MetadataOnly,
                required_companion_metadata: vec![
                    "signer_identity".to_owned(),
                    "mirror_source".to_owned(),
                    "retention".to_owned(),
                    "access_policy".to_owned(),
                    "audit_trail".to_owned(),
                ],
                local_first_by_default: false,
                exact_build_only: true,
                auto_upload_forbidden: true,
                mirrorable_when_claimed: true,
                supporting_contract_refs: vec![HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned()],
            },
        ],
        diagnostic_schemas: vec![
            DiagnosticSchemaGovernanceRow {
                signal_class: DiagnosticSignalClass::Crash,
                schema_id: "diagnostics.crash_payload".to_owned(),
                schema_ref: "schemas/support/crash_diagnostic_payload.schema.json".to_owned(),
                purpose: "Crash and diagnostic payloads for local failure triage, optional submission, and support-safe review.".to_owned(),
                data_class: DiagnosticDataClass::CodeAdjacent,
                opt_in_scope: DiagnosticOptInScope::ExplicitSubmissionOrPolicy,
                prohibited_content_classes: vec![
                    "raw_source_code".to_owned(),
                    "prompt_contents".to_owned(),
                    "terminal_history".to_owned(),
                    "clipboard_history".to_owned(),
                    "credential_bodies".to_owned(),
                ],
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                redaction_profile: RedactionProfileClass::OperatorOnlyRestricted,
                evidence_source_refs: vec![
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CONSENT_LEDGER_REF.to_owned(),
                ],
            },
            DiagnosticSchemaGovernanceRow {
                signal_class: DiagnosticSignalClass::Performance,
                schema_id: "performance.trace_support_bundle".to_owned(),
                schema_ref: PERFORMANCE_SUPPORT_SCHEMA_REF.to_owned(),
                purpose: "Profile and trace artifacts attached to incident workspaces, AI explanations, and support bundles under reviewed export.".to_owned(),
                data_class: DiagnosticDataClass::CodeAdjacent,
                opt_in_scope: DiagnosticOptInScope::UserInitiatedExportOnly,
                prohibited_content_classes: vec![
                    "raw_source_code".to_owned(),
                    "full_environment_dump".to_owned(),
                    "credential_bodies".to_owned(),
                    "tenant_private_urls".to_owned(),
                ],
                retention_class: RetentionClass::LocalUserOwnedUntilDeleteOrAttach,
                redaction_profile: RedactionProfileClass::LocalOnlyReviewRequired,
                evidence_source_refs: vec![
                    PERFORMANCE_SUPPORT_SCHEMA_REF.to_owned(),
                    "artifacts/perf/m5/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.json".to_owned(),
                ],
            },
            DiagnosticSchemaGovernanceRow {
                signal_class: DiagnosticSignalClass::Usage,
                schema_id: "usage.metering_export_packet".to_owned(),
                schema_ref: "schemas/governance/usage_export_record.schema.json".to_owned(),
                purpose: "Managed billing, quota, and contract-facing usage export packets.".to_owned(),
                data_class: DiagnosticDataClass::MetadataOnly,
                opt_in_scope: DiagnosticOptInScope::AdminPolicyGated,
                prohibited_content_classes: vec![
                    "raw_prompt_contents".to_owned(),
                    "raw_query_text".to_owned(),
                    "credential_bodies".to_owned(),
                    "private_ticket_threads".to_owned(),
                ],
                retention_class: RetentionClass::ManagedContractWindow,
                redaction_profile: RedactionProfileClass::MetadataSafeDefault,
                evidence_source_refs: vec![
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CONSENT_LEDGER_REF.to_owned(),
                ],
            },
            DiagnosticSchemaGovernanceRow {
                signal_class: DiagnosticSignalClass::Support,
                schema_id: "support.bundle_manifest".to_owned(),
                schema_ref: "schemas/support/support_bundle_manifest.schema.json".to_owned(),
                purpose: "User-initiated support export manifest and preview surface for governed evidence handoff.".to_owned(),
                data_class: DiagnosticDataClass::MetadataOnly,
                opt_in_scope: DiagnosticOptInScope::UserInitiatedExportOnly,
                prohibited_content_classes: vec![
                    "raw_dump_bytes".to_owned(),
                    "credential_bodies".to_owned(),
                    "full_shell_history".to_owned(),
                    "ambient_authority_handles".to_owned(),
                ],
                retention_class: RetentionClass::LocalManifestUntilSent,
                redaction_profile: RedactionProfileClass::MetadataSafeDefault,
                evidence_source_refs: vec![
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CONSENT_LEDGER_REF.to_owned(),
                ],
            },
        ],
        host_families: vec![
            HostFamilyGovernanceRow {
                host_family_id: "notebook_kernel_host".to_owned(),
                host_family_label: "Notebook kernel session".to_owned(),
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                restart_class: RestartClass::SessionScoped,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 2,
                checkpoint_source: CheckpointSourceClass::ExplicitRerunOrMetadataRestore,
                quarantine_trigger_classes: vec![
                    "kernel_protocol_contract_lost".to_owned(),
                    "restart_budget_exhausted".to_owned(),
                    "repeated_crash_or_stall".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "session_id".to_owned(),
                    "kernel_environment_ref".to_owned(),
                    "cell_artifact_refs".to_owned(),
                    "restart_lineage".to_owned(),
                    "quarantine_reason".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::MinidumpOrCoreArtifact,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Performance,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "restart_evidence_stale_narrows_host_claim".to_owned(),
                    "symbolication_gap_forces_local_only_forensics".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "data_api_connector_host".to_owned(),
                host_family_label: "Data/API connector and query runtime".to_owned(),
                fault_domain_class: FaultDomainClass::RemoteConnector,
                restart_class: RestartClass::PrivilegedExternallyMutating,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 1,
                checkpoint_source: CheckpointSourceClass::ReconnectTokenAndTargetMetadata,
                quarantine_trigger_classes: vec![
                    "repeated_auth_failure".to_owned(),
                    "credential_scope_revoked".to_owned(),
                    "target_mismatch".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "connection_identity".to_owned(),
                    "statement_class".to_owned(),
                    "route_transport_class".to_owned(),
                    "approval_ticket_ref".to_owned(),
                    "quarantine_reason".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "restart_evidence_stale_narrows_host_claim".to_owned(),
                    "diagnostic_schema_stale_blocks_managed_export_claim".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "preview_dev_server_host".to_owned(),
                host_family_label: "Preview dev server".to_owned(),
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                restart_class: RestartClass::SessionScoped,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 2,
                checkpoint_source: CheckpointSourceClass::ExplicitRerunOrMetadataRestore,
                quarantine_trigger_classes: vec![
                    "repeated_crash_or_stall".to_owned(),
                    "port_binding_conflict_loop".to_owned(),
                    "target_authority_revoked".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "preview_session_id".to_owned(),
                    "route_identity".to_owned(),
                    "log_refs".to_owned(),
                    "restart_lineage".to_owned(),
                    "stale_session_drift".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::MinidumpOrCoreArtifact,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Performance,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "restart_evidence_stale_narrows_host_claim".to_owned(),
                    "symbolication_gap_forces_local_only_forensics".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "provider_run_session_host".to_owned(),
                host_family_label: "Provider-backed run session".to_owned(),
                fault_domain_class: FaultDomainClass::AiToolBroker,
                restart_class: RestartClass::PrivilegedExternallyMutating,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 1,
                checkpoint_source: CheckpointSourceClass::ProviderRegistryAndAllowedLocalCache,
                quarantine_trigger_classes: vec![
                    "quota_exhaustion".to_owned(),
                    "ticket_revoke".to_owned(),
                    "repeated_side_effect_failure".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "provider_model_id".to_owned(),
                    "ticket_lineage".to_owned(),
                    "tool_evidence_refs".to_owned(),
                    "breaker_state".to_owned(),
                    "quarantine_reason".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                    CrashArtifactClass::MirroredSymbolService,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Usage,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "symbolication_gap_forces_local_only_forensics".to_owned(),
                    "diagnostic_schema_stale_blocks_managed_export_claim".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "profiler_replay_session_host".to_owned(),
                host_family_label: "Profiler and replay session".to_owned(),
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                restart_class: RestartClass::SessionScoped,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 2,
                checkpoint_source: CheckpointSourceClass::ExplicitRerunOrMetadataRestore,
                quarantine_trigger_classes: vec![
                    "repeated_capture_failure".to_owned(),
                    "symbolication_drift".to_owned(),
                    "restart_budget_exhausted".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "capture_session_id".to_owned(),
                    "trace_profile_artifact_refs".to_owned(),
                    "mapping_quality".to_owned(),
                    "restart_lineage".to_owned(),
                    "quarantine_reason".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                    CrashArtifactClass::MirroredSymbolService,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Performance,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
                    PERFORMANCE_SUPPORT_SCHEMA_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "symbolication_gap_forces_local_only_forensics".to_owned(),
                    "crash_artifact_proof_stale_narrows_crash_claim".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "pipeline_viewer_host".to_owned(),
                host_family_label: "Pipeline viewer session".to_owned(),
                fault_domain_class: FaultDomainClass::RemoteConnector,
                restart_class: RestartClass::StatelessHelper,
                default_strike_window_minutes: 5,
                default_automatic_restarts: 5,
                checkpoint_source: CheckpointSourceClass::ReconnectTokenAndTargetMetadata,
                quarantine_trigger_classes: vec![
                    "target_mismatch".to_owned(),
                    "repeated_auth_failure".to_owned(),
                    "capability_manifest_drift".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "pipeline_identity".to_owned(),
                    "provider_transport_class".to_owned(),
                    "event_stream_lineage".to_owned(),
                    "reconnect_lineage".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "restart_evidence_stale_narrows_host_claim".to_owned(),
                    "diagnostic_schema_stale_blocks_managed_export_claim".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "query_runtime_host".to_owned(),
                host_family_label: "Query/request runtime".to_owned(),
                fault_domain_class: FaultDomainClass::SessionExecutionHost,
                restart_class: RestartClass::SessionScoped,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 2,
                checkpoint_source: CheckpointSourceClass::ExplicitRerunOrMetadataRestore,
                quarantine_trigger_classes: vec![
                    "repeated_crash_or_stall".to_owned(),
                    "request_contract_lost".to_owned(),
                    "restart_budget_exhausted".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "runtime_session_id".to_owned(),
                    "request_family".to_owned(),
                    "artifact_log_refs".to_owned(),
                    "restart_lineage".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Performance,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    PERFORMANCE_SUPPORT_SCHEMA_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "restart_evidence_stale_narrows_host_claim".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "docs_browser_bridge_host".to_owned(),
                host_family_label: "Docs and browser bridge".to_owned(),
                fault_domain_class: FaultDomainClass::RemoteConnector,
                restart_class: RestartClass::StatelessHelper,
                default_strike_window_minutes: 5,
                default_automatic_restarts: 5,
                checkpoint_source: CheckpointSourceClass::ReconnectTokenAndTargetMetadata,
                quarantine_trigger_classes: vec![
                    "cross_origin_limit_change".to_owned(),
                    "session_drift".to_owned(),
                    "target_mismatch".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "bridge_session_id".to_owned(),
                    "origin_boundary".to_owned(),
                    "session_freshness".to_owned(),
                    "last_good_capability_manifest".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "restart_evidence_stale_narrows_host_claim".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "registry_database_connector_host".to_owned(),
                host_family_label: "Registry or database connector".to_owned(),
                fault_domain_class: FaultDomainClass::RemoteConnector,
                restart_class: RestartClass::PrivilegedExternallyMutating,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 1,
                checkpoint_source: CheckpointSourceClass::ReconnectTokenAndTargetMetadata,
                quarantine_trigger_classes: vec![
                    "credential_scope_revoked".to_owned(),
                    "signer_or_endpoint_mismatch".to_owned(),
                    "repeated_auth_failure".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "connector_identity".to_owned(),
                    "approval_ticket_ref".to_owned(),
                    "target_identity".to_owned(),
                    "quarantine_reason".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                    CrashArtifactClass::LocalSymbolicationReport,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Support,
                    DiagnosticSignalClass::Usage,
                ],
                canonical_support_packet_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "diagnostic_schema_stale_blocks_managed_export_claim".to_owned(),
                ],
            },
            HostFamilyGovernanceRow {
                host_family_id: "infra_helper_job".to_owned(),
                host_family_label: "Infrastructure helper".to_owned(),
                fault_domain_class: FaultDomainClass::PolicyVerifierHelper,
                restart_class: RestartClass::AuthorityVerifier,
                default_strike_window_minutes: 10,
                default_automatic_restarts: 0,
                checkpoint_source: CheckpointSourceClass::SignedBundleCacheSnapshot,
                quarantine_trigger_classes: vec![
                    "signature_failure".to_owned(),
                    "epoch_rollback".to_owned(),
                    "replay_nonce_breach".to_owned(),
                ],
                minimum_diagnostic_exports: vec![
                    "helper_job_id".to_owned(),
                    "bundle_id".to_owned(),
                    "verification_failure_code".to_owned(),
                    "epoch_numbers".to_owned(),
                ],
                required_crash_artifacts: vec![
                    CrashArtifactClass::CrashEnvelope,
                    CrashArtifactClass::SymbolOrSourceMapManifest,
                ],
                required_diagnostic_signal_classes: vec![
                    DiagnosticSignalClass::Crash,
                    DiagnosticSignalClass::Support,
                ],
                canonical_support_packet_refs: vec![
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CONSENT_LEDGER_REF.to_owned(),
                    CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                ],
                claim_state: ClaimStateClass::Qualified,
                stale_proof_tokens: vec![],
                downgrade_rule_ids: vec![
                    "diagnostic_schema_stale_blocks_managed_export_claim".to_owned(),
                ],
            },
        ],
        downgrade_rules: vec![
            DowngradeRuleRow {
                rule_id: "restart_evidence_stale_narrows_host_claim".to_owned(),
                applies_to: "all_m5_host_families".to_owned(),
                trigger_class: DowngradeTriggerClass::RestartEvidenceStale,
                source_claim_state: ClaimStateClass::Qualified,
                downgraded_claim_state: ClaimStateClass::NarrowedPreview,
                required_effect: "Host surfaces must show stale restart/quarantine proof and stop claiming stable recovery posture.".to_owned(),
                rationale: "If restart lineage, strike-window proof, or quarantine evidence goes stale, the host may remain inspectable but it cannot inherit current bounded-failure maturity.".to_owned(),
                evidence_refs: vec![
                    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                    M5_FAULT_CRASH_GOVERNANCE_ARTIFACT_REF.to_owned(),
                ],
            },
            DowngradeRuleRow {
                rule_id: "crash_artifact_proof_stale_narrows_crash_claim".to_owned(),
                applies_to: "crash_forensics_rows".to_owned(),
                trigger_class: DowngradeTriggerClass::CrashArtifactProofStale,
                source_claim_state: ClaimStateClass::Qualified,
                downgraded_claim_state: ClaimStateClass::NarrowedPreview,
                required_effect: "Crash viewers and support exports must label crash-artifact proof stale and stop claiming reproducible forensics.".to_owned(),
                rationale: "Stale crash envelope, dump-manifest, or symbol-manifest proof cannot support strong crash-forensics claims even when older artifacts still exist.".to_owned(),
                evidence_refs: vec![
                    HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
                    M5_FAULT_CRASH_GOVERNANCE_ARTIFACT_REF.to_owned(),
                ],
            },
            DowngradeRuleRow {
                rule_id: "symbolication_gap_forces_local_only_forensics".to_owned(),
                applies_to: "notebook_preview_provider_profiler_hosts".to_owned(),
                trigger_class: DowngradeTriggerClass::SymbolicationNotExactBuild,
                source_claim_state: ClaimStateClass::Qualified,
                downgraded_claim_state: ClaimStateClass::NarrowedLocalOnly,
                required_effect: "The row may reference crash evidence by id, but exact-build symbolication and shareable forensic conclusions must narrow to local-only review.".to_owned(),
                rationale: "A missing or mismatched symbolication proof must never be upgraded into exact-build crash truth for shared diagnostics or release evidence.".to_owned(),
                evidence_refs: vec![
                    HARDEN_CRASH_CAPTURE_SCHEMA_REF.to_owned(),
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                ],
            },
            DowngradeRuleRow {
                rule_id: "diagnostic_schema_stale_blocks_managed_export_claim".to_owned(),
                applies_to: "usage_and_support_governance_rows".to_owned(),
                trigger_class: DowngradeTriggerClass::DiagnosticSchemaStale,
                source_claim_state: ClaimStateClass::Qualified,
                downgraded_claim_state: ClaimStateClass::BlockedUnverified,
                required_effect: "Managed or shareable export claims must block until schema id, consent posture, retention class, and redaction profile proof is current again.".to_owned(),
                rationale: "If the schema/consent packet is stale, export-safe language cannot stay greener than the underlying governance proof.".to_owned(),
                evidence_refs: vec![
                    TELEMETRY_SUPPORT_SCHEMA_REGISTRY_REF.to_owned(),
                    CONSENT_LEDGER_REF.to_owned(),
                ],
            },
        ],
        export_safe_summary:
            "This M5 fault/crash governance packet is metadata-safe: it freezes fault-domain, restart-budget, crash-artifact, diagnostics-schema, and downgrade vocabulary without embedding raw dumps, raw logs, raw code bodies, or live authority.".to_owned(),
    }
}
