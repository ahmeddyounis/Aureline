use super::*;

const S_PROVIDER: &str = "sheet:provider-publish-later:0001";
const S_SETTINGS: &str = "sheet:settings-bulk-apply:0001";
const S_PACKAGE: &str = "sheet:package-lifecycle:0001";
const S_ADMIN: &str = "sheet:admin-source-management:0001";
const S_REQUEST: &str = "sheet:request-replay:0001";
const S_IMPORT: &str = "sheet:import-export-publish:0001";
const S_LABS: &str = "sheet:experimental-quick-apply:0001";

fn seeded() -> M5StagedReviewSheetSetPacket {
    seeded_m5_staged_review_sheet_set()
}

fn sheet<'a>(packet: &'a M5StagedReviewSheetSetPacket, id: &str) -> &'a ReviewSheetRecord {
    packet
        .sheets
        .iter()
        .find(|s| s.sheet_id == id)
        .unwrap_or_else(|| panic!("missing sheet {id}"))
}

fn cloned(packet: &M5StagedReviewSheetSetPacket, id: &str) -> ReviewSheetRecord {
    sheet(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers the
/// rendered claim to match — otherwise the sheet itself overclaims and floors.
fn render_all(s: &mut ReviewSheetRecord, claim: SheetClaim) {
    s.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = claim);
}

fn decide(s: &ReviewSheetRecord) -> SheetDecision {
    s.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_STAGED_REVIEW_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_STAGED_REVIEW_SCHEMA_VERSION);
    assert_eq!(packet.taxonomy_version, M5_STAGED_REVIEW_TAXONOMY_VERSION);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.sheets.len(), 7);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_staged_review_sheet_set()
        .expect("canonical staged-review set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seed_covers_every_taxonomy() {
    let packet = seeded();
    for f in MutationFlow::ALL {
        assert!(
            packet.represented_flows().contains(&f),
            "missing flow {f:?}"
        );
    }
    for l in FlowLane::ALL {
        assert!(
            packet.represented_lanes().contains(&l),
            "missing lane {l:?}"
        );
    }
    for k in ScopeKind::ALL {
        assert!(
            packet.represented_scope_kinds().contains(&k),
            "missing scope kind {k:?}"
        );
    }
    for m in ReviewMemberClass::ALL {
        assert!(
            packet.represented_member_classes().contains(&m),
            "missing member class {m:?}"
        );
    }
    for e in SideEffectClass::ALL {
        assert!(
            packet.represented_side_effect_classes().contains(&e),
            "missing side-effect class {e:?}"
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
fn every_certified_sheet_reconciles_counts() {
    let packet = seeded();
    for s in &packet.sheets {
        assert!(
            s.sheet.counts.reconciles(),
            "{} counts do not reconcile",
            s.sheet_id
        );
    }
}

// --------------------------------------------------------------------------- //
// Baseline claims.
// --------------------------------------------------------------------------- //

#[test]
fn clean_first_party_sheets_certify() {
    let packet = seeded();
    for id in [S_PROVIDER, S_SETTINGS, S_PACKAGE, S_ADMIN] {
        let decision = decide(sheet(&packet, id));
        assert_eq!(
            decision.effective_claim,
            SheetClaim::Certified,
            "{id} should certify, got {decision:?}"
        );
        assert!(!decision.narrowed, "{id} should not be narrowed");
    }
}

#[test]
fn request_sheet_narrows_on_proof_requires_review() {
    let packet = seeded();
    let decision = decide(sheet(&packet, S_REQUEST));
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![SheetNarrowingReason::VerificationProofStale]
    );
}

#[test]
fn import_sheet_is_overlay_not_apply() {
    let packet = seeded();
    let decision = decide(sheet(&packet, S_IMPORT));
    assert_eq!(decision.effective_claim, SheetClaim::ReviewOverlay);
    assert!(!decision.narrowed);
}

#[test]
fn labs_sheet_makes_no_claim() {
    let packet = seeded();
    let decision = decide(sheet(&packet, S_LABS));
    assert_eq!(decision.effective_claim, SheetClaim::LabsNotClaimed);
    assert!(!decision.narrowed);
}

// --------------------------------------------------------------------------- //
// Floor reasons.
// --------------------------------------------------------------------------- //

#[test]
fn target_scope_hidden_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.scope.scope_declared = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::TargetScopeHidden));
    assert!(s.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn member_counts_inconsistent_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.counts.total_matched = 99;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![SheetNarrowingReason::MemberCountsInconsistent]
    );
}

#[test]
fn hidden_members_uncounted_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_ADMIN);
    // A workspace-wide action with a collapsed member but zero hidden count: the
    // unshown scope is invisible. Keep the counts reconciled so only the
    // hidden-uncounted rule fires.
    s.sheet.counts.hidden = 0;
    s.sheet.counts.total_matched = 12;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![SheetNarrowingReason::HiddenMembersUncounted]
    );
}

#[test]
fn included_excluded_blocked_counts_hidden_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.counts.counts_visible = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::IncludedExcludedBlockedCountsHidden));
}

#[test]
fn single_object_counts_hidden_does_not_floor() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PROVIDER);
    // A single-object sheet does not need a counts breakdown, so hiding it is no
    // floor.
    s.sheet.counts.counts_visible = false;
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Certified);
}

#[test]
fn omitted_defaults_hidden_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.omitted_defaults_disclosed = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::OmittedDefaultsHidden));
}

#[test]
fn undisclosed_side_effect_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PACKAGE);
    s.sheet.side_effects[0].disclosed_before_commit = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::SideEffectUndisclosed));
}

#[test]
fn blocked_prereq_hidden_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PACKAGE);
    // The blocked package's reason is no longer labelled.
    s.sheet.members[3].reason_labeled = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::BlockedPrereqHidden));
}

#[test]
fn hidden_rollback_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.recoverability.rollback_path_present = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::RollbackConsequencesHidden));
}

#[test]
fn irreversible_without_export_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_REQUEST);
    // An irreversible re-issue with no export path hides the recovery consequence.
    s.sheet.recoverability.export_path_present = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::RollbackConsequencesHidden));
}

#[test]
fn generic_continue_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_PACKAGE);
    s.sheet.commit.commit_action_is_specific = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::GenericContinueAction));
}

#[test]
fn imported_review_reads_as_apply_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_IMPORT);
    s.integrity.imported_review_read_only = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::ImportedReviewReadsAsApply));
}

#[test]
fn reopen_path_lost_keeps_keyboard_fallback() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_reopen_target = ReopenTarget::NoneKeyboardFallback;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::ReopenPathLost));
    assert!(s.floored_keeps_fallback(decision.effective_claim));
}

#[test]
fn missing_backing_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_freshness_state = FreshnessState::Missing;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::SheetBackingMissing));
}

#[test]
fn overlay_with_any_gap_floors() {
    let packet = seeded();
    let mut s = cloned(&packet, S_IMPORT);
    // A non-floor gap on an overlay drops below the review overlay rather than
    // holding it.
    s.sheet.recoverability.recoverability_class_labeled = false;
    render_all(&mut s, SheetClaim::Unsafe);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
}

// --------------------------------------------------------------------------- //
// Narrowing (non-floor) reasons.
// --------------------------------------------------------------------------- //

#[test]
fn member_classes_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.members_classes_labeled = false;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::MemberClassesUnlabeled));
}

#[test]
fn side_effect_summary_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.side_effect_summary_labeled = false;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::SideEffectSummaryUnlabeled));
}

#[test]
fn cancel_action_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.commit.cancel_action_is_specific = false;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::CancelActionUnlabeled));
}

#[test]
fn recoverability_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.sheet.recoverability.recoverability_class_labeled = false;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::RecoverabilityClassUnlabeled));
}

#[test]
fn freshness_unlabeled_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.integrity.freshness_state_visible = false;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::FreshnessUnlabeled));
}

#[test]
fn superseded_unmarked_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_freshness_state = FreshnessState::SupersededByNewerSource;
    s.integrity.superseded_state_marked = false;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::SupersededScopeNotMarked));
}

#[test]
fn first_party_stale_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::ScopeStale));
}

#[test]
fn missing_proof_narrows() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    s.verification.proof_currency = ProofCurrency::MissingProof;
    s.verification.proof_ref = None;
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = decide(&s);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::VerificationProofMissing));
}

#[test]
fn stale_window_ages_current_proof() {
    let packet = seeded();
    let mut s = cloned(&packet, S_SETTINGS);
    render_all(&mut s, SheetClaim::Narrowed);
    let decision = s.narrow(true);
    assert_eq!(decision.effective_claim, SheetClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::VerificationProofStale));
}

// --------------------------------------------------------------------------- //
// Overclaim + structural validation.
// --------------------------------------------------------------------------- //

#[test]
fn rendering_overclaim_floors_via_validator() {
    let mut packet = seeded();
    // Leave the request sheet narrowed but make a rendering claim certified.
    let request = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == S_REQUEST)
        .unwrap();
    request.renderings[0].rendered_claim = SheetClaim::Certified;
    let decision = request.narrow(false);
    assert_eq!(decision.effective_claim, SheetClaim::Unsafe);
    assert!(decision
        .active_narrowing_reasons
        .contains(&SheetNarrowingReason::SheetOverclaims));
    assert!(!packet.validate().is_empty());
}

#[test]
fn duplicate_sheet_id_is_rejected() {
    let mut packet = seeded();
    let dup = cloned(&packet, S_SETTINGS);
    packet.sheets.push(dup);
    assert!(packet
        .validate()
        .contains(&M5StagedReviewViolation::DuplicateSheetId));
}

#[test]
fn overlay_without_provenance_ref_is_rejected() {
    let mut packet = seeded();
    let import = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == S_IMPORT)
        .unwrap();
    import.lineage.provider_ref = None;
    import.lineage.source_artifact_ref = None;
    assert!(packet
        .validate()
        .contains(&M5StagedReviewViolation::OverlayMissingProvenanceRef));
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
    assert!(report.contains("sheet_certified"));
    assert!(report.contains("sheet_narrowed"));
    assert!(report.contains(S_REQUEST));
}

#[test]
fn narrowed_label_is_not_generic() {
    let packet = seeded();
    let s = sheet(&packet, S_REQUEST);
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
    assert_eq!(dist.unsafe_sheets, 0);
    assert_eq!(dist.labs, 1);
}
