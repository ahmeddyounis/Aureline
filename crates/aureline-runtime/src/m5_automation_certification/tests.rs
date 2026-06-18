//! Inline unit coverage for the automation certification packet: seed stability,
//! the claimed surface matrix, the six graded certification dimensions, the
//! freshness/stale-narrowing roll-up, the certification index, and the
//! fail-closed guardrails against ad-hoc authoring, unreviewed parameters, missing
//! side-effect previews, lost run history, unsafe macros, invented labels, and
//! shareable claims without proof.

use super::*;

fn seed() -> AutomationCertificationPacket {
    seeded_automation_certification_packet()
}

fn surface_mut(
    input: &mut AutomationCertificationPacketInput,
    surface: AutomationSurface,
) -> &mut AutomationSurfaceCertification {
    input
        .surfaces
        .iter_mut()
        .find(|row| row.surface == surface)
        .expect("surface present")
}

#[test]
fn seed_materializes_stable() {
    let packet = seed();
    assert!(
        packet.validate().is_empty(),
        "seed must validate clean: {:?}",
        packet.validate()
    );
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::Stable
    );
    assert_eq!(packet.record_kind, AUTOMATION_CERTIFICATION_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        AUTOMATION_CERTIFICATION_SCHEMA_VERSION
    );
}

#[test]
fn seed_carries_every_claimed_surface() {
    let packet = seed();
    assert_eq!(
        packet.surface_tokens(),
        vec![
            "notebook_automation",
            "request_api_automation",
            "package_automation",
            "test_debug_automation",
            "incident_automation",
            "ai_linked_automation",
        ]
    );
}

#[test]
fn every_seed_surface_certifies_on_all_dimensions() {
    let packet = seed();
    for row in &packet.surfaces {
        assert!(
            row.certified,
            "surface {} must certify",
            row.surface.as_str()
        );
        assert_eq!(row.claim_state, SurfaceClaimState::Shareable);
        assert_eq!(
            row.dimension_outcomes.len(),
            AutomationCertificationDimension::ALL.len()
        );
        for outcome in &row.dimension_outcomes {
            assert!(
                outcome.passed,
                "surface {} dimension {} must pass",
                row.surface.as_str(),
                outcome.dimension.as_str()
            );
        }
    }
}

#[test]
fn every_seed_surface_authors_in_builder_and_cites_evidence() {
    let packet = seed();
    for row in &packet.surfaces {
        assert_eq!(
            row.authoring_path,
            AutomationAuthoringPath::DeclarativeRecipeBuilder
        );
        assert_eq!(
            row.evidence_refs.len(),
            AUTOMATION_CERTIFICATION_EVIDENCE_REFS.len()
        );
        assert!(!row.safety_labels.is_empty());
        assert!(row.presents_as_shareable);
    }
}

#[test]
fn seed_certification_index_is_current_and_certified() {
    let packet = seed();
    assert!(packet.certification_index.all_surfaces_current);
    assert!(packet.certification_index.all_surfaces_certified);
    assert_eq!(
        packet.certification_index.shareable_surfaces.len(),
        AutomationSurface::ALL.len()
    );
    assert!(packet.certification_index.narrowed_surfaces.is_empty());
    assert!(packet.certification_index.blocked_surfaces.is_empty());
    assert_eq!(
        packet.certification_index.certification_ref,
        AUTOMATION_CERTIFICATION_INDEX_REF
    );
}

#[test]
fn evidence_joins_explain_consistently() {
    let packet = seed();
    for surface in [
        CertificationEvidenceSurface::SupportBundle,
        CertificationEvidenceSurface::IncidentPacket,
        CertificationEvidenceSurface::AiEvidence,
    ] {
        let view = packet.evidence_join(surface, "view", "2026-06-18T00:01:00Z");
        assert!(
            view.explains_consistently(),
            "{} must explain",
            surface.as_str()
        );
        assert_eq!(view.surface_rows.len(), packet.surfaces.len());
        assert_eq!(view.surface_digest, packet.surface_digest);
        assert_eq!(view.certification_index, packet.certification_index);
    }
}

#[test]
fn cli_headless_view_explains_every_surface() {
    let packet = seed();
    let view = packet.cli_headless_view(
        AUTOMATION_CERTIFICATION_CLI_HEADLESS_ID,
        "2026-06-18T00:01:00Z",
    );
    assert!(view.every_surface_explained());
    assert_eq!(view.surface_rows.len(), packet.surfaces.len());
    assert_eq!(view.surface_digest, packet.surface_digest);
    assert_eq!(view.promotion_state, packet.promotion_state);
}

#[test]
fn support_export_round_trips_and_stays_safe() {
    let packet = seed();
    let export = packet.support_export(
        AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_ID,
        "2026-06-18T00:01:00Z",
    );
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serialize");
    let parsed: AutomationCertificationSupportExport = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed, export);
    assert!(parsed.packet.is_stable());
    assert_eq!(parsed.packet.surface_digest, packet.surface_digest);
}

#[test]
fn ad_hoc_authoring_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::NotebookAutomation).authoring_path =
        AutomationAuthoringPath::AdHocFeatureDialog;
    let packet = AutomationCertificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::AdHocAuthoring));
    assert!(packet
        .certification_index
        .blocked_surfaces
        .contains(&AutomationSurface::NotebookAutomation.as_str().to_owned()));
}

#[test]
fn missing_builder_evidence_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::RequestApiAutomation).evidence_refs = Vec::new();
    let packet = AutomationCertificationPacket::materialize(input);
    // A builder-conformant surface with no evidence fails builder parity and is
    // also reported as citing no upstream proof.
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::MissingBuilderEvidence));
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::MissingEvidenceRef));
}

#[test]
fn unreviewed_parameters_block_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::PackageAutomation).parameters_reviewed = false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ParameterReviewMissing));
}

#[test]
fn unsafe_secret_reference_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::RequestApiAutomation).secret_references_safe = false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ParameterReviewMissing));
}

#[test]
fn missing_side_effect_preview_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::TestDebugAutomation).side_effect_preview_shown =
        false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::SideEffectPreviewMissing));
}

#[test]
fn missing_run_history_integrity_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::IncidentAutomation).rerun_under_current_policy =
        false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::RunHistoryIntegrityMissing));
}

#[test]
fn unsafe_macro_scope_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::NotebookAutomation).macro_fails_closed_on_mismatch =
        false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::MacroScopeUnsafe));
}

#[test]
fn broken_label_reuse_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    surface_mut(&mut input, AutomationSurface::AiLinkedAutomation).reuses_controlled_labels = false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::LabelReuseBroken));
}

#[test]
fn shareable_claim_without_proof_is_caught() {
    let mut input = current_stable_automation_certification_input();
    // A surface that fails a dimension but still presents as shareable must be
    // flagged on the shareable claim itself, not only on the failing dimension.
    surface_mut(&mut input, AutomationSurface::PackageAutomation).side_effect_preview_shown = false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ShareableClaimUnproven));
}

#[test]
fn non_shareable_surface_without_proof_skips_the_shareable_finding() {
    let mut input = current_stable_automation_certification_input();
    let row = surface_mut(&mut input, AutomationSurface::PackageAutomation);
    row.presents_as_shareable = false;
    row.side_effect_preview_shown = false;
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::SideEffectPreviewMissing));
    assert!(!packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ShareableClaimUnproven));
}

#[test]
fn missing_surface_blocks_stable() {
    let mut input = current_stable_automation_certification_input();
    input
        .surfaces
        .retain(|row| row.surface != AutomationSurface::AiLinkedAutomation);
    let packet = AutomationCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::MissingSurface));
}

#[test]
fn stale_surface_narrows_below_stable_without_blocking() {
    let mut input = current_stable_automation_certification_input();
    let row = surface_mut(&mut input, AutomationSurface::IncidentAutomation);
    row.proof_age_days = row.freshness_window_days + 10;
    let packet = AutomationCertificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::NarrowedBelowStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::SurfaceEvidenceStale));
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ShareableClaimNarrowed));
    // Narrowing is a warning, not a blocker.
    assert!(packet.is_stable());
    assert!(!packet.certification_index.all_surfaces_current);
    assert!(packet
        .certification_index
        .narrowed_surfaces
        .contains(&AutomationSurface::IncidentAutomation.as_str().to_owned()));
}

#[test]
fn surface_digest_drift_is_caught() {
    let mut packet = seed();
    packet.surface_digest = "fnv1a64:0000000000000000".to_owned();
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::SurfaceDigestDrift));
}

#[test]
fn certification_index_drift_is_caught() {
    let mut packet = seed();
    packet.certification_index.all_surfaces_certified = false;
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::CertificationIndexDrift));
}

#[test]
fn surface_certification_drift_is_caught() {
    let mut packet = seed();
    if let Some(row) = packet.surfaces.first_mut() {
        row.certified = false;
    }
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::SurfaceCertificationDrift));
}

#[test]
fn claim_state_drift_is_caught() {
    let mut packet = seed();
    if let Some(row) = packet.surfaces.first_mut() {
        row.claim_state = SurfaceClaimState::Blocked;
    }
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::SurfaceClaimStateDrift));
}
