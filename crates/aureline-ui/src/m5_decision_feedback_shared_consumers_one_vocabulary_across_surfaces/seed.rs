//! Canonical seed for the decision-feedback shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export,
//! matrix CSV, Markdown summary, and narrowed fixtures. Every binding is derived from one
//! per-object [`DecisionFeedbackStateFacetValues`] so the same primitive object always
//! carries the same vocabulary across surfaces, and every narrowed representation derives
//! its disclosure from [`resolve_decision_feedback_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    disposition: &str,
    scope: &str,
    severity: &str,
    rationale: &str,
    recovery_path: &str,
    durable_object: &str,
) -> DecisionFeedbackStateFacetValues {
    DecisionFeedbackStateFacetValues {
        disposition_word: disposition.to_owned(),
        scope_word: scope.to_owned(),
        severity_word: severity.to_owned(),
        rationale_word: rationale.to_owned(),
        recovery_path_word: recovery_path.to_owned(),
        durable_object_word: durable_object.to_owned(),
    }
}

fn preserved_note_for(reason: DecisionFeedbackNarrowReason) -> String {
    match reason {
        DecisionFeedbackNarrowReason::CompactionNarrowed => {
            "disposition, scope, severity, rationale, recovery-path, and durable-object words preserved; only disclosure depth compacted"
        }
        DecisionFeedbackNarrowReason::RemoteProjectionNarrowed => {
            "all decision-feedback vocabulary preserved; the primitive is projected from the remote source of truth"
        }
        DecisionFeedbackNarrowReason::ExportRedactionNarrowed => {
            "all decision-feedback vocabulary preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: DecisionFeedbackNarrowNextAction) -> String {
    match action {
        DecisionFeedbackNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        DecisionFeedbackNarrowNextAction::OpenRemoteSource => "Open the remote source",
        DecisionFeedbackNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(component: M5DecisionFeedbackFamily) -> Vec<String> {
    vec![
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF.to_owned(),
        component.canonical_component_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    primitive_object_id: &str,
    primitive_object_label: &str,
    component: M5DecisionFeedbackFamily,
    consumer: M5DecisionFeedbackConsumerSurface,
    representation: DecisionFeedbackRepresentation,
    state_facets: DecisionFeedbackStateFacetValues,
) -> DecisionFeedbackConsumerBinding {
    let disclosure = resolve_decision_feedback_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        DecisionFeedbackNarrowNote {
            reason,
            preserved_vocabulary_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let remote_source_note = if disclosure.needs_remote_source_note {
        "projected from the remote source of truth; the source stays remote".to_owned()
    } else {
        String::new()
    };
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    DecisionFeedbackConsumerBinding {
        binding_id: binding_id.to_owned(),
        primitive_object_id: primitive_object_id.to_owned(),
        primitive_object_label: primitive_object_label.to_owned(),
        component,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        relies_on_color_alone_for_meaning: false,
        lets_a_popover_carry_the_only_critical_instruction: false,
        uses_generic_yes_no_confirmation_copy: false,
        represents_durable_work_as_toast_only_truth: false,
        blanks_a_useful_pane_during_loading: false,
        uses_a_full_screen_spinner_where_partial_capable: false,
        source_contract_refs: binding_refs(component),
    }
}

/// One consumer-surface adoption of a primitive object, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5DecisionFeedbackConsumerSurface,
    representation: DecisionFeedbackRepresentation,
}

/// One primitive object rendered across several consumer surfaces at one vocabulary.
struct ObjectSpec {
    object_id: &'static str,
    object_label: &'static str,
    component: M5DecisionFeedbackFamily,
    facets: DecisionFeedbackStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    object_id: &'static str,
    object_label: &'static str,
    component: M5DecisionFeedbackFamily,
    facets: DecisionFeedbackStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> ObjectSpec {
    ObjectSpec {
        object_id,
        object_label,
        component,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5DecisionFeedbackConsumerSurface,
    representation: DecisionFeedbackRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The eight primitive objects — one per B135 decision-feedback family — and the surfaces
/// that adopt each, drawn from the first claimed shell, help, entry, trust/repair,
/// update/advisory, provider/account, and export/support consumers.
fn object_specs() -> Vec<ObjectSpec> {
    use DecisionFeedbackRepresentation::*;
    use M5DecisionFeedbackConsumerSurface::*;
    use M5DecisionFeedbackFamily::*;

    vec![
        spec(
            "provider/account-trust-badge",
            "Provider account trust badge",
            BadgeChipPill,
            facets(
                "info",
                "provider_account",
                "informational",
                "provider_verified",
                "review_provider",
                "provider_account_record",
            ),
            vec![
                bs("cc-badge-settings", SettingsUi, DesktopFull),
                bs("cc-badge-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "help/keyboard-shortcut-popover",
            "Help keyboard-shortcut popover",
            Popover,
            facets(
                "info",
                "help_shortcuts",
                "informational",
                "shortcut_reference",
                "open_help_page",
                "help_topic_record",
            ),
            vec![
                bs("cc-popover-help", HelpUi, DesktopFull),
                bs("cc-popover-review", ReviewUi, CompactNarrowed),
            ],
        ),
        spec(
            "repair/confirm-destructive-dialog",
            "Repair confirm-destructive dialog",
            DialogSheet,
            facets(
                "warning",
                "repair_destructive",
                "high",
                "irreversible_repair",
                "cancel_or_review",
                "repair_task_record",
            ),
            vec![
                bs("cc-dialog-review", ReviewUi, DesktopFull),
                bs("cc-dialog-product", ProductUi, CompactNarrowed),
                bs("cc-dialog-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "updates/advisory-banner",
            "Updates advisory banner",
            BannerInlineNotice,
            facets(
                "warning",
                "update_advisory",
                "elevated",
                "advisory_published",
                "open_updates",
                "advisory_record",
            ),
            vec![
                bs("cc-banner-updates", UpdatesUi, DesktopFull),
                bs("cc-banner-shell", ShellUi, RemoteProjected),
            ],
        ),
        spec(
            "settings/saved-toast",
            "Settings saved toast",
            Toast,
            facets(
                "success",
                "settings_save",
                "informational",
                "changes_applied",
                "open_settings_record",
                "settings_change_record",
            ),
            vec![
                bs("cc-toast-settings", SettingsUi, DesktopFull),
                bs("cc-toast-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "review/empty-queue-state",
            "Review empty-queue state",
            EmptyState,
            facets(
                "info",
                "review_queue",
                "informational",
                "queue_empty",
                "open_review_intake",
                "review_queue_record",
            ),
            vec![
                bs("cc-empty-review", ReviewUi, DesktopFull),
                bs("cc-empty-shell", ShellUi, CompactNarrowed),
            ],
        ),
        spec(
            "shell/dependency-load-state",
            "Shell dependency-load state",
            LoadingState,
            facets(
                "pending",
                "dependency_load",
                "informational",
                "load_in_progress",
                "wait_or_open_activity",
                "dependency_load_record",
            ),
            vec![
                bs("cc-loading-shell", ShellUi, DesktopFull),
                bs("cc-loading-support", SupportUi, RemoteProjected),
                bs("cc-loading-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "provider/disconnect-consequence-block",
            "Provider disconnect consequence block",
            ConsequenceBlock,
            facets(
                "blocked",
                "provider_disconnect",
                "high",
                "disconnect_blast_radius",
                "review_rollback",
                "provider_disconnect_record",
            ),
            vec![
                bs("cc-consequence-settings", SettingsUi, DesktopFull),
                bs("cc-consequence-support", SupportUi, CompactNarrowed),
                bs(
                    "cc-consequence-support-export",
                    SupportExport,
                    ExportedRedacted,
                ),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<DecisionFeedbackConsumerBinding>
where
    F: Fn(&str, DecisionFeedbackRepresentation) -> DecisionFeedbackRepresentation,
{
    let mut bindings = Vec::new();
    for object in object_specs() {
        for spec in &object.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                object.object_id,
                object.object_label,
                object.component,
                spec.consumer,
                representation,
                object.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> DecisionFeedbackSharedConsumersTrustReview {
    DecisionFeedbackSharedConsumersTrustReview {
        primitive_reuse_proven_by_fixtures: true,
        same_object_same_vocabulary_across_surfaces: true,
        disposition_words_stay_in_frozen_vocabulary: true,
        meaning_never_relies_on_color_alone: true,
        popover_never_carries_only_critical_instruction: true,
        dialogs_never_use_generic_yes_no_copy: true,
        toast_never_becomes_only_durable_truth: true,
        loading_never_blanks_useful_pane: true,
        loading_never_uses_full_screen_spinner_when_partial_capable: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> DecisionFeedbackSharedConsumersProjection {
    DecisionFeedbackSharedConsumersProjection {
        shell_ui_reuses_shared_primitives: true,
        help_ui_reuses_shared_primitives: true,
        support_ui_reuses_shared_primitives: true,
        review_ui_reuses_shared_primitives: true,
        settings_ui_reuses_shared_primitives: true,
        updates_ui_reuses_shared_primitives: true,
        support_export_reuses_shared_primitives: true,
        every_primitive_adopted_by_two_or_more_consumers: true,
        vocabulary_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_contract_family: true,
    }
}

fn proof_freshness() -> DecisionFeedbackSharedConsumersProofFreshness {
    DecisionFeedbackSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_DECISION_FEEDBACK_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_DECISION_FEEDBACK_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_DECISION_FEEDBACK_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF.to_owned(),
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF.to_owned(),
    ];
    for family in M5DecisionFeedbackFamily::ALL {
        refs.push(family.canonical_component_schema_ref().to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    consumer_bindings: Vec<DecisionFeedbackConsumerBinding>,
) -> M5DecisionFeedbackSharedConsumersPacket {
    M5DecisionFeedbackSharedConsumersPacket::new(M5DecisionFeedbackSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: DecisionFeedbackSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5DecisionFeedbackConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in decision-feedback shared-consumer parity packet.
pub fn seeded_m5_decision_feedback_shared_consumers() -> M5DecisionFeedbackSharedConsumersPacket {
    packet_from_bindings(
        M5_DECISION_FEEDBACK_SHARED_CONSUMERS_PACKET_ID,
        "M5 decision / feedback shared consumers",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same objects with two more settings surfaces narrowed to compact and remote
/// representations, proving state changes propagate across compact and remote forms.
pub fn seeded_m5_decision_feedback_shared_consumers_compact_remote_narrowed(
) -> M5DecisionFeedbackSharedConsumersPacket {
    packet_from_bindings(
        "m5-decision-feedback-shared-consumers:compact-remote:0001",
        "M5 decision / feedback shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cc-badge-settings" => DecisionFeedbackRepresentation::CompactNarrowed,
            "cc-toast-settings" => DecisionFeedbackRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same objects with two more surfaces narrowed to exported, export-safe
/// representations, proving state changes propagate into exported forms.
pub fn seeded_m5_decision_feedback_shared_consumers_exported_redaction_narrowed(
) -> M5DecisionFeedbackSharedConsumersPacket {
    packet_from_bindings(
        "m5-decision-feedback-shared-consumers:exported-redaction:0001",
        "M5 decision / feedback shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cc-consequence-settings" => DecisionFeedbackRepresentation::ExportedRedacted,
            "cc-empty-review" => DecisionFeedbackRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
