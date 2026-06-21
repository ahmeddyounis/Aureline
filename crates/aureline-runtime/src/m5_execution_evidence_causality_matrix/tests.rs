use super::*;

/// The canonical lane ids exercised by the perturbation tests.
const LANE_STRUCTURED_DIAGNOSTIC: &str = "exec-evidence:local-task-structured-diagnostic:0001";
const LANE_HEURISTIC_PARSE: &str = "exec-evidence:local-task-heuristic-parse:0001";
const LANE_TEST_NORMALIZED: &str = "exec-evidence:local-test-normalized-event:0001";
const LANE_REMOTE_OVERLAY: &str = "exec-evidence:remote-linked-output-overlay:0001";
const LANE_IMPORTED_BUNDLE: &str = "exec-evidence:imported-provider-bundle:0001";
const LANE_LABS: &str = "exec-evidence:labs-cross-run-causal-graph:0001";
const LANE_FLOORED: &str = "exec-evidence:local-adapter-lineage-flattened-floored:0001";

fn canonical() -> M5ExecutionEvidenceCausalityMatrixPacket {
    current_m5_execution_evidence_causality_matrix()
        .expect("canonical execution-evidence causality matrix loads and validates")
}

fn row<'a>(
    packet: &'a M5ExecutionEvidenceCausalityMatrixPacket,
    lane_id: &str,
) -> &'a CausalityLaneRow {
    packet
        .rows
        .iter()
        .find(|row| row.lane_id == lane_id)
        .unwrap_or_else(|| panic!("missing lane {lane_id}"))
}

fn cloned(packet: &M5ExecutionEvidenceCausalityMatrixPacket, lane_id: &str) -> CausalityLaneRow {
    row(packet, lane_id).clone()
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn canonical_export_loads_and_validates_clean() {
    let packet = canonical();
    assert_eq!(
        packet.record_kind,
        M5_EXECUTION_EVIDENCE_CAUSALITY_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_VERSION
    );
    assert_eq!(
        packet.taxonomy_version,
        M5_EXECUTION_EVIDENCE_CAUSALITY_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.rows.len(), 14);
}

#[test]
fn canonical_claim_distribution_matches_report() {
    // The certification report freezes 8 certified, 1 narrowed, 3 read-only
    // overlay, 1 unreconstructable, 1 labs across the 14 lanes.
    let dist = canonical().claim_distribution();
    assert_eq!(dist.certified, 8);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 3);
    assert_eq!(dist.unreconstructable, 1);
    assert_eq!(dist.labs, 1);
}

#[test]
fn canonical_packet_covers_every_surface_and_origin() {
    let packet = canonical();
    for surface in SurfaceFamily::ALL {
        assert!(packet.represented_surface_families().contains(&surface));
    }
    for origin in OriginClass::ALL {
        assert!(packet.represented_origin_classes().contains(&origin));
    }
}

#[test]
fn canonical_export_carries_no_forbidden_material() {
    let packet = canonical();
    let value = serde_json::to_value(&packet).expect("serializes");
    assert!(!json_contains_forbidden_boundary_material(&value));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = canonical();
    let json = packet.export_safe_json();
    let reparsed: M5ExecutionEvidenceCausalityMatrixPacket =
        serde_json::from_str(&json).expect("round-trips");
    assert_eq!(reparsed, packet);
    assert!(reparsed.validate().is_empty());
}

#[test]
fn markdown_summary_lists_lanes_and_counts() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("# M5 Execution-Evidence Causality Matrix"));
    assert!(summary.contains("8 certified, 1 narrowed, 3 read-only overlay"));
    assert!(summary.contains(LANE_FLOORED));
}

// --------------------------------------------------------------------------- //
// Per-lane derivation (mirrors the perturbation corpus).
// --------------------------------------------------------------------------- //

#[test]
fn clean_local_lane_stays_certified() {
    let packet = canonical();
    let decision = row(&packet, LANE_STRUCTURED_DIAGNOSTIC).narrow(false);
    assert_eq!(decision.effective_causality_claim, CausalClaim::Certified);
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn heuristic_without_backlink_floors() {
    let packet = canonical();
    let mut lane = cloned(&packet, LANE_HEURISTIC_PARSE);
    lane.causal_chain.raw_output_backlink_present = false;
    let decision = lane.narrow(false);
    assert_eq!(decision.claimed_causality_claim, CausalClaim::Certified);
    assert_eq!(
        decision.effective_causality_claim,
        CausalClaim::Unreconstructable
    );
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![NarrowingReason::RawBacklinkMissing]
    );
    // A floored lane must keep a confidence floor and a precise label.
    assert_eq!(
        lane.effective_confidence(decision.effective_causality_claim),
        ConfidenceTier::UnmappedRequiresReview
    );
    let label = lane.narrowed_label(&decision).expect("floored label");
    assert!(!label_is_generic(&label));
    assert!(label.contains("reopenable"));
}

#[test]
fn heuristic_with_structured_tier_narrows_confidence() {
    let packet = canonical();
    let mut lane = cloned(&packet, LANE_HEURISTIC_PARSE);
    lane.declared_confidence_tier = ConfidenceTier::StructuredFull;
    let decision = lane.narrow(false);
    assert_eq!(decision.effective_causality_claim, CausalClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![NarrowingReason::ConfidenceUnlabeled]
    );
}

#[test]
fn overlay_dropping_read_only_marker_floors() {
    let packet = canonical();
    let mut lane = cloned(&packet, LANE_REMOTE_OVERLAY);
    lane.causal_chain.imported_overlay_read_only = false;
    let decision = lane.narrow(false);
    assert_eq!(
        decision.claimed_causality_claim,
        CausalClaim::ReadOnlyOverlay
    );
    assert_eq!(
        decision.effective_causality_claim,
        CausalClaim::Unreconstructable
    );
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![NarrowingReason::ImportedOverlayClaimsLive]
    );
}

#[test]
fn first_party_stale_projection_narrows() {
    let packet = canonical();
    let mut lane = cloned(&packet, LANE_TEST_NORMALIZED);
    lane.declared_freshness_state = FreshnessState::StaleExpired;
    let decision = lane.narrow(false);
    assert_eq!(decision.effective_causality_claim, CausalClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![NarrowingReason::StaleEvidence]
    );
}

#[test]
fn superseded_without_marker_narrows() {
    let packet = canonical();
    let mut lane = cloned(&packet, LANE_TEST_NORMALIZED);
    lane.declared_freshness_state = FreshnessState::SupersededByNewerRun;
    lane.causal_chain.superseded_state_marked = false;
    let decision = lane.narrow(false);
    assert_eq!(decision.effective_causality_claim, CausalClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![NarrowingReason::SupersededNotMarked]
    );
}

#[test]
fn elapsed_verification_window_narrows_current_proof() {
    let packet = canonical();
    // The stale-window flag stands in for the elapsed verification SLO.
    let decision = row(&packet, LANE_TEST_NORMALIZED).narrow(true);
    assert_eq!(decision.effective_causality_claim, CausalClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![NarrowingReason::StaleProof]
    );
}

#[test]
fn imported_overlay_with_stale_snapshot_stays_overlay() {
    // Stale is expected for an imported overlay snapshot, not dishonest.
    let packet = canonical();
    let decision = row(&packet, LANE_IMPORTED_BUNDLE).narrow(false);
    assert_eq!(
        decision.effective_causality_claim,
        CausalClaim::ReadOnlyOverlay
    );
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn labs_lane_makes_no_claim_and_never_narrows() {
    let packet = canonical();
    let decision = row(&packet, LANE_LABS).narrow(false);
    assert_eq!(
        decision.claimed_causality_claim,
        CausalClaim::LabsNotClaimed
    );
    assert_eq!(
        decision.effective_causality_claim,
        CausalClaim::LabsNotClaimed
    );
    assert!(!decision.narrowed);
    // Even though its lineage is flattened and its export packet is incomplete, a
    // Labs lane is never widened or narrowed against a public claim.
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn canonical_floored_lane_keeps_reopen_fallback() {
    let packet = canonical();
    let lane = row(&packet, LANE_FLOORED);
    let decision = lane.narrow(false);
    assert_eq!(
        decision.effective_causality_claim,
        CausalClaim::Unreconstructable
    );
    assert!(lane.floored_lane_keeps_fallback(decision.effective_causality_claim));
    assert!(lane.declared_reopen_target.is_raw_fallback());
}

// --------------------------------------------------------------------------- //
// Projection guard.
// --------------------------------------------------------------------------- //

#[test]
fn projection_guard_refuses_overlay_rendered_as_certified() {
    let packet = canonical();
    let decision = row(&packet, LANE_REMOTE_OVERLAY).narrow(false);
    assert_eq!(
        decision.effective_causality_claim,
        CausalClaim::ReadOnlyOverlay
    );
    // Rendering the wider certified claim overclaims; rendering the overlay claim
    // or a narrower one does not.
    assert!(decision.surface_overclaims(CausalClaim::Certified));
    assert!(!decision.surface_overclaims(CausalClaim::ReadOnlyOverlay));
    assert!(!decision.surface_overclaims(CausalClaim::Unreconstructable));
}

#[test]
fn labs_projection_only_renders_as_labs() {
    assert!(CausalClaim::LabsNotClaimed.overclaims_as(CausalClaim::Certified));
    assert!(!CausalClaim::LabsNotClaimed.overclaims_as(CausalClaim::LabsNotClaimed));
    assert!(CausalClaim::Certified.overclaims_as(CausalClaim::LabsNotClaimed));
}

// --------------------------------------------------------------------------- //
// Validation negatives.
// --------------------------------------------------------------------------- //

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = canonical();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::WrongRecordKind));
}

#[test]
fn invalid_redaction_class_is_rejected() {
    let mut packet = canonical();
    packet.redaction_class_token = "raw_dump".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::InvalidRedactionClass));
}

#[test]
fn overlay_without_provider_ref_is_rejected() {
    let mut packet = canonical();
    for lane in &mut packet.rows {
        if lane.lane_id == LANE_REMOTE_OVERLAY {
            lane.identity.provider_ref = None;
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::OverlayMissingProviderRef));
}

#[test]
fn floored_lane_without_fallback_is_rejected() {
    let mut packet = canonical();
    for lane in &mut packet.rows {
        if lane.lane_id == LANE_FLOORED {
            // Strip the raw-output fallback from a lane that floors.
            lane.declared_reopen_target = ReopenTarget::EditorAnchor;
            lane.causal_chain.raw_output_backlink_present = false;
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::FlooredLaneLosesFallback));
}

#[test]
fn missing_surface_family_is_rejected() {
    let mut packet = canonical();
    packet
        .rows
        .retain(|lane| !matches!(lane.surface_family, SurfaceFamily::EvidenceBundleExport));
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::SurfaceFamilyMissing));
}

#[test]
fn duplicate_lane_id_is_rejected() {
    let mut packet = canonical();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::DuplicateLaneId));
}

#[test]
fn packet_with_no_narrowing_demonstration_is_rejected() {
    let mut packet = canonical();
    // Keep only clean certified lanes, removing the narrowed and floored cases.
    packet.rows.retain(|lane| {
        matches!(lane.surface_family, SurfaceFamily::ProblemsPanel)
            && lane.lane_id == LANE_STRUCTURED_DIAGNOSTIC
    });
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::DowngradedRowCaseMissing));
}

#[test]
fn forbidden_material_in_label_is_rejected() {
    let mut packet = canonical();
    packet.rows[0].label_summary = "token bearer abcdef".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ExecutionEvidenceCausalityViolation::RawBoundaryMaterialInExport));
}

// --------------------------------------------------------------------------- //
// Builder and freshness.
// --------------------------------------------------------------------------- //

#[test]
fn builder_seals_record_constants() {
    let packet = canonical();
    let built =
        M5ExecutionEvidenceCausalityMatrixPacket::new(M5ExecutionEvidenceCausalityMatrixInput {
            packet_id: packet.packet_id.clone(),
            label: packet.label.clone(),
            as_of: packet.as_of.clone(),
            redaction_class_token: packet.redaction_class_token.clone(),
            verification_freshness: packet.verification_freshness.clone(),
            rows: packet.rows.clone(),
        });
    assert_eq!(built, packet);
    assert!(built.validate().is_empty());
}

#[test]
fn freshness_window_uses_the_slo() {
    let packet = canonical();
    // The packet's own as_of equals the last refresh, so it is not stale.
    assert!(!packet.stale_window());
    // 12 hours later is within the 168-hour SLO; 9 days later is past it.
    assert!(!packet.freshness_stale_at("2026-06-21T12:00:00Z"));
    assert!(packet.freshness_stale_at("2026-06-30T00:00:00Z"));
}

#[test]
fn freshness_honours_auto_downgrade_opt_out() {
    let mut packet = canonical();
    packet.verification_freshness.auto_downgrade_on_stale = false;
    assert!(!packet.freshness_stale_at("2027-01-01T00:00:00Z"));
}

#[test]
fn rfc3339_parser_matches_known_epochs() {
    assert_eq!(days_from_civil(1970, 1, 1), 0);
    assert_eq!(
        parse_rfc3339_to_epoch_seconds("1970-01-01T00:00:00Z"),
        Some(0)
    );
    assert_eq!(
        parse_rfc3339_to_epoch_seconds("1970-01-02T00:00:00Z"),
        Some(86_400)
    );
    assert_eq!(
        parse_rfc3339_to_epoch_seconds("2000-01-01T00:00:00Z"),
        Some(946_684_800)
    );
    // A +01:00 offset rolls back to the same instant as the zulu epoch.
    assert_eq!(
        parse_rfc3339_to_epoch_seconds("1970-01-01T01:00:00+01:00"),
        Some(0)
    );
    // Fractional seconds are truncated, not rejected.
    assert_eq!(
        parse_rfc3339_to_epoch_seconds("1970-01-01T00:00:00.500Z"),
        Some(0)
    );
    assert_eq!(parse_rfc3339_to_epoch_seconds("not-a-date"), None);
}

// --------------------------------------------------------------------------- //
// Token stability.
// --------------------------------------------------------------------------- //

#[test]
fn enum_tokens_round_trip_through_serde() {
    for surface in SurfaceFamily::ALL {
        let json = serde_json::to_string(&surface).expect("serializes");
        assert_eq!(json, format!("\"{}\"", surface.as_str()));
    }
    for origin in OriginClass::ALL {
        let json = serde_json::to_string(&origin).expect("serializes");
        assert_eq!(json, format!("\"{}\"", origin.as_str()));
    }
    for claim in [
        CausalClaim::Unreconstructable,
        CausalClaim::ReadOnlyOverlay,
        CausalClaim::Narrowed,
        CausalClaim::Certified,
        CausalClaim::LabsNotClaimed,
    ] {
        let json = serde_json::to_string(&claim).expect("serializes");
        assert_eq!(json, format!("\"{}\"", claim.as_str()));
    }
}
