//! M5 host-failure, crash-forensics, and diagnostics-governance certification.
//!
//! This module composes the previously frozen M5 host-failure packets into one
//! certification index that release, Help/About, service-health, and
//! support-export surfaces can ingest verbatim.
//!
//! The packet answers four questions for every claimed host family on every
//! claimed M5 profile:
//!
//! - which fault-domain and restart posture governs the host;
//! - which crash-artifact and symbolication packet proves exact-build or
//!   narrowed-forensics truth;
//! - which diagnostics-schema packet proves consent, retention, endpoint, and
//!   redaction posture; and
//! - which field-readiness drill packet proves restart-budget enforcement,
//!   scoped failure, checkpoint preservation, and no-silent-upload behavior.
//!
//! Rows that lose any of those proofs narrow automatically instead of
//! inheriting a greener neighboring claim.

use serde::{Deserialize, Serialize};

use crate::m5_fault_crash_governance::{
    seeded_m5_fault_crash_governance_packet, FaultDomainClass, RestartClass,
    M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF,
};
use crate::m5_forensic_packet::{seeded_m5_forensic_packet, M5_FORENSIC_PACKET_SCHEMA_REF};
use crate::m5_host_failure_drills::{
    seeded_m5_host_failure_drill_packet, M5_HOST_FAILURE_DRILL_SCHEMA_REF,
};
use crate::schema_registry::{
    seeded_depth_surface_schema_registry_packet, DEPTH_SCHEMA_REGISTRY_SCHEMA_REF,
};
use crate::{
    CRASH_STORE_VIEWER_SCHEMA_REF, RECOVERY_REVIEW_SCHEMA_REF,
    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF,
};

const CRASH_STORE_PACKET_REF: &str = "fixtures/support/m5/crash_store/packet.json";
const M5_HOST_FAILURE_DRILL_PACKET_REF: &str =
    "fixtures/support/m5/host_failure_drills/packet.json";
const DEPTH_SCHEMA_REGISTRY_PACKET_REF: &str =
    "fixtures/support/m5/depth_surface_schema_registry/packet.json";
const RECOVERY_REVIEW_PACKET_REF: &str = "fixtures/support/m5/recovery_review/packet.json";
const SYMBOLICATION_CONTRACT_SCHEMA_REF: &str = "schemas/debug/symbolication_contract.schema.json";
const SYMBOLICATION_PACKET_REF: &str = "fixtures/debug/symbolication/packet.json";
const SYMBOLICATION_EXACT_LOCAL_REF: &str = "fixtures/debug/symbolication/exact_local_report.json";
const SYMBOLICATION_APPROXIMATE_REF: &str =
    "fixtures/debug/symbolication/approximate_mirrored_report.json";
const SYMBOLICATION_SYMBOL_ONLY_REF: &str = "fixtures/debug/symbolication/symbol_only_report.json";
const SYMBOLICATION_UNRESOLVED_REF: &str =
    "fixtures/debug/symbolication/unresolved_mismatch_report.json";
const RELEASE_MANIFEST_CONSUMER_REF: &str =
    "artifacts/release/stable/claim-publication-manifest/manifest.json";
const SERVICE_HEALTH_CONSUMER_REF: &str =
    "crates/aureline-service-health/src/finalize_service_health_destination_truth/mod.rs";
const SUPPORT_EXPORT_CONSUMER_REF: &str = "schemas/support/support_bundle_manifest.schema.json";

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

const REQUIRED_PROJECTION_FIELDS: &[&str] = &[
    "certification_row_id",
    "host_family_id",
    "profile",
    "published_state",
    "stale_proof_tokens",
    "downgrade_rule_ids",
];

/// Stable record-kind tag carried by [`M5FaultCrashCertificationPacket`].
pub const M5_FAULT_CRASH_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "m5_fault_crash_certification_packet";

/// Frozen schema version for the M5 fault/crash certification packet.
pub const M5_FAULT_CRASH_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema.
pub const M5_FAULT_CRASH_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/support/m5-fault-crash-certification.schema.json";

/// Repository-relative path of the reviewer-facing help document.
pub const M5_FAULT_CRASH_CERTIFICATION_DOC_REF: &str =
    "docs/help/support/m5-fault-crash-certification.md";

/// Repository-relative path of the checked review artifact.
pub const M5_FAULT_CRASH_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/support/m5/fault-crash-certification.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_FAULT_CRASH_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/support/m5/fault_crash_certification";

/// Claimed M5 profile covered by the certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedM5Profile {
    /// Local-first desktop profile.
    DesktopLocalFirst,
    /// Desktop profile with remote attach and reattach claims.
    HybridRemoteAttach,
    /// Managed-cloud profile with provider-backed services.
    ManagedCloud,
    /// Self-hosted or sovereign deployment profile.
    SelfHostedSovereign,
    /// Air-gapped profile using local or mirrored assets only.
    AirGappedMirrorOnly,
}

impl ClaimedM5Profile {
    /// All claimed profiles in canonical order.
    pub const ALL: [Self; 5] = [
        Self::DesktopLocalFirst,
        Self::HybridRemoteAttach,
        Self::ManagedCloud,
        Self::SelfHostedSovereign,
        Self::AirGappedMirrorOnly,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopLocalFirst => "desktop_local_first",
            Self::HybridRemoteAttach => "hybrid_remote_attach",
            Self::ManagedCloud => "managed_cloud",
            Self::SelfHostedSovereign => "self_hosted_sovereign",
            Self::AirGappedMirrorOnly => "air_gapped_mirror_only",
        }
    }

    /// Returns true when the profile is air-gapped.
    pub const fn is_air_gapped(self) -> bool {
        matches!(self, Self::AirGappedMirrorOnly)
    }
}

/// Certification result published for one host/profile row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationStateClass {
    /// All backing evidence is current on the claimed profile.
    Qualified,
    /// The host remains claimed, but only for a narrower subset of the profile.
    LimitedProfileScoped,
    /// Only explicit local-only or inspect-only truth may be claimed.
    ExperimentalLocalOnly,
    /// The host family is not marketed on the named profile.
    NotMarketed,
    /// The row is blocked pending fresh proof.
    BlockedUnverified,
}

impl CertificationStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::LimitedProfileScoped => "limited_profile_scoped",
            Self::ExperimentalLocalOnly => "experimental_local_only",
            Self::NotMarketed => "not_marketed",
            Self::BlockedUnverified => "blocked_unverified",
        }
    }
}

/// Symbolication fidelity story surfaced by the certification row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolicationPostureClass {
    /// Exact-build local-first symbolication is current.
    ExactBuildLocalFirst,
    /// Exact-build symbolication is current and mirrored sources are allowed.
    ExactBuildMirrorAllowed,
    /// Imported or approximate symbolication stays visibly labeled.
    ImportedOrApproximateLabeled,
    /// Shared exact-build claims are blocked until symbolication is refreshed.
    SharedClaimBlocked,
}

impl SymbolicationPostureClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildLocalFirst => "exact_build_local_first",
            Self::ExactBuildMirrorAllowed => "exact_build_mirror_allowed",
            Self::ImportedOrApproximateLabeled => "imported_or_approximate_labeled",
            Self::SharedClaimBlocked => "shared_claim_blocked",
        }
    }
}

/// Diagnostics-governance story surfaced by the certification row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticsGovernancePostureClass {
    /// Schema-governed diagnostics remain local-first by default.
    LocalFirstSchemaGoverned,
    /// Schema-governed diagnostics may use explicit managed export flows.
    ManagedExportSchemaGoverned,
    /// Only manual local export is currently claimed.
    ManualExportOnly,
    /// Diagnostics-governance claims are blocked pending fresh schema proof.
    ClaimBlockedPendingSchemaRefresh,
}

impl DiagnosticsGovernancePostureClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFirstSchemaGoverned => "local_first_schema_governed",
            Self::ManagedExportSchemaGoverned => "managed_export_schema_governed",
            Self::ManualExportOnly => "manual_export_only",
            Self::ClaimBlockedPendingSchemaRefresh => "claim_blocked_pending_schema_refresh",
        }
    }
}

/// Stable consumer surface that ingests the certification result verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationSurfaceClass {
    /// Help/About proof and provenance cards.
    HelpAbout,
    /// Service-health and outage truth surfaces.
    ServiceHealth,
    /// Support-export and forensic handoff surfaces.
    SupportExport,
    /// Release manifest and publication control surfaces.
    ReleaseManifest,
}

impl CertificationSurfaceClass {
    /// All consumer surfaces in canonical order.
    pub const ALL: [Self; 4] = [
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::SupportExport,
        Self::ReleaseManifest,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::ReleaseManifest => "release_manifest",
        }
    }
}

/// Downgrade trigger automated by the certification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradeTriggerClass {
    /// Restart lineage or quarantine evidence is stale.
    RestartEvidenceStale,
    /// Crash-artifact capture or attribution proof is stale.
    CrashArtifactProofStale,
    /// Exact-build symbolication proof is stale or unresolved.
    SymbolicationNotExactBuild,
    /// Diagnostic schema, consent, or retention proof is stale.
    DiagnosticSchemaStale,
    /// Host-failure drill or field-readiness packet is stale.
    FieldReadinessDrillStale,
    /// The profile does not provide the minimum capability for the host family.
    ProfileCapabilityAbsent,
    /// One downstream surface stopped ingesting the certification by reference.
    ConsumerBindingMissing,
}

impl CertificationDowngradeTriggerClass {
    /// All downgrade triggers in canonical order.
    pub const ALL: [Self; 7] = [
        Self::RestartEvidenceStale,
        Self::CrashArtifactProofStale,
        Self::SymbolicationNotExactBuild,
        Self::DiagnosticSchemaStale,
        Self::FieldReadinessDrillStale,
        Self::ProfileCapabilityAbsent,
        Self::ConsumerBindingMissing,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestartEvidenceStale => "restart_evidence_stale",
            Self::CrashArtifactProofStale => "crash_artifact_proof_stale",
            Self::SymbolicationNotExactBuild => "symbolication_not_exact_build",
            Self::DiagnosticSchemaStale => "diagnostic_schema_stale",
            Self::FieldReadinessDrillStale => "field_readiness_drill_stale",
            Self::ProfileCapabilityAbsent => "profile_capability_absent",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
        }
    }
}

/// One host/profile certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostProfileCertificationRow {
    /// Stable row identifier.
    pub certification_row_id: String,
    /// Stable host-family identifier.
    pub host_family_id: String,
    /// Human-readable host-family label.
    pub host_family_label: String,
    /// Claimed M5 profile covered by the row.
    pub profile: ClaimedM5Profile,
    /// Published certification state for the row.
    pub published_state: CertificationStateClass,
    /// Governing fault-domain class.
    pub fault_domain_class: FaultDomainClass,
    /// Governing restart class.
    pub restart_class: RestartClass,
    /// Restart-budget or topology packet backing the row.
    pub restart_posture_packet_ref: String,
    /// Crash-artifact or forensic packet backing the row.
    pub crash_artifact_packet_ref: String,
    /// Symbolication packet or report backing the row.
    pub symbolication_packet_ref: String,
    /// Diagnostics-schema packet backing the row.
    pub diagnostic_schema_packet_ref: String,
    /// Field-readiness drill packet backing the row.
    pub field_readiness_packet_ref: String,
    /// Symbolication story visible to product and support surfaces.
    pub symbolication_posture: SymbolicationPostureClass,
    /// Diagnostics-governance story visible to product and support surfaces.
    pub diagnostics_governance_posture: DiagnosticsGovernancePostureClass,
    /// Active stale or capability-loss tokens narrowing the row.
    pub stale_proof_tokens: Vec<String>,
    /// Active downgrade-rule identifiers explaining the published state.
    pub downgrade_rule_ids: Vec<String>,
    /// Review-safe summary for downstream surfaces.
    pub summary: String,
}

/// One downgrade rule published by the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDowngradeRuleRow {
    /// Stable rule identifier.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger_class: CertificationDowngradeTriggerClass,
    /// Source certification state before the downgrade.
    pub source_state: CertificationStateClass,
    /// Resulting certification state after the downgrade.
    pub downgraded_state: CertificationStateClass,
    /// User-visible effect of the downgrade.
    pub required_effect: String,
    /// Reviewable rationale for the downgrade.
    pub rationale: String,
    /// Supporting evidence or contract refs used to inspect the rule.
    pub evidence_refs: Vec<String>,
}

/// One consumer-surface binding proving the same certification result is reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSurfaceBinding {
    /// Consumer surface that ingests the certification.
    pub surface: CertificationSurfaceClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// Number of certification rows the consumer exposes by reference.
    pub certification_row_count: usize,
    /// Fields the consumer must preserve verbatim from the packet.
    pub required_verbatim_fields: Vec<String>,
    /// True when the consumer narrows immediately on stale proof or blocked rows.
    pub narrow_on_stale_proof: bool,
    /// True when local-only or not-marketed states remain labeled explicitly.
    pub explicit_limited_state_labels_required: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

/// One validation error returned by [`M5FaultCrashCertificationPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FaultCrashCertificationViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 fault/crash certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FaultCrashCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing help document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing packets and contracts this certification composes.
    pub supporting_contract_refs: Vec<String>,
    /// Claimed M5 profiles covered by the packet.
    pub claimed_profiles: Vec<ClaimedM5Profile>,
    /// Canonical host/profile certification rows.
    pub certification_rows: Vec<HostProfileCertificationRow>,
    /// Automatic downgrade rules used by the packet.
    pub downgrade_rules: Vec<CertificationDowngradeRuleRow>,
    /// Consumer-surface bindings that prove one certification index is reused.
    pub surface_bindings: Vec<CertificationSurfaceBinding>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5FaultCrashCertificationPacket {
    /// Validates profile coverage, downgrade automation, and shared-surface bindings.
    pub fn validate(&self) -> Vec<M5FaultCrashCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FAULT_CRASH_CERTIFICATION_PACKET_RECORD_KIND {
            violations.push(M5FaultCrashCertificationViolation {
                path: "record_kind".to_owned(),
                message: "unexpected record_kind".to_owned(),
            });
        }
        if self.schema_version != M5_FAULT_CRASH_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5FaultCrashCertificationViolation {
                path: "schema_version".to_owned(),
                message: "unexpected schema_version".to_owned(),
            });
        }
        if self.doc_ref != M5_FAULT_CRASH_CERTIFICATION_DOC_REF {
            violations.push(M5FaultCrashCertificationViolation {
                path: "doc_ref".to_owned(),
                message: "packet must quote the canonical reviewer doc".to_owned(),
            });
        }
        if self.schema_ref != M5_FAULT_CRASH_CERTIFICATION_SCHEMA_REF {
            violations.push(M5FaultCrashCertificationViolation {
                path: "schema_ref".to_owned(),
                message: "packet must quote the canonical schema ref".to_owned(),
            });
        }
        if self.artifact_ref != M5_FAULT_CRASH_CERTIFICATION_ARTIFACT_REF {
            violations.push(M5FaultCrashCertificationViolation {
                path: "artifact_ref".to_owned(),
                message: "packet must quote the checked review artifact ref".to_owned(),
            });
        }

        for required in ClaimedM5Profile::ALL {
            if !self.claimed_profiles.contains(&required) {
                violations.push(M5FaultCrashCertificationViolation {
                    path: "claimed_profiles".to_owned(),
                    message: format!("missing claimed profile {}", required.as_str()),
                });
            }
        }

        for required in REQUIRED_HOST_FAMILY_IDS {
            for profile in ClaimedM5Profile::ALL {
                if !self
                    .certification_rows
                    .iter()
                    .any(|row| row.host_family_id == *required && row.profile == profile)
                {
                    violations.push(M5FaultCrashCertificationViolation {
                        path: "certification_rows".to_owned(),
                        message: format!(
                            "missing certification row for host {} on profile {}",
                            required,
                            profile.as_str()
                        ),
                    });
                }
            }
        }

        for row in &self.certification_rows {
            let base = format!("certification_rows.{}", row.certification_row_id);
            for (field, value) in [
                ("host_family_id", row.host_family_id.as_str()),
                ("host_family_label", row.host_family_label.as_str()),
                (
                    "restart_posture_packet_ref",
                    row.restart_posture_packet_ref.as_str(),
                ),
                (
                    "crash_artifact_packet_ref",
                    row.crash_artifact_packet_ref.as_str(),
                ),
                (
                    "symbolication_packet_ref",
                    row.symbolication_packet_ref.as_str(),
                ),
                (
                    "diagnostic_schema_packet_ref",
                    row.diagnostic_schema_packet_ref.as_str(),
                ),
                (
                    "field_readiness_packet_ref",
                    row.field_readiness_packet_ref.as_str(),
                ),
                ("summary", row.summary.as_str()),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5FaultCrashCertificationViolation {
                        path: format!("{base}.{field}"),
                        message: "row field may not be empty".to_owned(),
                    });
                }
            }

            if row.published_state == CertificationStateClass::Qualified
                && !row.stale_proof_tokens.is_empty()
            {
                violations.push(M5FaultCrashCertificationViolation {
                    path: format!("{base}.stale_proof_tokens"),
                    message: "qualified rows may not carry stale proof tokens".to_owned(),
                });
            }
            if row.published_state != CertificationStateClass::Qualified
                && row.downgrade_rule_ids.is_empty()
            {
                violations.push(M5FaultCrashCertificationViolation {
                    path: format!("{base}.downgrade_rule_ids"),
                    message: "non-qualified rows must cite downgrade rules".to_owned(),
                });
            }
            if row.profile.is_air_gapped()
                && matches!(
                    row.host_family_id.as_str(),
                    "provider_run_session_host" | "docs_browser_bridge_host"
                )
                && row.published_state == CertificationStateClass::Qualified
            {
                violations.push(M5FaultCrashCertificationViolation {
                    path: base,
                    message:
                        "air-gapped rows may not publish managed/browser bridge hosts as qualified"
                            .to_owned(),
                });
            }
        }

        for required in CertificationDowngradeTriggerClass::ALL {
            if !self
                .downgrade_rules
                .iter()
                .any(|row| row.trigger_class == required)
            {
                violations.push(M5FaultCrashCertificationViolation {
                    path: "downgrade_rules".to_owned(),
                    message: format!("missing downgrade trigger {}", required.as_str()),
                });
            }
        }

        for required in CertificationSurfaceClass::ALL {
            let Some(binding) = self
                .surface_bindings
                .iter()
                .find(|binding| binding.surface == required)
            else {
                violations.push(M5FaultCrashCertificationViolation {
                    path: "surface_bindings".to_owned(),
                    message: format!("missing surface binding {}", required.as_str()),
                });
                continue;
            };
            if binding.ingested_packet_id != self.packet_id {
                violations.push(M5FaultCrashCertificationViolation {
                    path: format!("surface_bindings.{}", binding.surface.as_str()),
                    message: "surface binding must ingest the canonical packet id".to_owned(),
                });
            }
            if binding.certification_row_count != self.certification_rows.len() {
                violations.push(M5FaultCrashCertificationViolation {
                    path: format!("surface_bindings.{}", binding.surface.as_str()),
                    message: "surface binding row count must match certification rows".to_owned(),
                });
            }
            for field in REQUIRED_PROJECTION_FIELDS {
                if !binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field)
                {
                    violations.push(M5FaultCrashCertificationViolation {
                        path: format!("surface_bindings.{}", binding.surface.as_str()),
                        message: format!("surface binding must preserve {}", field),
                    });
                }
            }
        }

        violations
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self
                .surface_bindings
                .iter()
                .all(|binding| binding.narrow_on_stale_proof)
    }
}

/// Returns the canonical seeded M5 fault/crash certification packet.
pub fn seeded_m5_fault_crash_certification_packet() -> M5FaultCrashCertificationPacket {
    build_packet(CertificationVariant::Canonical)
}

/// Returns a seeded packet with symbolication proof narrowed.
pub fn seeded_stale_symbolication_m5_fault_crash_certification_packet(
) -> M5FaultCrashCertificationPacket {
    build_packet(CertificationVariant::StaleSymbolication)
}

/// Returns a seeded packet with diagnostics-schema proof blocked.
pub fn seeded_stale_schema_m5_fault_crash_certification_packet() -> M5FaultCrashCertificationPacket
{
    build_packet(CertificationVariant::StaleSchema)
}

#[derive(Debug, Clone, Copy)]
enum CertificationVariant {
    Canonical,
    StaleSymbolication,
    StaleSchema,
}

fn build_packet(variant: CertificationVariant) -> M5FaultCrashCertificationPacket {
    let governance = seeded_m5_fault_crash_governance_packet();
    let drills = seeded_m5_host_failure_drill_packet();
    let forensic = seeded_m5_forensic_packet();
    let _schema_registry = seeded_depth_surface_schema_registry_packet();
    let mut certification_rows = Vec::new();

    for host in &governance.host_families {
        let drill = drills
            .drills
            .iter()
            .find(|row| row.host_family_id == host.host_family_id)
            .expect("drill row for governed host family");
        let _forensic_row = forensic
            .rows
            .iter()
            .find(|row| row.host_family_id == host.host_family_id)
            .expect("forensic row for governed host family");
        let diagnostic_posture = default_diagnostics_posture(&host.host_family_id);
        let symbolication_posture = default_symbolication_posture(&host.host_family_id);
        for profile in ClaimedM5Profile::ALL {
            certification_rows.push(seed_row(
                host.host_family_id.as_str(),
                host.host_family_label.as_str(),
                profile,
                host.fault_domain_class,
                host.restart_class,
                drill.evidence_fixture_ref.as_str(),
                symbolication_posture,
                diagnostic_posture,
                variant,
            ));
        }
    }

    let packet_id = "support.m5.fault_crash_certification.v1".to_owned();

    M5FaultCrashCertificationPacket {
        record_kind: M5_FAULT_CRASH_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_FAULT_CRASH_CERTIFICATION_SCHEMA_VERSION,
        packet_id: packet_id.clone(),
        generated_at: "2026-06-13T03:25:00Z".to_owned(),
        doc_ref: M5_FAULT_CRASH_CERTIFICATION_DOC_REF.to_owned(),
        schema_ref: M5_FAULT_CRASH_CERTIFICATION_SCHEMA_REF.to_owned(),
        artifact_ref: M5_FAULT_CRASH_CERTIFICATION_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".plans/M05-249.md".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#appendix-cd".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#appendix-ck".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#12706-appendix-ai-crash-capture-symbolication-and-field-readiness-matrix".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#2326-session-restore-fault-domain-and-crash-forensics-drills".to_owned(),
        ],
        supporting_contract_refs: vec![
            M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF.to_owned(),
            SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
            CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
            SYMBOLICATION_CONTRACT_SCHEMA_REF.to_owned(),
            DEPTH_SCHEMA_REGISTRY_SCHEMA_REF.to_owned(),
            RECOVERY_REVIEW_SCHEMA_REF.to_owned(),
            M5_FORENSIC_PACKET_SCHEMA_REF.to_owned(),
            M5_HOST_FAILURE_DRILL_SCHEMA_REF.to_owned(),
            CRASH_STORE_PACKET_REF.to_owned(),
            SYMBOLICATION_PACKET_REF.to_owned(),
            DEPTH_SCHEMA_REGISTRY_PACKET_REF.to_owned(),
            RECOVERY_REVIEW_PACKET_REF.to_owned(),
            M5_HOST_FAILURE_DRILL_PACKET_REF.to_owned(),
        ],
        claimed_profiles: ClaimedM5Profile::ALL.to_vec(),
        certification_rows,
        downgrade_rules: seeded_downgrade_rules(),
        surface_bindings: seeded_surface_bindings(&packet_id, governance.host_families.len() * ClaimedM5Profile::ALL.len()),
        export_safe_summary: "This metadata-safe certification index binds every claimed M5 host family and profile to explicit restart, crash, symbolication, schema-governance, and field-readiness proof; stale or inapplicable rows narrow instead of inheriting adjacent maturity.".to_owned(),
    }
}

fn default_symbolication_posture(host_family_id: &str) -> SymbolicationPostureClass {
    match host_family_id {
        "provider_run_session_host" | "preview_dev_server_host" => {
            SymbolicationPostureClass::ExactBuildMirrorAllowed
        }
        "profiler_replay_session_host" | "pipeline_viewer_host" => {
            SymbolicationPostureClass::ImportedOrApproximateLabeled
        }
        "docs_browser_bridge_host" | "registry_database_connector_host" => {
            SymbolicationPostureClass::ImportedOrApproximateLabeled
        }
        _ => SymbolicationPostureClass::ExactBuildLocalFirst,
    }
}

fn default_diagnostics_posture(host_family_id: &str) -> DiagnosticsGovernancePostureClass {
    match host_family_id {
        "provider_run_session_host"
        | "pipeline_viewer_host"
        | "docs_browser_bridge_host"
        | "registry_database_connector_host" => {
            DiagnosticsGovernancePostureClass::ManagedExportSchemaGoverned
        }
        _ => DiagnosticsGovernancePostureClass::LocalFirstSchemaGoverned,
    }
}

fn seed_row(
    host_family_id: &str,
    host_family_label: &str,
    profile: ClaimedM5Profile,
    fault_domain_class: FaultDomainClass,
    restart_class: RestartClass,
    drill_ref: &str,
    default_symbolication_posture: SymbolicationPostureClass,
    default_diagnostics_posture: DiagnosticsGovernancePostureClass,
    variant: CertificationVariant,
) -> HostProfileCertificationRow {
    let mut row = HostProfileCertificationRow {
        certification_row_id: format!("m5_fault_crash:{}:{}", host_family_id, profile.as_str()),
        host_family_id: host_family_id.to_owned(),
        host_family_label: host_family_label.to_owned(),
        profile,
        published_state: CertificationStateClass::Qualified,
        fault_domain_class,
        restart_class,
        restart_posture_packet_ref: SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
        crash_artifact_packet_ref: CRASH_STORE_PACKET_REF.to_owned(),
        symbolication_packet_ref: symbolication_ref_for(host_family_id).to_owned(),
        diagnostic_schema_packet_ref: DEPTH_SCHEMA_REGISTRY_PACKET_REF.to_owned(),
        field_readiness_packet_ref: drill_ref.to_owned(),
        symbolication_posture: default_symbolication_posture,
        diagnostics_governance_posture: if profile.is_air_gapped() {
            DiagnosticsGovernancePostureClass::ManualExportOnly
        } else {
            default_diagnostics_posture
        },
        stale_proof_tokens: Vec::new(),
        downgrade_rule_ids: Vec::new(),
        summary: format!(
            "{} on {} reuses the canonical restart, crash, symbolication, schema-governance, and field-readiness packets.",
            host_family_label,
            profile.as_str()
        ),
    };

    if profile.is_air_gapped() {
        match host_family_id {
            "provider_run_session_host" => apply_downgrade(
                &mut row,
                CertificationStateClass::NotMarketed,
                "profile_capability_absent:provider_plane",
                "profile_capability_absent_withholds_managed_or_provider_hosts",
                SymbolicationPostureClass::SharedClaimBlocked,
                DiagnosticsGovernancePostureClass::ManualExportOnly,
                "Provider-run sessions are not marketed on air-gapped profiles because the provider plane is unavailable.",
            ),
            "docs_browser_bridge_host" => apply_downgrade(
                &mut row,
                CertificationStateClass::NotMarketed,
                "profile_capability_absent:browser_bridge",
                "profile_capability_absent_withholds_browser_bridge_hosts",
                SymbolicationPostureClass::SharedClaimBlocked,
                DiagnosticsGovernancePostureClass::ManualExportOnly,
                "Docs/browser bridge hosts are not marketed on air-gapped profiles because the bridge cannot claim live handoff behavior.",
            ),
            "pipeline_viewer_host" => apply_downgrade(
                &mut row,
                CertificationStateClass::ExperimentalLocalOnly,
                "profile_scoped:imported_pipeline_packets_only",
                "profile_capability_absent_narrows_to_local_only",
                SymbolicationPostureClass::ImportedOrApproximateLabeled,
                DiagnosticsGovernancePostureClass::ManualExportOnly,
                "Pipeline viewers narrow to local-only imported packet review on air-gapped profiles.",
            ),
            "data_api_connector_host" | "registry_database_connector_host" => {
                let current_symbolication = row.symbolication_posture;
                apply_downgrade(
                    &mut row,
                    CertificationStateClass::LimitedProfileScoped,
                    "profile_scoped:local_or_mirrored_targets_only",
                    "profile_capability_absent_narrows_to_profile_scoped",
                    current_symbolication,
                    DiagnosticsGovernancePostureClass::ManualExportOnly,
                    "Connector hosts narrow to local or mirrored targets only on air-gapped profiles.",
                )
            }
            _ => {}
        }
    }

    match variant {
        CertificationVariant::Canonical => {}
        CertificationVariant::StaleSymbolication => {
            if matches!(
                host_family_id,
                "provider_run_session_host" | "profiler_replay_session_host"
            ) && profile != ClaimedM5Profile::AirGappedMirrorOnly
            {
                let current_diagnostics = row.diagnostics_governance_posture;
                apply_downgrade(
                    &mut row,
                    CertificationStateClass::ExperimentalLocalOnly,
                    "stale_symbolication_proof",
                    "symbolication_gap_forces_local_only_forensics",
                    SymbolicationPostureClass::SharedClaimBlocked,
                    current_diagnostics,
                    "Exact-build symbolication proof is stale, so shared crash-forensics claims narrow to local-only inspection.",
                );
            }
        }
        CertificationVariant::StaleSchema => {
            if matches!(
                host_family_id,
                "provider_run_session_host"
                    | "pipeline_viewer_host"
                    | "registry_database_connector_host"
            ) && matches!(
                profile,
                ClaimedM5Profile::ManagedCloud | ClaimedM5Profile::SelfHostedSovereign
            ) {
                let current_symbolication = row.symbolication_posture;
                apply_downgrade(
                    &mut row,
                    CertificationStateClass::BlockedUnverified,
                    "stale_diagnostic_schema_review",
                    "diagnostic_schema_stale_blocks_managed_export_claim",
                    current_symbolication,
                    DiagnosticsGovernancePostureClass::ClaimBlockedPendingSchemaRefresh,
                    "Managed/shareable diagnostics claims are blocked until schema, consent, and retention evidence is refreshed.",
                );
            }
        }
    }

    row
}

fn apply_downgrade(
    row: &mut HostProfileCertificationRow,
    state: CertificationStateClass,
    token: &str,
    rule_id: &str,
    symbolication_posture: SymbolicationPostureClass,
    diagnostics_governance_posture: DiagnosticsGovernancePostureClass,
    summary: &str,
) {
    row.published_state = state;
    row.symbolication_posture = symbolication_posture;
    row.diagnostics_governance_posture = diagnostics_governance_posture;
    row.stale_proof_tokens.push(token.to_owned());
    row.downgrade_rule_ids.push(rule_id.to_owned());
    row.summary = summary.to_owned();
}

fn symbolication_ref_for(host_family_id: &str) -> &'static str {
    match host_family_id {
        "notebook_kernel_host"
        | "data_api_connector_host"
        | "query_runtime_host"
        | "infra_helper_job" => SYMBOLICATION_EXACT_LOCAL_REF,
        "preview_dev_server_host" | "profiler_replay_session_host" => SYMBOLICATION_APPROXIMATE_REF,
        "pipeline_viewer_host" | "docs_browser_bridge_host" => SYMBOLICATION_SYMBOL_ONLY_REF,
        "provider_run_session_host" | "registry_database_connector_host" => {
            SYMBOLICATION_UNRESOLVED_REF
        }
        _ => SYMBOLICATION_PACKET_REF,
    }
}

fn seeded_downgrade_rules() -> Vec<CertificationDowngradeRuleRow> {
    vec![
        CertificationDowngradeRuleRow {
            rule_id: "restart_evidence_stale_narrows_host_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::RestartEvidenceStale,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::LimitedProfileScoped,
            required_effect: "Help/About, service health, support export, and release manifest must label restart evidence stale and stop publishing the full qualified host-failure claim.".to_owned(),
            rationale: "Stale restart lineage or quarantine proof cannot keep a broad M5 host-failure claim green.".to_owned(),
            evidence_refs: vec![
                SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF.to_owned(),
                M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "crash_artifact_proof_stale_narrows_crash_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::CrashArtifactProofStale,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::LimitedProfileScoped,
            required_effect: "Crash viewers and release claims must stop describing local crash capture as current when crash-artifact proof is stale.".to_owned(),
            rationale: "Crash capture claims require current local-first artifact proof.".to_owned(),
            evidence_refs: vec![
                CRASH_STORE_VIEWER_SCHEMA_REF.to_owned(),
                CRASH_STORE_PACKET_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "symbolication_gap_forces_local_only_forensics".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::SymbolicationNotExactBuild,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::ExperimentalLocalOnly,
            required_effect: "Shared crash-forensics claims must narrow to local-only inspection until exact-build symbolication proof is current.".to_owned(),
            rationale: "Exact-build symbolication is a hard ceiling for broad crash-forensics claims.".to_owned(),
            evidence_refs: vec![
                SYMBOLICATION_CONTRACT_SCHEMA_REF.to_owned(),
                SYMBOLICATION_PACKET_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "diagnostic_schema_stale_blocks_managed_export_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::DiagnosticSchemaStale,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::BlockedUnverified,
            required_effect: "Managed or shared diagnostics claims must block until schema, consent, endpoint, and retention proof is current.".to_owned(),
            rationale: "Schema-governed diagnostics may not inherit maturity from adjacent packets when their own consent or retention proof is stale.".to_owned(),
            evidence_refs: vec![
                DEPTH_SCHEMA_REGISTRY_SCHEMA_REF.to_owned(),
                DEPTH_SCHEMA_REGISTRY_PACKET_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "field_readiness_drift_blocks_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::FieldReadinessDrillStale,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::BlockedUnverified,
            required_effect: "Release and support surfaces must block host-failure claims when drill evidence is stale or missing.".to_owned(),
            rationale: "The certification cannot stay broad if the host-failure drill packet is no longer current.".to_owned(),
            evidence_refs: vec![
                M5_HOST_FAILURE_DRILL_SCHEMA_REF.to_owned(),
                M5_HOST_FAILURE_DRILL_PACKET_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "profile_capability_absent_narrows_to_profile_scoped".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::ProfileCapabilityAbsent,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::LimitedProfileScoped,
            required_effect: "Profiles lacking the necessary plane, bridge, or target capability publish a profile-scoped or withheld claim instead of inheriting a generic badge.".to_owned(),
            rationale: "A host family may only carry the supportability claim the named profile can actually prove.".to_owned(),
            evidence_refs: vec![
                M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF.to_owned(),
                DEPTH_SCHEMA_REGISTRY_PACKET_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "consumer_binding_missing_blocks_shared_truth".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::ConsumerBindingMissing,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::BlockedUnverified,
            required_effect: "If Help/About, service health, support export, or release manifest stops ingesting this packet by reference, the broad claim blocks until parity is restored.".to_owned(),
            rationale: "The task requires one certification index; broken consumer bindings invalidate that promise.".to_owned(),
            evidence_refs: vec![
                M5_FAULT_CRASH_CERTIFICATION_DOC_REF.to_owned(),
                RELEASE_MANIFEST_CONSUMER_REF.to_owned(),
                SERVICE_HEALTH_CONSUMER_REF.to_owned(),
                SUPPORT_EXPORT_CONSUMER_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "profile_capability_absent_withholds_managed_or_provider_hosts".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::ProfileCapabilityAbsent,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::NotMarketed,
            required_effect: "Managed or browser-bridge host families remain withheld on profiles that cannot honestly claim their control plane or handoff path.".to_owned(),
            rationale: "Some host families are entirely out of scope on air-gapped profiles and must stay explicitly withheld.".to_owned(),
            evidence_refs: vec![
                M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF.to_owned(),
                DEPTH_SCHEMA_REGISTRY_PACKET_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "profile_capability_absent_withholds_browser_bridge_hosts".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::ProfileCapabilityAbsent,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::NotMarketed,
            required_effect: "Browser-bridge host claims are withheld where the profile cannot honestly claim live bridge behavior.".to_owned(),
            rationale: "Docs/browser bridges must not publish a live crash/supportability badge on air-gapped profiles.".to_owned(),
            evidence_refs: vec![
                M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF.to_owned(),
                DEPTH_SCHEMA_REGISTRY_PACKET_REF.to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "profile_capability_absent_narrows_to_local_only".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::ProfileCapabilityAbsent,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::ExperimentalLocalOnly,
            required_effect: "The profile may keep local imported inspection but may not claim live shared supportability.".to_owned(),
            rationale: "Some hosts retain local imported forensic review even when the live plane is absent.".to_owned(),
            evidence_refs: vec![
                M5_HOST_FAILURE_DRILL_PACKET_REF.to_owned(),
                SYMBOLICATION_PACKET_REF.to_owned(),
            ],
        },
    ]
}

fn seeded_surface_bindings(
    packet_id: &str,
    certification_row_count: usize,
) -> Vec<CertificationSurfaceBinding> {
    vec![
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::HelpAbout,
            consumer_ref: M5_FAULT_CRASH_CERTIFICATION_DOC_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count,
            required_verbatim_fields: REQUIRED_PROJECTION_FIELDS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Help/About reuses the certification row ids, profile tokens, published state, and stale-proof tokens verbatim instead of paraphrasing host-failure maturity.".to_owned(),
        },
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::ServiceHealth,
            consumer_ref: SERVICE_HEALTH_CONSUMER_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count,
            required_verbatim_fields: REQUIRED_PROJECTION_FIELDS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Service-health surfaces ingest the same certification index so crash-loop and schema-governance degradations do not drift from Help/About or support-export truth.".to_owned(),
        },
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::SupportExport,
            consumer_ref: SUPPORT_EXPORT_CONSUMER_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count,
            required_verbatim_fields: REQUIRED_PROJECTION_FIELDS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Support-export packets attach the same row ids and downgrade tokens instead of inventing a parallel crash-support badge.".to_owned(),
        },
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::ReleaseManifest,
            consumer_ref: RELEASE_MANIFEST_CONSUMER_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count,
            required_verbatim_fields: REQUIRED_PROJECTION_FIELDS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Release manifests consume the same certification index so stale symbolication or schema evidence cannot keep a broader release claim green.".to_owned(),
        },
    ]
}
