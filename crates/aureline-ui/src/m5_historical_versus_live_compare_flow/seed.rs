//! Canonical seed for the historical-versus-live compare-flow packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`CompareHistoricalGrammar`] so the same preserved-snapshot profile always carries the same historical
//! grammar across surfaces, and every outcome derives its identity, freshness, disclosure, and action set
//! from [`resolve_compare_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every compare surface offers so the compare state, provenance, and
/// open-live-target action are discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5HistoricalReferenceAccessibilityRoute> {
    M5HistoricalReferenceAccessibilityRoute::ALL.to_vec()
}

fn grammar(
    historical_role: &str,
    snapshot_label: &str,
    capture_time: &str,
    provenance: &str,
    mutation_blocked_posture: &str,
) -> CompareHistoricalGrammar {
    CompareHistoricalGrammar {
        historical_role_word: historical_role.to_owned(),
        snapshot_label_word: snapshot_label.to_owned(),
        capture_time_word: capture_time.to_owned(),
        provenance_word: provenance.to_owned(),
        mutation_blocked_posture_word: mutation_blocked_posture.to_owned(),
    }
}

/// Stable position of an object class in the frozen declaration order, used to spread freshness / mismatch
/// choices deterministically across profiles without any per-binding hand-authoring.
fn object_class_index(object_class: M5HistoricalReferenceObject) -> usize {
    M5HistoricalReferenceObject::ALL
        .iter()
        .position(|candidate| *candidate == object_class)
        .expect("object class is a member of ALL")
}

/// The freshness / drift state a binding labels, derived from its outcome and object class. A live pairing
/// (paired or approximate) carries a verifiable freshness comparison; a missing or policy-blocked pairing has
/// no live side and is therefore unverifiable, but still labeled.
fn freshness_for(
    outcome: CompareOutcome,
    object_class: M5HistoricalReferenceObject,
) -> CompareFreshnessDriftState {
    if resolve_compare_render_disclosure(outcome).requires_live_freshness {
        const VERIFIABLE: [CompareFreshnessDriftState; 3] = [
            CompareFreshnessDriftState::InSyncNoDrift,
            CompareFreshnessDriftState::SnapshotBehindLive,
            CompareFreshnessDriftState::SnapshotDivergedFromLive,
        ];
        VERIFIABLE[object_class_index(object_class) % VERIFIABLE.len()]
    } else {
        CompareFreshnessDriftState::FreshnessUnverifiable
    }
}

/// The mismatch reason a narrowed binding names, chosen deterministically from the outcome's allowed set. A
/// confirmed pairing names none.
fn mismatch_reason_for(
    outcome: CompareOutcome,
    object_class: M5HistoricalReferenceObject,
) -> Option<CompareMismatchReason> {
    let allowed = outcome.allowed_mismatch_reasons();
    if allowed.is_empty() {
        None
    } else {
        Some(allowed[object_class_index(object_class) % allowed.len()])
    }
}

fn explanation_for(reason: CompareMismatchReason) -> String {
    match reason {
        CompareMismatchReason::MissingLiveTarget => {
            "the live target no longer exists; the historical packet stays inspectable and no live comparison can run"
        }
        CompareMismatchReason::ChangedScope => {
            "the live target's scope changed since capture, so the pairing is approximate rather than exact"
        }
        CompareMismatchReason::ChangedBranchOrWorktree => {
            "the live target moved to a different branch or worktree, so the pairing is approximate rather than exact"
        }
        CompareMismatchReason::RetiredCapability => {
            "the snapshot describes a retired capability with no live counterpart; the historical packet stays inspectable"
        }
        CompareMismatchReason::UnsupportedSkew => {
            "the snapshot and live object are on an unsupported version skew, so live comparison is narrowed"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: CompareNarrowNextAction) -> String {
    match action {
        CompareNarrowNextAction::OpenApproximatePairingDetail => {
            "Open the approximate-pairing detail"
        }
        CompareNarrowNextAction::InspectHistoricalPacketOnly => {
            "Inspect the historical packet (metadata only)"
        }
    }
    .to_owned()
}

fn preserved_grammar_note() -> String {
    "historical-role, snapshot-label, capture-time, provenance, and mutation-blocked-posture words preserved; only the live comparison is narrowed"
        .to_owned()
}

fn allowed_actions_for(disclosure: CompareRenderDisclosure) -> Vec<CompareAction> {
    let mut actions = CompareAction::ANALYSIS_ONLY_BASE.to_vec();
    if disclosure.offers_open_live_target {
        actions.push(CompareAction::OpenCurrentLiveObject);
    }
    actions
}

fn binding_refs(object_class: M5HistoricalReferenceObject) -> Vec<String> {
    vec![
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    snapshot_profile_id: &str,
    snapshot_profile_label: &str,
    object_class: M5HistoricalReferenceObject,
    consumer: M5HistoricalReferenceConsumerSurface,
    outcome: CompareOutcome,
    historical_grammar: CompareHistoricalGrammar,
    reviewed_handoff: Option<ReviewedMutationHandoff>,
) -> CompareFlowBinding {
    let disclosure = resolve_compare_render_disclosure(outcome);
    let freshness = freshness_for(outcome, object_class);
    let mismatch_note = if disclosure.needs_mismatch_note {
        let reason = mismatch_reason_for(outcome, object_class)
            .expect("a narrowed outcome always has at least one allowed mismatch reason");
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrowed outcome always carries a next action");
        Some(CompareMismatchNote {
            reason,
            explanation: explanation_for(reason),
            preserved_grammar_note: preserved_grammar_note(),
            next_action,
            next_action_label: next_action_label_for(next_action),
        })
    } else {
        None
    };

    CompareFlowBinding {
        binding_id: binding_id.to_owned(),
        snapshot_profile_id: snapshot_profile_id.to_owned(),
        snapshot_profile_label: snapshot_profile_label.to_owned(),
        object_class,
        consumer,
        outcome,
        historical_grammar,
        identity_match_state: disclosure.identity_match_state,
        freshness_drift_state: freshness,
        drift_summary: freshness.drift_summary_word().to_owned(),
        parity_state: disclosure.parity_state,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        mismatch_note,
        reviewed_mutation_handoff: reviewed_handoff,
        historical_side_mutation_blocked: true,
        collapses_snapshot_and_live_into_one_ambiguous_view: false,
        implies_apply_or_sync_historical_snapshot_is_safe: false,
        reopens_live_target_without_validating_identity_trust_route_and_authority: false,
        dead_ends_on_missing_or_mismatched_target: false,
        leaves_historical_side_mutable_or_unlabeled: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of a preserved-snapshot profile, before any outcome override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    outcome: CompareOutcome,
    reviewed_handoff: Option<(&'static str, &'static str)>,
}

/// One preserved-snapshot profile compared across several consumer surfaces at one historical grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: CompareHistoricalGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: CompareHistoricalGrammar,
    bindings: Vec<BindingSpec>,
) -> ProfileSpec {
    ProfileSpec {
        profile_id,
        profile_label,
        object_class,
        grammar,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    outcome: CompareOutcome,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        outcome,
        reviewed_handoff: None,
    }
}

fn bs_handoff(
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    outcome: CompareOutcome,
    reviewed_handoff: (&'static str, &'static str),
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        outcome,
        reviewed_handoff: Some(reviewed_handoff),
    }
}

/// The five preserved-snapshot profiles — one per B149 historical-reference object class — and the surfaces
/// that compare each against its live target, drawn from the shell / archive-viewer, help / docs, support,
/// review / incident, runbook-archive, release-center, companion / export, program-governance, and
/// CLI / export consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use CompareOutcome::*;
    use M5HistoricalReferenceConsumerSurface::*;
    use M5HistoricalReferenceObject::*;

    let read_only_posture = "read_only_non_authoritative_for_mutation";

    vec![
        spec(
            "retirement-snapshot/last-supported-archive",
            "Retirement / last-supported snapshot (compare against current line)",
            RetirementSnapshot,
            grammar(
                "snapshot_labeling",
                "retirement_last_supported_snapshot",
                "retirement_capture_time",
                "last_supported_build_provenance",
                read_only_posture,
            ),
            vec![
                bs_handoff(
                    "hvlc-retirement-release",
                    ReleaseCenter,
                    LiveTargetPaired,
                    (
                        "m5-migration-review-flow",
                        "Reviewed migration path (owns any actual mutation)",
                    ),
                ),
                bs("hvlc-retirement-shell", Shell, ApproximatePairing),
                bs("hvlc-retirement-cli", CliExport, PolicyBlockedPairing),
            ],
        ),
        spec(
            "support-export-evidence/captured-bundle",
            "Captured support / export evidence (compare against current object)",
            SupportExportEvidence,
            grammar(
                "provenance_attribution",
                "captured_support_export_evidence",
                "evidence_capture_time",
                "support_bundle_capture_context",
                read_only_posture,
            ),
            vec![
                bs("hvlc-support-evidence-support", Support, LiveTargetPaired),
                bs("hvlc-support-evidence-help", HelpDocs, LiveTargetMissing),
                bs(
                    "hvlc-support-evidence-companion",
                    CompanionExport,
                    PolicyBlockedPairing,
                ),
            ],
        ),
        spec(
            "archived-runbook-packet/historical-run",
            "Archived runbook execution packet (compare against current run)",
            ArchivedRunbookPacket,
            grammar(
                "live_target_handoff",
                "archived_runbook_execution_packet",
                "run_capture_time",
                "runbook_run_provenance",
                read_only_posture,
            ),
            vec![
                bs("hvlc-runbook-archive", RunbookArchive, LiveTargetPaired),
                bs("hvlc-runbook-review", ReviewIncident, LiveTargetMissing),
                bs(
                    "hvlc-runbook-program",
                    ProgramGovernance,
                    ApproximatePairing,
                ),
            ],
        ),
        spec(
            "imported-offline-route-evidence/offline-only",
            "Imported / offline route evidence (compare against current route)",
            ImportedOfflineRouteEvidence,
            grammar(
                "imported_offline_disclosure",
                "imported_offline_route_evidence",
                "import_capture_time",
                "import_offline_source_provenance",
                read_only_posture,
            ),
            vec![
                bs("hvlc-imported-shell", Shell, LiveTargetMissing),
                bs(
                    "hvlc-imported-runbook",
                    RunbookArchive,
                    PolicyBlockedPairing,
                ),
                bs("hvlc-imported-cli", CliExport, ApproximatePairing),
            ],
        ),
        spec(
            "review-incident-snapshot/evidence-reopen",
            "Review / incident snapshot (compare against current object)",
            ReviewIncidentSnapshot,
            grammar(
                "mutation_blocked_posture",
                "review_incident_snapshot",
                "incident_capture_time",
                "incident_evidence_provenance",
                read_only_posture,
            ),
            vec![
                bs("hvlc-review-review", ReviewIncident, LiveTargetPaired),
                bs("hvlc-review-shell", Shell, LiveTargetMissing),
                bs(
                    "hvlc-review-companion",
                    CompanionExport,
                    PolicyBlockedPairing,
                ),
            ],
        ),
    ]
}

/// Builds all compare bindings, applying `outcome_override` to override a binding's outcome.
fn build_bindings<F>(outcome_override: F) -> Vec<CompareFlowBinding>
where
    F: Fn(&str, CompareOutcome) -> CompareOutcome,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let outcome = outcome_override(spec.binding_id, spec.outcome);
            let reviewed_handoff =
                spec.reviewed_handoff
                    .map(|(id, label)| ReviewedMutationHandoff {
                        reviewed_path_id: id.to_owned(),
                        reviewed_path_label: label.to_owned(),
                    });
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.object_class,
                spec.consumer,
                outcome,
                profile.grammar.clone(),
                reviewed_handoff,
            ));
        }
    }
    bindings
}

fn trust_review() -> HistoricalVersusLiveCompareFlowTrustReview {
    HistoricalVersusLiveCompareFlowTrustReview {
        object_class_reuse_proven_by_fixtures: true,
        same_profile_same_historical_grammar_across_surfaces: true,
        historical_role_words_stay_in_frozen_vocabulary: true,
        mutation_blocked_posture_never_masquerades_as_live: true,
        compare_never_implies_apply_or_sync_is_safe: true,
        open_live_target_always_validates_identity_trust_route_authority: true,
        missing_or_mismatched_target_never_dead_ends: true,
        snapshot_and_live_never_collapsed_into_one_ambiguous_view: true,
        identity_freshness_and_drift_always_labeled: true,
        accessibility_routes_present_for_state_provenance_and_open_live_target: true,
        narrowing_disclosed_across_outcomes: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> HistoricalVersusLiveCompareFlowProjection {
    HistoricalVersusLiveCompareFlowProjection {
        shell_consumes_compare_flow: true,
        help_docs_consumes_compare_flow: true,
        support_consumes_compare_flow: true,
        review_incident_consumes_compare_flow: true,
        runbook_archive_consumes_compare_flow: true,
        release_center_consumes_compare_flow: true,
        companion_export_consumes_compare_flow: true,
        program_governance_consumes_compare_flow: true,
        cli_export_consumes_compare_flow: true,
        every_object_class_paired_by_two_or_more_consumers: true,
        historical_grammar_identical_for_same_profile: true,
        narrowing_disclosed_not_hidden: true,
        compare_maps_back_to_one_historical_reference_object: true,
    }
}

fn proof_freshness() -> HistoricalVersusLiveCompareFlowProofFreshness {
    HistoricalVersusLiveCompareFlowProofFreshness {
        proof_freshness_slo_hours: M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_SCHEMA_REF.to_owned(),
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_DOC_REF.to_owned(),
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF.to_owned(),
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned(),
    ];
    // The five object classes map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5HistoricalReferenceObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    compare_bindings: Vec<CompareFlowBinding>,
) -> M5HistoricalVersusLiveCompareFlowPacket {
    M5HistoricalVersusLiveCompareFlowPacket::new(M5HistoricalVersusLiveCompareFlowPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        compare_bindings,
        downgrade_triggers: HistoricalVersusLiveCompareFlowDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5HistoricalReferenceConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in historical-versus-live compare-flow packet.
pub fn seeded_m5_historical_versus_live_compare_flow() -> M5HistoricalVersusLiveCompareFlowPacket {
    packet_from_bindings(
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_PACKET_ID,
        "M5 historical-vs-live compare flows (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more confirmed pairings narrowed to a missing live target, proving the
/// historical grammar survives and the user still inspects the historical packet when the target is gone.
pub fn seeded_m5_historical_versus_live_compare_flow_missing_target_narrowed(
) -> M5HistoricalVersusLiveCompareFlowPacket {
    packet_from_bindings(
        "m5-historical-versus-live-compare-flow:missing-target:0001",
        "M5 historical-vs-live compare flows (missing-target narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "hvlc-support-evidence-support" => CompareOutcome::LiveTargetMissing,
            "hvlc-runbook-archive" => CompareOutcome::LiveTargetMissing,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more confirmed pairings narrowed to a policy-blocked pairing, proving
/// the historical grammar survives and the user still inspects the historical packet when policy blocks the
/// live comparison.
pub fn seeded_m5_historical_versus_live_compare_flow_policy_blocked_narrowed(
) -> M5HistoricalVersusLiveCompareFlowPacket {
    packet_from_bindings(
        "m5-historical-versus-live-compare-flow:policy-blocked:0001",
        "M5 historical-vs-live compare flows (policy-blocked narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "hvlc-review-review" => CompareOutcome::PolicyBlockedPairing,
            "hvlc-retirement-release" => CompareOutcome::PolicyBlockedPairing,
            _ => default,
        }),
    )
}
