use super::*;

const C_NATIVE: &str = "fallback:native-structured-language-problems:0001";
const C_NORMALIZED: &str = "fallback:normalized-task-event-terminal:0001";
const C_HEUR: &str = "fallback:heuristic-parse-terminal:0001";
const C_MALFORMED: &str = "fallback:malformed-output-heuristic:0001";
const C_IMPORTED: &str = "fallback:imported-provider-annotation:0001";
const C_RECONNECT: &str = "fallback:pipeline-reconnect:0001";
const C_NOTEBOOK_STALE: &str = "fallback:notebook-heuristic-stale:0001";
const C_SUPERSEDED: &str = "fallback:superseded-retry-marked:0001";
const C_VIRT: &str = "fallback:channel-virtualization-large-log:0001";
const C_PARTIAL: &str = "fallback:partial-export-support-bundle:0001";
const C_PERF: &str = "fallback:heuristic-stale-proof:0001";
const C_LOSTCH: &str = "fallback:lost-channel-remote:0001";
const C_LABS: &str = "fallback:labs-heuristic-notebook:0001";

fn seeded() -> M5FallbackEvidenceDrillSetPacket {
    seeded_fallback_evidence_drill_set()
}

fn case<'a>(packet: &'a M5FallbackEvidenceDrillSetPacket, id: &str) -> &'a FallbackDrillCase {
    packet
        .cases
        .iter()
        .find(|c| c.case_id == id)
        .unwrap_or_else(|| panic!("missing case {id}"))
}

fn cloned(packet: &M5FallbackEvidenceDrillSetPacket, id: &str) -> FallbackDrillCase {
    case(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers the
/// rendered claim to match — otherwise the profile itself overclaims and floors.
fn render_all(c: &mut FallbackDrillCase, claim: FallbackClaim) {
    c.profiles.iter_mut().for_each(|p| p.rendered_claim = claim);
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_FALLBACK_EVIDENCE_DRILL_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_VERSION
    );
    assert_eq!(
        packet.taxonomy_version,
        M5_FALLBACK_EVIDENCE_DRILL_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.cases.len(), 14);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_fallback_evidence_drill_set()
        .expect("canonical fallback-evidence drill set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seeded_covers_every_source_drill_and_profile() {
    let packet = seeded();
    for source in [
        ProblemSourceKind::StructuredLanguageDiagnostic,
        ProblemSourceKind::NormalizedTaskEvent,
        ProblemSourceKind::HeuristicOutputParse,
        ProblemSourceKind::ImportedProviderAnnotation,
    ] {
        assert!(
            packet.represented_source_kinds().contains(&source),
            "missing source {}",
            source.as_str()
        );
    }
    for drill in FallbackDrillKind::ALL {
        assert!(
            packet.represented_drills().contains(&drill),
            "missing drill {}",
            drill.as_str()
        );
    }
    for profile in ToolingProfile::ALL {
        assert!(
            packet.represented_profiles().contains(&profile),
            "missing profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn claim_distribution_is_stable() {
    // Eight first-party cases certify; the notebook-stale and stale-proof heuristics
    // narrow; the imported/pipeline/remote overlays stay read-only; the Labs case makes
    // no claim.
    let dist = seeded().claim_distribution();
    assert_eq!(dist.certified, 8);
    assert_eq!(dist.narrowed, 2);
    assert_eq!(dist.overlay, 3);
    assert_eq!(dist.unreconstructable, 0);
    assert_eq!(dist.labs, 1);
    assert_eq!(seeded().narrowed_case_count(), 2);
}

#[test]
fn export_safe_json_round_trips() {
    let packet = seeded();
    let json = packet.export_safe_json();
    let reparsed: M5FallbackEvidenceDrillSetPacket =
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
fn markdown_summary_lists_cases_and_counts() {
    let summary = seeded().render_markdown_summary();
    assert!(summary.contains("# M5 Structured-vs-Heuristic Fallback Evidence Drills"));
    assert!(summary.contains("8 certified, 2 narrowed, 3 read-only overlay"));
    assert!(summary.contains(C_PERF));
}

// --------------------------------------------------------------------------- //
// One canonical id across the claimed profiles.
// --------------------------------------------------------------------------- //

#[test]
fn imported_case_reuses_one_canonical_id_set_across_profiles() {
    let packet = seeded();
    let imported = case(&packet, C_IMPORTED);
    for profile in [
        ToolingProfile::PipelineOverlay,
        ToolingProfile::SupportExport,
        ToolingProfile::AiToolEvidence,
    ] {
        let p = imported
            .profiles
            .iter()
            .find(|p| p.profile == profile)
            .unwrap_or_else(|| panic!("missing profile {}", profile.as_str()));
        assert_eq!(p.bound_run_ref, imported.links.run_ref);
        assert_eq!(p.bound_channel_ref, imported.links.channel_ref);
        assert_eq!(p.bound_problem_ref, imported.links.problem_ref);
    }
    assert_eq!(
        imported.narrow(false).effective_fallback_claim,
        FallbackClaim::ReadOnlyOverlay
    );
}

#[test]
fn diverging_canonical_id_on_a_profile_floors() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.profiles[1].bound_run_ref = Some("run.some.other.0009".to_owned());
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::CanonicalIdDivergence));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
}

// --------------------------------------------------------------------------- //
// Per-case derivation (mirrors the perturbation corpus).
// --------------------------------------------------------------------------- //

#[test]
fn clean_native_case_certifies() {
    let decision = case(&seeded(), C_NATIVE).narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Certified);
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn clean_heuristic_case_certifies_visibly_distinct() {
    let packet = seeded();
    let c = case(&packet, C_HEUR);
    assert!(c.is_heuristic());
    let decision = c.narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Certified);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn flattening_source_kind_floors() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.integrity.preserves_source_kind = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::SourceKindFlattened));
    assert!(c.floored_keeps_fallback(decision.effective_fallback_claim));
}

#[test]
fn heuristic_indistinct_from_structured_floors() {
    let mut c = cloned(&seeded(), C_HEUR);
    c.integrity.heuristic_visibly_distinct_from_structured = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::HeuristicIndistinct));
}

#[test]
fn a_single_profile_hiding_the_heuristic_distinction_floors() {
    let mut c = cloned(&seeded(), C_HEUR);
    c.profiles[0].fallback_visibly_distinct = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::HeuristicIndistinct));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
}

#[test]
fn structured_case_is_immune_to_the_distinctness_gate() {
    // A non-heuristic case never accrues the heuristic-distinctness reason even if the
    // flag is false, because the distinction is irrelevant for structured evidence.
    let mut c = cloned(&seeded(), C_NATIVE);
    assert!(!c.is_heuristic());
    c.integrity.heuristic_visibly_distinct_from_structured = false;
    let decision = c.narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Certified);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn flattening_lineage_floors() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.integrity.preserves_run_channel_lineage = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::LineageFlattened));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
}

#[test]
fn a_profile_hiding_lineage_floors() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.profiles[0].lineage_visible = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::LineageFlattened));
}

#[test]
fn flattening_channel_identity_floors() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.integrity.channel_identity_stable = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::ChannelIdentityFlattened));
}

#[test]
fn heuristic_without_backlink_floors_and_keeps_fallback() {
    let mut c = cloned(&seeded(), C_HEUR);
    assert!(c.is_heuristic());
    c.integrity.raw_output_backlink_present = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::RawBacklinkMissing));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
    // The links raw-output backlink keeps the floored case reopenable.
    assert!(c.floored_keeps_fallback(decision.effective_fallback_claim));
}

#[test]
fn reopen_target_lost_floors() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.declared_reopen_target = ReopenTarget::NoneKeyboardFallback;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::ReopenTargetLost));
    assert!(c.floored_keeps_fallback(decision.effective_fallback_claim));
}

#[test]
fn reconnect_dropping_evidence_floors() {
    let mut c = cloned(&seeded(), C_RECONNECT);
    c.integrity.reconnect_preserves_evidence = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::ReconnectDropsEvidence));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
}

#[test]
fn lost_channel_dropping_evidence_floors() {
    let mut c = cloned(&seeded(), C_LOSTCH);
    c.integrity.reconnect_preserves_evidence = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::ReconnectDropsEvidence));
}

#[test]
fn partial_export_not_self_contained_floors() {
    let mut c = cloned(&seeded(), C_PARTIAL);
    c.integrity.partial_export_self_contained = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::PartialExportIncomplete));
}

#[test]
fn surface_overclaim_floors_and_is_caught_by_validate() {
    let mut c = cloned(&seeded(), C_PERF);
    // The stale-proof heuristic effectively narrows; a profile that renders certified
    // overclaims.
    c.profiles[0].rendered_claim = FallbackClaim::Certified;
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::SurfaceOverclaims));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );

    let mut packet = seeded();
    let idx = packet
        .cases
        .iter()
        .position(|x| x.case_id == C_PERF)
        .unwrap();
    packet.cases[idx] = c;
    assert!(packet
        .validate()
        .contains(&M5FallbackEvidenceDrillViolation::ProfileOverclaims));
}

#[test]
fn imported_overlay_claiming_live_floors() {
    let mut c = cloned(&seeded(), C_IMPORTED);
    assert!(c.is_overlay_origin());
    c.integrity.imported_evidence_read_only = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::ImportedOverlayClaimsLive));
}

#[test]
fn overlay_with_any_other_gap_floors_below_overlay() {
    let mut c = cloned(&seeded(), C_IMPORTED);
    c.integrity.freshness_state_labeled = false;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::FreshnessUnlabeled));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
}

#[test]
fn clean_overlay_stays_read_only_overlay() {
    let decision = case(&seeded(), C_IMPORTED).narrow(false);
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::ReadOnlyOverlay
    );
    assert!(!decision.narrowed);
}

#[test]
fn missing_evidence_floors() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.declared_freshness_state = FreshnessState::Missing;
    render_all(&mut c, FallbackClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::EvidenceMissing));
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::Unreconstructable
    );
}

#[test]
fn confidence_unlabeled_narrows() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.integrity.confidence_label_visible = false;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::ConfidenceUnlabeled));
    assert!(decision.narrowed);
}

#[test]
fn freshness_unlabeled_narrows_first_party() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.integrity.freshness_state_labeled = false;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::FreshnessUnlabeled));
}

#[test]
fn superseded_not_marked_narrows() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.declared_freshness_state = FreshnessState::SupersededByNewerRun;
    c.integrity.superseded_state_marked = false;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::SupersededNotMarked));
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
}

#[test]
fn superseded_but_marked_stays_certified() {
    let decision = case(&seeded(), C_SUPERSEDED).narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Certified);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn virtualization_not_stream_first_narrows() {
    let mut c = cloned(&seeded(), C_VIRT);
    c.virtualization.stream_first = false;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::VirtualizationNotStreamFirst));
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
}

#[test]
fn search_unavailable_narrows() {
    let mut c = cloned(&seeded(), C_VIRT);
    c.virtualization.searchable = false;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::SearchUnavailable));
}

#[test]
fn copy_export_unavailable_narrows() {
    let mut c = cloned(&seeded(), C_VIRT);
    c.virtualization.copy_exportable = false;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::CopyExportUnavailable));
}

#[test]
fn first_party_stale_narrows() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::StaleEvidence));
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
}

#[test]
fn missing_proof_narrows() {
    let mut c = cloned(&seeded(), C_NATIVE);
    c.verification.proof_currency = ProofCurrency::MissingProof;
    c.verification.proof_ref = None;
    render_all(&mut c, FallbackClaim::Narrowed);
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::MissingProof));
}

#[test]
fn stale_window_ages_out_a_current_proof() {
    let c = cloned(&seeded(), C_NATIVE);
    // The same clean case narrows once the verification window has elapsed.
    let mut staled = c.clone();
    render_all(&mut staled, FallbackClaim::Narrowed);
    let decision = staled.narrow(true);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::StaleProof));
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
    // And is clean at its own window.
    assert!(c.narrow(false).active_narrowing_reasons.is_empty());
}

#[test]
fn canonical_perf_case_narrows_via_stale_proof() {
    let decision = case(&seeded(), C_PERF).narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::StaleProof));
}

#[test]
fn canonical_notebook_case_narrows_via_stale_evidence() {
    let decision = case(&seeded(), C_NOTEBOOK_STALE).narrow(false);
    assert_eq!(decision.effective_fallback_claim, FallbackClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&FallbackNarrowingReason::StaleEvidence));
}

#[test]
fn labs_case_makes_no_claim() {
    let decision = case(&seeded(), C_LABS).narrow(false);
    assert_eq!(
        decision.effective_fallback_claim,
        FallbackClaim::LabsNotClaimed
    );
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn floored_case_reports_unmapped_confidence() {
    let mut c = cloned(&seeded(), C_HEUR);
    c.integrity.preserves_run_channel_lineage = false;
    let decision = c.narrow(false);
    assert_eq!(
        c.effective_confidence(decision.effective_fallback_claim),
        ConfidenceTier::UnmappedRequiresReview
    );
}

#[test]
fn normalized_and_malformed_cases_certify() {
    assert_eq!(
        case(&seeded(), C_NORMALIZED)
            .narrow(false)
            .effective_fallback_claim,
        FallbackClaim::Certified
    );
    assert_eq!(
        case(&seeded(), C_MALFORMED)
            .narrow(false)
            .effective_fallback_claim,
        FallbackClaim::Certified
    );
}

// --------------------------------------------------------------------------- //
// Packet-level validation.
// --------------------------------------------------------------------------- //

#[test]
fn duplicate_case_id_is_rejected() {
    let mut packet = seeded();
    let dup = packet.cases[0].clone();
    packet.cases.push(dup);
    assert!(packet
        .validate()
        .contains(&M5FallbackEvidenceDrillViolation::DuplicateCaseId));
}

#[test]
fn dropping_a_drill_is_rejected() {
    let mut packet = seeded();
    // Remove the only lost-channel case.
    packet
        .cases
        .retain(|c| c.drill_kind != FallbackDrillKind::LostChannel);
    assert!(packet
        .validate()
        .contains(&M5FallbackEvidenceDrillViolation::DrillKindMissing));
}

#[test]
fn dropping_a_profile_is_rejected() {
    let mut packet = seeded();
    // The debug console only appears on the native case.
    for c in &mut packet.cases {
        c.profiles
            .retain(|p| p.profile != ToolingProfile::DebugConsole);
    }
    // Keep every case rendered on at least one profile.
    assert!(packet.cases.iter().all(|c| !c.profiles.is_empty()));
    assert!(packet
        .validate()
        .contains(&M5FallbackEvidenceDrillViolation::ProfileMissing));
}

#[test]
fn dropping_every_heuristic_case_is_rejected() {
    let mut packet = seeded();
    packet.cases.retain(|c| !c.is_heuristic());
    let violations = packet.validate();
    assert!(violations.contains(&M5FallbackEvidenceDrillViolation::HeuristicCaseMissing));
}

#[test]
fn overlay_missing_provider_ref_is_rejected() {
    let mut packet = seeded();
    let idx = packet
        .cases
        .iter()
        .position(|c| c.case_id == C_IMPORTED)
        .unwrap();
    packet.cases[idx].links.provider_ref = None;
    assert!(packet
        .validate()
        .contains(&M5FallbackEvidenceDrillViolation::OverlayMissingProviderRef));
}

#[test]
fn real_channel_missing_channel_ref_is_rejected() {
    let mut packet = seeded();
    let idx = packet
        .cases
        .iter()
        .position(|c| c.case_id == C_NATIVE)
        .unwrap();
    packet.cases[idx].links.channel_ref = None;
    assert!(packet
        .validate()
        .contains(&M5FallbackEvidenceDrillViolation::RealChannelMissingChannelRef));
}

#[test]
fn heuristic_missing_backlink_ref_is_rejected() {
    let mut packet = seeded();
    let idx = packet
        .cases
        .iter()
        .position(|c| c.case_id == C_HEUR)
        .unwrap();
    packet.cases[idx].links.raw_output_backlink_ref = None;
    assert!(packet
        .validate()
        .contains(&M5FallbackEvidenceDrillViolation::HeuristicMissingBacklinkRef));
}

#[test]
fn narrowed_case_carries_a_precise_label() {
    let packet = seeded();
    let perf = case(&packet, C_PERF);
    let decision = perf.narrow(false);
    let label = perf.narrowed_label(&decision).expect("narrowed label");
    assert!(!label_is_generic(&label));
    assert!(label.contains("verification proof stale"));
}
