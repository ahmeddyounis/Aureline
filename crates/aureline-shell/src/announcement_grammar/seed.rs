//! Canonical seed builders for the M5 live-announcement grammar catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so
//! the in-code grammar, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical grammar catalog.
pub const M5_ANNOUNCEMENT_GRAMMAR_CATALOG_PACKET_ID: &str =
    "m5-live-announcement-grammar:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn placeholder(
    name: &str,
    value_kind: M5AnnouncementValueKind,
    required: bool,
) -> M5AnnouncementPlaceholder {
    M5AnnouncementPlaceholder {
        name: name.to_owned(),
        value_kind,
        required,
    }
}

fn template(
    message_id: &str,
    template: &str,
    placeholders: Vec<M5AnnouncementPlaceholder>,
) -> M5AnnouncementMessageTemplate {
    M5AnnouncementMessageTemplate {
        message_id: message_id.to_owned(),
        template: template.to_owned(),
        placeholders,
    }
}

fn budget(
    strategy: A11yCoalescingStrategy,
    max_announcements_per_window: u32,
    window_seconds: u32,
    min_interval_ms: u32,
) -> M5CoalescingBudget {
    M5CoalescingBudget {
        strategy,
        max_announcements_per_window,
        window_seconds,
        min_interval_ms,
        suppress_unchanged_meaning: true,
    }
}

fn fallback(surface: M5DurableFallbackSurface, surface_ref: &str) -> M5DurableFallbackRef {
    M5DurableFallbackRef {
        surface,
        surface_ref: surface_ref.to_owned(),
        reopenable: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn class(
    class_id: &str,
    event_class: M5AnnouncementEventClass,
    label: &str,
    qualification: M5DynamicSurfaceA11yQualificationClass,
    channel: A11yAnnouncementPoliteness,
    message_template: M5AnnouncementMessageTemplate,
    required_fields: &[&str],
    coalescing_budget: M5CoalescingBudget,
    suppression_rules: Vec<M5AnnouncementSuppressionRule>,
    fallback_durability: A11yFallbackDurability,
    durable_fallback: M5DurableFallbackRef,
    downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    required_proof_packet_refs: &[&str],
    source_contract_refs: &[&str],
    consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
) -> M5AnnouncementGrammarClass {
    M5AnnouncementGrammarClass {
        class_id: class_id.to_owned(),
        event_class,
        label: label.to_owned(),
        owner_role: "Accessibility owner".to_owned(),
        qualification,
        channel,
        message_template,
        required_fields: strings(required_fields),
        coalescing_budget,
        suppression_rules,
        fallback_durability,
        durable_fallback,
        downgrade_triggers,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        source_contract_refs: strings(source_contract_refs),
        consumer_surfaces,
    }
}

/// The downgrade triggers every governed announcement class carries: both bridge
/// degradation paths, the two live-region failure modes, and stale proof.
fn standard_downgrade_triggers() -> Vec<M5DynamicSurfaceA11yDowngradeTrigger> {
    use M5DynamicSurfaceA11yDowngradeTrigger as D;
    vec![
        D::BridgePartialOrStale,
        D::BridgeUnavailable,
        D::LiveRegionSpam,
        D::AnnouncementMeaningLost,
        D::ProofStale,
    ]
}

fn classes() -> Vec<M5AnnouncementGrammarClass> {
    use A11yAnnouncementPoliteness as Channel;
    use A11yCoalescingStrategy as Strategy;
    use A11yFallbackDurability as Durability;
    use M5AnnouncementSuppressionRule as Suppress;
    use M5AnnouncementValueKind as Kind;
    use M5DurableFallbackSurface as Surface;
    use M5DynamicSurfaceA11yConsumerSurface as Consumer;
    use M5DynamicSurfaceA11yQualificationClass::Stable;

    vec![
        // Mode / state change: polite, deduped, reopenable on the status detail.
        class(
            "announcement:mode-or-state-change",
            M5AnnouncementEventClass::ModeOrStateChange,
            "Mode or state change",
            Stable,
            Channel::Polite,
            template(
                "announcement.mode_or_state_change.entered",
                "{surface_name} entered {mode_name} mode.",
                vec![
                    placeholder("surface_name", Kind::SurfaceName, true),
                    placeholder("mode_name", Kind::ModeName, true),
                ],
            ),
            &["surface_name", "mode_name"],
            budget(Strategy::DedupeSameMeaning, 4, 5, 250),
            vec![
                Suppress::SuppressUnchangedMeaning,
                Suppress::SuppressRepaintOnlyTicks,
            ],
            Durability::OnFocus,
            fallback(Surface::StatusDetail, "status-detail:mode-strip"),
            standard_downgrade_triggers(),
            &["evidence:live-announcement-class-conformance:m5"],
            &[
                M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
                M5_ANNOUNCEMENT_GRAMMAR_MESSAGE_ID_CONTRACT_REF,
            ],
            vec![Consumer::Shell, Consumer::Editor, Consumer::SupportExport],
        ),
        // Blocker: assertive interruption, deduped, reopenable on a banner detail.
        class(
            "announcement:blocker-raised",
            M5AnnouncementEventClass::BlockerRaised,
            "Blocker raised",
            Stable,
            Channel::Assertive,
            template(
                "announcement.blocker_raised.disclosed",
                "{severity_label}: {state_name} blocks {surface_name}.",
                vec![
                    placeholder("severity_label", Kind::SeverityLabel, true),
                    placeholder("state_name", Kind::StateName, true),
                    placeholder("surface_name", Kind::SurfaceName, true),
                ],
            ),
            &["severity_label", "state_name", "surface_name"],
            budget(Strategy::DedupeSameMeaning, 3, 10, 0),
            vec![
                Suppress::SuppressDuplicateWithinWindow,
                Suppress::SuppressUnchangedMeaning,
            ],
            Durability::Immediate,
            fallback(Surface::BannerDetail, "banner-detail:blocker"),
            standard_downgrade_triggers(),
            &["evidence:live-announcement-class-conformance:m5"],
            &[
                M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
                M5_ANNOUNCEMENT_GRAMMAR_MESSAGE_ID_CONTRACT_REF,
            ],
            vec![Consumer::Shell, Consumer::Editor, Consumer::SupportExport],
        ),
        // Progress milestone: polite, start-and-terminal only, reopenable on the run header.
        class(
            "announcement:progress-milestone",
            M5AnnouncementEventClass::ProgressMilestone,
            "Progress milestone",
            Stable,
            Channel::Polite,
            template(
                "announcement.progress_milestone.reached",
                "{surface_name}: {state_name} at {count}.",
                vec![
                    placeholder("surface_name", Kind::SurfaceName, true),
                    placeholder("state_name", Kind::StateName, true),
                    placeholder("count", Kind::Count, true),
                ],
            ),
            &["surface_name", "state_name", "count"],
            budget(Strategy::StartAndTerminalOnly, 3, 30, 2000),
            vec![
                Suppress::SuppressLowValueProgressMidpoints,
                Suppress::SuppressRepaintOnlyTicks,
                Suppress::SuppressDuplicateWithinWindow,
            ],
            Durability::Coalesced,
            fallback(Surface::RunHeader, "run-header:active-run"),
            standard_downgrade_triggers(),
            &["evidence:live-announcement-class-conformance:m5"],
            &[
                M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
                M5_ANNOUNCEMENT_GRAMMAR_COLLECTION_CONTRACT_REF,
            ],
            vec![
                Consumer::Terminal,
                Consumer::Notebook,
                Consumer::SupportExport,
            ],
        ),
        // Selection / context change: polite, last-meaning-wins, reopenable on the selection summary.
        class(
            "announcement:selection-or-context-change",
            M5AnnouncementEventClass::SelectionOrContextChange,
            "Selection or context change",
            Stable,
            Channel::Polite,
            template(
                "announcement.selection_or_context_change.updated",
                "{count} selected in {surface_name}.",
                vec![
                    placeholder("count", Kind::Count, true),
                    placeholder("surface_name", Kind::SurfaceName, true),
                ],
            ),
            &["count", "surface_name"],
            budget(Strategy::LastMeaningWinsWithCount, 5, 5, 200),
            vec![
                Suppress::SuppressUnchangedMeaning,
                Suppress::SuppressDuplicateWithinWindow,
            ],
            Durability::OnFocus,
            fallback(Surface::SelectionSummary, "selection-summary:active"),
            standard_downgrade_triggers(),
            &["evidence:live-announcement-class-conformance:m5"],
            &[
                M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
                M5_ANNOUNCEMENT_GRAMMAR_COLLECTION_CONTRACT_REF,
            ],
            vec![
                Consumer::DataGrid,
                Consumer::Review,
                Consumer::SupportExport,
            ],
        ),
        // Success with recovery: polite, deduped, reopenable on an activity row.
        class(
            "announcement:success-with-recovery",
            M5AnnouncementEventClass::SuccessWithRecovery,
            "Success with recovery",
            Stable,
            Channel::Polite,
            template(
                "announcement.success_with_recovery.completed",
                "{state_name} succeeded; {recovery_label}.",
                vec![
                    placeholder("state_name", Kind::StateName, true),
                    placeholder("recovery_label", Kind::RecoveryLabel, true),
                ],
            ),
            &["state_name", "recovery_label"],
            budget(Strategy::DedupeSameMeaning, 2, 10, 500),
            vec![
                Suppress::SuppressUnchangedMeaning,
                Suppress::SuppressWhenDurableSurfaceVisible,
            ],
            Durability::DurableSurfaceOnly,
            fallback(Surface::ActivityRow, "activity-row:recovery"),
            standard_downgrade_triggers(),
            &["evidence:live-announcement-class-conformance:m5"],
            &[
                M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
                M5_ANNOUNCEMENT_GRAMMAR_MESSAGE_ID_CONTRACT_REF,
            ],
            vec![Consumer::Shell, Consumer::Review, Consumer::SupportExport],
        ),
        // Degraded / stale truth: polite, deduped, heavy background suppression,
        // reopenable on a durable notification entry.
        class(
            "announcement:degraded-or-stale-truth",
            M5AnnouncementEventClass::DegradedOrStaleTruth,
            "Degraded or stale truth",
            Stable,
            Channel::Polite,
            template(
                "announcement.degraded_or_stale_truth.disclosed",
                "{surface_name} is {freshness_label}.",
                vec![
                    placeholder("surface_name", Kind::SurfaceName, true),
                    placeholder("freshness_label", Kind::FreshnessLabel, true),
                ],
            ),
            &["surface_name", "freshness_label"],
            budget(Strategy::DedupeSameMeaning, 2, 60, 5000),
            vec![
                Suppress::SuppressBackgroundRefreshWhenUnfocused,
                Suppress::SuppressDuplicateWithinWindow,
                Suppress::SuppressUnchangedMeaning,
            ],
            Durability::DurableSurfaceOnly,
            fallback(
                Surface::NotificationCenterEntry,
                "notification-center:stale-truth",
            ),
            standard_downgrade_triggers(),
            &["evidence:live-announcement-class-conformance:m5"],
            &[
                M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
                M5_ANNOUNCEMENT_GRAMMAR_MESSAGE_ID_CONTRACT_REF,
            ],
            vec![Consumer::Shell, Consumer::Help, Consumer::SupportExport],
        ),
    ]
}

fn conformance_review() -> M5AnnouncementGrammarConformanceReview {
    M5AnnouncementGrammarConformanceReview {
        one_governed_grammar_not_per_surface_prose: true,
        stable_message_ids_with_placeholders_not_concatenated_fragments: true,
        polite_assertive_channel_rules_enforced: true,
        coalescing_budgets_bound_repeated_narration: true,
        repeated_polls_and_refreshes_do_not_flood_live_region: true,
        every_high_value_announcement_has_durable_fallback: true,
        narrated_state_points_back_to_durable_surface: true,
        announcements_convey_meaning_not_repaint_noise: true,
        claimed_classes_auto_narrow_when_bridge_or_proof_stale: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> M5AnnouncementGrammarConsumerProjection {
    M5AnnouncementGrammarConsumerProjection {
        shell_consumes_grammar: true,
        editor_consumes_grammar: true,
        terminal_consumes_grammar: true,
        notebook_consumes_grammar: true,
        data_grid_consumes_grammar: true,
        review_consumes_grammar: true,
        notifications_consume_grammar: true,
        help_documents_grammar: true,
        support_export_reuses_grammar: true,
        at_conformance_packets_reuse_grammar: true,
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
        release_packet_ref: "evidence:announcement-grammar-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:announcement-grammar-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ANNOUNCEMENT_GRAMMAR_SCHEMA_REF,
        M5_ANNOUNCEMENT_GRAMMAR_DOC_REF,
        M5_ANNOUNCEMENT_GRAMMAR_MATRIX_REF,
        M5_ANNOUNCEMENT_GRAMMAR_SCREEN_READER_CONTRACT_REF,
        M5_ANNOUNCEMENT_GRAMMAR_COLLECTION_CONTRACT_REF,
        M5_ANNOUNCEMENT_GRAMMAR_MESSAGE_ID_CONTRACT_REF,
    ])
}

fn base_input() -> M5AnnouncementGrammarCatalogPacketInput {
    M5AnnouncementGrammarCatalogPacketInput {
        packet_id: M5_ANNOUNCEMENT_GRAMMAR_CATALOG_PACKET_ID.to_owned(),
        catalog_label: "M5 Live-Announcement Grammar".to_owned(),
        classes: classes(),
        shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        grammar_vocabulary_set: M5AnnouncementGrammarVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable announcement-grammar catalog packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_announcement_grammar_catalog() -> M5AnnouncementGrammarCatalogPacket {
    M5AnnouncementGrammarCatalogPacket::new(base_input())
}

/// Builds a narrowed variant where the success-with-recovery class's
/// assistive-tech proof has gone stale, proving the class narrows from Stable to
/// Beta and keeps its proof-stale trigger and durable fallback intact.
pub fn seeded_m5_announcement_grammar_catalog_proof_stale_narrowed(
) -> M5AnnouncementGrammarCatalogPacket {
    let mut input = base_input();
    input.packet_id = "m5-live-announcement-grammar:proof-stale-narrowed:0001".to_owned();
    for class in &mut input.classes {
        if class.event_class == M5AnnouncementEventClass::SuccessWithRecovery {
            class.qualification = M5DynamicSurfaceA11yQualificationClass::Beta;
        }
    }
    M5AnnouncementGrammarCatalogPacket::new(input)
}

/// Builds a narrowed variant where the progress-milestone class's OS live region
/// is unavailable, proving the class narrows from Stable to Preview, shifts its
/// delivery to a durable surface, and keeps its `bridge_unavailable` trigger — the
/// announcement still has a durable counterpart even with no live region.
pub fn seeded_m5_announcement_grammar_catalog_live_region_unavailable_narrowed(
) -> M5AnnouncementGrammarCatalogPacket {
    let mut input = base_input();
    input.packet_id =
        "m5-live-announcement-grammar:live-region-unavailable-narrowed:0001".to_owned();
    for class in &mut input.classes {
        if class.event_class == M5AnnouncementEventClass::ProgressMilestone {
            class.qualification = M5DynamicSurfaceA11yQualificationClass::Preview;
            class.fallback_durability = A11yFallbackDurability::DurableSurfaceOnly;
        }
    }
    M5AnnouncementGrammarCatalogPacket::new(input)
}
