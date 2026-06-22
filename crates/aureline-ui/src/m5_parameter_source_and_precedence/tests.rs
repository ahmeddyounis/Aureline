use super::*;

const F_PROVIDER: &str = "field:provider-account-mapping:0001";
const F_ADMIN: &str = "field:source-registration:0001";
const F_REQUEST: &str = "field:request-environment:0001";
const F_PACKAGE: &str = "field:package-install-config:0001";
const F_SETTINGS: &str = "field:settings-config-editor:0001";
const F_IMPORT: &str = "field:import-migration-mapping:0001";
const F_LABS: &str = "field:project-bootstrap:0001";

fn seeded() -> M5ParameterSourceSetPacket {
    seeded_m5_parameter_source_set()
}

fn field<'a>(packet: &'a M5ParameterSourceSetPacket, id: &str) -> &'a ParameterFieldRecord {
    packet
        .fields
        .iter()
        .find(|f| f.field_id == id)
        .unwrap_or_else(|| panic!("missing field {id}"))
}

fn cloned(packet: &M5ParameterSourceSetPacket, id: &str) -> ParameterFieldRecord {
    field(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers the
/// rendered claim to match — otherwise the field itself overclaims and floors.
fn render_all(f: &mut ParameterFieldRecord, claim: ParameterClaim) {
    f.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = claim);
}

fn decide(f: &ParameterFieldRecord) -> FieldDecision {
    f.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_PARAMETER_SOURCE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_PARAMETER_SOURCE_SCHEMA_VERSION);
    assert_eq!(
        packet.taxonomy_version,
        M5_PARAMETER_SOURCE_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.fields.len(), 7);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_parameter_source_set()
        .expect("canonical parameter-source set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seed_covers_every_taxonomy() {
    let packet = seeded();
    for x in FieldForm::ALL {
        assert!(
            packet.represented_forms().contains(&x),
            "missing form {x:?}"
        );
    }
    for l in FieldLane::ALL {
        assert!(
            packet.represented_lanes().contains(&l),
            "missing lane {l:?}"
        );
    }
    for s in SourceLayer::ALL {
        assert!(
            packet.represented_source_layers().contains(&s),
            "missing source layer {s:?}"
        );
    }
    for s in ValueScope::ALL {
        assert!(
            packet.represented_value_scopes().contains(&s),
            "missing value scope {s:?}"
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
fn every_effective_layer_is_highest_present() {
    let packet = seeded();
    for f in &packet.fields {
        assert_eq!(
            f.highest_present_layer(),
            Some(f.inspector.effective.effective_source_layer),
            "{} effective layer is not the highest present candidate",
            f.field_id
        );
    }
}

// --------------------------------------------------------------------------- //
// Baseline claims.
// --------------------------------------------------------------------------- //

#[test]
fn clean_first_party_fields_certify() {
    let packet = seeded();
    for id in [F_PROVIDER, F_ADMIN, F_PACKAGE, F_SETTINGS] {
        let decision = decide(field(&packet, id));
        assert_eq!(
            decision.effective_claim,
            ParameterClaim::Certified,
            "{id} should certify, got {decision:?}"
        );
        assert!(!decision.narrowed, "{id} should not be narrowed");
    }
}

#[test]
fn request_field_narrows_on_proof_requires_review() {
    let packet = seeded();
    let decision = decide(field(&packet, F_REQUEST));
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![ParameterNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn import_field_is_overlay_not_user_set() {
    let packet = seeded();
    let decision = decide(field(&packet, F_IMPORT));
    assert_eq!(decision.effective_claim, ParameterClaim::ReviewOverlay);
    assert!(!decision.narrowed);
}

#[test]
fn labs_field_makes_no_claim() {
    let packet = seeded();
    let decision = decide(field(&packet, F_LABS));
    assert_eq!(decision.effective_claim, ParameterClaim::LabsNotClaimed);
    assert!(!decision.narrowed);
}

#[test]
fn admin_field_keeps_override_distinct_under_policy_lock() {
    let packet = seeded();
    let f = field(&packet, F_ADMIN);
    // The user override is present but does not win; the policy lock is honoured.
    assert!(f.inspector.policy_lock.policy_locked);
    assert!(f
        .inspector
        .candidates
        .iter()
        .any(|c| c.source_layer == SourceLayer::UserOverride && c.present));
    assert_eq!(
        f.inspector.effective.effective_source_layer,
        SourceLayer::PolicyProvided
    );
    assert_eq!(decide(f).effective_claim, ParameterClaim::Certified);
}

// --------------------------------------------------------------------------- //
// Floor reasons.
// --------------------------------------------------------------------------- //

#[test]
fn effective_source_hidden_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.inspector.effective.effective_source_visible = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![ParameterNarrowingReason::EffectiveSourceHidden]
    );
    assert!(f.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn sources_collapsed_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.inspector.sources_distinct = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![ParameterNarrowingReason::SourcesCollapsed]
    );
}

#[test]
fn precedence_inconsistent_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    // Declare a lower layer effective while a higher-precedence candidate is present.
    f.inspector.effective.effective_source_layer = SourceLayer::Detected;
    f.inspector.effective.precedence_rank_declared = SourceLayer::Detected.precedence_rank();
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::PrecedenceInconsistent));
}

#[test]
fn declared_rank_mismatch_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.inspector.effective.precedence_rank_declared = 0;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::PrecedenceInconsistent));
}

#[test]
fn policy_lock_hidden_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_ADMIN);
    f.inspector.policy_lock.lock_surfaced = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::PolicyLockHidden));
}

#[test]
fn policy_lock_not_enforced_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_ADMIN);
    f.inspector.policy_lock.override_allowed_despite_lock = true;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::PolicyLockNotEnforced));
}

#[test]
fn imported_value_reads_as_user_set_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_IMPORT);
    f.integrity.imported_review_read_only = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::ImportedValueReadsAsUserSet));
}

#[test]
fn fallback_reason_hidden_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_PACKAGE);
    f.inspector.fallback.fallback_reason_disclosed = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::FallbackReasonHidden));
}

#[test]
fn value_scope_hidden_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.inspector.effective.effective_scope_visible = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::ValueScopeHidden));
}

#[test]
fn ambiguous_submit_allowed_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.integrity.submit_gated_on_source_clarity = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::AmbiguousSubmitAllowed));
}

#[test]
fn overlay_does_not_gate_submit() {
    let packet = seeded();
    let mut f = cloned(&packet, F_IMPORT);
    // An overlay is read-only, so the submit gate never applies even when off.
    f.integrity.submit_gated_on_source_clarity = false;
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::ReviewOverlay);
    assert!(!decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::AmbiguousSubmitAllowed));
}

#[test]
fn inspect_path_lost_keeps_keyboard_fallback() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.declared_reopen_target = ReopenTarget::NoneKeyboardFallback;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::InspectPathLost));
    assert!(f.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn provenance_backing_missing_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.declared_detection_state = DetectionState::Missing;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::ProvenanceBackingMissing));
}

#[test]
fn overlay_with_any_gap_floors() {
    let packet = seeded();
    let mut f = cloned(&packet, F_IMPORT);
    // A non-floor gap on an overlay drops below the review overlay rather than
    // holding it.
    f.inspector.precedence_explained = false;
    render_all(&mut f, ParameterClaim::Unsafe);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
}

// --------------------------------------------------------------------------- //
// Narrowing (non-floor) reasons.
// --------------------------------------------------------------------------- //

#[test]
fn source_labels_unlabeled_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    // The imported (non-winning) candidate loses its source label.
    f.inspector.candidates[1].source_labeled = false;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::SourceLabelsUnlabeled));
}

#[test]
fn absent_candidate_unlabeled_does_not_narrow() {
    let packet = seeded();
    let mut f = cloned(&packet, F_PACKAGE);
    // The override candidate is absent, so its missing label is exempt.
    f.inspector.candidates[1].source_labeled = false;
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Certified);
}

#[test]
fn scope_labels_unlabeled_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    // The detected (non-winning) candidate loses its scope label.
    f.inspector.candidates[2].scope_labeled = false;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::ScopeLabelsUnlabeled));
}

#[test]
fn fallback_reason_unlabeled_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_PACKAGE);
    f.inspector.fallback.fallback_reason_labeled = false;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::FallbackReasonUnlabeled));
}

#[test]
fn precedence_explanation_unlabeled_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.inspector.precedence_explained = false;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::PrecedenceExplanationUnlabeled));
}

#[test]
fn detection_state_unlabeled_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.integrity.detection_state_visible = false;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::DetectionStateUnlabeled));
}

#[test]
fn detection_superseded_unmarked_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.declared_detection_state = DetectionState::SupersededByNewerSource;
    f.integrity.superseded_state_marked = false;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::DetectionSupersededUnmarked));
}

#[test]
fn detection_superseded_marked_stays_certified() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.declared_detection_state = DetectionState::SupersededByNewerSource;
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Certified);
}

#[test]
fn detection_stale_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.declared_detection_state = DetectionState::StaleExpired;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::DetectionStale));
}

#[test]
fn missing_proof_narrows() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.verification.proof_currency = ProofCurrency::MissingProof;
    f.verification.proof_ref = None;
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = decide(&f);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::VerificationProofMissing));
}

#[test]
fn stale_window_ages_current_proof() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    render_all(&mut f, ParameterClaim::Narrowed);
    let decision = f.narrow(true);
    assert_eq!(decision.effective_claim, ParameterClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::VerificationProofStale));
}

// --------------------------------------------------------------------------- //
// Overclaim + structural validation.
// --------------------------------------------------------------------------- //

#[test]
fn rendering_overclaim_floors_via_validator() {
    let mut packet = seeded();
    // Leave the request field narrowed but make a rendering claim certified.
    let request = packet
        .fields
        .iter_mut()
        .find(|f| f.field_id == F_REQUEST)
        .unwrap();
    request.renderings[0].rendered_claim = ParameterClaim::Certified;
    let decision = request.narrow(false);
    assert_eq!(decision.effective_claim, ParameterClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ParameterNarrowingReason::InspectorOverclaims));
    assert!(!packet.validate().is_empty());
}

#[test]
fn duplicate_field_id_is_rejected() {
    let mut packet = seeded();
    let dup = cloned(&packet, F_SETTINGS);
    packet.fields.push(dup);
    assert!(packet
        .validate()
        .contains(&M5ParameterSourceViolation::DuplicateFieldId));
}

#[test]
fn overlay_without_provenance_ref_is_rejected() {
    let mut packet = seeded();
    let import = packet
        .fields
        .iter_mut()
        .find(|f| f.field_id == F_IMPORT)
        .unwrap();
    import.lineage.provider_ref = None;
    import.lineage.source_artifact_ref = None;
    assert!(packet
        .validate()
        .contains(&M5ParameterSourceViolation::OverlayMissingProvenanceRef));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded();
    let json = packet.export_safe_json();
    let lower = json.to_lowercase();
    for needle in ["api_key", "password", "secret", "bearer "] {
        assert!(!lower.contains(needle), "export leaked {needle}");
    }
}

#[test]
fn report_renders_and_lists_narrowed() {
    let packet = seeded();
    let report = packet.render_markdown_report();
    assert!(report.contains("parameter_certified"));
    assert!(report.contains("parameter_narrowed"));
    assert!(report.contains(F_REQUEST));
}

#[test]
fn narrowed_label_is_not_generic() {
    let packet = seeded();
    let f = field(&packet, F_REQUEST);
    let decision = decide(f);
    let label = f.narrowed_label(&decision).expect("narrowed label");
    assert!(!label_is_generic(&label), "label was generic: {label}");
}

#[test]
fn claim_distribution_matches_seed() {
    let packet = seeded();
    let dist = packet.claim_distribution();
    assert_eq!(dist.certified, 4);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 1);
    assert_eq!(dist.unsafe_fields, 0);
    assert_eq!(dist.labs, 1);
}
