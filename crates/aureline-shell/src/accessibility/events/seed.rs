//! Canonical seed builders for the M5 event-class coverage catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code coverage, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical event-coverage catalog.
pub const M5_EVENT_COVERAGE_CATALOG_PACKET_ID: &str = "m5-event-coverage:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

/// Proof packet ref every governed family carries.
const COVERAGE_PROOF_REF: &str = "evidence:event-coverage-conformance:m5";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn fallback(surface: M5DurableFallbackSurface, surface_ref: &str) -> M5DurableFallbackRef {
    M5DurableFallbackRef {
        surface,
        surface_ref: surface_ref.to_owned(),
        reopenable: true,
    }
}

/// Builds a normal (non-degraded) meaning-changing event.
fn event(
    event_id: &str,
    label: &str,
    announcement_event_class: M5AnnouncementEventClass,
    identity_message_id: &str,
    durable_fallback: M5DurableFallbackRef,
) -> M5DynamicEventMapping {
    M5DynamicEventMapping {
        event_id: event_id.to_owned(),
        label: label.to_owned(),
        meaning_changing: true,
        announcement_event_class,
        identity_message_id: identity_message_id.to_owned(),
        degraded_disclosure: M5EventDegradedDisclosure {
            announces_reason: false,
            reason_class: M5EventReasonClass::NotApplicable,
        },
        durable_fallback,
    }
}

/// Builds a blocked/degraded event that discloses a reason. The announcement class is
/// derived from the reason so the channel rule is honored by construction.
fn degraded_event(
    event_id: &str,
    label: &str,
    reason_class: M5EventReasonClass,
    identity_message_id: &str,
    durable_fallback: M5DurableFallbackRef,
) -> M5DynamicEventMapping {
    let announcement_event_class = reason_class
        .required_announcement_class()
        .expect("a degraded event has a required announcement class");
    M5DynamicEventMapping {
        event_id: event_id.to_owned(),
        label: label.to_owned(),
        meaning_changing: true,
        announcement_event_class,
        identity_message_id: identity_message_id.to_owned(),
        degraded_disclosure: M5EventDegradedDisclosure {
            announces_reason: true,
            reason_class,
        },
        durable_fallback,
    }
}

/// The downgrade triggers every governed family carries: both bridge degradation
/// paths, stale proof, lost announcement meaning, and lost non-visual fidelity.
fn standard_downgrade_triggers() -> Vec<M5DynamicSurfaceA11yDowngradeTrigger> {
    use M5DynamicSurfaceA11yDowngradeTrigger as D;
    vec![
        D::ProofStale,
        D::BridgeUnavailable,
        D::BridgePartialOrStale,
        D::AnnouncementMeaningLost,
        D::NonVisualFidelityLost,
    ]
}

#[allow(clippy::too_many_arguments)]
fn family(
    family_id: &str,
    family: M5EventFamily,
    label: &str,
    producers: Vec<M5EventProducer>,
    events: Vec<M5DynamicEventMapping>,
    consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
) -> M5EventFamilyCoverage {
    M5EventFamilyCoverage {
        family_id: family_id.to_owned(),
        family,
        label: label.to_owned(),
        owner_role: "Accessibility owner".to_owned(),
        qualification: M5DynamicSurfaceA11yQualificationClass::Stable,
        non_visual_fidelity: A11yNonVisualFidelity::FullAccessible,
        producers,
        events,
        downgrade_triggers: standard_downgrade_triggers(),
        required_proof_packet_refs: strings(&[COVERAGE_PROOF_REF]),
        source_contract_refs: strings(&[
            M5_EVENT_COVERAGE_ANNOUNCEMENT_GRAMMAR_REF,
            M5_EVENT_COVERAGE_SCREEN_READER_CONTRACT_REF,
        ]),
        consumer_surfaces,
    }
}

fn families() -> Vec<M5EventFamilyCoverage> {
    use M5AnnouncementEventClass as Class;
    use M5DurableFallbackSurface as Surface;
    use M5DynamicSurfaceA11yConsumerSurface as Consumer;
    use M5EventProducer as Producer;
    use M5EventReasonClass as Reason;

    vec![
        // Diagnostics: problems published / cleared narrate as state changes; a
        // blocking error interrupts assertively with its reason.
        family(
            "event-family:diagnostics",
            M5EventFamily::Diagnostics,
            "Diagnostics",
            vec![Producer::Editor, Producer::Notebook],
            vec![
                event(
                    "event:diagnostics.published",
                    "Diagnostics published or updated",
                    Class::ModeOrStateChange,
                    "event.diagnostics.published",
                    fallback(Surface::ActivityRow, "activity-row:problems"),
                ),
                event(
                    "event:diagnostics.cleared",
                    "Diagnostics cleared",
                    Class::ModeOrStateChange,
                    "event.diagnostics.cleared",
                    fallback(Surface::StatusDetail, "status-detail:problems"),
                ),
                degraded_event(
                    "event:diagnostics.blocking-error",
                    "Blocking diagnostic error",
                    Reason::Blocked,
                    "event.diagnostics.blocking_error",
                    fallback(Surface::BannerDetail, "banner-detail:blocking-error"),
                ),
            ],
            vec![Consumer::Editor, Consumer::Notebook, Consumer::SupportExport],
        ),
        // Completion / snippet / editor-assist session changes.
        family(
            "event-family:completion-and-session",
            M5EventFamily::CompletionAndSession,
            "Completion and assist sessions",
            vec![Producer::Editor],
            vec![
                event(
                    "event:completion.list-opened",
                    "Completion list opened",
                    Class::SelectionOrContextChange,
                    "event.completion.list_opened",
                    fallback(Surface::SelectionSummary, "selection-summary:completion"),
                ),
                event(
                    "event:snippet.session-entered",
                    "Snippet session entered",
                    Class::ModeOrStateChange,
                    "event.snippet.session_entered",
                    fallback(Surface::StatusDetail, "status-detail:snippet-session"),
                ),
                degraded_event(
                    "event:assist.unavailable",
                    "Editor assist unavailable",
                    Reason::Unavailable,
                    "event.assist.unavailable",
                    fallback(Surface::NotificationCenterEntry, "notification-center:assist"),
                ),
            ],
            vec![Consumer::Editor, Consumer::SupportExport],
        ),
        // Run / debug / test transitions.
        family(
            "event-family:run-debug-test",
            M5EventFamily::RunDebugTest,
            "Run, debug, and test state",
            vec![Producer::Debug, Producer::Notebook],
            vec![
                event(
                    "event:run.started",
                    "Run started",
                    Class::ProgressMilestone,
                    "event.run.started",
                    fallback(Surface::RunHeader, "run-header:active-run"),
                ),
                event(
                    "event:run.completed",
                    "Run completed",
                    Class::ModeOrStateChange,
                    "event.run.completed",
                    fallback(Surface::RunHeader, "run-header:result"),
                ),
                event(
                    "event:debug.paused",
                    "Debugger paused at breakpoint",
                    Class::ModeOrStateChange,
                    "event.debug.paused",
                    fallback(Surface::StatusDetail, "status-detail:debug-paused"),
                ),
                degraded_event(
                    "event:run.blocked",
                    "Run blocked before start",
                    Reason::Blocked,
                    "event.run.blocked",
                    fallback(Surface::BannerDetail, "banner-detail:run-blocked"),
                ),
            ],
            vec![
                Consumer::Editor,
                Consumer::Notebook,
                Consumer::SupportExport,
            ],
        ),
        // Terminal command boundaries (where shell integration allows).
        family(
            "event-family:terminal-boundary",
            M5EventFamily::TerminalBoundary,
            "Terminal command boundaries",
            vec![Producer::Terminal],
            vec![
                event(
                    "event:terminal.command-started",
                    "Terminal command started",
                    Class::ModeOrStateChange,
                    "event.terminal.command_started",
                    fallback(Surface::StatusDetail, "status-detail:terminal-command"),
                ),
                event(
                    "event:terminal.command-exited",
                    "Terminal command exited",
                    Class::ModeOrStateChange,
                    "event.terminal.command_exited",
                    fallback(Surface::ActivityRow, "activity-row:terminal-history"),
                ),
                degraded_event(
                    "event:terminal.boundary-unavailable",
                    "Command boundaries unavailable",
                    Reason::Unavailable,
                    "event.terminal.boundary_unavailable",
                    fallback(
                        Surface::NotificationCenterEntry,
                        "notification-center:terminal-boundary",
                    ),
                ),
            ],
            vec![Consumer::Terminal, Consumer::SupportExport],
        ),
        // Collaboration control and recording changes.
        family(
            "event-family:collaboration-control",
            M5EventFamily::CollaborationControl,
            "Collaboration control and recording",
            vec![Producer::Collab],
            vec![
                event(
                    "event:collab.role-changed",
                    "Collaboration role changed",
                    Class::ModeOrStateChange,
                    "event.collab.role_changed",
                    fallback(Surface::StatusDetail, "status-detail:collab-role"),
                ),
                event(
                    "event:collab.recording-changed",
                    "Session recording changed",
                    Class::ModeOrStateChange,
                    "event.collab.recording_changed",
                    fallback(Surface::BannerDetail, "banner-detail:collab-recording"),
                ),
                degraded_event(
                    "event:collab.control-restricted",
                    "Collaboration control restricted",
                    Reason::PolicyRestricted,
                    "event.collab.control_restricted",
                    fallback(Surface::BannerDetail, "banner-detail:collab-restricted"),
                ),
            ],
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        // AI patch / review milestone states.
        family(
            "event-family:ai-patch-review",
            M5EventFamily::AiPatchReview,
            "AI patch and review milestones",
            vec![Producer::Ai, Producer::Review],
            vec![
                event(
                    "event:ai.generation-started",
                    "AI generation started",
                    Class::ProgressMilestone,
                    "event.ai.generation_started",
                    fallback(Surface::ActivityRow, "activity-row:ai-generation"),
                ),
                event(
                    "event:ai.patch-proposed",
                    "AI patch proposed",
                    Class::ModeOrStateChange,
                    "event.ai.patch_proposed",
                    fallback(Surface::PatchReviewHeader, "patch-review-header:ai-patch"),
                ),
                event(
                    "event:review.milestone-reached",
                    "Review milestone reached",
                    Class::ProgressMilestone,
                    "event.review.milestone_reached",
                    fallback(Surface::PatchReviewHeader, "patch-review-header:review"),
                ),
                degraded_event(
                    "event:ai.generation-blocked",
                    "AI generation blocked",
                    Reason::Blocked,
                    "event.ai.generation_blocked",
                    fallback(Surface::BannerDetail, "banner-detail:ai-blocked"),
                ),
            ],
            vec![
                Consumer::AiSurfaces,
                Consumer::Review,
                Consumer::SupportExport,
            ],
        ),
        // Stale / degraded truth transitions aggregated across surfaces.
        family(
            "event-family:stale-degraded-truth",
            M5EventFamily::StaleDegradedTruth,
            "Stale and degraded truth",
            vec![Producer::Shell],
            vec![
                degraded_event(
                    "event:truth.went-stale",
                    "Truth went stale",
                    Reason::Stale,
                    "event.truth.went_stale",
                    fallback(
                        Surface::NotificationCenterEntry,
                        "notification-center:stale-truth",
                    ),
                ),
                degraded_event(
                    "event:truth.bridge-degraded",
                    "Bridge degraded",
                    Reason::Degraded,
                    "event.truth.bridge_degraded",
                    fallback(
                        Surface::NotificationCenterEntry,
                        "notification-center:bridge-degraded",
                    ),
                ),
                event(
                    "event:truth.refreshed",
                    "Truth refreshed to current",
                    Class::SuccessWithRecovery,
                    "event.truth.refreshed",
                    fallback(Surface::ActivityRow, "activity-row:truth-refreshed"),
                ),
            ],
            vec![Consumer::Shell, Consumer::Help, Consumer::SupportExport],
        ),
    ]
}

fn conformance_review() -> M5EventCoverageConformanceReview {
    M5EventCoverageConformanceReview {
        workflows_narrate_transitions_without_visual_only_cues: true,
        each_family_announces_identity_plus_blocked_or_degraded_reason: true,
        events_route_through_one_announcement_grammar_not_per_surface_prose: true,
        durable_fallback_preserves_event_identity_and_state_labels: true,
        only_meaning_changing_events_enter_assistive_channel: true,
        support_export_can_reconstruct_what_user_should_have_been_told: true,
        claimed_families_auto_narrow_when_bridge_or_proof_stale: true,
        downgrade_narrows_instead_of_hides: true,
        no_visual_only_or_pointer_only_event_source: true,
    }
}

fn consumer_projection() -> M5EventCoverageConsumerProjection {
    M5EventCoverageConsumerProjection {
        editor_routes_diagnostics_and_assist: true,
        terminal_routes_command_boundaries: true,
        debug_and_test_route_transitions: true,
        review_and_ai_route_milestones: true,
        collaboration_routes_control_changes: true,
        notebook_routes_session_changes: true,
        shell_routes_stale_degraded_truth: true,
        support_export_reuses_coverage: true,
        docs_help_reuse_coverage: true,
        at_conformance_packets_reuse_coverage: true,
    }
}

fn proof_freshness() -> M5DynamicSurfaceA11yProofFreshness {
    M5DynamicSurfaceA11yProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DynamicSurfaceA11yReleasePosture {
    M5DynamicSurfaceA11yReleasePosture {
        release_packet_ref: "evidence:event-coverage-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:event-coverage-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EVENT_COVERAGE_SCHEMA_REF,
        M5_EVENT_COVERAGE_DOC_REF,
        M5_EVENT_COVERAGE_MATRIX_REF,
        M5_EVENT_COVERAGE_ANNOUNCEMENT_GRAMMAR_REF,
        M5_EVENT_COVERAGE_SURFACE_DESCRIPTOR_REF,
        M5_EVENT_COVERAGE_SCREEN_READER_CONTRACT_REF,
    ])
}

fn base_input() -> M5EventCoverageCatalogPacketInput {
    M5EventCoverageCatalogPacketInput {
        packet_id: M5_EVENT_COVERAGE_CATALOG_PACKET_ID.to_owned(),
        catalog_label: "M5 Event-Class Non-Visual Coverage".to_owned(),
        families: families(),
        shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        announcement_vocabulary_set: M5AnnouncementGrammarVocabularySet::canonical(),
        coverage_vocabulary_set: M5EventCoverageVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable event-coverage catalog packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_event_coverage_catalog() -> M5EventCoverageCatalogPacket {
    M5EventCoverageCatalogPacket::new(base_input())
}

/// Builds a narrowed variant where the AI/patch-review family's assistive-tech proof
/// has gone stale, proving the family narrows from Stable to Beta while keeping its
/// events, identities, durable fallbacks, and `proof_stale` trigger intact.
pub fn seeded_m5_event_coverage_catalog_proof_stale_narrowed() -> M5EventCoverageCatalogPacket {
    let mut input = base_input();
    input.packet_id = "m5-event-coverage:proof-stale-narrowed:0001".to_owned();
    for family in &mut input.families {
        if family.family == M5EventFamily::AiPatchReview {
            family.qualification = M5DynamicSurfaceA11yQualificationClass::Beta;
        }
    }
    M5EventCoverageCatalogPacket::new(input)
}

/// Builds a narrowed variant where the terminal-boundary family's OS bridge is
/// unavailable, proving the family narrows from Stable to Preview and drops its
/// non-visual fidelity to `degraded_accessible` while keeping its
/// `boundary-unavailable` event, durable fallbacks, and `bridge_unavailable` trigger —
/// the boundaries still narrate their unavailable reason rather than disappearing.
pub fn seeded_m5_event_coverage_catalog_bridge_unavailable_narrowed() -> M5EventCoverageCatalogPacket
{
    let mut input = base_input();
    input.packet_id = "m5-event-coverage:bridge-unavailable-narrowed:0001".to_owned();
    for family in &mut input.families {
        if family.family == M5EventFamily::TerminalBoundary {
            family.qualification = M5DynamicSurfaceA11yQualificationClass::Preview;
            family.non_visual_fidelity = A11yNonVisualFidelity::DegradedAccessible;
        }
    }
    M5EventCoverageCatalogPacket::new(input)
}
