use super::*;

const F_PROVIDER: &str = "form:provider-connection:0001";
const F_SETTINGS: &str = "form:settings-config:0001";
const F_PROJECTS: &str = "wizard:project-bootstrap:0001";
const F_PACKAGE: &str = "sheet:package-install:0001";
const F_ADMIN: &str = "sheet:admin-policy:0001";
const F_REQUEST: &str = "dialog:request-run:0001";
const F_IMPORT: &str = "dialog:migration-restore:0001";
const F_LABS: &str = "wizard:labs-onboarding:0001";

fn seeded() -> M5DraftStateSetPacket {
    seeded_m5_draft_state_set()
}

fn rec<'a>(packet: &'a M5DraftStateSetPacket, id: &str) -> &'a DraftJournalRecord {
    packet
        .surfaces
        .iter()
        .find(|r| r.surface_id == id)
        .unwrap_or_else(|| panic!("missing surface {id}"))
}

fn cloned(packet: &M5DraftStateSetPacket, id: &str) -> DraftJournalRecord {
    rec(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers
/// the rendered claim to match — otherwise the surface itself overclaims and
/// floors.
fn render_all(r: &mut DraftJournalRecord, claim: DraftClaim) {
    r.renderings
        .iter_mut()
        .for_each(|x| x.rendered_claim = claim);
}

fn decide(r: &DraftJournalRecord) -> DraftDecision {
    r.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_DRAFT_STATE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_DRAFT_STATE_SCHEMA_VERSION);
    assert_eq!(packet.taxonomy_version, M5_DRAFT_STATE_TAXONOMY_VERSION);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.surfaces.len(), 8);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical =
        current_m5_draft_state_set().expect("canonical draft-state set loads and validates");
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
    for t in DraftPersistence::ALL {
        assert!(
            packet.represented_persistence_tiers().contains(&t),
            "missing persistence tier {t:?}"
        );
    }
    for a in RecoveryAvailability::ALL {
        assert!(
            packet.represented_recovery_availabilities().contains(&a),
            "missing recovery availability {a:?}"
        );
    }
    for k in InterruptionKind::ALL {
        assert!(
            packet.represented_interruption_kinds().contains(&k),
            "missing interruption kind {k:?}"
        );
    }
    for c in AutosaveClaimScope::ALL {
        assert!(
            packet.represented_autosave_claim_scopes().contains(&c),
            "missing autosave claim scope {c:?}"
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
fn clean_first_party_surfaces_certify() {
    let packet = seeded();
    for id in [F_PROVIDER, F_SETTINGS, F_PROJECTS, F_PACKAGE, F_ADMIN] {
        let decision = decide(rec(&packet, id));
        assert_eq!(
            decision.effective_claim,
            DraftClaim::Certified,
            "{id} should certify, got {decision:?}"
        );
        assert!(!decision.narrowed, "{id} should not be narrowed");
    }
}

#[test]
fn applied_local_settings_never_claims_remote() {
    // A surface applied to the local target with a local-only claim is certified;
    // it is not blocked, because committed_local is not a draft tier and the
    // indicator never claims a remote sync.
    let packet = seeded();
    let settings = rec(&packet, F_SETTINGS);
    assert_eq!(
        settings.journal.persistence_tier,
        DraftPersistence::CommittedLocal
    );
    assert_eq!(
        settings.journal.autosave_claim_scope,
        AutosaveClaimScope::ClaimsLocalOnly
    );
    assert_eq!(decide(settings).effective_claim, DraftClaim::Certified);
}

#[test]
fn remote_committed_surface_may_claim_remote() {
    let packet = seeded();
    let admin = rec(&packet, F_ADMIN);
    assert_eq!(
        admin.journal.persistence_tier,
        DraftPersistence::CommittedRemote
    );
    assert_eq!(
        admin.journal.autosave_claim_scope,
        AutosaveClaimScope::ClaimsRemoteSynced
    );
    assert_eq!(decide(admin).effective_claim, DraftClaim::Certified);
}

#[test]
fn in_flight_autosave_narrows_as_baseline() {
    let decision = decide(rec(&seeded(), F_REQUEST));
    assert_eq!(decision.effective_claim, DraftClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![DraftNarrowingReason::AutosavePending]
    );
}

#[test]
fn imported_restore_is_review_overlay() {
    let decision = decide(rec(&seeded(), F_IMPORT));
    assert_eq!(decision.effective_claim, DraftClaim::ReviewOverlay);
    assert!(!decision.narrowed);
}

#[test]
fn labs_surface_makes_no_claim() {
    let decision = decide(rec(&seeded(), F_LABS));
    assert_eq!(decision.effective_claim, DraftClaim::LabsNotClaimed);
    assert!(!decision.narrowed);
}

// --------------------------------------------------------------------------- //
// Floors (break the draft/autosave/recovery contract).
// --------------------------------------------------------------------------- //

#[test]
fn autosave_indicator_claiming_remote_for_local_draft_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.journal.autosave_claim_scope = AutosaveClaimScope::ClaimsRemoteSynced;
    render_all(&mut r, DraftClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, DraftClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![DraftNarrowingReason::AutosaveOverclaimsRemote]
    );
    assert!(r.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn ambiguous_draft_applied_state_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.draft_state.draft_distinct_from_applied = false;
    // Close the gate so this isolates the ambiguity, not the submit gate.
    r.submit_gate.submit_allowed = false;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::DraftAppliedAmbiguous]
    );
}

#[test]
fn local_draft_labelled_applied_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    // The provider draft lives in a local journal (a draft tier); labelling it
    // applied is a lie.
    r.draft_state.draft_applied_state = DraftAppliedState::Applied;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::LocalDraftReadsAsApplied]
    );
}

#[test]
fn recover_implying_remote_write_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.recovery.recover_implies_remote_write = true;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::RecoverImpliesRemoteWrite]
    );
}

#[test]
fn submit_from_ambiguous_state_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    // Submit stays open but the draft/applied state is not disambiguated first.
    r.submit_gate.draft_applied_disambiguated_before_submit = false;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::SubmitFromAmbiguousState]
    );
}

#[test]
fn recovery_deleting_unrelated_state_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.recovery.recover_preserves_unrelated_state = false;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::RecoveryDeletesUnrelatedState]
    );
}

#[test]
fn applied_without_named_target_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.draft_state.applied_target_named = false;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::AppliedTargetUnnamed]
    );
}

#[test]
fn lost_recover_action_with_journal_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    // A journal exists (recoverable) but the recover-draft action is gone.
    r.recovery.recover_action_present = false;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::RecoverActionLost]
    );
}

#[test]
fn unenumerable_affected_surfaces_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROVIDER);
    r.recovery.enumerates_affected_surfaces = false;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::AffectedSurfacesUnenumerable]
    );
}

#[test]
fn imported_review_that_submits_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_IMPORT);
    r.submit_gate.submit_allowed = true;
    render_all(&mut r, DraftClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, DraftClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![DraftNarrowingReason::ImportedDraftReadsAsApplied]
    );
}

#[test]
fn overclaiming_rendering_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_REQUEST);
    // The request composer narrows; a rendering that still shows certified floors.
    r.renderings[0].rendered_claim = DraftClaim::Certified;
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, DraftClaim::Blocked);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![
            DraftNarrowingReason::RenderingOverclaims,
            DraftNarrowingReason::AutosavePending
        ]
    );
}

#[test]
fn missing_journal_backing_floors() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::Missing;
    render_all(&mut r, DraftClaim::Blocked);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::JournalBackingMissing]
    );
}

#[test]
fn silent_blocked_fallback_loses_recovery() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::Missing;
    r.declared_blocked_fallback = BlockedSubmitFallback::NoneSilent;
    render_all(&mut r, DraftClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, DraftClaim::Blocked);
    assert!(!r.floored_keeps_fallback(decision.effective_claim));
}

// --------------------------------------------------------------------------- //
// Narrows (recoverable, stays usable).
// --------------------------------------------------------------------------- //

#[test]
fn unlabeled_autosave_indicator_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.journal.indicator_labeled = false;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::AutosaveStateUnlabeled]
    );
}

#[test]
fn unsaved_in_memory_edits_narrow() {
    let packet = seeded();
    let mut r = cloned(&packet, F_PROJECTS);
    r.draft_state.unsaved_change_count = 2;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::DraftUnsavedPending]
    );
}

#[test]
fn unlabeled_freshness_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.integrity.freshness_state_visible = false;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::FreshnessUnlabeled]
    );
}

#[test]
fn first_party_stale_narrows_but_overlay_holds() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::DraftStale]
    );
}

#[test]
fn superseded_unmarked_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    r.integrity.superseded_state_marked = false;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::SupersededStateNotMarked]
    );
}

#[test]
fn superseded_marked_holds_certified() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, DraftClaim::Certified);
    assert!(!decision.narrowed);
}

#[test]
fn missing_proof_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.verification.proof_currency = ProofCurrency::MissingProof;
    r.verification.proof_ref = None;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::VerificationProofMissing]
    );
}

#[test]
fn requires_review_proof_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.verification.proof_currency = ProofCurrency::RequiresReview;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn stale_window_ages_current_proof_out() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    render_all(&mut r, DraftClaim::Narrowed);
    let decision = r.narrow(true);
    assert_eq!(decision.effective_claim, DraftClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![DraftNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn reopen_path_lost_narrows() {
    let packet = seeded();
    let mut r = cloned(&packet, F_SETTINGS);
    r.integrity.reopen_visible_on_demand = false;
    render_all(&mut r, DraftClaim::Narrowed);
    assert_eq!(
        decide(&r).active_narrowing_reasons,
        vec![DraftNarrowingReason::ReopenPathLost]
    );
}

#[test]
fn overlay_with_any_gap_floors_below_overlay() {
    let packet = seeded();
    let mut r = cloned(&packet, F_IMPORT);
    r.integrity.freshness_state_visible = false;
    render_all(&mut r, DraftClaim::Blocked);
    let decision = decide(&r);
    assert_eq!(decision.effective_claim, DraftClaim::Blocked);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![DraftNarrowingReason::FreshnessUnlabeled]
    );
}

// --------------------------------------------------------------------------- //
// Validator wiring.
// --------------------------------------------------------------------------- //

#[test]
fn validator_flags_silent_floored_surface() {
    let mut packet = seeded();
    let r = packet
        .surfaces
        .iter_mut()
        .find(|r| r.surface_id == F_SETTINGS)
        .unwrap();
    r.declared_freshness_state = FreshnessState::Missing;
    r.declared_blocked_fallback = BlockedSubmitFallback::NoneSilent;
    render_all(r, DraftClaim::Blocked);
    let violations = packet.validate();
    assert!(violations.contains(&M5DraftStateViolation::FlooredSurfaceLosesFallback));
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
        .contains(&M5DraftStateViolation::OverlayMissingProvenanceRef));
}

#[test]
fn validator_flags_overclaiming_rendering() {
    let mut packet = seeded();
    let r = packet
        .surfaces
        .iter_mut()
        .find(|r| r.surface_id == F_REQUEST)
        .unwrap();
    r.renderings[0].rendered_claim = DraftClaim::Certified;
    assert!(packet
        .validate()
        .contains(&M5DraftStateViolation::RenderingSurfaceOverclaims));
}

#[test]
fn validator_flags_missing_persistence_tier() {
    let mut packet = seeded();
    // Collapse every surface to a single tier so the union is incomplete.
    for s in &mut packet.surfaces {
        s.journal.persistence_tier = DraftPersistence::LocalJournal;
    }
    assert!(packet
        .validate()
        .contains(&M5DraftStateViolation::PersistenceTierMissing));
}

#[test]
fn narrowed_label_is_specific() {
    let packet = seeded();
    let r = rec(&packet, F_REQUEST);
    let decision = decide(r);
    let label = r.narrowed_label(&decision).expect("narrowed label present");
    assert!(label.contains("draft_narrowed"));
    assert!(label.to_lowercase().contains("autosave write"));
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
