//! Canonical seed builders for the M5 dynamic-surface assistive-tech certification packet.
//!
//! These builders are the single producer of the checked-in certification support export,
//! the published dashboard, and the stale-proof / regression / waiver drill fixtures. The
//! headless emitter and the inline tests both call them so the in-code certification, the
//! artifacts, and the fixtures never drift. The canonical packet is certified *from* the
//! all-green AT diagnostics report; the drills mutate one surface and let the row derivation
//! recompute the status, gate, effective claim, and stale-proof causes.

use super::*;

use crate::accessibility::diagnostics::{
    seeded_m5_dynamic_a11y_diagnostics_report,
    seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked,
    seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked,
};

/// Stable packet id for the canonical (all-certified) certification packet.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_PACKET_ID: &str = "m5-at-certification:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DYNAMIC_A11Y_CERTIFICATION_SCHEMA_REF,
        M5_DYNAMIC_A11Y_DASHBOARD_SCHEMA_REF,
        M5_DYNAMIC_A11Y_CERTIFICATION_DOC_REF,
        M5_DYNAMIC_A11Y_CERTIFICATION_MATRIX_REF,
        M5_DYNAMIC_A11Y_CERTIFICATION_DIAGNOSTICS_REF,
        M5_PROOF_BRIDGE_DESCRIPTOR_REF,
        M5_PROOF_LIVE_ANNOUNCEMENT_REF,
        M5_PROOF_EVENT_COVERAGE_REF,
        M5_PROOF_FOCUS_RETURN_REF,
        M5_PROOF_NONVISUAL_SUMMARY_REF,
        M5_PROOF_DIAGNOSTICS_REF,
    ])
}

fn conformance_review() -> M5A11yCertificationConformanceReview {
    M5A11yCertificationConformanceReview {
        every_dynamic_surface_has_certification_row: true,
        matrix_six_dimensions_covered_per_surface: true,
        bridge_health_certified_from_diagnostics: true,
        announcement_coverage_certified: true,
        focus_return_certified: true,
        non_visual_summaries_certified: true,
        zoom_contrast_motion_parity_certified: true,
        stale_proof_downgrade_rules_enforced: true,
        stale_or_missing_proof_auto_narrows_before_stable: true,
        unwaived_regressions_block_stable_promotion: true,
        regressions_not_invisible_in_release_truth: true,
        active_waivers_disclosed_with_scope_and_expiry: true,
        exact_stale_proof_causes_named: true,
        dashboard_traffic_light_matches_rows: true,
        surfaces_reuse_descriptor_object_identity: true,
        support_export_carries_no_raw_boundary_material: true,
    }
}

fn consumer_projection() -> M5A11yCertificationConsumerProjection {
    M5A11yCertificationConsumerProjection {
        release_center_consumes_certification: true,
        support_export_consumes_certification: true,
        docs_help_documents_certification: true,
        onboarding_reflects_certification: true,
        presentation_reflects_certification: true,
        shell_editor_notebook_data_review_consume_certification: true,
        release_public_truth_gates_on_certification: true,
        stable_claim_matrix_reads_certification: true,
    }
}

fn proof_freshness() -> M5DynamicSurfaceA11yProofFreshness {
    M5DynamicSurfaceA11yProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DynamicSurfaceA11yReleasePosture {
    M5DynamicSurfaceA11yReleasePosture {
        release_packet_ref: "evidence:at-certification-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:at-certification-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

/// Builds the packet-level release gate from the per-surface certification gates.
fn aggregate_release_gate(
    surfaces: &[M5A11ySurfaceCertificationRow],
) -> M5A11yCertificationReleaseGate {
    let collect = |predicate: &dyn Fn(&M5A11ySurfaceCertificationRow) -> bool| -> Vec<String> {
        let mut ids: Vec<String> = surfaces
            .iter()
            .filter(|s| predicate(s))
            .map(|s| s.surface_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let blocked = collect(&|s| s.is_blocked());
    M5A11yCertificationReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_surface_ids: blocked,
        auto_narrowed_surface_ids: collect(&|s| s.is_auto_narrowed()),
        certified_surface_ids: collect(&|s| s.is_certified()),
        waived_surface_ids: collect(&|s| !s.waivers.is_empty()),
        gate_message_id: format!("{}release_gate", M5_CERTIFICATION_MESSAGE_ID_PREFIX),
    }
}

/// Assembles a certification packet from a set of (already reconciled) surface rows.
fn build_packet(
    packet_id: &str,
    surfaces: Vec<M5A11ySurfaceCertificationRow>,
) -> M5DynamicA11yCertificationPacket {
    let release_gate = aggregate_release_gate(&surfaces);
    M5DynamicA11yCertificationPacket::new(M5DynamicA11yCertificationPacketInput {
        packet_id: packet_id.to_owned(),
        report_label: "M5 Dynamic-Surface Assistive-Tech Certification".to_owned(),
        surfaces,
        vocabulary_set: M5A11yCertificationVocabularySet::canonical(),
        shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        release_gate,
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Builds the canonical all-certified certification packet from the all-green AT
/// diagnostics report.
///
/// This is the single producer of the checked-in support export and dashboard: every
/// claimed dynamic surface is current and conformant across all six proof dimensions, so
/// the release gate certifies every row for Stable promotion.
pub fn seeded_m5_dynamic_a11y_certification() -> M5DynamicA11yCertificationPacket {
    let surfaces =
        certification_rows_from_diagnostics(&seeded_m5_dynamic_a11y_diagnostics_report());
    build_packet(M5_DYNAMIC_A11Y_CERTIFICATION_PACKET_ID, surfaces)
}

/// Finds the index of the first surface in the given family.
fn surface_index(surfaces: &[M5A11ySurfaceCertificationRow], family: M5SurfaceFamily) -> usize {
    surfaces
        .iter()
        .position(|s| s.surface_family == family)
        .expect("surface family present")
}

/// Certification packet where one surface's bridge-health proof has regressed (an unwaived
/// blocking regression), so the row is degraded and blocked from Stable promotion — and
/// named, not hidden, in the release packet.
pub fn seeded_m5_dynamic_a11y_certification_regression_blocked() -> M5DynamicA11yCertificationPacket
{
    let surfaces = certification_rows_from_diagnostics(
        &seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked(),
    );
    build_packet(
        "m5-at-certification:drill:bridge-regression-blocked",
        surfaces,
    )
}

/// Certification packet where one surface's stale-proof downgrade evidence has fallen out of
/// SLO, so the row is retest-pending and auto-narrows to Beta before Stable promotion.
pub fn seeded_m5_dynamic_a11y_certification_stale_proof_retest_pending(
) -> M5DynamicA11yCertificationPacket {
    let mut surfaces =
        certification_rows_from_diagnostics(&seeded_m5_dynamic_a11y_diagnostics_report());
    let index = surface_index(&surfaces, M5SurfaceFamily::DenseCollection);
    let row = &mut surfaces[index];
    if let Some(dimension) = row
        .dimensions
        .iter_mut()
        .find(|d| d.dimension == M5A11yProofDimension::StaleProofDowngrade)
    {
        dimension.proof_freshness = M5A11yProofFreshness::Stale;
        dimension.stale_cause = Some(M5DynamicSurfaceA11yDowngradeTrigger::ProofStale);
    }
    row.recompute_derived();
    build_packet(
        "m5-at-certification:drill:stale-proof-retest-pending",
        surfaces,
    )
}

/// Certification packet where a surface's blocking announcement-coverage regression is
/// accepted under an active, disclosed waiver, so the row ships auto-narrowed to its waived
/// claim while the true status stays degraded (red) and the waiver is named with scope,
/// owner, and expiry.
pub fn seeded_m5_dynamic_a11y_certification_waived_narrowed() -> M5DynamicA11yCertificationPacket {
    let mut surfaces = certification_rows_from_diagnostics(
        &seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked(),
    );
    let index = surface_index(&surfaces, M5SurfaceFamily::DenseCollection);
    let row = &mut surfaces[index];
    row.waivers.push(M5A11yCertificationWaiver {
        waiver_id: "waiver:dense-collection-announcement-coverage".to_owned(),
        dimension: M5A11yProofDimension::AnnouncementCoverage,
        reason_message_id: format!(
            "{}{}.waiver.announcement_coverage",
            M5_CERTIFICATION_MESSAGE_ID_PREFIX, row.surface_id
        ),
        owner_role: "Accessibility owner".to_owned(),
        expires_at: "2026-09-26T00:00:00Z".to_owned(),
        narrowed_to: M5DynamicSurfaceA11yQualificationClass::Preview,
    });
    row.recompute_derived();
    build_packet("m5-at-certification:drill:waived-narrowed", surfaces)
}
