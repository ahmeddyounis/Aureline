//! Canonical seed for the file-state badge-group consumer packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`FileStateBadgeGroupGrammar`] so the same constrained-object profile always carries the same badge grammar
//! across surfaces, and every narrowed posture derives its disclosure and action set from
//! [`resolve_badge_group_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every badge group offers so the state class, reason, and next safe action
/// are discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5ConstrainedFileStateAccessibilityRoute> {
    M5ConstrainedFileStateAccessibilityRoute::ALL.to_vec()
}

fn grammar(
    badge_role: &str,
    state_class_label: &str,
    reason: &str,
    canonical_source: &str,
    write_disposition: &str,
    safe_next_step: &str,
    co_applicable_state_labels: &[&str],
) -> FileStateBadgeGroupGrammar {
    FileStateBadgeGroupGrammar {
        badge_role_word: badge_role.to_owned(),
        state_class_label_word: state_class_label.to_owned(),
        reason_word: reason.to_owned(),
        canonical_source_word: canonical_source.to_owned(),
        write_disposition_word: write_disposition.to_owned(),
        safe_next_step_word: safe_next_step.to_owned(),
        co_applicable_state_labels: co_applicable_state_labels
            .iter()
            .map(|label| (*label).to_owned())
            .collect(),
    }
}

fn preserved_note_for(reason: BadgeGroupNarrowReason) -> String {
    match reason {
        BadgeGroupNarrowReason::CompactedToStatusChip => {
            "badge-role, state-class-label, reason, canonical-source, write-disposition, and safe-next-step words preserved; the badge and reason are compacted to a status chip"
        }
        BadgeGroupNarrowReason::PaletteAvailabilityGatedDisclosed => {
            "all badge grammar preserved; the command-palette write availability is gated behind the safe-next-step review"
        }
        BadgeGroupNarrowReason::ExportRedactionNarrowed => {
            "all badge grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: BadgeGroupNarrowNextAction) -> String {
    match action {
        BadgeGroupNarrowNextAction::OpenFullBadgeGroup => "Open the full badge group",
        BadgeGroupNarrowNextAction::OpenCommandDetail => "Open the command detail",
        BadgeGroupNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn allowed_actions_for(disclosure: BadgeGroupRenderDisclosure) -> Vec<BadgeGroupAction> {
    let mut actions = BadgeGroupAction::SAFE_BASE.to_vec();
    if disclosure.offers_safe_next_step {
        actions.push(BadgeGroupAction::OpenSafeNextStep);
    }
    actions
}

fn binding_refs(object_class: M5ConstrainedFileStateObject) -> Vec<String> {
    vec![
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    object_profile_id: &str,
    object_profile_label: &str,
    object_class: M5ConstrainedFileStateObject,
    co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    consumer: M5ConstrainedFileStateConsumerSurface,
    posture: BadgeRenderPosture,
    badge_grammar: FileStateBadgeGroupGrammar,
) -> FileStateBadgeGroupConsumerBinding {
    let disclosure = resolve_badge_group_render_disclosure(posture);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        BadgeGroupNarrowNote {
            reason,
            preserved_grammar_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let palette_availability_note = if disclosure.needs_palette_availability_note {
        "write availability gated behind the safe-next-step review; no silent lossy direct write"
            .to_owned()
    } else {
        String::new()
    };
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    FileStateBadgeGroupConsumerBinding {
        binding_id: binding_id.to_owned(),
        object_profile_id: object_profile_id.to_owned(),
        object_profile_label: object_profile_label.to_owned(),
        object_class,
        co_applicable_states,
        consumer,
        posture,
        badge_grammar,
        parity_state: disclosure.parity_state,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        narrow_note,
        palette_availability_note,
        export_detail_note,
        presents_constrained_object_as_directly_writable_or_hides_recovery_path: false,
        lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write:
            false,
        gives_ai_automation_import_or_repair_flows_a_hidden_bypass: false,
        leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated: false,
        lets_one_state_class_hide_another_when_both_materially_affect_behavior: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of a constrained-object profile, before any posture override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5ConstrainedFileStateConsumerSurface,
    posture: BadgeRenderPosture,
}

/// One constrained-object profile rendered across several consumer surfaces at one badge grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    grammar: FileStateBadgeGroupGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    grammar: FileStateBadgeGroupGrammar,
    bindings: Vec<BindingSpec>,
) -> ProfileSpec {
    ProfileSpec {
        profile_id,
        profile_label,
        object_class,
        co_applicable_states,
        grammar,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5ConstrainedFileStateConsumerSurface,
    posture: BadgeRenderPosture,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        posture,
    }
}

/// The six constrained-object profiles — one per B150 constrained-file-state object class — and the surfaces
/// that adopt each, drawn from the tab-chrome, breadcrumb-trail, status-bar, command-palette, editor-banner,
/// diff / review-header, write-review-sheet, AI / automation-path, and support / export consumers. Two profiles
/// are multi-state (`Generated` plus `Policy locked`, `Managed` plus `Captured snapshot`) so both facts stay
/// visible across surfaces.
fn profile_specs() -> Vec<ProfileSpec> {
    use BadgeRenderPosture::*;
    use M5ConstrainedFileStateConsumerSurface::*;
    use M5ConstrainedFileStateObject::*;

    vec![
        spec(
            "read-only/vendored-source",
            "Read-only vendored source (blocked in-place write)",
            ReadOnly,
            vec![],
            grammar(
                "state_badge_classification",
                "read_only",
                "read_only_vendored_source_cannot_be_written_in_place",
                "vendored_source_of_truth",
                "read_only_blocked",
                "duplicate_into_a_writable_copy",
                &[],
            ),
            vec![
                bs("fsbg-readonly-editor", EditorBanner, FullBadgeGroup),
                bs("fsbg-readonly-tab", TabChrome, CompactStatusChip),
                bs("fsbg-readonly-support", SupportExportPacket, ExportRedacted),
            ],
        ),
        spec(
            "generated/policy-locked-artifact",
            "Generated artifact that is also policy locked (multi-state)",
            Generated,
            vec![PolicyLocked],
            grammar(
                "blocked_write_reason",
                "generated",
                "generated_artifact_edits_are_overwritten_on_regenerate_and_policy_gated",
                "generator_source_of_truth",
                "regenerate_only_plus_approval_gated",
                "edit_the_generator_or_request_approval",
                &["policy_locked"],
            ),
            vec![
                bs("fsbg-generated-editor", EditorBanner, FullBadgeGroup),
                bs("fsbg-generated-diff", DiffReviewHeader, FullBadgeGroup),
                bs("fsbg-generated-status", StatusBar, CompactStatusChip),
                bs(
                    "fsbg-generated-palette",
                    CommandPalette,
                    PaletteAvailabilityGated,
                ),
            ],
        ),
        spec(
            "policy-locked/protected-config",
            "Policy-locked protected config (approval-gated write)",
            PolicyLocked,
            vec![],
            grammar(
                "canonical_source_relation",
                "policy_locked",
                "protected_config_writes_require_an_approval_owner",
                "policy_owner_of_record",
                "approval_gated",
                "request_approval_from_the_policy_owner",
                &[],
            ),
            vec![
                bs("fsbg-policy-writesheet", WriteReviewSheet, FullBadgeGroup),
                bs("fsbg-policy-breadcrumb", BreadcrumbTrail, CompactStatusChip),
                bs("fsbg-policy-support", SupportExportPacket, ExportRedacted),
            ],
        ),
        spec(
            "managed/captured-snapshot-mirror",
            "Managed mirror that is also a captured snapshot (multi-state)",
            Managed,
            vec![CapturedSnapshot],
            grammar(
                "exact_write_target",
                "managed",
                "managed_mirror_is_owned_upstream_and_this_view_is_a_captured_snapshot",
                "upstream_managing_owner",
                "detach_required_plus_restore_only",
                "detach_to_edit_locally_or_restore_from_snapshot",
                &["captured_snapshot"],
            ),
            vec![
                bs("fsbg-managed-writesheet", WriteReviewSheet, FullBadgeGroup),
                bs(
                    "fsbg-managed-ai",
                    AiAutomationPath,
                    PaletteAvailabilityGated,
                ),
                bs("fsbg-managed-status", StatusBar, CompactStatusChip),
            ],
        ),
        spec(
            "projection/virtual-view",
            "Projection / virtual view (writes resolve to the backing source)",
            Projection,
            vec![],
            grammar(
                "canonical_source_relation",
                "projection",
                "projection_writes_resolve_back_to_the_backing_source_object",
                "backing_source_object",
                "detach_required",
                "open_the_backing_source_or_overlay_a_local_view",
                &[],
            ),
            vec![
                bs("fsbg-projection-diff", DiffReviewHeader, FullBadgeGroup),
                bs(
                    "fsbg-projection-palette",
                    CommandPalette,
                    PaletteAvailabilityGated,
                ),
                bs("fsbg-projection-tab", TabChrome, CompactStatusChip),
            ],
        ),
        spec(
            "captured-snapshot/preserved-state",
            "Captured snapshot of a preserved past state (not the current live object)",
            CapturedSnapshot,
            vec![],
            grammar(
                "state_badge_classification",
                "captured_snapshot",
                "captured_snapshot_preserves_a_past_state_and_is_not_the_current_live_object",
                "live_object_of_record",
                "restore_only",
                "restore_from_snapshot_or_open_the_current_live_object",
                &[],
            ),
            vec![
                bs("fsbg-snapshot-editor", EditorBanner, FullBadgeGroup),
                bs(
                    "fsbg-snapshot-ai",
                    AiAutomationPath,
                    PaletteAvailabilityGated,
                ),
                bs("fsbg-snapshot-support", SupportExportPacket, ExportRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `posture_override` to override a binding's posture.
fn build_bindings<F>(posture_override: F) -> Vec<FileStateBadgeGroupConsumerBinding>
where
    F: Fn(&str, BadgeRenderPosture) -> BadgeRenderPosture,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let posture = posture_override(spec.binding_id, spec.posture);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.object_class,
                profile.co_applicable_states.clone(),
                spec.consumer,
                posture,
                profile.grammar.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> FileStateBadgeGroupConsumersTrustReview {
    FileStateBadgeGroupConsumersTrustReview {
        object_class_reuse_proven_by_fixtures: true,
        same_profile_same_badge_across_surfaces: true,
        badge_role_words_stay_in_frozen_vocabulary: true,
        write_disposition_never_masquerades_as_directly_writable: true,
        constrained_object_never_directly_writable_recovery_never_hidden: true,
        no_silent_lossy_direct_write_fallback: true,
        no_hidden_bypass_for_ai_automation_import_repair: true,
        canonical_source_write_target_sync_recovery_always_stated: true,
        multi_state_objects_keep_every_state_visible: true,
        accessibility_routes_present_for_state_reason_and_next_step: true,
        narrowing_disclosed_across_postures: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> FileStateBadgeGroupConsumersProjection {
    FileStateBadgeGroupConsumersProjection {
        tab_chrome_consumes_badge_group: true,
        breadcrumb_trail_consumes_badge_group: true,
        status_bar_consumes_badge_group: true,
        command_palette_consumes_badge_group: true,
        editor_banner_consumes_badge_group: true,
        diff_review_header_consumes_badge_group: true,
        write_review_sheet_consumes_badge_group: true,
        ai_automation_path_consumes_badge_group: true,
        support_export_packet_consumes_badge_group: true,
        every_object_class_adopted_by_two_or_more_consumers: true,
        badge_grammar_identical_for_same_profile: true,
        multi_state_objects_keep_both_facts_visible: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_constrained_file_state_object: true,
    }
}

fn proof_freshness() -> FileStateBadgeGroupConsumersProofFreshness {
    FileStateBadgeGroupConsumersProofFreshness {
        proof_freshness_slo_hours: M5_FILE_STATE_BADGE_GROUP_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_DOC_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned(),
    ];
    // The six object classes map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5ConstrainedFileStateObject::ALL {
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
    consumer_bindings: Vec<FileStateBadgeGroupConsumerBinding>,
) -> M5FileStateBadgeGroupConsumersPacket {
    M5FileStateBadgeGroupConsumersPacket::new(M5FileStateBadgeGroupConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: FileStateBadgeGroupConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5ConstrainedFileStateConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in file-state badge-group consumer packet.
pub fn seeded_m5_file_state_badge_group_consumers() -> M5FileStateBadgeGroupConsumersPacket {
    packet_from_bindings(
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_PACKET_ID,
        "M5 file-state badge groups & reason strips (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more full-badge-group surfaces narrowed to a compact status chip,
/// proving the badge grammar survives when the badge is compacted.
pub fn seeded_m5_file_state_badge_group_consumers_compact_status_narrowed(
) -> M5FileStateBadgeGroupConsumersPacket {
    packet_from_bindings(
        "m5-file-state-badge-group-consumers:compact-status:0001",
        "M5 file-state badge groups (compact status chip narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "fsbg-readonly-editor" => BadgeRenderPosture::CompactStatusChip,
            "fsbg-snapshot-editor" => BadgeRenderPosture::CompactStatusChip,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more surfaces narrowed to gated command-palette availability, proving
/// the badge grammar survives into gated-availability forms.
pub fn seeded_m5_file_state_badge_group_consumers_palette_gated_narrowed(
) -> M5FileStateBadgeGroupConsumersPacket {
    packet_from_bindings(
        "m5-file-state-badge-group-consumers:palette-gated:0001",
        "M5 file-state badge groups (palette availability gated narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "fsbg-policy-writesheet" => BadgeRenderPosture::PaletteAvailabilityGated,
            "fsbg-managed-writesheet" => BadgeRenderPosture::PaletteAvailabilityGated,
            _ => default,
        }),
    )
}
