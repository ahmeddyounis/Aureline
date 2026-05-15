//! Support-bundle linkage for crash incident trails.
//!
//! The support preview consumes [`aureline_crash::CrashIncidentTrail`] by
//! reference and emits one manifest row. The row is environment-adjacent
//! metadata: it carries crash IDs, trace IDs, exact-build refs,
//! symbolication state, and support-bundle linkage, but never raw dump
//! bytes or stack bodies.

use aureline_crash::{CrashIncidentTrail, ModuleMappingQuality, SymbolicationState};

use super::exact_build::ExactBuildCapture;
use super::manifest::{CrashSymbolicatedFrameProjection, SizeEstimate};
use super::preview::{
    PreviewItemSeed, SupportBundlePreview, SupportBundlePreviewBuilder, SupportBundlePreviewError,
};
use super::vocabulary::{ActionabilityImpactClass, DiagnosticDataClass, HighRiskContentClass};

/// Support-pack item id for a crash incident-trail manifest row.
pub const SUPPORT_ITEM_CRASH_INCIDENT_TRAIL: &str = "support.item.crash_incident_trail";

/// Build the support preview row that carries a crash incident trail.
pub fn crash_incident_trail_seed(trail: &CrashIncidentTrail) -> PreviewItemSeed {
    let estimated_bytes = 4096_u64
        .saturating_add((trail.module_summaries.len() as u64).saturating_mul(1024))
        .saturating_add((trail.trace_ids.len() as u64).saturating_mul(256));
    let mut source_refs = vec![
        "docs/support/incident_trail_alpha.md".into(),
        trail.crash_envelope_ref.clone(),
        trail.crash_dump_ref.clone(),
        trail.primary_exact_build_identity_ref.clone(),
    ];
    if let Some(report_ref) = &trail.symbolication_report_ref {
        source_refs.push(report_ref.clone());
    }
    if let Some(manifest_ref) = &trail.support_bundle_linkage.support_bundle_manifest_ref {
        source_refs.push(manifest_ref.clone());
    }

    PreviewItemSeed {
        support_pack_item_id: SUPPORT_ITEM_CRASH_INCIDENT_TRAIL.into(),
        title: "Crash incident trail".into(),
        data_class: DiagnosticDataClass::EnvironmentAdjacent,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "crash_and_recovery_truth".into(),
        artifact_kind_class: "crash_incident_trail_alpha_record".into(),
        manifest_path_ref: "preview_items[0]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/crash_incident_trail/{}.json",
            safe_member_name(&trail.incident_trail_id)
        )),
        source_refs,
        size_estimate: SizeEstimate {
            estimated_bytes: Some(estimated_bytes),
            confidence_class: "estimated".into(),
            display_label: format!("{} KB", estimated_bytes.div_ceil(1024)),
            size_source_class: "incident_trail_row_count_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::BlocksFirstActionableDiagnosis,
        impact_summary:
            "Without this row, support cannot reconstruct the crash, exact-build symbols, trace \
             IDs, support-bundle manifest, and safest next recovery actions as one incident trail."
                .into(),
        notes: incident_trail_notes(trail),
    }
}

/// Build a local-first support preview whose single row is the crash
/// incident trail.
pub fn crash_incident_trail_preview(
    exact_build: ExactBuildCapture,
    generated_at: impl Into<String>,
    trail: &CrashIncidentTrail,
) -> Result<SupportBundlePreview, SupportBundlePreviewError> {
    let bundle_id = format!(
        "support-bundle:crash-incident-trail:{}",
        safe_member_name(&trail.incident_trail_id)
    );
    let mut builder = SupportBundlePreviewBuilder::new(
        bundle_id,
        "Crash incident trail support preview",
        generated_at,
        exact_build,
    );
    builder.add_item(crash_incident_trail_seed(trail));
    builder.add_crash_symbolication_frames(crash_symbolicated_frame_projections(trail));
    builder.build()
}

/// Project exact-build symbolicated frames from a crash trail into a
/// support-bundle manifest.
pub fn crash_symbolicated_frame_projections(
    trail: &CrashIncidentTrail,
) -> Vec<CrashSymbolicatedFrameProjection> {
    let Some(report_ref) = &trail.symbolication_report_ref else {
        return Vec::new();
    };
    if trail.primary_exact_build_identity_ref.trim().is_empty() {
        return Vec::new();
    }
    if !trail.is_support_bundle_linked() {
        return Vec::new();
    }

    trail
        .module_summaries
        .iter()
        .filter(|module| {
            matches!(
                module.mapping_quality,
                ModuleMappingQuality::Exact | ModuleMappingQuality::Partial
            ) && !module.resolved_frame_summary.is_empty()
        })
        .map(|module| CrashSymbolicatedFrameProjection {
            preview_item_id: "support.preview.item.crash_incident_trail".into(),
            support_pack_item_id: SUPPORT_ITEM_CRASH_INCIDENT_TRAIL.into(),
            crash_envelope_ref: trail.crash_envelope_ref.clone(),
            symbolication_report_ref: report_ref.clone(),
            module_id: module.module_id.clone(),
            module_kind: module.module_kind.clone(),
            mapping_quality: module.mapping_quality.as_str().into(),
            exact_build_identity_ref: trail.primary_exact_build_identity_ref.clone(),
            symbolication_identity_ref: module.symbolication_identity_ref.clone(),
            resolved_frame_summary: module.resolved_frame_summary.clone(),
            redaction_class: "operator_only_restricted".into(),
            raw_stack_body_exported: false,
            notes:
                "Frame summaries came from exact-build symbolication; raw stack bodies stay out."
                    .into(),
        })
        .collect()
}

fn incident_trail_notes(trail: &CrashIncidentTrail) -> String {
    let symbolication_label = match trail.symbolication_state {
        SymbolicationState::Exact => "exact-build symbolication complete",
        SymbolicationState::Partial => "partial symbolication retained with unresolved modules",
        SymbolicationState::Missing => "symbolication report missing",
        SymbolicationState::BuildMismatch => "exact-build mismatch retained as incomplete evidence",
    };

    let frame_summary_count = trail
        .module_summaries
        .iter()
        .map(|module| module.resolved_frame_summary.len())
        .sum::<usize>();

    format!(
        "{}; support linkage {}; {} trace id(s), {} module row(s), {} symbolicated frame summary row(s), raw dump exported: {}.",
        symbolication_label,
        trail.support_bundle_linkage.linkage_state.as_str(),
        trail.trace_ids.len(),
        trail.module_summaries.len(),
        frame_summary_count,
        trail.raw_dump_exported
    )
}

fn safe_member_name(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
