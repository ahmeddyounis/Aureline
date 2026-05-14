//! Support-bundle linkage for crash incident trails.
//!
//! The support preview consumes [`aureline_crash::CrashIncidentTrail`] by
//! reference and emits one manifest row. The row is environment-adjacent
//! metadata: it carries crash IDs, trace IDs, exact-build refs,
//! symbolication state, and support-bundle linkage, but never raw dump
//! bytes or stack bodies.

use aureline_crash::{CrashIncidentTrail, SymbolicationState};

use super::exact_build::ExactBuildCapture;
use super::manifest::SizeEstimate;
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
    builder.build()
}

fn incident_trail_notes(trail: &CrashIncidentTrail) -> String {
    let symbolication_label = match trail.symbolication_state {
        SymbolicationState::Exact => "exact-build symbolication complete",
        SymbolicationState::Partial => "partial symbolication retained with unresolved modules",
        SymbolicationState::Missing => "symbolication report missing",
        SymbolicationState::BuildMismatch => "exact-build mismatch retained as incomplete evidence",
    };

    format!(
        "{}; support linkage {}; {} trace id(s), {} module row(s), raw dump exported: {}.",
        symbolication_label,
        trail.support_bundle_linkage.linkage_state.as_str(),
        trail.trace_ids.len(),
        trail.module_summaries.len(),
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
