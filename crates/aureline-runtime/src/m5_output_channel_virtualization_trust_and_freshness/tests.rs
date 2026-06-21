use super::*;

const C_RAW_LOCAL: &str = "channel:raw-log-local-test:0001";
const C_STRUCTURED_LOCAL: &str = "channel:structured-report-local-test:0001";
const C_HTML_LOCAL: &str = "channel:html-bundle-local-task:0001";
const C_RAW_NARROWED: &str = "channel:raw-log-local-task:0001";
const C_PIPELINE: &str = "channel:raw-log-pipeline-provider:0001";
const C_IMPORTED: &str = "channel:structured-report-imported-provider:0001";
const C_LABS: &str = "channel:html-bundle-labs:0001";

fn seeded() -> M5OutputChannelSetPacket {
    seeded_m5_output_channel_set()
}

fn channel<'a>(packet: &'a M5OutputChannelSetPacket, id: &str) -> &'a OutputChannelRecord {
    packet
        .channels
        .iter()
        .find(|c| c.channel_id == id)
        .unwrap_or_else(|| panic!("missing channel {id}"))
}

fn cloned(packet: &M5OutputChannelSetPacket, id: &str) -> OutputChannelRecord {
    channel(packet, id).clone()
}

/// A faithful consumer renders the effective claim, so a narrowing test lowers the
/// rendered claim to match — otherwise the surface itself overclaims and floors.
fn render_all(c: &mut OutputChannelRecord, claim: ChannelClaim) {
    c.renderings
        .iter_mut()
        .for_each(|r| r.rendered_claim = claim);
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_OUTPUT_CHANNELS_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_OUTPUT_CHANNELS_SCHEMA_VERSION);
    assert_eq!(packet.taxonomy_version, M5_OUTPUT_CHANNELS_TAXONOMY_VERSION);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.channels.len(), 9);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical =
        current_m5_output_channel_set().expect("canonical channel set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seeded_covers_every_kind_trust_and_surface() {
    let packet = seeded();
    for kind in ChannelPayloadKind::ALL {
        assert!(
            packet.represented_payload_kinds().contains(&kind),
            "missing payload kind {}",
            kind.as_str()
        );
    }
    for trust in ContentTrustClass::ALL {
        assert!(
            packet.represented_trust_classes().contains(&trust),
            "missing trust class {}",
            trust.as_str()
        );
    }
    for surface in ChannelSurface::ALL {
        assert!(
            packet.represented_surfaces().contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn claim_distribution_is_stable() {
    // Five first-party channels certify; the stale-proof raw log narrows; the pipeline
    // and imported channels stay read-only; the Labs bundle makes no claim.
    let dist = seeded().claim_distribution();
    assert_eq!(dist.certified, 5);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.overlay, 2);
    assert_eq!(dist.unreconstructable, 0);
    assert_eq!(dist.labs, 1);
    assert_eq!(seeded().narrowed_channel_count(), 1);
}

#[test]
fn export_safe_json_round_trips() {
    let packet = seeded();
    let json = packet.export_safe_json();
    let reparsed: M5OutputChannelSetPacket = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(reparsed, packet);
    assert!(reparsed.validate().is_empty());
}

#[test]
fn export_carries_no_forbidden_material() {
    let value = serde_json::to_value(seeded()).expect("serializes");
    assert!(!json_contains_forbidden_boundary_material(&value));
}

#[test]
fn markdown_summary_lists_channels_and_counts() {
    let summary = seeded().render_markdown_summary();
    assert!(summary.contains("# M5 Output-Channel Virtualization, Trust, and Freshness"));
    assert!(summary.contains("5 certified, 1 narrowed, 2 read-only overlay"));
    assert!(summary.contains(C_RAW_NARROWED));
}

// --------------------------------------------------------------------------- //
// Per-channel derivation (mirrors the perturbation corpus).
// --------------------------------------------------------------------------- //

#[test]
fn clean_raw_channel_certifies() {
    let decision = channel(&seeded(), C_RAW_LOCAL).narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Certified);
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn flattening_channel_identity_floors() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.lineage.canonical_channel_ref = None;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ChannelIdentityFlattened));
    assert!(c.floored_keeps_fallback(decision.effective_channel_claim));
}

#[test]
fn flattening_run_step_lineage_floors() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.integrity.preserves_run_step_lineage = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::RunStepLineageFlattened));
}

#[test]
fn lineage_not_visible_on_a_surface_floors() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    render_all(&mut c, ChannelClaim::Unreconstructable);
    // render_all only rewrites rendered_claim; the hidden-lineage flag stays set.
    c.renderings[0].lineage_visible = false;
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::LineageNotVisible));
}

#[test]
fn large_log_not_stream_first_floors() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    assert!(c.requires_virtualization());
    c.virtualization.stream_first = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::StreamNotVirtualized));
}

#[test]
fn large_log_unbounded_memory_floors() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.virtualization.bounded_memory = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::UnboundedMemory));
}

#[test]
fn large_log_export_forces_full_materialization_floors() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.virtualization.exportable_without_full_materialization = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ExportForcesFullMaterialization));
}

#[test]
fn small_log_without_stream_first_stays_certified() {
    // A small structured report is not required to be stream-first.
    let packet = seeded();
    let c = channel(&packet, C_STRUCTURED_LOCAL);
    assert!(!c.requires_virtualization());
    assert_eq!(
        c.narrow(false).effective_channel_claim,
        ChannelClaim::Certified
    );
}

#[test]
fn active_content_without_confirmation_floors() {
    let mut c = cloned(&seeded(), C_HTML_LOCAL);
    assert!(c.trust_class.is_active_content());
    c.access.open_in_external_requires_confirmation = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ActiveContentAutoOpens));
}

#[test]
fn blurred_trust_boundary_floors() {
    let mut c = cloned(&seeded(), C_HTML_LOCAL);
    c.access.trust_boundary_preserved = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::TrustBoundaryBlurred));
}

#[test]
fn unsafe_export_floors() {
    let mut c = cloned(&seeded(), C_HTML_LOCAL);
    c.access.export_is_safe = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ExportUnsafe));
}

#[test]
fn provider_backed_masquerading_as_live_floors() {
    let mut c = cloned(&seeded(), C_PIPELINE);
    c.freshness.live_state_honest = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::StaleChannelClaimsLive));
}

#[test]
fn imported_channel_claiming_live_floors() {
    let mut c = cloned(&seeded(), C_PIPELINE);
    c.integrity.imported_channel_read_only = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ImportedChannelClaimsLive));
}

#[test]
fn overlay_with_any_gap_floors() {
    // An overlay is the minimal honest claim, so any non-floor gap drops it below the
    // read-only overlay rather than holding it.
    let mut c = cloned(&seeded(), C_PIPELINE);
    c.access.trust_class_labeled = false;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::TrustClassUnlabeled));
}

#[test]
fn imported_overlay_cached_stays_overlay() {
    let decision = channel(&seeded(), C_IMPORTED).narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::ReadOnlyOverlay
    );
    assert!(!decision.narrowed);
}

#[test]
fn trust_class_unlabeled_narrows_first_party() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.access.trust_class_labeled = false;
    render_all(&mut c, ChannelClaim::Narrowed);
    let decision = c.narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Narrowed);
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![ChannelNarrowingReason::TrustClassUnlabeled]
    );
}

#[test]
fn chunk_ids_unstable_narrows() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.virtualization.stable_chunk_ids = false;
    render_all(&mut c, ChannelClaim::Narrowed);
    let decision = c.narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ChunkIdsUnstable));
}

#[test]
fn follow_mode_unavailable_narrows() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.virtualization.follow_mode_supported = false;
    render_all(&mut c, ChannelClaim::Narrowed);
    let decision = c.narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::FollowModeUnavailable));
}

#[test]
fn pin_control_unavailable_narrows() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.access.pin_supported = false;
    render_all(&mut c, ChannelClaim::Narrowed);
    let decision = c.narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::PinControlUnavailable));
}

#[test]
fn first_party_stale_narrows() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.declared_freshness_state = FreshnessState::StaleExpired;
    render_all(&mut c, ChannelClaim::Narrowed);
    let decision = c.narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::StaleEvidence));
}

#[test]
fn superseded_marked_stays_certified() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.declared_freshness_state = FreshnessState::SupersededByNewerRun;
    // superseded_state_marked stays true from clean_integrity().
    let decision = c.narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Certified);
    assert!(!decision.narrowed);
}

#[test]
fn missing_content_floors() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.declared_freshness_state = FreshnessState::Missing;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ChannelContentMissing));
}

#[test]
fn stale_window_ages_out_proof() {
    // An elapsed verification window ages out a current proof to narrowed.
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    render_all(&mut c, ChannelClaim::Narrowed);
    let decision = c.narrow(true);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Narrowed);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::StaleProof));
}

#[test]
fn narrowed_channel_in_seed_is_stale_proof() {
    let decision = channel(&seeded(), C_RAW_NARROWED).narrow(false);
    assert_eq!(decision.effective_channel_claim, ChannelClaim::Narrowed);
    assert_eq!(
        decision.active_narrowing_reasons,
        vec![ChannelNarrowingReason::StaleProof]
    );
    assert!(channel(&seeded(), C_RAW_NARROWED)
        .narrowed_label(&decision)
        .is_some());
}

#[test]
fn labs_channel_makes_no_claim() {
    let decision = channel(&seeded(), C_LABS).narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::LabsNotClaimed
    );
    assert!(!decision.narrowed);
    assert!(decision.active_narrowing_reasons.is_empty());
}

#[test]
fn surface_overclaim_floors_and_is_caught_by_validate() {
    let mut c = cloned(&seeded(), C_RAW_NARROWED);
    // The narrowed raw log: a surface that renders certified overclaims.
    c.renderings[0].rendered_claim = ChannelClaim::Certified;
    let decision = c.narrow(false);
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::SurfaceOverclaims));
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );

    let mut packet = seeded();
    let idx = packet
        .channels
        .iter()
        .position(|x| x.channel_id == C_RAW_NARROWED)
        .unwrap();
    packet.channels[idx] = c;
    assert!(packet
        .validate()
        .contains(&M5OutputChannelViolation::RenderingSurfaceOverclaims));
}

#[test]
fn reopen_target_lost_floors_but_keeps_keyboard_fallback() {
    let mut c = cloned(&seeded(), C_RAW_LOCAL);
    c.declared_reopen_target = ReopenTarget::NoneKeyboardFallback;
    render_all(&mut c, ChannelClaim::Unreconstructable);
    let decision = c.narrow(false);
    assert_eq!(
        decision.effective_channel_claim,
        ChannelClaim::Unreconstructable
    );
    assert!(decision
        .active_narrowing_reasons
        .contains(&ChannelNarrowingReason::ReopenTargetLost));
    assert!(c.floored_keeps_fallback(decision.effective_channel_claim));
}
