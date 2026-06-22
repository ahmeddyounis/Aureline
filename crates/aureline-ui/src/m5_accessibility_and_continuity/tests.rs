use super::*;

const S_PROVIDER: &str = "surface:provider-connect-wizard:0001";
const S_ADMIN: &str = "surface:admin-source-batch-review:0001";
const S_REQUEST: &str = "surface:request-environment-validation:0001";
const S_PACKAGE: &str = "surface:package-install-review:0001";
const S_SETTINGS: &str = "surface:settings-config-editor:0001";
const S_IMPORT: &str = "surface:import-migration-review:0001";
const S_LABS: &str = "surface:project-bootstrap-wizard:0001";

fn seeded() -> M5AccessibilityContinuitySetPacket {
    seeded_m5_accessibility_continuity_set()
}

fn surface<'a>(packet: &'a M5AccessibilityContinuitySetPacket, id: &str) -> &'a SurfaceRecord {
    packet
        .surfaces
        .iter()
        .find(|s| s.surface_id == id)
        .unwrap_or_else(|| panic!("missing surface {id}"))
}

fn cloned(packet: &M5AccessibilityContinuitySetPacket, id: &str) -> SurfaceRecord {
    surface(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers the
/// rendered claim to match — otherwise the surface itself overclaims and floors.
fn render_all(s: &mut SurfaceRecord, claim: ContinuityClaim) {
    s.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = claim);
}

fn decide(s: &SurfaceRecord) -> SurfaceDecision {
    s.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_ACCESSIBILITY_CONTINUITY_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        M5_ACCESSIBILITY_CONTINUITY_SCHEMA_VERSION
    );
    assert_eq!(
        packet.taxonomy_version,
        M5_ACCESSIBILITY_CONTINUITY_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.surfaces.len(), 7);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_accessibility_continuity_set()
        .expect("canonical accessibility-continuity set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seed_covers_every_taxonomy() {
    let packet = seeded();
    for x in SurfaceKind::ALL {
        assert!(
            packet.represented_kinds().contains(&x),
            "missing kind {x:?}"
        );
    }
    for l in SurfaceLane::ALL {
        assert!(
            packet.represented_lanes().contains(&l),
            "missing lane {l:?}"
        );
    }
    for o in SurfaceOrigin::ALL {
        assert!(
            packet.represented_origins().contains(&o),
            "missing origin {o:?}"
        );
    }
    for p in InterruptionPath::ALL {
        assert!(
            packet.represented_interruption_paths().contains(&p),
            "missing interruption path {p:?}"
        );
    }
    for cs in ConsumerSurface::ALL {
        assert!(
            packet.represented_consumer_surfaces().contains(&cs),
            "missing consumer surface {cs:?}"
        );
    }
    for rm in ALL_REDUCED_MOTION_CLASSES {
        assert!(
            packet
                .surfaces
                .iter()
                .any(|s| s.accessibility.reduced_motion.substitution_class == rm),
            "missing reduced-motion class {rm:?}"
        );
    }
}

// --------------------------------------------------------------------------- //
// Baseline claims.
// --------------------------------------------------------------------------- //

#[test]
fn clean_first_party_surfaces_certify() {
    let packet = seeded();
    for id in [S_PROVIDER, S_ADMIN, S_PACKAGE, S_SETTINGS] {
        let decision = decide(surface(&packet, id));
        assert_eq!(
            decision.effective_claim,
            ContinuityClaim::Certified,
            "{id} should certify, got {decision:?}"
        );
        assert!(!decision.narrowed, "{id} should not be narrowed");
    }
}

#[test]
fn request_surface_narrows_on_partial_journal() {
    let packet = seeded();
    let decision = decide(surface(&packet, S_REQUEST));
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![ContinuityNarrowingReason::JournalPartial]
    );
}

#[test]
fn import_surface_is_overlay_not_apply() {
    let packet = seeded();
    let decision = decide(surface(&packet, S_IMPORT));
    assert_eq!(decision.effective_claim, ContinuityClaim::ReviewOverlay);
    assert!(!decision.narrowed);
}

#[test]
fn labs_surface_makes_no_claim() {
    let packet = seeded();
    let decision = decide(surface(&packet, S_LABS));
    assert_eq!(decision.effective_claim, ContinuityClaim::LabsNotClaimed);
    assert!(!decision.narrowed);
}

#[test]
fn batch_sheet_keeps_keyboard_parity() {
    let packet = seeded();
    let s = surface(&packet, S_ADMIN);
    assert!(s.surface_kind.has_batch_actions());
    assert!(s.accessibility.keyboard.batch_actions_keyboard_parity);
    assert_eq!(decide(s).effective_claim, ContinuityClaim::Certified);
}

// --------------------------------------------------------------------------- //
// Floor reasons.
// --------------------------------------------------------------------------- //

#[test]
fn keyboard_path_incomplete_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.keyboard.all_controls_reachable = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![ContinuityNarrowingReason::KeyboardPathIncomplete]
    );
    assert!(s.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn focus_trap_inescapable_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROVIDER);
    s.accessibility.keyboard.focus_trap_escapable = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::KeyboardPathIncomplete));
}

#[test]
fn focus_order_undefined_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.keyboard.focus_order_defined = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::FocusOrderUndefined));
}

#[test]
fn batch_actions_keyboard_unreachable_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_ADMIN);
    s.accessibility.keyboard.batch_actions_keyboard_parity = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::BatchActionsKeyboardUnreachable));
}

#[test]
fn batch_parity_exempt_for_non_batch_surface() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    // A config editor has no batch actions, so an unset parity flag is irrelevant.
    s.accessibility.keyboard.batch_actions_keyboard_parity = false;
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Certified);
}

#[test]
fn screen_reader_labels_missing_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.assistive_tech.screen_reader_labels_present = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ScreenReaderLabelsMissing));
}

#[test]
fn validation_links_not_announced_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_REQUEST);
    s.accessibility.assistive_tech.validation_links_announced = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ValidationLinksNotAnnounced));
}

#[test]
fn validation_links_exempt_for_batch_sheet() {
    let packet = seeded();
    let mut s = cloned(&packet, S_ADMIN);
    // A batch-review sheet has no inline field validation links, so an unset announce
    // flag is irrelevant.
    s.accessibility.assistive_tech.validation_links_announced = false;
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Certified);
}

#[test]
fn blocked_submit_not_announced_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.assistive_tech.blocked_submit_live_region = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::BlockedSubmitNotAnnounced));
}

#[test]
fn motion_only_state_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.reduced_motion.state_conveyed_without_motion = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::MotionOnlyState));
}

#[test]
fn current_step_lost_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROVIDER);
    s.continuity.current_step_preserved = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::CurrentStepLost));
}

#[test]
fn blocked_fields_lost_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROVIDER);
    s.continuity.blocked_fields_preserved = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::BlockedFieldsLost));
}

#[test]
fn draft_state_lost_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROVIDER);
    s.continuity.draft_state_preserved = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::DraftStateLost));
}

#[test]
fn imported_review_mutable_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_IMPORT);
    s.integrity.imported_review_read_only = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ImportedReviewMutable));
}

#[test]
fn recovery_path_lost_keeps_keyboard_fallback() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_recovery_target = RecoveryTarget::NoneKeyboardFallback;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::RecoveryPathLost));
    assert!(s.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn continuity_journal_missing_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.continuity.journal_state = JournalState::Missing;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ContinuityJournalMissing));
}

#[test]
fn overlay_continuity_floors_are_exempt() {
    let packet = seeded();
    // The import overlay declares no journal and no preserved draft, but as a read-only
    // review it is exempt from the continuity floors and stays a review overlay.
    let decision = decide(surface(&packet, S_IMPORT));
    assert_eq!(decision.effective_claim, ContinuityClaim::ReviewOverlay);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn overlay_with_any_gap_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_IMPORT);
    // A non-floor gap on an overlay drops below the review overlay rather than holding
    // it.
    s.accessibility.reduced_motion.substitution_labeled = false;
    render_all(&mut s, ContinuityClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
}

// --------------------------------------------------------------------------- //
// Narrowing (non-floor) reasons.
// --------------------------------------------------------------------------- //

#[test]
fn step_position_unannounced_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.assistive_tech.step_position_announced = false;
    s.integrity.step_position_announced = false;
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::StepPositionUnannounced));
}

#[test]
fn focus_trap_escape_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.keyboard.focus_trap_escape_labeled = false;
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::FocusTrapEscapeUnlabeled));
}

#[test]
fn reduced_motion_substitution_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.reduced_motion.substitution_labeled = false;
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ReducedMotionSubstitutionUnlabeled));
}

#[test]
fn progress_marker_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.accessibility.reduced_motion.progress_marker_labeled = false;
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ProgressMarkerUnlabeled));
}

#[test]
fn absent_progress_marker_unlabeled_does_not_narrow() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    // With no progress marker in use, an unset marker label is exempt.
    s.accessibility.reduced_motion.progress_non_motion_marker = false;
    s.accessibility.reduced_motion.progress_marker_labeled = false;
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Certified);
}

#[test]
fn journal_stale_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.continuity.journal_state = JournalState::Stale;
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::JournalPartial));
}

#[test]
fn missing_proof_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.verification.proof_currency = ProofCurrency::MissingProof;
    s.verification.proof_ref = None;
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ContinuityProofMissing));
}

#[test]
fn requires_review_proof_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.verification.proof_currency = ProofCurrency::RequiresReview;
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ContinuityProofStale));
}

#[test]
fn stale_window_ages_current_proof() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    render_all(&mut s, ContinuityClaim::Narrowed);
    let decision = s.narrow(true);
    assert_eq!(decision.effective_claim, ContinuityClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::ContinuityProofStale));
}

// --------------------------------------------------------------------------- //
// Overclaim + structural validation.
// --------------------------------------------------------------------------- //

#[test]
fn rendering_overclaim_floors_via_validator() {
    let mut packet = seeded();
    // Leave the request surface narrowed but make a rendering claim certified.
    let request = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface_id == S_REQUEST)
        .unwrap();
    request.renderings[0].rendered_claim = ContinuityClaim::Certified;
    let decision = request.narrow(false);
    assert_eq!(decision.effective_claim, ContinuityClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ContinuityNarrowingReason::SurfaceOverclaims));
    assert!(!packet.validate().is_empty());
}

#[test]
fn duplicate_surface_id_is_rejected() {
    let mut packet = seeded();
    let dup = cloned(&packet, S_SETTINGS);
    packet.surfaces.push(dup);
    assert!(packet
        .validate()
        .contains(&M5AccessibilityContinuityViolation::DuplicateSurfaceId));
}

#[test]
fn overlay_without_provenance_ref_is_rejected() {
    let mut packet = seeded();
    let import = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface_id == S_IMPORT)
        .unwrap();
    import.lineage.provider_ref = None;
    import.lineage.source_artifact_ref = None;
    assert!(packet
        .validate()
        .contains(&M5AccessibilityContinuityViolation::OverlayMissingProvenanceRef));
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
    assert!(report.contains("continuity_certified"));
    assert!(report.contains("continuity_narrowed"));
    assert!(report.contains(S_REQUEST));
}

#[test]
fn narrowed_label_is_not_generic() {
    let packet = seeded();
    let s = surface(&packet, S_REQUEST);
    let decision = decide(s);
    let label = s.narrowed_label(&decision).expect("narrowed label");
    assert!(!label_is_generic(&label), "label was generic: {label}");
}

#[test]
fn claim_distribution_matches_seed() {
    let packet = seeded();
    let dist = packet.claim_distribution();
    assert_eq!(dist.certified, 4);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 1);
    assert_eq!(dist.unsafe_surfaces, 0);
    assert_eq!(dist.labs, 1);
}
