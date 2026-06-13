//! Symbolication manifests, reports, and cross-surface fidelity labels.
//!
//! This module owns the canonical M5 packet that ties exact-build symbol and
//! source-map manifests to local-first symbolication reports and the user-visible
//! fidelity labels consumed by debug, profiler, preview, browser-runtime, and
//! support surfaces.
//!
//! The packet deliberately reuses the existing vocabulary frozen in
//! `docs/execution/debug_truth_contract.md`,
//! `docs/debug/artifact_resolution_seed.md`, and the execution-facing mapping
//! quality and crash-dump-card schemas instead of inventing a parallel label set.
//! Its job is narrower: keep symbol manifests, symbol-source selection, exact-build
//! match state, mirror policy, unresolved-frame counts, and cross-surface
//! disclosure truth in one attributable record.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto every symbolication contract packet.
pub const SYMBOLICATION_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`SymbolicationContractPacket`].
pub const SYMBOLICATION_CONTRACT_RECORD_KIND: &str = "symbolication_contract_packet";

/// Repo-relative path to the checked-in packet JSON.
pub const SYMBOLICATION_CONTRACT_PACKET_PATH: &str = "artifacts/debug/symbolication_contract.json";

/// Embedded checked-in packet JSON.
pub const SYMBOLICATION_CONTRACT_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/debug/symbolication_contract.json"
));

/// Repo-relative path to the boundary schema.
pub const SYMBOLICATION_CONTRACT_SCHEMA_REF: &str =
    "schemas/debug/symbolication_contract.schema.json";

/// Repo-relative path to the reviewer-facing contract document.
pub const SYMBOLICATION_CONTRACT_DOC_REF: &str = "docs/debug/symbolication.md";

/// Repo-relative path to the reviewer-facing evidence note.
pub const SYMBOLICATION_CONTRACT_ARTIFACT_DOC_REF: &str =
    "artifacts/debug/symbolication_contract.md";

/// Repo-relative path to the protected fixture corpus.
pub const SYMBOLICATION_CONTRACT_FIXTURE_DIR: &str = "fixtures/debug/symbolication";

/// Where a manifest, source usage row, or report is consumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolicationSurfaceKind {
    /// Debugger frame stack or current-frame strip.
    DebugFrameStack,
    /// Inspect-only crash or dump card.
    CrashDumpCard,
    /// Profiler hotspot workspace.
    ProfilerHotspotWorkspace,
    /// Shared trace-viewer or replay-event lane.
    ProfilerTraceViewer,
    /// Preview/runtime frame or inspect-to-source row.
    PreviewRuntimeFrame,
    /// Browser-runtime stack or inspector jump.
    BrowserRuntimeStack,
    /// Support export / escalation packet projection.
    SupportExportPacket,
    /// Incident or crash-review card.
    IncidentCrashCard,
}

impl SymbolicationSurfaceKind {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebugFrameStack => "debug_frame_stack",
            Self::CrashDumpCard => "crash_dump_card",
            Self::ProfilerHotspotWorkspace => "profiler_hotspot_workspace",
            Self::ProfilerTraceViewer => "profiler_trace_viewer",
            Self::PreviewRuntimeFrame => "preview_runtime_frame",
            Self::BrowserRuntimeStack => "browser_runtime_stack",
            Self::SupportExportPacket => "support_export_packet",
            Self::IncidentCrashCard => "incident_crash_card",
        }
    }
}

/// Closed artifact-format vocabulary for symbol and source-map manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DebugFormatClass {
    /// Windows PDB symbols.
    #[serde(rename = "pdb")]
    Pdb,
    /// Apple dSYM symbols.
    #[serde(rename = "dsym")]
    Dsym,
    /// Split or bundled DWARF symbols.
    #[serde(rename = "dwarf")]
    Dwarf,
    /// JavaScript source-map bundle.
    #[serde(rename = "javascript_source_map")]
    JavaScriptSourceMap,
    /// TypeScript source-map bundle.
    #[serde(rename = "typescript_source_map")]
    TypeScriptSourceMap,
    /// CSS source-map bundle.
    #[serde(rename = "css_source_map")]
    CssSourceMap,
}

impl DebugFormatClass {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pdb => "pdb",
            Self::Dsym => "dsym",
            Self::Dwarf => "dwarf",
            Self::JavaScriptSourceMap => "javascript_source_map",
            Self::TypeScriptSourceMap => "typescript_source_map",
            Self::CssSourceMap => "css_source_map",
        }
    }
}

/// Storage or retention posture for symbol/source-map material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionPostureClass {
    /// Bounded local cache or workspace output.
    LocalBounded,
    /// Release artifact-graph ref or trusted local store.
    ReleaseRetainedReference,
    /// Enterprise or self-hosted mirror retention.
    MirrorRetainedReference,
    /// Explicit user import with bounded lifetime.
    ImportedEphemeral,
}

impl RetentionPostureClass {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalBounded => "local_bounded",
            Self::ReleaseRetainedReference => "release_retained_reference",
            Self::MirrorRetainedReference => "mirror_retained_reference",
            Self::ImportedEphemeral => "imported_ephemeral",
        }
    }
}

/// Source identity for symbolication inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceIdentityClass {
    /// Workspace output or local resolver cache.
    LocalWorkspaceOrCache,
    /// Local trusted artifact store or release ref.
    LocalTrustedStore,
    /// Enterprise-controlled mirror.
    MirroredEnterpriseStore,
    /// Managed mirror disclosed by the product.
    MirroredManagedStore,
    /// Explicit user side-load or imported evidence.
    ImportedAttachment,
}

impl SourceIdentityClass {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspaceOrCache => "local_workspace_or_cache",
            Self::LocalTrustedStore => "local_trusted_store",
            Self::MirroredEnterpriseStore => "mirrored_enterprise_store",
            Self::MirroredManagedStore => "mirrored_managed_store",
            Self::ImportedAttachment => "imported_attachment",
        }
    }

    /// Returns true when the source identity is mirrored.
    pub const fn is_mirrored(self) -> bool {
        matches!(
            self,
            Self::MirroredEnterpriseStore | Self::MirroredManagedStore
        )
    }
}

/// Where a report found or attempted a symbol/source-map input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionSourceClass {
    /// Workspace build output or last verified run output.
    WorkspaceOutput,
    /// Bounded local resolver cache.
    LocalResolverCache,
    /// Trusted local symbol/source-map store.
    LocalTrustedStore,
    /// Enterprise or managed mirror.
    MirroredStore,
    /// Explicit user attachment.
    UserAttachedImport,
}

impl ResolutionSourceClass {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceOutput => "workspace_output",
            Self::LocalResolverCache => "local_resolver_cache",
            Self::LocalTrustedStore => "local_trusted_store",
            Self::MirroredStore => "mirrored_store",
            Self::UserAttachedImport => "user_attached_import",
        }
    }
}

/// Exact-build match state carried beside the user-facing fidelity label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildMatchState {
    /// Build identity matched field-for-field.
    ExactBuildVerified,
    /// Resolver found only an approximate candidate.
    ApproximateCandidateOnly,
    /// Resolver found a candidate but rejected it because the build mismatched.
    MismatchedCandidateRejected,
    /// Resolver found no candidate to verify.
    NoCandidateAvailable,
}

impl BuildMatchState {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildVerified => "exact_build_verified",
            Self::ApproximateCandidateOnly => "approximate_candidate_only",
            Self::MismatchedCandidateRejected => "mismatched_candidate_rejected",
            Self::NoCandidateAvailable => "no_candidate_available",
        }
    }

    /// Returns true when the state proves an exact-build match.
    pub const fn proves_exact_build(self) -> bool {
        matches!(self, Self::ExactBuildVerified)
    }
}

/// Closed user-facing fidelity vocabulary shared across M5 symbolication consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolicationFidelityLabel {
    /// Exact symbol and source mapping backed by an exact-build match.
    Exact,
    /// Approximate source mapping, such as line-only or stale-but-nearby mapping.
    Approximate,
    /// Symbol name resolved but authoritative source lines are absent.
    SymbolOnly,
    /// Source and symbol resolution remain insufficient to claim mapping.
    Unresolved,
}

impl SymbolicationFidelityLabel {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::SymbolOnly => "symbol_only",
            Self::Unresolved => "unresolved",
        }
    }

    /// Returns true when the label allows direct source navigation.
    pub const fn allows_source_navigation(self) -> bool {
        matches!(self, Self::Exact | Self::Approximate)
    }
}

/// Report/export redaction posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolicationRedactionClass {
    /// Metadata-only or hashes-only export posture.
    MetadataAndHashesOnly,
    /// Support/export packet may include bounded code-adjacent summaries.
    SupportBundleScoped,
    /// Operator-only review or broadened escalation path.
    OperatorOnlyRestricted,
}

impl SymbolicationRedactionClass {
    /// Stable snake-case token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataAndHashesOnly => "metadata_and_hashes_only",
            Self::SupportBundleScoped => "support_bundle_scoped",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
        }
    }
}

/// One symbol or source-map manifest row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolManifestRow {
    /// Stable manifest id.
    pub manifest_id: String,
    /// Stable debug-artifact ref for the symbol or source-map artifact.
    pub debug_artifact_ref: String,
    /// Stable exact-build identity ref the artifact belongs to.
    pub exact_build_identity_ref: String,
    /// Stable build id surfaced to users without opening the exact-build packet.
    pub build_id: String,
    /// Target triple the symbol or source-map artifact binds to.
    pub target_triple: String,
    /// Native symbol format or source-map class.
    pub debug_format: DebugFormatClass,
    /// Digest of the symbol or source-map artifact itself.
    pub artifact_digest: String,
    /// Retention posture for the artifact.
    pub retention_posture: RetentionPostureClass,
    /// Local, mirrored, or imported source identity.
    pub source_identity: SourceIdentityClass,
    /// Stable source or store ref.
    pub source_ref: String,
    /// Optional mirror policy ref when the source is mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_policy_ref: Option<String>,
    /// True when the row explicitly discloses that the source is mirrored.
    pub mirrored_source_visible: bool,
    /// Optional module or source-map identity ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_identity_ref: Option<String>,
    /// Reviewer-facing note.
    pub notes: String,
}

/// One mirror-policy row governing mirrored symbolication inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorPolicyRow {
    /// Stable policy id.
    pub policy_id: String,
    /// Mirrored source identity governed by the policy.
    pub source_identity: SourceIdentityClass,
    /// Stable mirror/store ref.
    pub mirror_source_ref: String,
    /// Signer or provenance identity ref for the mirror.
    pub signer_identity_ref: String,
    /// Stable access-policy ref.
    pub access_policy_ref: String,
    /// Retention posture declared by the mirror.
    pub retention_posture: RetentionPostureClass,
    /// True when local lookup stays authoritative and mirror lookup only follows a local miss.
    pub requires_local_miss_before_lookup: bool,
    /// True when mirror use is disclosed to the user.
    pub mirror_usage_disclosed: bool,
    /// Reviewer-facing note.
    pub notes: String,
}

/// One source-usage row inside a local or mirrored symbolication report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicationSourceUsageRow {
    /// Stable usage id.
    pub source_usage_id: String,
    /// Stable manifest id naming the source artifact.
    pub manifest_id: String,
    /// Where the symbolicator found or attempted the source.
    pub resolution_source_class: ResolutionSourceClass,
    /// Stable source/store ref for this lookup.
    pub source_ref: String,
    /// Identity posture of the selected source.
    pub source_identity: SourceIdentityClass,
    /// Exact-build match result for this selected source.
    pub build_match_state: BuildMatchState,
    /// True when the source was actually selected for symbolication.
    pub selected_for_symbolication: bool,
    /// True when the row visibly explains mirrored use or non-use.
    pub mirror_usage_visible: bool,
    /// Reviewer-facing note.
    pub notes: String,
}

/// One symbolication report row for crash, profile, preview, or support evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicationReportRow {
    /// Stable report id.
    pub report_id: String,
    /// Stable context ref such as a crash dump, profile capture, or support export packet.
    pub context_ref: String,
    /// Stable exact-build identity ref expected by the context.
    pub primary_exact_build_identity_ref: String,
    /// Ordered source-usage refs consulted by the report.
    pub symbol_source_usage_refs: Vec<String>,
    /// User-visible roll-up fidelity label.
    pub fidelity_label: SymbolicationFidelityLabel,
    /// Exact-build match state behind the roll-up label.
    pub build_match_state: BuildMatchState,
    /// Count of frames mapped exactly.
    pub exact_frame_count: u32,
    /// Count of frames mapped approximately.
    pub approximate_frame_count: u32,
    /// Count of frames with symbol-only resolution.
    pub symbol_only_frame_count: u32,
    /// Count of unresolved frames.
    pub unresolved_frame_count: u32,
    /// Redaction posture for the report.
    pub redaction_class: SymbolicationRedactionClass,
    /// True when the report keeps mirror use visible.
    pub mirror_usage_visible: bool,
    /// True when local-first semantics remained authoritative.
    pub local_first_semantics_preserved: bool,
    /// Reviewer-facing note.
    pub notes: String,
}

/// One per-surface projection of the symbolication report truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceProjectionRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Surface family consuming the report.
    pub surface_kind: SymbolicationSurfaceKind,
    /// Stable context ref rendered by the surface.
    pub context_ref: String,
    /// Stable symbolication report ref consumed by the surface.
    pub report_ref: String,
    /// Surface-visible fidelity label.
    pub fidelity_label: SymbolicationFidelityLabel,
    /// Exact-build match state the surface preserves.
    pub build_match_state: BuildMatchState,
    /// Count of unresolved frames surfaced by the row.
    pub unresolved_frame_count: u32,
    /// True when the surface shows the fidelity label directly.
    pub shows_fidelity_label: bool,
    /// True when the surface lets the user inspect symbol sources used.
    pub shows_symbol_sources_used: bool,
    /// True when a build mismatch is disclosed before any source jump or trust decision.
    pub shows_build_mismatch_before_navigation: bool,
    /// True when mirrored symbol/source-map use is disclosed.
    pub shows_mirror_visibility: bool,
    /// True when the surface preserves the redaction class.
    pub shows_redaction_class: bool,
    /// Reviewer-facing note.
    pub notes: String,
}

/// Summary counts carried by [`SymbolicationContractPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicationContractSummary {
    /// Total manifest rows.
    pub manifest_count: usize,
    /// Total symbolication reports.
    pub report_count: usize,
    /// Total surface projections.
    pub surface_count: usize,
    /// Number of exact reports.
    pub exact_report_count: usize,
    /// Number of approximate reports.
    pub approximate_report_count: usize,
    /// Number of symbol-only reports.
    pub symbol_only_report_count: usize,
    /// Number of unresolved reports.
    pub unresolved_report_count: usize,
}

/// One validation failure produced by [`SymbolicationContractPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicationContractViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewer-facing explanation.
    pub message: String,
}

/// Canonical symbolication packet for M5 debug, profiler, preview, and support flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicationContractPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Human-readable evidence doc ref.
    pub artifact_doc_ref: String,
    /// Authoritative spec anchors quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Symbol or source-map manifest rows.
    pub manifests: Vec<SymbolManifestRow>,
    /// Mirror-policy rows.
    pub mirror_policies: Vec<MirrorPolicyRow>,
    /// Source-selection rows.
    pub source_usages: Vec<SymbolicationSourceUsageRow>,
    /// Local or mirrored symbolication reports.
    pub reports: Vec<SymbolicationReportRow>,
    /// Cross-surface projections.
    pub surfaces: Vec<SurfaceProjectionRow>,
    /// Precomputed roll-up summary.
    pub summary: SymbolicationContractSummary,
}

impl SymbolicationContractPacket {
    /// Computes the roll-up summary from the report set.
    pub fn computed_summary(&self) -> SymbolicationContractSummary {
        let exact_report_count = self
            .reports
            .iter()
            .filter(|report| report.fidelity_label == SymbolicationFidelityLabel::Exact)
            .count();
        let approximate_report_count = self
            .reports
            .iter()
            .filter(|report| report.fidelity_label == SymbolicationFidelityLabel::Approximate)
            .count();
        let symbol_only_report_count = self
            .reports
            .iter()
            .filter(|report| report.fidelity_label == SymbolicationFidelityLabel::SymbolOnly)
            .count();
        let unresolved_report_count = self
            .reports
            .iter()
            .filter(|report| report.fidelity_label == SymbolicationFidelityLabel::Unresolved)
            .count();

        SymbolicationContractSummary {
            manifest_count: self.manifests.len(),
            report_count: self.reports.len(),
            surface_count: self.surfaces.len(),
            exact_report_count,
            approximate_report_count,
            symbol_only_report_count,
            unresolved_report_count,
        }
    }

    /// Validates referential integrity and the local-first symbolication rules.
    pub fn validate(&self) -> Vec<SymbolicationContractViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SYMBOLICATION_CONTRACT_RECORD_KIND {
            violations.push(SymbolicationContractViolation {
                path: "record_kind".to_owned(),
                message: format!("record_kind must be {SYMBOLICATION_CONTRACT_RECORD_KIND}"),
            });
        }
        if self.schema_version != SYMBOLICATION_CONTRACT_SCHEMA_VERSION {
            violations.push(SymbolicationContractViolation {
                path: "schema_version".to_owned(),
                message: format!("schema_version must be {SYMBOLICATION_CONTRACT_SCHEMA_VERSION}"),
            });
        }
        if self.doc_ref != SYMBOLICATION_CONTRACT_DOC_REF {
            violations.push(SymbolicationContractViolation {
                path: "doc_ref".to_owned(),
                message: format!("doc_ref must be {SYMBOLICATION_CONTRACT_DOC_REF}"),
            });
        }
        if self.schema_ref != SYMBOLICATION_CONTRACT_SCHEMA_REF {
            violations.push(SymbolicationContractViolation {
                path: "schema_ref".to_owned(),
                message: format!("schema_ref must be {SYMBOLICATION_CONTRACT_SCHEMA_REF}"),
            });
        }
        if self.artifact_doc_ref != SYMBOLICATION_CONTRACT_ARTIFACT_DOC_REF {
            violations.push(SymbolicationContractViolation {
                path: "artifact_doc_ref".to_owned(),
                message: format!(
                    "artifact_doc_ref must be {SYMBOLICATION_CONTRACT_ARTIFACT_DOC_REF}"
                ),
            });
        }

        let mut manifest_ids = BTreeSet::new();
        let mut manifest_by_id = BTreeMap::new();
        for manifest in &self.manifests {
            if !manifest_ids.insert(manifest.manifest_id.as_str()) {
                violations.push(SymbolicationContractViolation {
                    path: "manifests".to_owned(),
                    message: format!("duplicate manifest_id {}", manifest.manifest_id),
                });
            }
            if manifest.source_identity.is_mirrored() {
                if manifest.mirror_policy_ref.is_none() {
                    violations.push(SymbolicationContractViolation {
                        path: format!("manifests[{}].mirror_policy_ref", manifest.manifest_id),
                        message: "mirrored manifests must cite a mirror policy".to_owned(),
                    });
                }
                if !manifest.mirrored_source_visible {
                    violations.push(SymbolicationContractViolation {
                        path: format!(
                            "manifests[{}].mirrored_source_visible",
                            manifest.manifest_id
                        ),
                        message: "mirrored manifests must disclose mirrored_source_visible"
                            .to_owned(),
                    });
                }
            } else if manifest.mirror_policy_ref.is_some() {
                violations.push(SymbolicationContractViolation {
                    path: format!("manifests[{}].mirror_policy_ref", manifest.manifest_id),
                    message: "non-mirrored manifests must not cite a mirror policy".to_owned(),
                });
            }
            manifest_by_id.insert(manifest.manifest_id.as_str(), manifest);
        }

        let mut mirror_policy_ids = BTreeSet::new();
        for policy in &self.mirror_policies {
            if !mirror_policy_ids.insert(policy.policy_id.as_str()) {
                violations.push(SymbolicationContractViolation {
                    path: "mirror_policies".to_owned(),
                    message: format!("duplicate policy_id {}", policy.policy_id),
                });
            }
            if !policy.source_identity.is_mirrored() {
                violations.push(SymbolicationContractViolation {
                    path: format!("mirror_policies[{}].source_identity", policy.policy_id),
                    message: "mirror policies must govern mirrored source identities".to_owned(),
                });
            }
            if !policy.requires_local_miss_before_lookup {
                violations.push(SymbolicationContractViolation {
                    path: format!(
                        "mirror_policies[{}].requires_local_miss_before_lookup",
                        policy.policy_id
                    ),
                    message: "mirror policies must remain subordinate to local-first lookup"
                        .to_owned(),
                });
            }
            if !policy.mirror_usage_disclosed {
                violations.push(SymbolicationContractViolation {
                    path: format!(
                        "mirror_policies[{}].mirror_usage_disclosed",
                        policy.policy_id
                    ),
                    message: "mirror policies must disclose mirror usage".to_owned(),
                });
            }
        }

        let mut source_usage_ids = BTreeSet::new();
        let mut source_usage_by_id = BTreeMap::new();
        for usage in &self.source_usages {
            if !source_usage_ids.insert(usage.source_usage_id.as_str()) {
                violations.push(SymbolicationContractViolation {
                    path: "source_usages".to_owned(),
                    message: format!("duplicate source_usage_id {}", usage.source_usage_id),
                });
            }
            let Some(manifest) = manifest_by_id.get(usage.manifest_id.as_str()) else {
                violations.push(SymbolicationContractViolation {
                    path: format!("source_usages[{}].manifest_id", usage.source_usage_id),
                    message: format!("unknown manifest_id {}", usage.manifest_id),
                });
                continue;
            };
            if usage.source_ref != manifest.source_ref {
                violations.push(SymbolicationContractViolation {
                    path: format!("source_usages[{}].source_ref", usage.source_usage_id),
                    message: "source usage must quote the manifest source_ref verbatim".to_owned(),
                });
            }
            if usage.source_identity != manifest.source_identity {
                violations.push(SymbolicationContractViolation {
                    path: format!("source_usages[{}].source_identity", usage.source_usage_id),
                    message: "source usage must preserve the manifest source_identity".to_owned(),
                });
            }
            if usage.source_identity.is_mirrored() && !usage.mirror_usage_visible {
                violations.push(SymbolicationContractViolation {
                    path: format!(
                        "source_usages[{}].mirror_usage_visible",
                        usage.source_usage_id
                    ),
                    message: "mirrored source usage must stay visible".to_owned(),
                });
            }
            source_usage_by_id.insert(usage.source_usage_id.as_str(), usage);
        }

        let mut report_ids = BTreeSet::new();
        let mut report_by_id = BTreeMap::new();
        for report in &self.reports {
            if !report_ids.insert(report.report_id.as_str()) {
                violations.push(SymbolicationContractViolation {
                    path: "reports".to_owned(),
                    message: format!("duplicate report_id {}", report.report_id),
                });
            }
            if report.symbol_source_usage_refs.is_empty() {
                violations.push(SymbolicationContractViolation {
                    path: format!("reports[{}].symbol_source_usage_refs", report.report_id),
                    message: "reports must cite at least one symbol source usage".to_owned(),
                });
            }
            let mut any_mirrored_source = false;
            for usage_ref in &report.symbol_source_usage_refs {
                let Some(usage) = source_usage_by_id.get(usage_ref.as_str()) else {
                    violations.push(SymbolicationContractViolation {
                        path: format!("reports[{}].symbol_source_usage_refs", report.report_id),
                        message: format!("unknown source_usage_id {usage_ref}"),
                    });
                    continue;
                };
                any_mirrored_source |= usage.source_identity.is_mirrored();
            }

            if any_mirrored_source && !report.mirror_usage_visible {
                violations.push(SymbolicationContractViolation {
                    path: format!("reports[{}].mirror_usage_visible", report.report_id),
                    message: "reports using mirrored sources must disclose mirror usage".to_owned(),
                });
            }
            if any_mirrored_source && !report.local_first_semantics_preserved {
                violations.push(SymbolicationContractViolation {
                    path: format!(
                        "reports[{}].local_first_semantics_preserved",
                        report.report_id
                    ),
                    message: "reports using mirrored sources must preserve local-first semantics"
                        .to_owned(),
                });
            }

            let total_frames = report.exact_frame_count
                + report.approximate_frame_count
                + report.symbol_only_frame_count
                + report.unresolved_frame_count;
            if total_frames == 0 {
                violations.push(SymbolicationContractViolation {
                    path: format!("reports[{}]", report.report_id),
                    message: "reports must cover at least one frame".to_owned(),
                });
            }

            match report.fidelity_label {
                SymbolicationFidelityLabel::Exact => {
                    if !report.build_match_state.proves_exact_build()
                        || report.approximate_frame_count != 0
                        || report.symbol_only_frame_count != 0
                        || report.unresolved_frame_count != 0
                        || report.exact_frame_count == 0
                    {
                        violations.push(SymbolicationContractViolation {
                            path: format!("reports[{}].fidelity_label", report.report_id),
                            message:
                                "exact reports require an exact-build match and only exact frames"
                                    .to_owned(),
                        });
                    }
                }
                SymbolicationFidelityLabel::Approximate => {
                    if report.approximate_frame_count == 0
                        || matches!(
                            report.build_match_state,
                            BuildMatchState::MismatchedCandidateRejected
                                | BuildMatchState::NoCandidateAvailable
                        )
                    {
                        violations.push(SymbolicationContractViolation {
                            path: format!("reports[{}].fidelity_label", report.report_id),
                            message:
                                "approximate reports require approximate frames and a non-rejected candidate"
                                    .to_owned(),
                        });
                    }
                }
                SymbolicationFidelityLabel::SymbolOnly => {
                    if report.symbol_only_frame_count == 0 || report.unresolved_frame_count != 0 {
                        violations.push(SymbolicationContractViolation {
                            path: format!("reports[{}].fidelity_label", report.report_id),
                            message:
                                "symbol-only reports require symbol-only frames and no unresolved frames"
                                    .to_owned(),
                        });
                    }
                }
                SymbolicationFidelityLabel::Unresolved => {
                    if report.unresolved_frame_count == 0 {
                        violations.push(SymbolicationContractViolation {
                            path: format!("reports[{}].fidelity_label", report.report_id),
                            message: "unresolved reports require unresolved frames".to_owned(),
                        });
                    }
                }
            }

            if report.build_match_state == BuildMatchState::MismatchedCandidateRejected
                && report.fidelity_label != SymbolicationFidelityLabel::Unresolved
            {
                violations.push(SymbolicationContractViolation {
                    path: format!("reports[{}].build_match_state", report.report_id),
                    message: "rejected mismatched candidates must narrow the report to unresolved"
                        .to_owned(),
                });
            }
            report_by_id.insert(report.report_id.as_str(), report);
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.as_str()) {
                violations.push(SymbolicationContractViolation {
                    path: "surfaces".to_owned(),
                    message: format!("duplicate surface_id {}", surface.surface_id),
                });
            }
            let Some(report) = report_by_id.get(surface.report_ref.as_str()) else {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].report_ref", surface.surface_id),
                    message: format!("unknown report_ref {}", surface.report_ref),
                });
                continue;
            };
            if surface.fidelity_label != report.fidelity_label {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].fidelity_label", surface.surface_id),
                    message: "surface must preserve the report fidelity label verbatim".to_owned(),
                });
            }
            if surface.build_match_state != report.build_match_state {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].build_match_state", surface.surface_id),
                    message: "surface must preserve the report build match state verbatim"
                        .to_owned(),
                });
            }
            if surface.unresolved_frame_count != report.unresolved_frame_count {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].unresolved_frame_count", surface.surface_id),
                    message: "surface must preserve the report unresolved-frame count verbatim"
                        .to_owned(),
                });
            }
            if !surface.shows_fidelity_label {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].shows_fidelity_label", surface.surface_id),
                    message: "every surface must show the fidelity label".to_owned(),
                });
            }
            if !surface.shows_symbol_sources_used {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].shows_symbol_sources_used", surface.surface_id),
                    message: "every surface must let users inspect symbol sources used".to_owned(),
                });
            }
            if report.build_match_state == BuildMatchState::MismatchedCandidateRejected
                && !surface.shows_build_mismatch_before_navigation
            {
                violations.push(SymbolicationContractViolation {
                    path: format!(
                        "surfaces[{}].shows_build_mismatch_before_navigation",
                        surface.surface_id
                    ),
                    message:
                        "surfaces must show exact-build mismatches before any source navigation"
                            .to_owned(),
                });
            }
            if report.mirror_usage_visible && !surface.shows_mirror_visibility {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].shows_mirror_visibility", surface.surface_id),
                    message: "surfaces must disclose mirrored symbol/source-map usage".to_owned(),
                });
            }
            if !surface.shows_redaction_class {
                violations.push(SymbolicationContractViolation {
                    path: format!("surfaces[{}].shows_redaction_class", surface.surface_id),
                    message: "surfaces must preserve the report redaction class".to_owned(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(SymbolicationContractViolation {
                path: "summary".to_owned(),
                message: "summary does not match the packet contents".to_owned(),
            });
        }

        violations
    }
}

/// Parse the embedded checked-in packet.
pub fn current_symbolication_contract(
) -> Result<SymbolicationContractPacket, SymbolicationContractArtifactError> {
    serde_json::from_str(SYMBOLICATION_CONTRACT_PACKET_JSON)
        .map_err(SymbolicationContractArtifactError::Parse)
}

/// Errors returned by [`current_symbolication_contract`].
#[derive(Debug)]
pub enum SymbolicationContractArtifactError {
    /// The embedded JSON packet did not parse.
    Parse(serde_json::Error),
}

impl fmt::Display for SymbolicationContractArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "failed to parse symbolication contract packet: {err}"),
        }
    }
}

impl Error for SymbolicationContractArtifactError {}
