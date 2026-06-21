use super::*;

const R_PROVIDER_ENDPOINT: &str = "row:provider-endpoint:0001";
const R_PROVIDER_TOKEN: &str = "row:provider-token:0001";
const R_SOURCE_URL: &str = "row:source-url:0001";
const R_SOURCE_KIND: &str = "row:source-kind:0001";
const R_SOURCE_TRUST: &str = "row:source-trust-policy:0001";
const R_REQUEST_BASE_URL: &str = "row:request-base-url:0001";
const R_REQUEST_HEALTH: &str = "row:request-endpoint-health:0001";
const R_PACKAGE_SCOPE: &str = "row:package-install-scope:0001";
const R_IMPORT_MAPPING: &str = "row:import-mapping:0001";
const R_LABS: &str = "row:labs-import-preview:0001";

fn seeded() -> M5FieldControlRowSetPacket {
    seeded_m5_field_control_row_set()
}

fn row<'a>(packet: &'a M5FieldControlRowSetPacket, id: &str) -> &'a FieldControlRow {
    packet
        .rows
        .iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("missing row {id}"))
}

fn cloned(packet: &M5FieldControlRowSetPacket, id: &str) -> FieldControlRow {
    row(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers
/// the rendered claim to match — otherwise the row itself overclaims and floors.
fn render_all(r: &mut FieldControlRow, claim: RowClaim) {
    r.renderings
        .iter_mut()
        .for_each(|x| x.rendered_claim = claim);
}

fn decide(r: &FieldControlRow) -> RowDecision {
    r.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_FIELD_CONTROL_ROW_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_FIELD_CONTROL_ROW_SCHEMA_VERSION);
    assert_eq!(
        packet.taxonomy_version,
        M5_FIELD_CONTROL_ROW_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.rows.len(), 13);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_field_control_row_set()
        .expect("canonical field-control-row set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seed_covers_every_taxonomy() {
    let packet = seeded();
    for l in RowConsumerLane::ALL {
        assert!(
            packet.represented_lanes().contains(&l),
            "missing lane {l:?}"
        );
    }
    for c in SourceOfValueClass::ALL {
        assert!(
            packet.represented_source_classes().contains(&c),
            "missing source class {c:?}"
        );
    }
    for i in LifecycleImplication::ALL {
        assert!(
            packet.represented_lifecycle_implications().contains(&i),
            "missing lifecycle implication {i:?}"
        );
    }
    for q in Requirement::ALL {
        assert!(
            packet.represented_requirements().contains(&q),
            "missing requirement class {q:?}"
        );
    }
    for cs in ConsumerSurface::ALL {
        assert!(
            packet.represented_consumer_surfaces().contains(&cs),
            "missing consumer surface {cs:?}"
        );
    }
}

#[test]
fn export_safe_json_carries_no_raw_boundary_material() {
    let json = seeded().export_safe_json();
    let lower = json.to_lowercase();
    for forbidden in ["api_key", "password", "secret", "bearer "] {
        assert!(!lower.contains(forbidden), "leaked {forbidden}");
    }
}

// --------------------------------------------------------------------------- //
// Baseline claims.
// --------------------------------------------------------------------------- //

#[test]
fn clean_first_party_rows_certify() {
    let packet = seeded();
    for id in [
        R_PROVIDER_ENDPOINT,
        R_PROVIDER_TOKEN,
        R_SOURCE_URL,
        R_SOURCE_KIND,
        R_SOURCE_TRUST,
        R_REQUEST_BASE_URL,
        R_PACKAGE_SCOPE,
    ] {
        let decision = decide(row(&packet, id));
        assert_eq!(
            decision.effective_claim,
            RowClaim::Certified,
            "{id} should certify, got {decision:?}"
        );
        assert!(!decision.narrowed, "{id} should not be narrowed");
    }
}

#[test]
fn pending_async_row_narrows_as_baseline() {
    let decision = decide(row(&seeded(), R_REQUEST_HEALTH));
    assert_eq!(decision.effective_claim, RowClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![RowNarrowingReason::AsyncValidationPending]
    );
}

#[test]
fn imported_row_is_review_overlay() {
    let decision = decide(row(&seeded(), R_IMPORT_MAPPING));
    assert_eq!(decision.effective_claim, RowClaim::ReviewOverlay);
    assert!(!decision.narrowed);
}

#[test]
fn labs_row_makes_no_claim() {
    let decision = decide(row(&seeded(), R_LABS));
    assert_eq!(decision.effective_claim, RowClaim::LabsNotClaimed);
    assert!(!decision.narrowed);
}

// --------------------------------------------------------------------------- //
// Floors (break the row primitive contract).
// --------------------------------------------------------------------------- //

#[test]
fn placeholder_only_label_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.label_mode = LabelMode::PlaceholderOnly;
    render_all(&mut r, RowClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![RowNarrowingReason::LabelNotPermanent]
    );
    assert!(r.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn hidden_source_tag_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.source_tag_visible = false;
    render_all(&mut r, RowClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![RowNarrowingReason::SourceTagHidden]
    );
}

#[test]
fn override_not_distinct_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.override_distinct_from_origin = false;
    render_all(&mut r, RowClaim::Blocked);
    assert_eq!(decide(&r).effective_claim, RowClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::SourceTagHidden]
    );
}

#[test]
fn rendering_that_hides_anchor_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.renderings[0].anchor_visible = false;
    render_all(&mut r, RowClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::SourceTagHidden]
    );
}

#[test]
fn silently_overridden_policy_lock_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_TRUST);
    r.policy_lock_respected = false;
    render_all(&mut r, RowClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::PolicyLockOverridden]
    );
}

#[test]
fn summary_only_blocking_validation_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.validation.state = ValidationState::InvalidBlocking;
    r.validation.anchored_to_field = false;
    r.validation.summary_banner_only = true;
    r.validation.exact_rule_text_present = false;
    render_all(&mut r, RowClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::ValidationAnchorMissing]
    );
}

#[test]
fn hidden_lifecycle_implication_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_PACKAGE_SCOPE);
    r.lifecycle.surfaced_on_row = false;
    render_all(&mut r, RowClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::LifecycleImplicationHidden]
    );
}

#[test]
fn imported_value_that_reads_editable_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_IMPORT_MAPPING);
    r.field_state = FieldState::Editable;
    render_all(&mut r, RowClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![RowNarrowingReason::ImportedValueReadsAsEditable]
    );
}

#[test]
fn missing_backing_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.declared_freshness_state = FreshnessState::Missing;
    render_all(&mut r, RowClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::RowBackingMissing]
    );
}

#[test]
fn overclaiming_rendering_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, R_REQUEST_HEALTH);
    // The endpoint-health row narrows; a rendering that still shows certified
    // overclaims and floors.
    r.renderings[0].rendered_claim = RowClaim::Certified;
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![
            RowNarrowingReason::RowOverclaims,
            RowNarrowingReason::AsyncValidationPending
        ]
    );
}

#[test]
fn silent_blocked_fallback_loses_recovery() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.label_mode = LabelMode::None;
    r.blocked_fallback = BlockedFallback::NoneSilent;
    render_all(&mut r, RowClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Blocked);
    assert!(!r.floored_keeps_fallback(decision.effective_claim));
}

// --------------------------------------------------------------------------- //
// Narrows (recoverable, stays usable).
// --------------------------------------------------------------------------- //

#[test]
fn unmarked_requirement_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_KIND);
    r.requirement_marked = false;
    render_all(&mut r, RowClaim::Narrowed);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![RowNarrowingReason::RequirementUnmarked]
    );
}

#[test]
fn unlabeled_validation_state_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.validation.state_labeled = false;
    render_all(&mut r, RowClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::ValidationStateUnlabeled]
    );
}

#[test]
fn unlabeled_freshness_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.freshness_state_visible = false;
    render_all(&mut r, RowClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::FreshnessUnlabeled]
    );
}

#[test]
fn first_party_stale_narrows_but_overlay_holds() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut r, RowClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::RowStale]
    );
}

#[test]
fn superseded_unmarked_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    r.superseded_state_marked = false;
    render_all(&mut r, RowClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::SupersededStateNotMarked]
    );
}

#[test]
fn superseded_marked_holds_certified() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Certified);
    assert!(!decision.narrowed);
}

#[test]
fn missing_proof_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    r.verification.proof_currency = ProofCurrency::MissingProof;
    r.verification.proof_ref = None;
    render_all(&mut r, RowClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![RowNarrowingReason::VerificationProofMissing]
    );
}

#[test]
fn stale_window_ages_current_proof_out() {
    let packet = seeded();
    let mut r = cloned(&packet, R_SOURCE_URL);
    render_all(&mut r, RowClaim::Narrowed);
    let decision = r.narrow(true);
    assert_eq!(decision.effective_claim, RowClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![RowNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn overlay_with_any_gap_floors_below_overlay() {
    let packet = seeded();
    let mut r = cloned(&packet, R_IMPORT_MAPPING);
    r.freshness_state_visible = false;
    render_all(&mut r, RowClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, RowClaim::Blocked);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![RowNarrowingReason::FreshnessUnlabeled]
    );
}

// --------------------------------------------------------------------------- //
// Validator wiring.
// --------------------------------------------------------------------------- //

#[test]
fn validator_flags_silent_floored_row() {
    let mut packet = seeded();
    let r = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == R_SOURCE_URL)
        .unwrap();
    r.label_mode = LabelMode::None;
    r.blocked_fallback = BlockedFallback::NoneSilent;
    render_all(r, RowClaim::Blocked);
    let violations = packet.validate();
    assert!(violations.contains(&M5FieldControlRowViolation::FlooredRowLosesFallback));
}

#[test]
fn validator_flags_overlay_without_provenance() {
    let mut packet = seeded();
    let r = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == R_IMPORT_MAPPING)
        .unwrap();
    r.provenance_ref = None;
    assert!(packet
        .validate()
        .contains(&M5FieldControlRowViolation::OverlayMissingProvenanceRef));
}

#[test]
fn validator_flags_overclaiming_rendering() {
    let mut packet = seeded();
    let r = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == R_REQUEST_HEALTH)
        .unwrap();
    r.renderings[0].rendered_claim = RowClaim::Certified;
    assert!(packet
        .validate()
        .contains(&M5FieldControlRowViolation::RenderingRowOverclaims));
}

#[test]
fn narrowed_label_is_specific() {
    let packet = seeded();
    let r = row(&packet, R_REQUEST_HEALTH);
    let decision = decide(r);
    let label = r.narrowed_label(&decision).expect("narrowed label present");
    assert!(label.contains("row_narrowed"));
    assert!(label.to_lowercase().contains("async validation"));
}

#[test]
fn distribution_counts_every_class() {
    let dist = seeded().claim_distribution();
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 1);
    assert_eq!(dist.labs, 1);
    assert_eq!(dist.blocked, 0);
    assert_eq!(dist.certified, 10);
}
