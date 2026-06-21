use super::*;

const P_COVERAGE_LOCAL: &str = "projection:coverage-local-test:0001";
const P_FLAKY_LOCAL: &str = "projection:flaky-history-local-test:0001";
const P_PERF_LOCAL: &str = "projection:perf-regression-local-task:0001";
const P_NOTEBOOK_VERDICT: &str = "projection:notebook-verdict-cell:0001";
const P_PIPELINE_OVERLAY: &str = "projection:pipeline-annotation-provider:0001";
const P_IMPORTED_COVERAGE: &str = "projection:coverage-imported-provider:0001";
const P_LABS: &str = "projection:notebook-verdict-labs:0001";

fn seeded() -> M5ExecutionEvidenceProjectionSetPacket {
    seeded_execution_evidence_projection_set()
}

fn projection<'a>(
    packet: &'a M5ExecutionEvidenceProjectionSetPacket,
    id: &str,
) -> &'a ExecutionEvidenceProjection {
    packet
        .projections
        .iter()
        .find(|p| p.projection_id == id)
        .unwrap_or_else(|| panic!("missing projection {id}"))
}

fn cloned(
    packet: &M5ExecutionEvidenceProjectionSetPacket,
    id: &str,
) -> ExecutionEvidenceProjection {
    projection(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers the
/// rendered claim to match — otherwise the surface itself overclaims and floors.
fn render_all(p: &mut ExecutionEvidenceProjection, claim: ProjectionClaim) {
    p.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = claim);
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(
        packet.record_kind,
        M5_EXECUTION_EVIDENCE_PROJECTIONS_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_EXECUTION_EVIDENCE_PROJECTIONS_SCHEMA_VERSION
    );
    assert_eq!(
        packet.taxonomy_version,
        M5_EXECUTION_EVIDENCE_PROJECTIONS_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.projections.len(), 8);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_execution_evidence_projection_set()
        .expect("canonical projection set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seeded_covers_every_kind_and_surface() {
    let packet = seeded();
    for kind in ProjectionKind::ALL {
        assert!(
            packet.represented_kinds().contains(&kind),
            "missing kind {}",
            kind.as_str()
        );
    }
    for surface in ProjectionSurface::ALL {
        assert!(
            packet.represented_surfaces().contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn claim_distribution_is_stable() {
    // Coverage, flaky, notebook-verdict, and review-marker certify; perf narrows via
    // a stale proof; the pipeline and imported coverage overlays stay read-only; the
    // notebook Labs projection makes no claim.
    let dist = seeded().claim_distribution();
    assert_eq!(dist.certified, 4);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 2);
    assert_eq!(dist.unreconstructable, 0);
    assert_eq!(dist.labs, 1);
    assert_eq!(seeded().narrowed_projection_count(), 1);
}

#[test]
fn export_safe_json_round_trips() {
    let packet = seeded();
    let json = packet.export_safe_json();
    let reparsed: M5ExecutionEvidenceProjectionSetPacket =
        serde_json::from_str(&json).expect("round-trips");
    assert_eq!(reparsed, packet);
    assert!(reparsed.validate().is_empty());
}

#[test]
fn export_carries_no_forbidden_material() {
    let value = serde_json::to_value(seeded()).expect("serializes");
    assert!(!json_contains_forbidden_boundary_material(&value));
}

#[test]
fn markdown_summary_lists_projections_and_counts() {
    let summary = seeded().render_markdown_summary();
    assert!(summary.contains("# M5 Execution-Evidence Projection Overlays"));
    assert!(summary.contains("4 certified, 1 narrowed, 2 read-only overlay"));
    assert!(summary.contains(P_PERF_LOCAL));
}

// --------------------------------------------------------------------------- //
// Per-projection derivation (mirrors the perturbation corpus).
// --------------------------------------------------------------------------- //

#[test]
fn clean_coverage_projection_certifies() {
    let decision = projection(&seeded(), P_COVERAGE_LOCAL).narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Certified
    );
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn flattening_origin_run_step_floors() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.integrity.preserves_origin_run_step = false;
    let decision = p.narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::OriginRunStepFlattened));
    // A floored projection keeps a reopen fallback.
    assert!(p.floored_keeps_fallback(decision.effective_projection_claim));
}

#[test]
fn flattening_provider_artifact_floors() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.integrity.preserves_provider_artifact = false;
    let decision = p.narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::ProviderArtifactFlattened));
}

#[test]
fn lineage_not_visible_on_a_surface_floors() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.renderings[0].lineage_visible = false;
    let decision = p.narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::LineageNotVisible));
}

#[test]
fn heuristic_without_backlink_floors() {
    let mut p = cloned(&seeded(), P_PERF_LOCAL);
    assert!(p.declared_confidence_tier.is_heuristic_tier());
    p.integrity.raw_output_backlink_present = false;
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::RawBacklinkMissing));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
    // The lineage raw-output backlink keeps the floored projection reopenable.
    assert!(p.floored_keeps_fallback(decision.effective_projection_claim));
}

#[test]
fn reopen_target_lost_floors() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.declared_reopen_target = ReopenTarget::NoneKeyboardFallback;
    let decision = p.narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::ReopenTargetLost));
    // NoneKeyboardFallback is itself a raw fallback target.
    assert!(p.floored_keeps_fallback(decision.effective_projection_claim));
}

#[test]
fn surface_overclaim_floors_and_is_caught_by_validate() {
    let mut p = cloned(&seeded(), P_PERF_LOCAL);
    // The perf projection effectively narrows; a surface that renders certified
    // overclaims.
    p.renderings[0].rendered_claim = ProjectionClaim::Certified;
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::SurfaceOverclaims));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );

    let mut packet = seeded();
    let idx = packet
        .projections
        .iter()
        .position(|x| x.projection_id == P_PERF_LOCAL)
        .unwrap();
    packet.projections[idx] = p;
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceProjectionViolation::RenderingSurfaceOverclaims));
}

#[test]
fn imported_overlay_claiming_live_floors() {
    let mut p = cloned(&seeded(), P_PIPELINE_OVERLAY);
    assert!(p.is_overlay_origin());
    p.integrity.imported_overlay_read_only = false;
    let decision = p.narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::ImportedOverlayClaimsLive));
}

#[test]
fn overlay_with_any_other_gap_floors_below_overlay() {
    // An overlay is already the minimal honest claim; a non-floor gap still drops it
    // to unreconstructable rather than holding a clean read-only overlay.
    let mut p = cloned(&seeded(), P_PIPELINE_OVERLAY);
    p.revision_remap.remap_quality_labeled = false;
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::RemapQualityUnlabeled));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
}

#[test]
fn missing_evidence_floors() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.declared_freshness_state = FreshnessState::Missing;
    let decision = p.narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::EvidenceMissing));
}

#[test]
fn remap_quality_unlabeled_narrows_first_party() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.revision_remap.remap_quality_labeled = false;
    render_all(&mut p, ProjectionClaim::Narrowed);
    let decision = p.narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::RemapQualityUnlabeled));
    assert!(decision.narrowed);
    assert!(p.narrowed_label(&decision).is_some());
}

#[test]
fn stale_unmapped_anchor_claiming_current_narrows() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.revision_remap.quality = RemapQuality::StaleUnmapped;
    p.revision_remap.anchored_to_current_revision = true;
    render_all(&mut p, ProjectionClaim::Narrowed);
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::StaleRemapUnlabeled));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
}

#[test]
fn freshness_unlabeled_narrows() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.integrity.freshness_state_labeled = false;
    render_all(&mut p, ProjectionClaim::Narrowed);
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::FreshnessUnlabeled));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
}

#[test]
fn confidence_unlabeled_narrows() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.integrity.confidence_label_visible = false;
    render_all(&mut p, ProjectionClaim::Narrowed);
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::ConfidenceUnlabeled));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
}

#[test]
fn superseded_unmarked_narrows() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.declared_freshness_state = FreshnessState::SupersededByNewerRun;
    p.integrity.superseded_state_marked = false;
    render_all(&mut p, ProjectionClaim::Narrowed);
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::SupersededNotMarked));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
}

#[test]
fn superseded_marked_stays_certified() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.declared_freshness_state = FreshnessState::SupersededByNewerRun;
    p.integrity.superseded_state_marked = true;
    let decision = p.narrow(false);
    assert!(decision.active_narrowing_reasons.is_empty());
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Certified
    );
}

#[test]
fn first_party_stale_evidence_narrows() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.declared_freshness_state = FreshnessState::StaleExpired;
    p.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = ProjectionClaim::Narrowed);
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::StaleEvidence));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
}

#[test]
fn overlay_cached_snapshot_stays_overlay() {
    // A read-only overlay showing a cached snapshot is expected, not narrowed.
    let decision = projection(&seeded(), P_IMPORTED_COVERAGE).narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::ReadOnlyOverlay
    );
    assert!(!decision.narrowed);
}

#[test]
fn missing_proof_narrows_first_party() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.verification.proof_currency = ProofCurrency::MissingProof;
    p.verification.proof_ref = None;
    render_all(&mut p, ProjectionClaim::Narrowed);
    let decision = p.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::MissingProof));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
}

#[test]
fn stale_window_ages_out_current_proof() {
    let mut p = cloned(&seeded(), P_COVERAGE_LOCAL);
    p.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = ProjectionClaim::Narrowed);
    let decision = p.narrow(true);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ProjectionNarrowingReason::StaleProof));
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
}

#[test]
fn labs_projection_makes_no_claim_and_never_widens() {
    let decision = projection(&seeded(), P_LABS).narrow(false);
    assert_eq!(
        decision.claimed_projection_claim,
        ProjectionClaim::LabsNotClaimed
    );
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::LabsNotClaimed
    );
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn notebook_verdict_certifies_and_flaky_certifies() {
    let packet = seeded();
    assert_eq!(
        projection(&packet, P_NOTEBOOK_VERDICT)
            .narrow(false)
            .effective_projection_claim,
        ProjectionClaim::Certified
    );
    assert_eq!(
        projection(&packet, P_FLAKY_LOCAL)
            .narrow(false)
            .effective_projection_claim,
        ProjectionClaim::Certified
    );
}

#[test]
fn perf_projection_narrows_via_stale_proof() {
    let decision = projection(&seeded(), P_PERF_LOCAL).narrow(false);
    assert_eq!(
        decision.effective_projection_claim,
        ProjectionClaim::Narrowed
    );
    assert!(decision.narrowed);
    assert_eq!(
        decision.downgrade_trigger(),
        Some(ProjectionNarrowingReason::StaleProof)
    );
}

// --------------------------------------------------------------------------- //
// Packet-level validation failures.
// --------------------------------------------------------------------------- //

#[test]
fn overlay_without_provider_ref_is_flagged() {
    let mut packet = seeded();
    let idx = packet
        .projections
        .iter()
        .position(|x| x.projection_id == P_PIPELINE_OVERLAY)
        .unwrap();
    packet.projections[idx].lineage.provider_ref = None;
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceProjectionViolation::OverlayMissingProviderRef));
}

#[test]
fn duplicate_projection_id_is_flagged() {
    let mut packet = seeded();
    let dup = packet.projections[0].clone();
    packet.projections.push(dup);
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceProjectionViolation::DuplicateProjectionId));
}

#[test]
fn missing_kind_is_flagged() {
    let mut packet = seeded();
    packet
        .projections
        .retain(|p| p.projection_kind != ProjectionKind::PerfRegressionNote);
    let violations = packet.validate();
    assert!(violations.contains(&M5ExecutionEvidenceProjectionViolation::ProjectionKindMissing));
    // Removing the only narrowed projection also trips the narrowing-demo guard.
    assert!(violations
        .contains(&M5ExecutionEvidenceProjectionViolation::DowngradedProjectionCaseMissing));
}

#[test]
fn invalid_redaction_class_is_flagged() {
    let mut packet = seeded();
    packet.redaction_class_token = "everything".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceProjectionViolation::InvalidRedactionClass));
}

#[test]
fn projection_without_rendering_is_flagged() {
    let mut packet = seeded();
    packet.projections[0].renderings.clear();
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceProjectionViolation::ProjectionMissingRendering));
}

#[test]
fn overclaim_detection_respects_rank() {
    assert!(ProjectionClaim::Narrowed.overclaims_as(ProjectionClaim::Certified));
    assert!(!ProjectionClaim::Certified.overclaims_as(ProjectionClaim::Narrowed));
    assert!(ProjectionClaim::ReadOnlyOverlay.overclaims_as(ProjectionClaim::Narrowed));
    assert!(!ProjectionClaim::LabsNotClaimed.overclaims_as(ProjectionClaim::LabsNotClaimed));
    assert!(ProjectionClaim::LabsNotClaimed.overclaims_as(ProjectionClaim::Certified));
}
