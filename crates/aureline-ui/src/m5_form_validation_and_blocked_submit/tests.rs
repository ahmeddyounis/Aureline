use super::*;

const F_PROVIDER: &str = "form:provider-connection:0001";
const F_SETTINGS: &str = "form:settings-config:0001";
const F_PROJECTS: &str = "wizard:project-bootstrap:0001";
const F_PACKAGE: &str = "sheet:package-install:0001";
const F_ADMIN: &str = "sheet:admin-policy-rollout:0001";
const F_REQUEST: &str = "dialog:request-run:0001";
const F_IMPORT: &str = "dialog:migration-restore:0001";
const F_LABS: &str = "wizard:labs-onboarding:0001";

fn seeded() -> M5FormValidationSetPacket {
    seeded_m5_form_validation_set()
}

fn rec<'a>(packet: &'a M5FormValidationSetPacket, id: &str) -> &'a FormValidationRecord {
    packet
        .surfaces
        .iter()
        .find(|r| r.surface_id == id)
        .unwrap_or_else(|| panic!("missing form {id}"))
}

fn cloned(packet: &M5FormValidationSetPacket, id: &str) -> FormValidationRecord {
    rec(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers
/// the rendered claim to match — otherwise the form itself overclaims and floors.
fn render_all(r: &mut FormValidationRecord, claim: FormClaim) {
    r.renderings
        .iter_mut()
        .for_each(|x| x.rendered_claim = claim);
}

fn decide(r: &FormValidationRecord) -> FormDecision {
    r.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_FORM_VALIDATION_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_FORM_VALIDATION_SCHEMA_VERSION);
    assert_eq!(packet.taxonomy_version, M5_FORM_VALIDATION_TAXONOMY_VERSION);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.surfaces.len(), 8);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_form_validation_set()
        .expect("canonical form-validation set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seed_covers_every_taxonomy() {
    let packet = seeded();
    for l in FormLane::ALL {
        assert!(
            packet.represented_lanes().contains(&l),
            "missing lane {l:?}"
        );
    }
    for k in DependencyKind::ALL {
        assert!(
            packet.represented_dependency_kinds().contains(&k),
            "missing dependency kind {k:?}"
        );
    }
    for r in DependencyRelation::ALL {
        assert!(
            packet.represented_dependency_relations().contains(&r),
            "missing dependency relation {r:?}"
        );
    }
    for c in SubmitBlockerClass::ALL {
        assert!(
            packet.represented_blocker_classes().contains(&c),
            "missing blocker class {c:?}"
        );
    }
    for c in BlockedSubmitConsumer::ALL {
        assert!(
            packet.represented_blocked_consumers().contains(&c),
            "missing blocked consumer {c:?}"
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
fn clean_first_party_forms_certify() {
    let packet = seeded();
    for id in [F_PROVIDER, F_SETTINGS, F_PROJECTS, F_PACKAGE, F_ADMIN] {
        let decision = decide(rec(&packet, id));
        assert_eq!(
            decision.effective_claim,
            FormClaim::Certified,
            "{id} should certify, got {decision:?}"
        );
        assert!(!decision.narrowed, "{id} should not be narrowed");
    }
}

#[test]
fn certified_form_with_active_blocker_keeps_gate_closed() {
    // A provider form with an explained, reusable, machine-readable blocker is
    // still certified — its blocked-submit truth is honest and the gate is closed.
    let packet = seeded();
    let provider = rec(&packet, F_PROVIDER);
    assert!(!provider.submit_gate.submit_allowed);
    assert!(provider.any_active_blocker());
    assert_eq!(decide(provider).effective_claim, FormClaim::Certified);
}

#[test]
fn pending_async_form_narrows_as_baseline() {
    let decision = decide(rec(&seeded(), F_REQUEST));
    assert_eq!(decision.effective_claim, FormClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![FormNarrowingReason::AsyncValidationPending]
    );
}

#[test]
fn imported_form_is_review_overlay() {
    let decision = decide(rec(&seeded(), F_IMPORT));
    assert_eq!(decision.effective_claim, FormClaim::ReviewOverlay);
    assert!(!decision.narrowed);
}

#[test]
fn labs_form_makes_no_claim() {
    let decision = decide(rec(&seeded(), F_LABS));
    assert_eq!(decision.effective_claim, FormClaim::LabsNotClaimed);
    assert!(!decision.narrowed);
}

// --------------------------------------------------------------------------- //
// Floors (break the form-validation contract).
// --------------------------------------------------------------------------- //

#[test]
fn submit_open_while_blocked_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.submit_gate.submit_allowed = true;
    render_all(&mut r, FormClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, FormClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![FormNarrowingReason::SubmitAllowedWhileBlockedHidden]
    );
    assert!(r.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn unexplained_blocking_reason_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.blocked_submit_reasons[0].explained_before_submit = false;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::BlockedReasonUnexplained]
    );
}

#[test]
fn hidden_cross_field_invalidation_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.dependencies[0].blocks_submit = true;
    r.dependencies[0].explained_before_submit = false;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::CrossFieldInvalidationHidden]
    );
}

#[test]
fn summary_contradicting_fields_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.form_summary.consistent_with_fields = false;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::FieldFormValidationContradicts]
    );
}

#[test]
fn blocking_field_without_reason_contradicts() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    // Make a field invalid-blocking with no backing blocked-submit reason. The
    // gate is correctly closed, so the only gap is the field/form contradiction.
    r.field_anchors[0].validation_state = ValidationState::InvalidBlocking;
    r.submit_gate.submit_allowed = false;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::FieldFormValidationContradicts]
    );
}

#[test]
fn summary_replacing_anchors_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.form_summary.replaces_field_anchors = true;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::FormSummaryReplacesFieldAnchors]
    );
}

#[test]
fn hidden_derived_constraint_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_ADMIN);
    r.form_summary.derived_constraints_disclosed = false;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::DerivedConstraintHidden]
    );
}

#[test]
fn non_machine_readable_reason_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.blocked_submit_reasons[0].machine_code = "  ".to_owned();
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::BlockedReasonNotMachineReadable]
    );
}

#[test]
fn non_reusable_blocking_reason_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    // Drop the machine consumers from a blocking reason.
    r.blocked_submit_reasons[0].reusable_by = vec![BlockedSubmitConsumer::Desktop];
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::BlockedReasonNotReusable]
    );
}

#[test]
fn banner_only_blocking_validation_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    // The account field is already invalid-blocking; strip its field anchor.
    r.field_anchors[1].anchored_to_field = false;
    r.field_anchors[1].exact_rule_text_present = false;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::ValidationAnchorMissing]
    );
}

#[test]
fn imported_review_that_submits_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_IMPORT);
    r.submit_gate.submit_allowed = true;
    render_all(&mut r, FormClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, FormClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![FormNarrowingReason::ImportedSubmitReadsAsApplied]
    );
}

#[test]
fn overclaiming_rendering_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_REQUEST);
    // The request dialog narrows; a rendering that still shows certified floors.
    r.renderings[0].rendered_claim = FormClaim::Certified;
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, FormClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![
            FormNarrowingReason::RenderingOverclaims,
            FormNarrowingReason::AsyncValidationPending
        ]
    );
}

#[test]
fn missing_backing_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::Missing;
    render_all(&mut r, FormClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::ValidationBackingMissing]
    );
}

#[test]
fn silent_blocked_fallback_loses_recovery() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::Missing;
    r.declared_blocked_fallback = BlockedSubmitFallback::NoneSilent;
    render_all(&mut r, FormClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, FormClaim::Blocked);
    assert!(!r.floored_keeps_fallback(decision.effective_claim));
}

// --------------------------------------------------------------------------- //
// Narrows (recoverable, stays usable).
// --------------------------------------------------------------------------- //

#[test]
fn deferred_dependency_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    // A non-blocking dependency left unexplained narrows rather than floors.
    r.dependencies[0].explained_before_submit = false;
    render_all(&mut r, FormClaim::Narrowed);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, FormClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![FormNarrowingReason::CrossFieldDependencyDeferred]
    );
}

#[test]
fn missing_resolution_hint_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.blocked_submit_reasons[0].resolution_hint_present = false;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::ResolutionHintMissing]
    );
}

#[test]
fn unlabeled_validation_state_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.field_anchors[0].state_labeled = false;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::ValidationStateUnlabeled]
    );
}

#[test]
fn unlabeled_freshness_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.integrity.freshness_state_visible = false;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::FreshnessUnlabeled]
    );
}

#[test]
fn first_party_stale_narrows_but_overlay_holds() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::FormStale]
    );
}

#[test]
fn superseded_unmarked_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    r.integrity.superseded_state_marked = false;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::SupersededStateNotMarked]
    );
}

#[test]
fn superseded_marked_holds_certified() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, FormClaim::Certified);
    assert!(!decision.narrowed);
}

#[test]
fn missing_proof_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.verification.proof_currency = ProofCurrency::MissingProof;
    r.verification.proof_ref = None;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::VerificationProofMissing]
    );
}

#[test]
fn requires_review_proof_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.verification.proof_currency = ProofCurrency::RequiresReview;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn stale_window_ages_current_proof_out() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    render_all(&mut r, FormClaim::Narrowed);
    let decision = r.narrow(true);
    assert_eq!(decision.effective_claim, FormClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![FormNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn reopen_path_lost_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.integrity.reopen_visible_on_demand = false;
    render_all(&mut r, FormClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![FormNarrowingReason::ReopenPathLost]
    );
}

#[test]
fn overlay_with_any_gap_floors_below_overlay() {
    let packet = seeded();
    let mut r = cloned(&packet, F_IMPORT);
    r.integrity.freshness_state_visible = false;
    render_all(&mut r, FormClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, FormClaim::Blocked);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![FormNarrowingReason::FreshnessUnlabeled]
    );
}

// --------------------------------------------------------------------------- //
// Validator wiring.
// --------------------------------------------------------------------------- //

#[test]
fn validator_flags_silent_floored_form() {
    let mut packet = seeded();
    let r = packet
        .surfaces
        .iter_mut()
        .find(|r| r.surface_id == F_SETTINGS)
        .unwrap();
    r.declared_freshness_state = FreshnessState::Missing;
    r.declared_blocked_fallback = BlockedSubmitFallback::NoneSilent;
    render_all(r, FormClaim::Blocked);
    let violations = packet.validate();
    assert!(violations.contains(&M5FormValidationViolation::FlooredSurfaceLosesFallback));
}

#[test]
fn validator_flags_overlay_without_provenance() {
    let mut packet = seeded();
    let r = packet
        .surfaces
        .iter_mut()
        .find(|r| r.surface_id == F_IMPORT)
        .unwrap();
    r.lineage.provider_ref = None;
    r.lineage.source_artifact_ref = None;
    assert!(packet
        .validate()
        .contains(&M5FormValidationViolation::OverlayMissingProvenanceRef));
}

#[test]
fn validator_flags_overclaiming_rendering() {
    let mut packet = seeded();
    let r = packet
        .surfaces
        .iter_mut()
        .find(|r| r.surface_id == F_REQUEST)
        .unwrap();
    r.renderings[0].rendered_claim = FormClaim::Certified;
    assert!(packet
        .validate()
        .contains(&M5FormValidationViolation::RenderingSurfaceOverclaims));
}

#[test]
fn validator_flags_missing_blocked_consumer() {
    let mut packet = seeded();
    // Strip every machine/desktop consumer down to one so the union is incomplete.
    for s in &mut packet.surfaces {
        for b in &mut s.blocked_submit_reasons {
            b.reusable_by = vec![BlockedSubmitConsumer::Desktop];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5FormValidationViolation::BlockedConsumerMissing));
}

#[test]
fn narrowed_label_is_specific() {
    let packet = seeded();
    let r = rec(&packet, F_REQUEST);
    let decision = decide(r);
    let label = r.narrowed_label(&decision).expect("narrowed label present");
    assert!(label.contains("form_narrowed"));
    assert!(label.to_lowercase().contains("async validation"));
}

#[test]
fn distribution_counts_every_class() {
    let dist = seeded().claim_distribution();
    assert_eq!(dist.certified, 5);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 1);
    assert_eq!(dist.labs, 1);
    assert_eq!(dist.blocked, 0);
}
