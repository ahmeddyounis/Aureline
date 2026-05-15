//! Crash incident-trail state model.
//!
//! The model consumes the same fixture-shaped envelope and symbolication
//! records used by the exact-build smoke path, then produces one trail record
//! that support, recovery, and export surfaces can inspect without parsing raw
//! dumps or stack bodies.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried on every crash incident-trail record.
pub const CRASH_INCIDENT_TRAIL_RECORD_KIND: &str = "crash_incident_trail_alpha_record";

/// Schema version for the alpha incident-trail payload.
pub const CRASH_INCIDENT_TRAIL_SCHEMA_VERSION: u32 = 1;

/// Crash-envelope metadata captured by the local crash handler or supervisor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashEnvelope {
    /// Envelope schema version from the producer.
    pub schema_version: u32,
    /// Producer record-kind tag.
    pub record_kind: String,
    /// Optional fixture id used by protected tests.
    #[serde(default)]
    pub fixture_id: Option<String>,
    /// Stable crash envelope ref.
    pub crash_envelope_ref: String,
    /// RFC 3339 UTC capture time.
    pub captured_at: String,
    /// Chronology capture posture such as `captured_without_recording`.
    pub chronology_capture_state: String,
    /// Fault-domain id that bounded the crash.
    pub fault_domain_id: String,
    /// Primary exact-build identity for the crashing runtime.
    pub primary_exact_build_identity_ref: String,
    /// Manifest ref for the raw dump or core metadata.
    pub crash_dump_manifest_ref: String,
    /// Support bundle ref selected by the crash capture path.
    pub support_bundle_ref: String,
    /// Trace IDs known to overlap the crash window.
    #[serde(default)]
    pub trace_ids: Vec<String>,
    /// Module identities observed in the crash envelope.
    #[serde(default)]
    pub modules: Vec<CrashModule>,
}

/// Module identity captured in a crash envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashModule {
    /// Stable module id in the envelope.
    pub module_id: String,
    /// Module class such as `native_binary` or `web_bundle`.
    pub module_kind: String,
    /// Release artifact family for the module.
    pub artifact_family_class: String,
    /// Exact-build identity ref for this module or source-map family.
    pub exact_build_identity_ref: String,
    /// Optional module identity details used by local symbolication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_identity: Option<CrashModuleIdentity>,
    /// Faulting frames captured for this module.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub faulting_frames: Vec<CrashFrame>,
}

/// Module-specific identity fields carried by crash envelopes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashModuleIdentity {
    /// Native code file name when the module is a binary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_file_name: Option<String>,
    /// Native build id when the module is a binary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// Native debug id when the module is a binary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_id: Option<String>,
    /// Native image base when the module is a binary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_base: Option<String>,
    /// Renderer bundle revision ref when the module is a generated asset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_revision_ref: Option<String>,
    /// Source-map digest when the module is a generated asset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_map_digest: Option<String>,
    /// Generated asset ref when the module is a generated asset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_asset_ref: Option<String>,
}

/// One faulting frame captured in a crash envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashFrame {
    /// Stable frame index within the captured stack.
    pub frame_index: u32,
    /// Native instruction address for binary frames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Generated source location for source-map frames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_location: Option<String>,
    /// Symbol name or generated function hint captured with the frame.
    pub symbol_hint: String,
}

/// Metadata-only manifest for a dump or core artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashDumpManifest {
    /// Manifest schema version from the producer.
    pub schema_version: u32,
    /// Producer record-kind tag.
    pub record_kind: String,
    /// Stable dump or core ref.
    pub crash_dump_ref: String,
    /// RFC 3339 UTC capture time.
    pub captured_at: String,
    /// Dump format class such as `minidump`.
    pub dump_format_class: String,
    /// Record-class registry id.
    pub record_class_id: String,
    /// Diagnostic data class for the dump metadata.
    pub data_class: String,
    /// Redaction class governing this manifest.
    pub redaction_class: String,
    /// Default support export posture.
    pub support_export_posture: String,
    /// Storage mode for the raw dump body.
    pub storage_mode: String,
    /// Whether the body is embedded, by reference, or omitted.
    pub embedding_state: String,
    /// Optional artifact digest for the retained dump body.
    #[serde(default)]
    pub artifact_sha256: Option<String>,
    /// Optional local crash-store retention ref.
    #[serde(default)]
    pub local_retention_ref: Option<String>,
    /// Primary exact-build identity for the dump.
    pub primary_exact_build_identity_ref: String,
    /// Fault domains referenced by this dump.
    #[serde(default)]
    pub fault_domain_refs: Vec<String>,
    /// Module refs observed in the dump manifest.
    #[serde(default)]
    pub module_refs: Vec<String>,
    /// Support bundle ref selected by the dump manifest.
    pub support_bundle_ref: String,
    /// Redaction-safe reviewer note.
    pub notes: String,
}

/// Local symbolication report generated for a crash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicationReport {
    /// Report schema version from the producer.
    pub schema_version: u32,
    /// Producer record-kind tag.
    pub record_kind: String,
    /// Optional fixture id used by protected tests.
    #[serde(default)]
    pub fixture_id: Option<String>,
    /// Stable symbolication report ref.
    pub symbolication_report_ref: String,
    /// RFC 3339 UTC report generation time.
    pub generated_at: String,
    /// Crash envelope ref the report was built from.
    pub crash_envelope_ref: String,
    /// Primary exact-build identity used by the report.
    pub primary_exact_build_identity_ref: String,
    /// Producer result state, for example `exact_match` or `partial`.
    pub result_state: String,
    /// Per-module mapping results.
    #[serde(default)]
    pub module_results: Vec<SymbolicatedModuleResult>,
    /// Dump or core ref resolved by the report.
    pub crash_dump_ref: String,
    /// Support bundle ref selected by the report.
    pub support_bundle_ref: String,
    /// Optional release evidence packet ref.
    #[serde(default)]
    pub release_evidence_packet_ref: Option<String>,
    /// Optional release claim refs.
    #[serde(default)]
    pub claim_row_refs: Vec<String>,
    /// Optional retention seed ref used by the report.
    #[serde(default)]
    pub retention_seed_ref: Option<String>,
    /// Redaction-safe reviewer note.
    pub notes: String,
}

/// Per-module symbolication result in a local report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicatedModuleResult {
    /// Stable module id.
    pub module_id: String,
    /// Module class such as `native_binary` or `web_bundle`.
    pub module_kind: String,
    /// Mapping quality token emitted by the symbolicator.
    pub mapping_state: String,
    /// Runtime exact-build identity ref.
    pub runtime_identity_ref: String,
    /// Symbol or source-map identity used for this module.
    #[serde(default)]
    pub symbolication_identity_ref: Option<String>,
    /// Optional crash-symbol archive identity used for native modules.
    #[serde(default)]
    pub support_archive_identity_ref: Option<String>,
    /// Optional symbol tag matched by the local symbolicator.
    #[serde(default)]
    pub matched_symbol_tag: Option<String>,
    /// Optional unresolved reason emitted by partial reports.
    #[serde(default)]
    pub unresolved_reason: Option<String>,
    /// Redaction-safe symbolicated frame summaries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resolved_frame_summary: Vec<String>,
}

/// Inputs used to mint one [`CrashIncidentTrail`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashIncidentTrailInputs {
    /// Stable trail id.
    pub incident_trail_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Alpha publication or partner-channel ref that scoped the crash.
    pub alpha_channel_ref: String,
    /// Captured crash envelope.
    pub crash_envelope: CrashEnvelope,
    /// Metadata-only dump manifest.
    pub crash_dump_manifest: CrashDumpManifest,
    /// Optional local symbolication report.
    pub symbolication_report: Option<SymbolicationReport>,
    /// Support-bundle manifest ref that should carry this trail.
    pub support_bundle_manifest_ref: Option<String>,
    /// Optional local preview snapshot ref for the support bundle.
    pub support_preview_snapshot_ref: Option<String>,
}

/// Crash incident trail joining crash, symbolication, and support bundle refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashIncidentTrail {
    /// Incident-trail schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable trail id.
    pub incident_trail_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Alpha publication or partner-channel ref that scoped the crash.
    pub alpha_channel_ref: String,
    /// Stable crash envelope ref.
    pub crash_envelope_ref: String,
    /// Stable dump or core ref.
    pub crash_dump_ref: String,
    /// Optional symbolication report ref.
    #[serde(default)]
    pub symbolication_report_ref: Option<String>,
    /// Primary exact-build identity for the crashing runtime.
    pub primary_exact_build_identity_ref: String,
    /// Fault-domain id that bounded the crash.
    pub fault_domain_id: String,
    /// RFC 3339 UTC capture time.
    pub captured_at: String,
    /// Chronology capture posture such as `captured_without_recording`.
    pub chronology_capture_state: String,
    /// Trace IDs known to overlap the crash window.
    pub trace_ids: Vec<String>,
    /// Overall symbolication state for this crash.
    pub symbolication_state: SymbolicationState,
    /// Per-module mapping summary.
    pub module_summaries: Vec<ModuleIncidentSummary>,
    /// Support-bundle linkage state.
    pub support_bundle_linkage: SupportBundleLinkage,
    /// Redaction-safe evidence refs needed to reconstruct the incident.
    pub evidence_refs: Vec<IncidentEvidenceRef>,
    /// Safest next actions preserved for the crash path.
    pub next_safe_actions: Vec<NextSafeAction>,
    /// Honest notes about missing, partial, or mismatched evidence.
    pub honesty_notes: Vec<String>,
    /// Always false for the alpha trail; raw dumps stay local or opt-in only.
    pub raw_dump_exported: bool,
    /// Redaction-safe reviewer summary.
    pub notes: String,
}

impl CrashIncidentTrail {
    /// Mint an incident trail from a crash envelope, dump manifest, optional
    /// symbolication report, and support-bundle manifest ref.
    pub fn from_inputs(inputs: CrashIncidentTrailInputs) -> Self {
        let report_ref = inputs
            .symbolication_report
            .as_ref()
            .map(|report| report.symbolication_report_ref.clone());
        let report_by_module = inputs
            .symbolication_report
            .as_ref()
            .map(module_results_by_id)
            .unwrap_or_default();

        let hard_exact_build_mismatch = inputs.crash_dump_manifest.primary_exact_build_identity_ref
            != inputs.crash_envelope.primary_exact_build_identity_ref
            || inputs.symbolication_report.as_ref().is_some_and(|report| {
                report.primary_exact_build_identity_ref
                    != inputs.crash_envelope.primary_exact_build_identity_ref
                    || report.crash_envelope_ref != inputs.crash_envelope.crash_envelope_ref
                    || report.crash_dump_ref != inputs.crash_dump_manifest.crash_dump_ref
            });

        let module_summaries = inputs
            .crash_envelope
            .modules
            .iter()
            .map(|module| {
                summarize_module(
                    module,
                    report_by_module.get(&module.module_id),
                    &inputs.crash_envelope.primary_exact_build_identity_ref,
                    hard_exact_build_mismatch,
                    inputs.symbolication_report.is_some(),
                )
            })
            .collect::<Vec<_>>();

        let symbolication_state = symbolication_state(
            inputs.symbolication_report.as_ref(),
            &module_summaries,
            hard_exact_build_mismatch,
        );
        let support_bundle_linkage = support_bundle_linkage(&inputs);
        let evidence_refs = evidence_refs(&inputs);
        let next_safe_actions = next_safe_actions(&inputs.crash_envelope.fault_domain_id);
        let honesty_notes = honesty_notes(symbolication_state, &support_bundle_linkage);
        let notes = notes_for(symbolication_state, &support_bundle_linkage);

        Self {
            schema_version: CRASH_INCIDENT_TRAIL_SCHEMA_VERSION,
            record_kind: CRASH_INCIDENT_TRAIL_RECORD_KIND.to_owned(),
            incident_trail_id: inputs.incident_trail_id,
            generated_at: inputs.generated_at,
            alpha_channel_ref: inputs.alpha_channel_ref,
            crash_envelope_ref: inputs.crash_envelope.crash_envelope_ref,
            crash_dump_ref: inputs.crash_dump_manifest.crash_dump_ref,
            symbolication_report_ref: report_ref,
            primary_exact_build_identity_ref: inputs
                .crash_envelope
                .primary_exact_build_identity_ref,
            fault_domain_id: inputs.crash_envelope.fault_domain_id,
            captured_at: inputs.crash_envelope.captured_at,
            chronology_capture_state: inputs.crash_envelope.chronology_capture_state,
            trace_ids: inputs.crash_envelope.trace_ids,
            symbolication_state,
            module_summaries,
            support_bundle_linkage,
            evidence_refs,
            next_safe_actions,
            honesty_notes,
            raw_dump_exported: false,
            notes,
        }
    }

    /// True when the trail is linked to a support-bundle manifest and all
    /// crash-side support refs agree.
    pub fn is_support_bundle_linked(&self) -> bool {
        matches!(
            self.support_bundle_linkage.linkage_state,
            SupportBundleLinkageState::Linked
        )
    }

    /// True when the trail explicitly labels exact, partial, missing, or
    /// mismatched symbolication.
    pub fn labels_symbolication_honestly(&self) -> bool {
        !self.honesty_notes.is_empty()
    }

    /// True when all four crash-loop recovery actions are present.
    pub fn preserves_safe_next_actions(&self) -> bool {
        let has = |kind| {
            self.next_safe_actions
                .iter()
                .any(|action| action.action_kind == kind && action.enabled)
        };
        has(NextSafeActionKind::SafeMode)
            && has(NextSafeActionKind::OpenWithoutRestore)
            && has(NextSafeActionKind::ExportEvidence)
            && has(NextSafeActionKind::RetryFaultDomain)
    }
}

/// Overall symbolication state for an incident trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolicationState {
    /// Every known module used exact-build symbols or source maps.
    Exact,
    /// At least one module is unresolved or less than exact.
    Partial,
    /// No symbolication report was available.
    Missing,
    /// Exact-build identities disagreed across envelope, dump, or report.
    BuildMismatch,
}

impl SymbolicationState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::BuildMismatch => "build_mismatch",
        }
    }
}

/// Mapping quality for one crash module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleMappingQuality {
    /// Module symbols or source maps matched the exact-build family.
    Exact,
    /// Module was partially mapped but at least one frame remains weaker than exact.
    Partial,
    /// Module had no report row or no symbolication report existed.
    Missing,
    /// Module mapping used a mismatched exact-build family.
    BuildMismatch,
}

impl ModuleMappingQuality {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::BuildMismatch => "build_mismatch",
        }
    }
}

/// Per-module incident trail summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleIncidentSummary {
    /// Stable module id.
    pub module_id: String,
    /// Module class such as `native_binary` or `web_bundle`.
    pub module_kind: String,
    /// Exact-build identity ref from the crash envelope.
    pub envelope_exact_build_identity_ref: String,
    /// Optional symbolication identity ref from the report.
    #[serde(default)]
    pub symbolication_identity_ref: Option<String>,
    /// Optional crash-symbol archive identity ref from the report.
    #[serde(default)]
    pub support_archive_identity_ref: Option<String>,
    /// Mapping quality after exact-build checks.
    pub mapping_quality: ModuleMappingQuality,
    /// Optional symbol tag matched by the symbolicator.
    #[serde(default)]
    pub matched_symbol_tag: Option<String>,
    /// Optional reason when mapping is not exact.
    #[serde(default)]
    pub unresolved_reason: Option<String>,
    /// Redaction-safe symbolicated frame summaries carried into support bundles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resolved_frame_summary: Vec<String>,
}

/// Support-bundle linkage state for an incident trail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundleLinkage {
    /// Support bundle ref from the crash envelope.
    pub support_bundle_ref: String,
    /// Optional support-bundle manifest ref for this trail.
    #[serde(default)]
    pub support_bundle_manifest_ref: Option<String>,
    /// Optional local support-preview snapshot ref.
    #[serde(default)]
    pub support_preview_snapshot_ref: Option<String>,
    /// Linkage state after comparing envelope, dump, and report refs.
    pub linkage_state: SupportBundleLinkageState,
    /// Redaction-safe reviewer summary.
    pub summary: String,
}

/// Result of linking crash evidence to a support-bundle manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportBundleLinkageState {
    /// Envelope, dump, report, and manifest refs are joinable.
    Linked,
    /// No support-bundle manifest ref was supplied.
    MissingManifestRef,
    /// Crash envelope, dump, or report named different support bundle refs.
    MismatchedBundleRef,
}

impl SupportBundleLinkageState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::MissingManifestRef => "missing_manifest_ref",
            Self::MismatchedBundleRef => "mismatched_bundle_ref",
        }
    }
}

/// Redaction-safe evidence ref carried by the incident trail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentEvidenceRef {
    /// Evidence kind.
    pub evidence_kind: IncidentEvidenceKind,
    /// Opaque evidence ref.
    pub evidence_ref: String,
    /// Redaction class applied to this evidence.
    pub redaction_class: String,
    /// Whether the evidence is embedded, by reference, local-only, or omitted.
    pub embedding_state: String,
    /// Whether this evidence is needed for first actionable diagnosis.
    pub required_for_first_diagnosis: bool,
}

/// Evidence kinds carried by a crash incident trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentEvidenceKind {
    /// Crash envelope metadata.
    CrashEnvelope,
    /// Dump/core metadata manifest.
    CrashDumpManifest,
    /// Retained local dump/core body ref.
    CrashDump,
    /// Local symbolication report.
    SymbolicationReport,
    /// Support-bundle manifest ref.
    SupportBundleManifest,
    /// Local support-preview snapshot ref.
    SupportPreviewSnapshot,
}

/// Recovery or export action preserved by the trail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextSafeAction {
    /// Action kind.
    pub action_kind: NextSafeActionKind,
    /// Stable action ref.
    pub action_ref: String,
    /// Reviewer-facing label.
    pub label: String,
    /// Whether the action is available in this alpha trail.
    pub enabled: bool,
    /// Blast radius summary.
    pub blast_radius: String,
    /// Whether the action preserves user-owned state by design.
    pub preserves_user_state: bool,
    /// Redaction-safe reviewer note.
    pub notes: String,
}

/// Closed set of safest crash-loop next actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextSafeActionKind {
    /// Reopen in safe mode.
    SafeMode,
    /// Open without restoring the crashed session.
    OpenWithoutRestore,
    /// Export crash and support evidence.
    ExportEvidence,
    /// Retry only the affected fault domain or lane.
    RetryFaultDomain,
}

impl NextSafeActionKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::ExportEvidence => "export_evidence",
            Self::RetryFaultDomain => "retry_fault_domain",
        }
    }
}

fn module_results_by_id(
    report: &SymbolicationReport,
) -> BTreeMap<String, SymbolicatedModuleResult> {
    report
        .module_results
        .iter()
        .map(|module| (module.module_id.clone(), module.clone()))
        .collect()
}

fn summarize_module(
    module: &CrashModule,
    report_module: Option<&SymbolicatedModuleResult>,
    primary_exact_build_ref: &str,
    hard_exact_build_mismatch: bool,
    report_present: bool,
) -> ModuleIncidentSummary {
    let Some(report_module) = report_module else {
        return ModuleIncidentSummary {
            module_id: module.module_id.clone(),
            module_kind: module.module_kind.clone(),
            envelope_exact_build_identity_ref: module.exact_build_identity_ref.clone(),
            symbolication_identity_ref: None,
            support_archive_identity_ref: None,
            mapping_quality: ModuleMappingQuality::Missing,
            matched_symbol_tag: None,
            unresolved_reason: Some(if report_present {
                "module_missing_from_symbolication_report".into()
            } else {
                "symbolication_report_missing".into()
            }),
            resolved_frame_summary: Vec::new(),
        };
    };

    let same_family =
        exact_build_family_matches(primary_exact_build_ref, &module.exact_build_identity_ref)
            && exact_build_family_matches(
                primary_exact_build_ref,
                &report_module.runtime_identity_ref,
            )
            && report_module
                .symbolication_identity_ref
                .as_ref()
                .map_or(true, |identity_ref| {
                    exact_build_family_matches(primary_exact_build_ref, identity_ref)
                })
            && report_module
                .support_archive_identity_ref
                .as_ref()
                .map_or(true, |identity_ref| {
                    exact_build_family_matches(primary_exact_build_ref, identity_ref)
                });

    let mapping_quality = if hard_exact_build_mismatch || !same_family {
        ModuleMappingQuality::BuildMismatch
    } else if report_module.mapping_state == "exact" {
        ModuleMappingQuality::Exact
    } else {
        ModuleMappingQuality::Partial
    };

    let unresolved_reason = match mapping_quality {
        ModuleMappingQuality::Exact => report_module.unresolved_reason.clone(),
        ModuleMappingQuality::BuildMismatch => Some("exact_build_identity_mismatch".into()),
        ModuleMappingQuality::Partial => Some(
            report_module
                .unresolved_reason
                .clone()
                .unwrap_or_else(|| format!("mapping_state_{}", report_module.mapping_state)),
        ),
        ModuleMappingQuality::Missing => Some("symbolication_report_missing".into()),
    };
    let resolved_frame_summary = if mapping_quality == ModuleMappingQuality::BuildMismatch {
        Vec::new()
    } else {
        report_module.resolved_frame_summary.clone()
    };

    ModuleIncidentSummary {
        module_id: module.module_id.clone(),
        module_kind: module.module_kind.clone(),
        envelope_exact_build_identity_ref: module.exact_build_identity_ref.clone(),
        symbolication_identity_ref: report_module.symbolication_identity_ref.clone(),
        support_archive_identity_ref: report_module.support_archive_identity_ref.clone(),
        mapping_quality,
        matched_symbol_tag: report_module.matched_symbol_tag.clone(),
        unresolved_reason,
        resolved_frame_summary,
    }
}

fn exact_build_family_matches(primary_exact_build_ref: &str, candidate: &str) -> bool {
    candidate == primary_exact_build_ref
        || candidate
            .strip_prefix(primary_exact_build_ref)
            .is_some_and(|suffix| suffix.starts_with(':'))
}

fn symbolication_state(
    report: Option<&SymbolicationReport>,
    modules: &[ModuleIncidentSummary],
    hard_exact_build_mismatch: bool,
) -> SymbolicationState {
    if hard_exact_build_mismatch
        || modules
            .iter()
            .any(|module| module.mapping_quality == ModuleMappingQuality::BuildMismatch)
    {
        return SymbolicationState::BuildMismatch;
    }

    let Some(report) = report else {
        return SymbolicationState::Missing;
    };

    if report.result_state == "exact_match"
        && !modules.is_empty()
        && modules
            .iter()
            .all(|module| module.mapping_quality == ModuleMappingQuality::Exact)
    {
        SymbolicationState::Exact
    } else {
        SymbolicationState::Partial
    }
}

fn support_bundle_linkage(inputs: &CrashIncidentTrailInputs) -> SupportBundleLinkage {
    let manifest_ref = inputs.support_bundle_manifest_ref.clone();
    let support_bundle_ref = inputs.crash_envelope.support_bundle_ref.clone();
    let report_matches = inputs.symbolication_report.as_ref().map_or(true, |report| {
        report.support_bundle_ref == support_bundle_ref
    });
    let dump_matches = inputs.crash_dump_manifest.support_bundle_ref == support_bundle_ref;

    let linkage_state = if manifest_ref.is_none() {
        SupportBundleLinkageState::MissingManifestRef
    } else if dump_matches && report_matches {
        SupportBundleLinkageState::Linked
    } else {
        SupportBundleLinkageState::MismatchedBundleRef
    };

    let summary = match linkage_state {
        SupportBundleLinkageState::Linked => {
            "Crash envelope, dump manifest, symbolication report, and support-bundle manifest share one incident trail.".into()
        }
        SupportBundleLinkageState::MissingManifestRef => {
            "Crash evidence is present, but no support-bundle manifest ref has been linked yet.".into()
        }
        SupportBundleLinkageState::MismatchedBundleRef => {
            "Crash evidence names different support-bundle refs; the trail is preserved but not treated as fully linked.".into()
        }
    };

    SupportBundleLinkage {
        support_bundle_ref,
        support_bundle_manifest_ref: manifest_ref,
        support_preview_snapshot_ref: inputs.support_preview_snapshot_ref.clone(),
        linkage_state,
        summary,
    }
}

fn evidence_refs(inputs: &CrashIncidentTrailInputs) -> Vec<IncidentEvidenceRef> {
    let mut refs = vec![
        IncidentEvidenceRef {
            evidence_kind: IncidentEvidenceKind::CrashEnvelope,
            evidence_ref: inputs.crash_envelope.crash_envelope_ref.clone(),
            redaction_class: "metadata_safe_default".into(),
            embedding_state: "embedded_export_copy".into(),
            required_for_first_diagnosis: true,
        },
        IncidentEvidenceRef {
            evidence_kind: IncidentEvidenceKind::CrashDumpManifest,
            evidence_ref: inputs.crash_envelope.crash_dump_manifest_ref.clone(),
            redaction_class: inputs.crash_dump_manifest.redaction_class.clone(),
            embedding_state: "embedded_export_copy".into(),
            required_for_first_diagnosis: true,
        },
        IncidentEvidenceRef {
            evidence_kind: IncidentEvidenceKind::CrashDump,
            evidence_ref: inputs.crash_dump_manifest.crash_dump_ref.clone(),
            redaction_class: inputs.crash_dump_manifest.redaction_class.clone(),
            embedding_state: inputs.crash_dump_manifest.embedding_state.clone(),
            required_for_first_diagnosis: false,
        },
    ];

    if let Some(report) = &inputs.symbolication_report {
        refs.push(IncidentEvidenceRef {
            evidence_kind: IncidentEvidenceKind::SymbolicationReport,
            evidence_ref: report.symbolication_report_ref.clone(),
            redaction_class: "operator_only_restricted".into(),
            embedding_state: "embedded_export_copy".into(),
            required_for_first_diagnosis: true,
        });
    }

    if let Some(manifest_ref) = &inputs.support_bundle_manifest_ref {
        refs.push(IncidentEvidenceRef {
            evidence_kind: IncidentEvidenceKind::SupportBundleManifest,
            evidence_ref: manifest_ref.clone(),
            redaction_class: "metadata_safe_default".into(),
            embedding_state: "by_reference".into(),
            required_for_first_diagnosis: true,
        });
    }

    if let Some(snapshot_ref) = &inputs.support_preview_snapshot_ref {
        refs.push(IncidentEvidenceRef {
            evidence_kind: IncidentEvidenceKind::SupportPreviewSnapshot,
            evidence_ref: snapshot_ref.clone(),
            redaction_class: "metadata_safe_default".into(),
            embedding_state: "by_reference".into(),
            required_for_first_diagnosis: false,
        });
    }

    refs
}

fn next_safe_actions(fault_domain_id: &str) -> Vec<NextSafeAction> {
    vec![
        NextSafeAction {
            action_kind: NextSafeActionKind::SafeMode,
            action_ref: "recovery_action:safe_mode.crash_loop_entry".into(),
            label: "Open safe mode".into(),
            enabled: true,
            blast_radius: "whole_product_minimal_runtime_profile".into(),
            preserves_user_state: true,
            notes: "Disables third-party extensions, repo activators, auto-rejoin, and heavy background services while preserving local editing and diagnostics.".into(),
        },
        NextSafeAction {
            action_kind: NextSafeActionKind::OpenWithoutRestore,
            action_ref: "recovery_action:open_without_restore.evidence_preserved".into(),
            label: "Open without restore".into(),
            enabled: true,
            blast_radius: "session_restore_skipped_evidence_retained".into(),
            preserves_user_state: true,
            notes: "Starts cleanly without replaying the crashed session and keeps crash evidence available for export.".into(),
        },
        NextSafeAction {
            action_kind: NextSafeActionKind::ExportEvidence,
            action_ref: "support_action:export_crash_incident_trail".into(),
            label: "Export evidence".into(),
            enabled: true,
            blast_radius: "audit_only_support_bundle_preview".into(),
            preserves_user_state: true,
            notes: "Exports the incident trail and manifest refs without embedding raw dump bytes by default.".into(),
        },
        NextSafeAction {
            action_kind: NextSafeActionKind::RetryFaultDomain,
            action_ref: format!("recovery_action:retry_fault_domain:{fault_domain_id}"),
            label: "Retry one lane".into(),
            enabled: true,
            blast_radius: fault_domain_id.to_owned(),
            preserves_user_state: true,
            notes: "Retries only the affected fault domain; no global reset or hidden rerun is implied.".into(),
        },
    ]
}

fn honesty_notes(
    symbolication_state: SymbolicationState,
    support_bundle_linkage: &SupportBundleLinkage,
) -> Vec<String> {
    let mut notes = vec![match symbolication_state {
        SymbolicationState::Exact => {
            "Exact-build symbolication is complete for every module in the crash envelope.".into()
        }
        SymbolicationState::Partial => {
            "Symbolication is partial; unresolved or weaker-than-exact module mappings remain visible on the trail.".into()
        }
        SymbolicationState::Missing => {
            "No symbolication report is linked; the crash trail keeps envelope, dump, trace, and support-bundle refs without implying a stack is available.".into()
        }
        SymbolicationState::BuildMismatch => {
            "Exact-build identities differ across crash evidence; the trail preserves refs but refuses to label the stack exact.".into()
        }
    }];

    match support_bundle_linkage.linkage_state {
        SupportBundleLinkageState::Linked => notes.push(
            "Support-bundle manifest linkage is present and agrees with the crash-side refs.".into(),
        ),
        SupportBundleLinkageState::MissingManifestRef => notes.push(
            "Support-bundle manifest linkage is missing and must be created before support handoff.".into(),
        ),
        SupportBundleLinkageState::MismatchedBundleRef => notes.push(
            "Support-bundle refs disagree; support handoff should treat the trail as incomplete.".into(),
        ),
    }

    notes
}

fn notes_for(
    symbolication_state: SymbolicationState,
    support_bundle_linkage: &SupportBundleLinkage,
) -> String {
    format!(
        "Crash incident trail recorded with symbolication_state={} and support_linkage_state={}; raw dump bytes are not exported by default.",
        symbolication_state.as_str(),
        support_bundle_linkage.linkage_state.as_str()
    )
}
