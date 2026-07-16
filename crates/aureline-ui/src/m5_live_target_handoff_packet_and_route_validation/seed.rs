//! Canonical seed for the live-target-handoff packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`HandoffHistoricalGrammar`] so the same preserved-snapshot profile always carries the same historical
//! grammar across surfaces, and every outcome derives its parity, blocker note, precondition check, and action
//! set from [`resolve_handoff_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every handoff surface offers so the handoff state, provenance, and
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
) -> HandoffHistoricalGrammar {
    HandoffHistoricalGrammar {
        historical_role_word: historical_role.to_owned(),
        snapshot_label_word: snapshot_label.to_owned(),
        capture_time_word: capture_time.to_owned(),
        provenance_word: provenance.to_owned(),
        mutation_blocked_posture_word: mutation_blocked_posture.to_owned(),
    }
}

/// Stable position of an object class in the frozen declaration order, used to spread route / trust / authority
/// and blocker choices deterministically across profiles without any per-binding hand-authoring.
fn object_class_index(object_class: M5HistoricalReferenceObject) -> usize {
    M5HistoricalReferenceObject::ALL
        .iter()
        .position(|candidate| *candidate == object_class)
        .expect("object class is a member of ALL")
}

/// The blocker reason a blocked binding names, chosen deterministically from the outcome's allowed set. A
/// cleared handoff names none.
fn blocker_reason_for(
    outcome: HandoffOutcome,
    object_class: M5HistoricalReferenceObject,
) -> Option<HandoffBlockerReason> {
    let allowed = outcome.allowed_blocker_reasons();
    if allowed.is_empty() {
        None
    } else {
        Some(allowed[object_class_index(object_class) % allowed.len()])
    }
}

/// Builds a precondition check consistent with the outcome: a cleared handoff clears every precondition; a
/// blocked handoff fails exactly the precondition its named blocker reason maps to.
fn precondition_for(reason: Option<HandoffBlockerReason>) -> HandoffPreconditionCheck {
    let mut check = HandoffPreconditionCheck {
        target_exists: true,
        target_in_current_scope: true,
        route_available: true,
        trust_posture_satisfied: true,
        auth_and_approval_satisfied: true,
    };
    if let Some(reason) = reason {
        match reason {
            HandoffBlockerReason::TargetDoesNotExist
            | HandoffBlockerReason::RetiredCapabilityNoLiveCounterpart => {
                check.target_exists = false;
            }
            HandoffBlockerReason::TargetOutsideCurrentScope => {
                check.target_in_current_scope = false;
            }
            HandoffBlockerReason::RouteUnavailable
            | HandoffBlockerReason::PolicyOrLifecycleBlocked => {
                check.route_available = false;
            }
            HandoffBlockerReason::TrustPostureInsufficient => {
                check.trust_posture_satisfied = false;
            }
            HandoffBlockerReason::AuthOrApprovalMissing => {
                check.auth_and_approval_satisfied = false;
            }
        }
    }
    check
}

fn route_for(object_class: M5HistoricalReferenceObject) -> LiveTargetRouteClass {
    LiveTargetRouteClass::ALL[object_class_index(object_class) % LiveTargetRouteClass::ALL.len()]
}

/// The authority both the requested and direct-open classes share for this object, so the handoff never widens
/// authority relative to a direct open.
fn authority_for(object_class: M5HistoricalReferenceObject) -> LiveTargetAuthorityClass {
    LiveTargetAuthorityClass::ALL
        [object_class_index(object_class) % LiveTargetAuthorityClass::ALL.len()]
}

fn trust_posture_for(reason: Option<HandoffBlockerReason>) -> LiveTargetTrustPosture {
    match reason {
        Some(HandoffBlockerReason::TrustPostureInsufficient) => {
            LiveTargetTrustPosture::NeedsTrustRevalidation
        }
        _ => LiveTargetTrustPosture::TrustedCurrentSession,
    }
}

fn target_kind_for(object_class: M5HistoricalReferenceObject) -> &'static str {
    match object_class {
        M5HistoricalReferenceObject::RetirementSnapshot => "current_supported_line",
        M5HistoricalReferenceObject::SupportExportEvidence => "current_object",
        M5HistoricalReferenceObject::ArchivedRunbookPacket => "runbook_run",
        M5HistoricalReferenceObject::ImportedOfflineRouteEvidence => "current_route",
        M5HistoricalReferenceObject::ReviewIncidentSnapshot => "incident_object",
    }
}

fn target_identity_for(object_class: M5HistoricalReferenceObject) -> LiveTargetIdentity {
    LiveTargetIdentity {
        target_id: format!("live-target-{}", object_class.as_str()),
        target_label: format!("current live {}", target_kind_for(object_class)),
        target_kind: target_kind_for(object_class).to_owned(),
    }
}

fn explanation_for(reason: HandoffBlockerReason) -> String {
    match reason {
        HandoffBlockerReason::TargetDoesNotExist => {
            "the live target no longer exists; the historical packet stays inspectable and no live reopen can run"
        }
        HandoffBlockerReason::TargetOutsideCurrentScope => {
            "the live target is outside the current scope / workset, so the handoff cannot reopen it here"
        }
        HandoffBlockerReason::RouteUnavailable => {
            "the remote / managed route to the live target is unavailable; satisfy the route prerequisite and retry"
        }
        HandoffBlockerReason::TrustPostureInsufficient => {
            "the trust posture must be revalidated before the live target may be reopened"
        }
        HandoffBlockerReason::AuthOrApprovalMissing => {
            "a required auth / approval prerequisite is missing; complete it through its reviewed path and retry"
        }
        HandoffBlockerReason::RetiredCapabilityNoLiveCounterpart => {
            "the snapshot describes a retired capability with no live counterpart; the historical packet stays inspectable"
        }
        HandoffBlockerReason::PolicyOrLifecycleBlocked => {
            "a policy or lifecycle rule blocks reopening the live target; the historical packet stays inspectable"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: HandoffBlockerNextAction) -> String {
    match action {
        HandoffBlockerNextAction::SatisfyPrerequisiteThenRetry => {
            "Satisfy the prerequisite through its reviewed path, then retry"
        }
        HandoffBlockerNextAction::InspectHistoricalPacketOnly => {
            "Inspect the historical packet (metadata only)"
        }
    }
    .to_owned()
}

fn preserved_historical_note() -> String {
    "historical-role, snapshot-label, capture-time, provenance, and mutation-blocked-posture words preserved; the historical side stays non-live and only the live pivot is blocked"
        .to_owned()
}

fn allowed_actions_for(disclosure: HandoffRenderDisclosure) -> Vec<HandoffAction> {
    let mut actions = HandoffAction::ANALYSIS_ONLY_BASE.to_vec();
    if disclosure.offers_open_live_target {
        actions.push(HandoffAction::OpenCurrentLiveObject);
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
    outcome: HandoffOutcome,
    historical_grammar: HandoffHistoricalGrammar,
    reviewed_authority_handoff: Option<ReviewedAuthorityHandoff>,
) -> LiveTargetHandoffBinding {
    let disclosure = resolve_handoff_render_disclosure(outcome);
    let reason = blocker_reason_for(outcome, object_class);
    let precondition_check = precondition_for(reason);
    let authority = authority_for(object_class);

    let handoff_request = LiveTargetHandoffRequest {
        source_snapshot_id: format!("historical-snapshot-{}", object_class.as_str()),
        target_identity: target_identity_for(object_class),
        required_route_class: route_for(object_class),
        required_trust_posture: trust_posture_for(reason),
        required_auth_prerequisites: HandoffAuthPrerequisite::ALL.to_vec(),
        requested_authority_class: authority,
        direct_open_authority_class: authority,
        precondition_check,
        fallback_behavior: disclosure.fallback_behavior,
    };

    let blocker_note = if disclosure.needs_blocker_note {
        let reason =
            reason.expect("a blocked outcome always has at least one allowed blocker reason");
        let next_action = disclosure
            .blocker_next_action
            .expect("a blocked outcome always carries a next action");
        Some(HandoffBlockerNote {
            reason,
            explanation: explanation_for(reason),
            preserved_historical_note: preserved_historical_note(),
            fallback_behavior: disclosure.fallback_behavior,
            next_action,
            next_action_label: next_action_label_for(next_action),
        })
    } else {
        None
    };

    LiveTargetHandoffBinding {
        binding_id: binding_id.to_owned(),
        snapshot_profile_id: snapshot_profile_id.to_owned(),
        snapshot_profile_label: snapshot_profile_label.to_owned(),
        object_class,
        consumer,
        outcome,
        historical_grammar,
        handoff_request,
        parity_state: disclosure.parity_state,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        blocker_note,
        reviewed_authority_handoff,
        historical_side_mutation_blocked: true,
        reopens_live_target_without_validating_identity_trust_route_and_authority: false,
        widens_authority_beyond_direct_open: false,
        dead_ends_when_target_unavailable: false,
        leaks_secret_or_ambient_credential: false,
        presents_snapshot_as_current_live_object: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of a preserved-snapshot profile, before any outcome override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    outcome: HandoffOutcome,
    reviewed_authority_handoff: Option<(&'static str, &'static str)>,
}

/// One preserved-snapshot profile handed off across several consumer surfaces at one historical grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: HandoffHistoricalGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: HandoffHistoricalGrammar,
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
    outcome: HandoffOutcome,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        outcome,
        reviewed_authority_handoff: None,
    }
}

fn bs_handoff(
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    outcome: HandoffOutcome,
    reviewed_authority_handoff: (&'static str, &'static str),
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        outcome,
        reviewed_authority_handoff: Some(reviewed_authority_handoff),
    }
}

/// The five preserved-snapshot profiles — one per B149 historical-reference object class — and the surfaces
/// that hand each off to its live target, drawn from the shell / archive-viewer, help / docs, support,
/// review / incident, runbook-archive, release-center, companion / export, program-governance, and
/// CLI / export consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use HandoffOutcome::*;
    use M5HistoricalReferenceConsumerSurface::*;
    use M5HistoricalReferenceObject::*;

    let read_only_posture = "read_only_non_authoritative_for_mutation";

    vec![
        spec(
            "retirement-snapshot/last-supported-archive",
            "Retirement / last-supported snapshot (handoff to current line)",
            RetirementSnapshot,
            grammar(
                "snapshot_labeling",
                "retirement_last_supported_snapshot",
                "retirement_capture_time",
                "last_supported_build_provenance",
                read_only_posture,
            ),
            vec![
                bs("lth-retirement-release", ReleaseCenter, HandoffCleared),
                bs_handoff(
                    "lth-retirement-shell",
                    Shell,
                    BlockedNeedsPrerequisite,
                    (
                        "m5-auth-refresh-review-flow",
                        "Reviewed auth-refresh path (owns satisfying the prerequisite)",
                    ),
                ),
                bs("lth-retirement-cli", CliExport, BlockedByPolicy),
            ],
        ),
        spec(
            "support-export-evidence/captured-bundle",
            "Captured support / export evidence (handoff to current object)",
            SupportExportEvidence,
            grammar(
                "provenance_attribution",
                "captured_support_export_evidence",
                "evidence_capture_time",
                "support_bundle_capture_context",
                read_only_posture,
            ),
            vec![
                bs("lth-support-evidence-support", Support, HandoffCleared),
                bs(
                    "lth-support-evidence-help",
                    HelpDocs,
                    BlockedTargetUnavailable,
                ),
                bs(
                    "lth-support-evidence-companion",
                    CompanionExport,
                    BlockedByPolicy,
                ),
            ],
        ),
        spec(
            "archived-runbook-packet/historical-run",
            "Archived runbook execution packet (handoff to current run)",
            ArchivedRunbookPacket,
            grammar(
                "live_target_handoff",
                "archived_runbook_execution_packet",
                "run_capture_time",
                "runbook_run_provenance",
                read_only_posture,
            ),
            vec![
                bs("lth-runbook-archive", RunbookArchive, HandoffCleared),
                bs(
                    "lth-runbook-review",
                    ReviewIncident,
                    BlockedTargetUnavailable,
                ),
                bs_handoff(
                    "lth-runbook-program",
                    ProgramGovernance,
                    BlockedNeedsPrerequisite,
                    (
                        "m5-approval-review-flow",
                        "Reviewed approval path (owns any actual elevation)",
                    ),
                ),
            ],
        ),
        spec(
            "imported-offline-route-evidence/offline-only",
            "Imported / offline route evidence (handoff to current route)",
            ImportedOfflineRouteEvidence,
            grammar(
                "imported_offline_disclosure",
                "imported_offline_route_evidence",
                "import_capture_time",
                "import_offline_source_provenance",
                read_only_posture,
            ),
            vec![
                bs("lth-imported-shell", Shell, BlockedTargetUnavailable),
                bs("lth-imported-runbook", RunbookArchive, BlockedByPolicy),
                bs("lth-imported-cli", CliExport, BlockedNeedsPrerequisite),
            ],
        ),
        spec(
            "review-incident-snapshot/evidence-reopen",
            "Review / incident snapshot (handoff to current object)",
            ReviewIncidentSnapshot,
            grammar(
                "mutation_blocked_posture",
                "review_incident_snapshot",
                "incident_capture_time",
                "incident_evidence_provenance",
                read_only_posture,
            ),
            vec![
                bs("lth-review-review", ReviewIncident, HandoffCleared),
                bs("lth-review-shell", Shell, BlockedTargetUnavailable),
                bs("lth-review-companion", CompanionExport, BlockedByPolicy),
            ],
        ),
    ]
}

/// Builds all handoff bindings, applying `outcome_override` to override a binding's outcome.
fn build_bindings<F>(outcome_override: F) -> Vec<LiveTargetHandoffBinding>
where
    F: Fn(&str, HandoffOutcome) -> HandoffOutcome,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let outcome = outcome_override(spec.binding_id, spec.outcome);
            let reviewed_authority_handoff =
                spec.reviewed_authority_handoff
                    .map(|(id, label)| ReviewedAuthorityHandoff {
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
                reviewed_authority_handoff,
            ));
        }
    }
    bindings
}

fn trust_review() -> LiveTargetHandoffTrustReview {
    LiveTargetHandoffTrustReview {
        object_class_reuse_proven_by_fixtures: true,
        same_profile_same_historical_grammar_across_surfaces: true,
        historical_role_words_stay_in_frozen_vocabulary: true,
        mutation_blocked_posture_never_masquerades_as_live: true,
        every_handoff_validates_before_completing: true,
        handoff_never_widens_authority_beyond_direct_open: true,
        blocked_handoff_never_dead_ends: true,
        snapshot_never_presented_as_current_live_object: true,
        auth_prerequisites_named_never_embedded_as_secrets: true,
        actual_elevation_delegated_to_reviewed_authority_handoff: true,
        accessibility_routes_present_for_state_provenance_and_open_live_target: true,
        blocking_disclosed_across_outcomes: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> LiveTargetHandoffProjection {
    LiveTargetHandoffProjection {
        shell_consumes_handoff: true,
        help_docs_consumes_handoff: true,
        support_consumes_handoff: true,
        review_incident_consumes_handoff: true,
        runbook_archive_consumes_handoff: true,
        release_center_consumes_handoff: true,
        companion_export_consumes_handoff: true,
        program_governance_consumes_handoff: true,
        cli_export_consumes_handoff: true,
        every_object_class_handed_off_by_two_or_more_consumers: true,
        historical_grammar_identical_for_same_profile: true,
        blocking_disclosed_not_hidden: true,
        handoff_maps_back_to_one_historical_reference_object: true,
    }
}

fn proof_freshness() -> LiveTargetHandoffProofFreshness {
    LiveTargetHandoffProofFreshness {
        proof_freshness_slo_hours: M5_LIVE_TARGET_HANDOFF_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_LIVE_TARGET_HANDOFF_SCHEMA_REF.to_owned(),
        M5_LIVE_TARGET_HANDOFF_DOC_REF.to_owned(),
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
    handoff_bindings: Vec<LiveTargetHandoffBinding>,
) -> M5LiveTargetHandoffPacket {
    M5LiveTargetHandoffPacket::new(M5LiveTargetHandoffPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        handoff_bindings,
        downgrade_triggers: LiveTargetHandoffDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5HistoricalReferenceConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in live-target-handoff packet.
pub fn seeded_m5_live_target_handoff() -> M5LiveTargetHandoffPacket {
    packet_from_bindings(
        M5_LIVE_TARGET_HANDOFF_PACKET_ID,
        "M5 live-target handoff packets (one validation across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more cleared handoffs narrowed to a blocked-target-unavailable outcome,
/// proving the historical grammar survives and the user still inspects the historical packet when the target
/// is gone.
pub fn seeded_m5_live_target_handoff_blocked_target_narrowed() -> M5LiveTargetHandoffPacket {
    packet_from_bindings(
        "m5-live-target-handoff:blocked-target:0001",
        "M5 live-target handoff packets (blocked-target narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "lth-support-evidence-support" => HandoffOutcome::BlockedTargetUnavailable,
            "lth-runbook-archive" => HandoffOutcome::BlockedTargetUnavailable,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more cleared handoffs narrowed to a blocked-needs-prerequisite outcome,
/// proving the handoff never completes while a route / trust / auth prerequisite is unmet and instead offers a
/// satisfy-prerequisite-then-retry fallback.
pub fn seeded_m5_live_target_handoff_needs_prerequisite_narrowed() -> M5LiveTargetHandoffPacket {
    packet_from_bindings(
        "m5-live-target-handoff:needs-prerequisite:0001",
        "M5 live-target handoff packets (needs-prerequisite narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "lth-review-review" => HandoffOutcome::BlockedNeedsPrerequisite,
            "lth-retirement-release" => HandoffOutcome::BlockedNeedsPrerequisite,
            _ => default,
        }),
    )
}
