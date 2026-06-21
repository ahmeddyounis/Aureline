use super::*;

const E_STARTED: &str = "chronology:run-started-local-task:0001";
const E_PROGRESS: &str = "chronology:run-progress-local-test:0001";
const E_RETRIED: &str = "chronology:run-retried-local-task:0001";
const E_CANCELLED: &str = "chronology:run-cancelled-local-task:0001";
const E_FAILED: &str = "chronology:run-failed-local-test:0001";
const E_COMPLETED_NOTEBOOK: &str = "chronology:run-completed-notebook:0001";
const E_PIPELINE: &str = "chronology:run-failed-pipeline-provider:0001";
const E_PERF: &str = "chronology:run-completed-perf-local:0001";
const E_LABS: &str = "chronology:run-progress-labs:0001";

fn seeded() -> M5ChronologyReuseSetPacket {
    seeded_chronology_reuse_set()
}

fn entry<'a>(packet: &'a M5ChronologyReuseSetPacket, id: &str) -> &'a ChronologyEntry {
    packet
        .entries
        .iter()
        .find(|e| e.entry_id == id)
        .unwrap_or_else(|| panic!("missing entry {id}"))
}

fn cloned(packet: &M5ChronologyReuseSetPacket, id: &str) -> ChronologyEntry {
    entry(packet, id).clone()
}

/// A faithful consumer reuses the effective claim, so a narrowing test lowers the
/// rendered claim to match — otherwise the surface itself overclaims and floors.
fn render_all(e: &mut ChronologyEntry, claim: ChronologyClaim) {
    e.bindings.iter_mut().for_each(|b| b.rendered_claim = claim);
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_CHRONOLOGY_REUSE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_CHRONOLOGY_REUSE_SCHEMA_VERSION);
    assert_eq!(
        packet.taxonomy_version,
        M5_CHRONOLOGY_REUSE_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.entries.len(), 9);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical =
        current_m5_chronology_reuse_set().expect("canonical chronology set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seeded_covers_every_phase_and_surface() {
    let packet = seeded();
    for phase in ChronologyPhase::ALL {
        assert!(
            packet.represented_phases().contains(&phase),
            "missing phase {}",
            phase.as_str()
        );
    }
    for surface in ChronologySurface::ALL {
        assert!(
            packet.represented_surfaces().contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn claim_distribution_is_stable() {
    // Six first-party lifecycle events reuse cleanly; the perf verdict narrows via a
    // stale proof; the pipeline failure stays a read-only overlay; the Labs entry makes
    // no claim.
    let dist = seeded().claim_distribution();
    assert_eq!(dist.reused, 6);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 1);
    assert_eq!(dist.unreconstructable, 0);
    assert_eq!(dist.labs, 1);
    assert_eq!(seeded().narrowed_entry_count(), 1);
}

#[test]
fn export_safe_json_round_trips() {
    let packet = seeded();
    let json = packet.export_safe_json();
    let reparsed: M5ChronologyReuseSetPacket = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(reparsed, packet);
    assert!(reparsed.validate().is_empty());
}

#[test]
fn export_carries_no_forbidden_material() {
    let value = serde_json::to_value(seeded()).expect("serializes");
    assert!(!json_contains_forbidden_boundary_material(&value));
}

#[test]
fn markdown_summary_lists_entries_and_counts() {
    let summary = seeded().render_markdown_summary();
    assert!(summary.contains("# M5 Task/Problem/Output Chronology Reuse"));
    assert!(summary.contains("6 reused, 1 narrowed, 1 read-only overlay"));
    assert!(summary.contains(E_PERF));
}

// --------------------------------------------------------------------------- //
// The cross-surface failure points at one canonical id set.
// --------------------------------------------------------------------------- //

#[test]
fn failure_reuses_one_canonical_id_set_across_surfaces() {
    let packet = seeded();
    let failed = entry(&packet, E_FAILED);
    // The failure is reused across the activity center, support bundle, and AI evidence
    // (plus history/issue), and every binding points at one run/channel/problem id.
    for surface in [
        ChronologySurface::ActivityCenter,
        ChronologySurface::SupportBundle,
        ChronologySurface::AiEvidencePacket,
    ] {
        let b = failed
            .bindings
            .iter()
            .find(|b| b.surface == surface)
            .unwrap_or_else(|| panic!("missing binding {}", surface.as_str()));
        assert_eq!(b.bound_run_ref, failed.links.run_ref);
        assert_eq!(b.bound_channel_ref, failed.links.channel_ref);
        assert_eq!(b.bound_problem_ref, failed.links.problem_ref);
    }
    assert_eq!(
        failed.narrow(false).effective_chronology_claim,
        ChronologyClaim::Reused
    );
}

#[test]
fn diverging_canonical_id_on_a_surface_floors() {
    let mut e = cloned(&seeded(), E_FAILED);
    // A support bundle that points at a different run id breaks the single-id contract.
    let support = e
        .bindings
        .iter_mut()
        .find(|b| b.surface == ChronologySurface::SupportBundle)
        .unwrap();
    support.bound_run_ref = Some("run.some.other.0009".to_owned());
    render_all(&mut e, ChronologyClaim::Unreconstructable);
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::CanonicalIdDivergence));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
}

// --------------------------------------------------------------------------- //
// Per-entry derivation (mirrors the perturbation corpus).
// --------------------------------------------------------------------------- //

#[test]
fn clean_started_entry_reuses() {
    let decision = entry(&seeded(), E_STARTED).narrow(false);
    assert_eq!(decision.effective_chronology_claim, ChronologyClaim::Reused);
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn flattening_grammar_floors() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.integrity.preserves_actor_action_object_outcome = false;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::GrammarFlattened));
    assert!(e.floored_keeps_fallback(decision.effective_chronology_claim));
}

#[test]
fn flattening_provider_adapter_floors() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.integrity.preserves_provider_adapter = false;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::ProviderAdapterFlattened));
}

#[test]
fn flattening_target_scope_floors() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.integrity.preserves_target_scope = false;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::TargetScopeFlattened));
}

#[test]
fn flattening_retry_lineage_floors() {
    let mut e = cloned(&seeded(), E_RETRIED);
    e.integrity.preserves_retry_lineage = false;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::RetryLineageFlattened));
}

#[test]
fn flattening_canonical_ids_floors() {
    let mut e = cloned(&seeded(), E_FAILED);
    e.integrity.preserves_canonical_ids = false;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::CanonicalIdFlattened));
}

#[test]
fn lineage_not_visible_on_a_surface_floors() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.bindings[0].lineage_visible = false;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::LineageNotVisible));
}

#[test]
fn heuristic_without_backlink_floors() {
    let mut e = cloned(&seeded(), E_PERF);
    assert!(e.declared_confidence_tier.is_heuristic_tier());
    e.integrity.raw_output_backlink_present = false;
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::RawBacklinkMissing));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    // The links raw-output backlink keeps the floored entry reopenable.
    assert!(e.floored_keeps_fallback(decision.effective_chronology_claim));
}

#[test]
fn reopen_target_lost_floors() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.declared_reopen_target = ReopenTarget::NoneKeyboardFallback;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::ReopenTargetLost));
    assert!(e.floored_keeps_fallback(decision.effective_chronology_claim));
}

#[test]
fn export_not_self_contained_floors_an_exported_entry() {
    let mut e = cloned(&seeded(), E_FAILED);
    // The failure is reused in an issue packet / support bundle / AI evidence, so a
    // non-self-contained export floors it.
    e.integrity.export_self_contained = false;
    render_all(&mut e, ChronologyClaim::Unreconstructable);
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::ExportNotSelfContained));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
}

#[test]
fn export_not_self_contained_is_inert_without_an_export_surface() {
    // The started entry is reused only in live UI surfaces (activity center / history),
    // so the export flag does not gate it.
    let mut e = cloned(&seeded(), E_STARTED);
    e.integrity.export_self_contained = false;
    let decision = e.narrow(false);
    assert_eq!(decision.effective_chronology_claim, ChronologyClaim::Reused);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn surface_overclaim_floors_and_is_caught_by_validate() {
    let mut e = cloned(&seeded(), E_PERF);
    // The perf entry effectively narrows; a surface that renders reused overclaims.
    e.bindings[0].rendered_claim = ChronologyClaim::Reused;
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::SurfaceOverclaims));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );

    let mut packet = seeded();
    let idx = packet
        .entries
        .iter()
        .position(|x| x.entry_id == E_PERF)
        .unwrap();
    packet.entries[idx] = e;
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::BindingSurfaceOverclaims));
}

#[test]
fn imported_chronology_claiming_live_floors() {
    let mut e = cloned(&seeded(), E_PIPELINE);
    assert!(e.is_overlay_origin());
    e.integrity.imported_chronology_read_only = false;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::ImportedChronologyClaimsLive));
}

#[test]
fn overlay_with_any_other_gap_floors_below_overlay() {
    // An overlay is already the minimal honest claim; a non-floor gap still drops it to
    // unreconstructable rather than holding a clean read-only overlay.
    let mut e = cloned(&seeded(), E_PIPELINE);
    e.integrity.freshness_state_labeled = false;
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::FreshnessUnlabeled));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
}

#[test]
fn missing_evidence_floors() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.declared_freshness_state = FreshnessState::Missing;
    let decision = e.narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::EvidenceMissing));
}

#[test]
fn freshness_unlabeled_narrows() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.integrity.freshness_state_labeled = false;
    render_all(&mut e, ChronologyClaim::Narrowed);
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::FreshnessUnlabeled));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Narrowed
    );
}

#[test]
fn confidence_unlabeled_narrows() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.integrity.confidence_label_visible = false;
    render_all(&mut e, ChronologyClaim::Narrowed);
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::ConfidenceUnlabeled));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Narrowed
    );
}

#[test]
fn superseded_unmarked_narrows() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.declared_freshness_state = FreshnessState::SupersededByNewerRun;
    e.integrity.superseded_state_marked = false;
    render_all(&mut e, ChronologyClaim::Narrowed);
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::SupersededNotMarked));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Narrowed
    );
}

#[test]
fn superseded_marked_stays_reused() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.declared_freshness_state = FreshnessState::SupersededByNewerRun;
    e.integrity.superseded_state_marked = true;
    let decision = e.narrow(false);
    assert!(decision.active_narrowing_reasons.is_empty());
    assert_eq!(decision.effective_chronology_claim, ChronologyClaim::Reused);
}

#[test]
fn first_party_stale_evidence_narrows() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut e, ChronologyClaim::Narrowed);
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::StaleEvidence));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Narrowed
    );
}

#[test]
fn overlay_cached_snapshot_stays_overlay() {
    // A read-only overlay showing a cached snapshot is expected, not narrowed.
    let decision = entry(&seeded(), E_PIPELINE).narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::ReadOnlyOverlay
    );
    assert!(!decision.narrowed);
}

#[test]
fn missing_proof_narrows_first_party() {
    let mut e = cloned(&seeded(), E_STARTED);
    e.verification.proof_currency = ProofCurrency::MissingProof;
    e.verification.proof_ref = None;
    render_all(&mut e, ChronologyClaim::Narrowed);
    let decision = e.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::MissingProof));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Narrowed
    );
}

#[test]
fn stale_window_ages_out_current_proof() {
    let mut e = cloned(&seeded(), E_STARTED);
    render_all(&mut e, ChronologyClaim::Narrowed);
    let decision = e.narrow(true);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChronologyNarrowingReason::StaleProof));
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Narrowed
    );
}

#[test]
fn labs_entry_makes_no_claim_and_never_widens() {
    let decision = entry(&seeded(), E_LABS).narrow(false);
    assert_eq!(
        decision.claimed_chronology_claim,
        ChronologyClaim::LabsNotClaimed
    );
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::LabsNotClaimed
    );
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn perf_entry_narrows_via_stale_proof() {
    let decision = entry(&seeded(), E_PERF).narrow(false);
    assert_eq!(
        decision.effective_chronology_claim,
        ChronologyClaim::Narrowed
    );
    assert!(decision.narrowed);
    assert_eq!(
        decision.downgrade_trigger(),
        Some(ChronologyNarrowingReason::StaleProof)
    );
}

#[test]
fn cancelled_and_completed_entries_reuse() {
    let packet = seeded();
    assert_eq!(
        entry(&packet, E_CANCELLED)
            .narrow(false)
            .effective_chronology_claim,
        ChronologyClaim::Reused
    );
    assert_eq!(
        entry(&packet, E_COMPLETED_NOTEBOOK)
            .narrow(false)
            .effective_chronology_claim,
        ChronologyClaim::Reused
    );
    assert_eq!(
        entry(&packet, E_PROGRESS)
            .narrow(false)
            .effective_chronology_claim,
        ChronologyClaim::Reused
    );
}

// --------------------------------------------------------------------------- //
// Structural validation failures.
// --------------------------------------------------------------------------- //

#[test]
fn phase_outcome_mismatch_is_flagged() {
    let mut packet = seeded();
    let idx = packet
        .entries
        .iter()
        .position(|x| x.entry_id == E_FAILED)
        .unwrap();
    // A failed run that records a succeeded outcome is an internal contradiction.
    packet.entries[idx].grammar.outcome = ChronologyOutcome::Succeeded;
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::PhaseOutcomeMismatch));
}

#[test]
fn retry_entry_without_lineage_is_flagged() {
    let mut packet = seeded();
    let idx = packet
        .entries
        .iter()
        .position(|x| x.entry_id == E_RETRIED)
        .unwrap();
    packet.entries[idx].retry_lineage = RetryLineage {
        attempt_index: 1,
        retry_of_run_ref: None,
        previous_attempt_ref: None,
    };
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::RetryEntryMissingLineage));
}

#[test]
fn overlay_without_provider_ref_is_flagged() {
    let mut packet = seeded();
    let idx = packet
        .entries
        .iter()
        .position(|x| x.entry_id == E_PIPELINE)
        .unwrap();
    packet.entries[idx].links.provider_ref = None;
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::OverlayMissingProviderRef));
}

#[test]
fn duplicate_entry_id_is_flagged() {
    let mut packet = seeded();
    let dup = packet.entries[0].clone();
    packet.entries.push(dup);
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::DuplicateEntryId));
}

#[test]
fn missing_phase_is_flagged() {
    let mut packet = seeded();
    packet
        .entries
        .retain(|e| e.phase() != ChronologyPhase::RunCancelled);
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::ChronologyPhaseMissing));
}

#[test]
fn missing_narrowed_entry_trips_the_demo_guard() {
    let mut packet = seeded();
    // Removing the only narrowed entry trips the narrowing-demo guard.
    packet.entries.retain(|e| e.entry_id != E_PERF);
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::DowngradedEntryCaseMissing));
}

#[test]
fn invalid_redaction_class_is_flagged() {
    let mut packet = seeded();
    packet.redaction_class_token = "everything".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::InvalidRedactionClass));
}

#[test]
fn entry_without_binding_is_flagged() {
    let mut packet = seeded();
    packet.entries[0].bindings.clear();
    assert!(packet
        .validate()
        .contains(&M5ChronologyReuseViolation::EntryMissingBinding));
}

#[test]
fn overclaim_detection_respects_rank() {
    assert!(ChronologyClaim::Narrowed.overclaims_as(ChronologyClaim::Reused));
    assert!(!ChronologyClaim::Reused.overclaims_as(ChronologyClaim::Narrowed));
    assert!(ChronologyClaim::ReadOnlyOverlay.overclaims_as(ChronologyClaim::Narrowed));
    assert!(!ChronologyClaim::LabsNotClaimed.overclaims_as(ChronologyClaim::LabsNotClaimed));
    assert!(ChronologyClaim::LabsNotClaimed.overclaims_as(ChronologyClaim::Reused));
}
