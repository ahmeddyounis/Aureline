use super::*;

const S_PROVIDER: &str = "form:provider-credentials:0001";
const S_SETTINGS: &str = "form:settings-config:0001";
const S_PROJECTS: &str = "wizard:project-bootstrap:0001";
const S_PACKAGE: &str = "sheet:package-install-review:0001";
const S_ADMIN: &str = "sheet:admin-policy-rollout:0001";
const S_REQUEST: &str = "dialog:request-workspace-run:0001";
const S_IMPORT: &str = "dialog:migration-restore-review:0001";
const S_LABS: &str = "wizard:experimental-onboarding:0001";

fn seeded() -> M5StructuredInputSetPacket {
    seeded_m5_structured_input_set()
}

fn surface<'a>(packet: &'a M5StructuredInputSetPacket, id: &str) -> &'a FormSurfaceRecord {
    packet
        .surfaces
        .iter()
        .find(|s| s.surface_id == id)
        .unwrap_or_else(|| panic!("missing surface {id}"))
}

fn cloned(packet: &M5StructuredInputSetPacket, id: &str) -> FormSurfaceRecord {
    surface(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers
/// the rendered claim to match — otherwise the surface itself overclaims and
/// floors.
fn render_all(s: &mut FormSurfaceRecord, claim: SurfaceClaim) {
    s.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = claim);
}

fn decide(s: &FormSurfaceRecord) -> SurfaceDecision {
    s.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_STRUCTURED_INPUT_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_STRUCTURED_INPUT_SCHEMA_VERSION);
    assert_eq!(
        packet.taxonomy_version,
        M5_STRUCTURED_INPUT_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.surfaces.len(), 8);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_structured_input_set()
        .expect("canonical structured-input set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seed_covers_every_taxonomy() {
    let packet = seeded();
    for k in FormSurfaceKind::ALL {
        assert!(
            packet.represented_kinds().contains(&k),
            "missing kind {k:?}"
        );
    }
    for l in FormLane::ALL {
        assert!(
            packet.represented_lanes().contains(&l),
            "missing lane {l:?}"
        );
    }
    for m in MutationBackingClass::ALL {
        assert!(
            packet.represented_mutation_classes().contains(&m),
            "missing mutation class {m:?}"
        );
    }
    for c in SourceOfValueClass::ALL {
        assert!(
            packet.represented_source_classes().contains(&c),
            "missing source class {c:?}"
        );
    }
    for cs in ConsumerSurface::ALL {
        assert!(
            packet.represented_consumer_surfaces().contains(&cs),
            "missing consumer surface {cs:?}"
        );
    }
}

// --------------------------------------------------------------------------- //
// Baseline claims.
// --------------------------------------------------------------------------- //

#[test]
fn clean_first_party_surfaces_certify() {
    let packet = seeded();
    for id in [S_PROVIDER, S_SETTINGS, S_PROJECTS, S_PACKAGE, S_ADMIN] {
        let decision = decide(surface(&packet, id));
        assert_eq!(
            decision.effective_claim,
            SurfaceClaim::Certified,
            "{id} should certify, got {decision:?}"
        );
        assert!(!decision.narrowed, "{id} should not be narrowed");
    }
}

#[test]
fn request_dialog_narrows_on_stale_proof() {
    let packet = seeded();
    let decision = decide(surface(&packet, S_REQUEST));
    assert_eq!(decision.effective_claim, SurfaceClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![SurfaceNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn import_review_is_overlay_not_apply() {
    let packet = seeded();
    let decision = decide(surface(&packet, S_IMPORT));
    assert_eq!(decision.effective_claim, SurfaceClaim::ReviewOverlay);
    assert!(!decision.narrowed);
}

#[test]
fn labs_surface_makes_no_claim() {
    let packet = seeded();
    let decision = decide(surface(&packet, S_LABS));
    assert_eq!(decision.effective_claim, SurfaceClaim::LabsNotClaimed);
    assert!(!decision.narrowed);
}

// --------------------------------------------------------------------------- //
// Floor reasons.
// --------------------------------------------------------------------------- //

#[test]
fn hidden_field_source_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROVIDER);
    s.fields[0].source_class_labeled = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::FieldSourceHidden));
    assert!(s.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn user_override_not_distinct_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    // font_size is the user override.
    s.fields[1].override_distinct_from_default = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::FieldSourceHidden));
}

#[test]
fn draft_applied_ambiguous_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.integrity.draft_applied_distinct = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::DraftAppliedAmbiguous));
}

#[test]
fn silent_policy_override_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROVIDER);
    // region is the policy-locked field.
    s.fields[1].policy_lock_respected = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::PolicyLockOverriddenSilently));
}

#[test]
fn invalid_blocking_without_blocker_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.fields[0].validation_state = ValidationState::InvalidBlocking;
    // No submit blocker present -> the form would submit over an invalid field.
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::SubmitAllowedWhileBlockingInvalid));
}

#[test]
fn blocked_prereq_hidden_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROJECTS);
    s.submit_blockers[0].explained_before_submit = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::BlockedPrereqHidden));
}

#[test]
fn target_scope_hidden_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PACKAGE);
    s.staged_review.target_scope_declared = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::TargetScopeHidden));
}

#[test]
fn omitted_defaults_hidden_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.staged_review.omitted_defaults_disclosed = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::OmittedDefaultsHidden));
}

#[test]
fn undisclosed_side_effect_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PACKAGE);
    s.staged_review.side_effects[0].disclosed_before_commit = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::SideEffectUndisclosed));
}

#[test]
fn hidden_rollback_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PACKAGE);
    s.staged_review.rollback_path_present = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::RollbackConsequencesHidden));
}

#[test]
fn generic_continue_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROJECTS);
    s.staged_review.commit_action_is_specific = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::GenericContinueAction));
}

#[test]
fn lost_draft_recovery_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.draft_recovery.recoverable_after_interruption = false;
    s.draft_recovery.recovery_behavior = InterruptionBehavior::NoRecovery;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::DraftRecoveryLost));
}

#[test]
fn imported_state_reads_as_applied_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_IMPORT);
    s.integrity.imported_review_read_only = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::ImportedStateReadsAsApplied));
}

#[test]
fn reopen_path_lost_keeps_keyboard_fallback() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_reopen_target = ReopenTarget::NoneKeyboardFallback;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::ReopenPathLost));
    assert!(s.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn surface_hides_provenance_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.renderings[0].provenance_visible = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::ReopenPathLost));
}

#[test]
fn overlay_with_any_gap_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_IMPORT);
    // A non-floor gap on an overlay drops below the review overlay rather than
    // holding it.
    s.session.autosave_enabled = false;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
}

#[test]
fn missing_backing_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_freshness_state = FreshnessState::Missing;
    render_all(&mut s, SurfaceClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::FormBackingMissing));
}

// --------------------------------------------------------------------------- //
// Narrowing (non-floor) reasons.
// --------------------------------------------------------------------------- //

#[test]
fn missing_autosave_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.session.autosave_enabled = false;
    render_all(&mut s, SurfaceClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![SurfaceNarrowingReason::AutosaveUnavailable]
    );
}

#[test]
fn validation_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.fields[0].validation_state_labeled = false;
    render_all(&mut s, SurfaceClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::ValidationStateUnlabeled));
}

#[test]
fn first_party_stale_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut s, SurfaceClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::SurfaceStale));
}

#[test]
fn superseded_unmarked_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    s.integrity.superseded_state_marked = false;
    render_all(&mut s, SurfaceClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::SupersededStateNotMarked));
}

#[test]
fn excluded_members_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PACKAGE);
    s.staged_review.members_classes_labeled = false;
    render_all(&mut s, SurfaceClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SurfaceClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::ExcludedMembersUnlabeled));
}

#[test]
fn stale_window_ages_current_proof() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    render_all(&mut s, SurfaceClaim::Narrowed);
    let decision = s.narrow(true);
    assert_eq!(decision.effective_claim, SurfaceClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::VerificationProofStale));
}

// --------------------------------------------------------------------------- //
// Overclaim + structural validation.
// --------------------------------------------------------------------------- //

#[test]
fn rendering_overclaim_floors_via_validator() {
    let mut packet = seeded();
    // Leave the request dialog narrowed but make a rendering claim certified.
    let request = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface_id == S_REQUEST)
        .unwrap();
    request.renderings[0].rendered_claim = SurfaceClaim::Certified;
    let decision = request.narrow(false);
    assert_eq!(decision.effective_claim, SurfaceClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SurfaceNarrowingReason::SurfaceOverclaims));
    assert!(!packet.validate().is_empty());
}

#[test]
fn duplicate_surface_id_is_rejected() {
    let mut packet = seeded();
    let dup = cloned(&packet, S_SETTINGS);
    packet.surfaces.push(dup);
    assert!(packet
        .validate()
        .contains(&M5StructuredInputViolation::DuplicateSurfaceId));
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
        .contains(&M5StructuredInputViolation::OverlayMissingProvenanceRef));
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
    assert!(report.contains("surface_certified"));
    assert!(report.contains("surface_narrowed"));
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
    assert_eq!(dist.certified, 5);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 1);
    assert_eq!(dist.unsafe_surfaces, 0);
    assert_eq!(dist.labs, 1);
}
