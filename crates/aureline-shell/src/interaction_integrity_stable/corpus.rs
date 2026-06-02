//! Deterministic claimed-stable matrix for interaction-parity records.
//!
//! Every record here is a genuine projection of the live interaction-integrity
//! packet in [`crate::interaction_integrity`]. The corpus mints the seeded beta
//! packet, validates it, groups its object-interaction states by surface, and
//! projects each dense-surface family into the governed
//! [`InteractionParityRecord::build`] builder, so a record can never drift from
//! the shared object-interaction vocabulary that ships.
//!
//! The matrix covers all five dense-surface families and spans Stable and
//! narrowed rows:
//!
//! - A **git change tree** (tree), a **search results list** (virtualized list),
//!   a **review queue** (grid), a **command palette / quick-open** (palette-like
//!   over the search-result family), and a **review inspector** (inspector/detail
//!   bound to the selected review object) qualify **Stable**.
//! - A **focus-return drop-to-body drill** narrows below Stable because a dialog
//!   close drops focus to the document body.
//! - An **async-update focus-theft drill** narrows below Stable because a
//!   streamed insert steals focus from the active task.
//! - A **coordination-collapse drill** narrows below Stable because focus,
//!   current item, selection, and anchor are collapsed onto a single value.
//! - A **preview-surface inspector** proves every pillar but binds a
//!   keyboard-help surface still in preview, so it narrows to **Preview** by its
//!   lowest binding surface marker instead of inheriting an adjacent green row.

use crate::interaction_integrity::{
    seeded_interaction_integrity_beta_packet, validate_interaction_integrity_beta_packet,
    InteractionIntegrityBetaPacket, ObjectInteractionStateRecord,
    INTERACTION_INTEGRITY_SHARED_CONTRACT_REF,
};
use crate::notification_attention_stable::model::{
    AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, RecoveryRouteRecord, StableClaimClass,
};

use super::model::{
    required_recovery_routes, AsyncUpdateClass, AsyncUpdateRow, CoordinationStateModel,
    DisappearanceResolution, FocusReturnRow, FocusReturnTrigger, InteractionA11yCues,
    InteractionClaimCeiling, InteractionParityInput, InteractionParityRecord,
    InteractionSurfaceClass, InteractionSurfaceProjectionInput, InteractionTruthSurface,
    InteractionUpstream, KeyboardModelClass, KeyboardModelRow, PlatformConformanceRow,
    PlatformProfileClass,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/interaction-parity";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/interaction-parity";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-interaction-parity";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-interaction-parity";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-interaction-parity";

/// One scenario in the claimed-stable interaction-parity matrix.
#[derive(Debug, Clone)]
pub struct InteractionParityScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Posture token pinned for the scenario.
    pub expected_posture: String,
    /// Expected dense-surface family.
    pub expected_surface_class: InteractionSurfaceClass,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest binding surface).
    pub expected_surface_marker: LifecycleMarker,
    record: InteractionParityRecord,
}

impl InteractionParityScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> InteractionParityRecord {
        self.record.clone()
    }
}

/// The claimed-stable interaction-parity matrix, in canonical order.
pub fn interaction_parity_corpus() -> Vec<InteractionParityScenario> {
    let packet = seeded_interaction_integrity_beta_packet();
    validate_interaction_integrity_beta_packet(&packet)
        .expect("seeded interaction-integrity packet must validate");
    let packet_ref = packet.packet_id.clone();

    vec![
        git_change_tree(&packet, &packet_ref),
        search_results_list(&packet, &packet_ref),
        review_queue_grid(&packet, &packet_ref),
        command_palette_quick_open(&packet, &packet_ref),
        review_inspector_detail(&packet, &packet_ref),
        focus_return_drop_to_body_drill(&packet, &packet_ref),
        async_update_focus_theft_drill(&packet, &packet_ref),
        coordination_collapse_drill(&packet, &packet_ref),
        preview_surface_inspector(&packet, &packet_ref),
    ]
}

// ---------------------------------------------------------------------------
// Projection helpers
// ---------------------------------------------------------------------------

/// Knobs that turn a Stable projection into an adversarial drill.
#[derive(Debug, Clone, Copy, Default)]
struct DrillKnobs {
    /// Collapse the coordination states onto a single value.
    coordination_collapsed: bool,
    /// Make a streamed insert steal focus from the active task.
    async_steals_focus: bool,
    /// Make the dialog focus-return drop focus to the document body.
    focus_return_to_body: bool,
    /// Bind a keyboard-help surface still in preview.
    preview_surface: bool,
    /// Render the posture as a single-object inspector selection.
    inspector_selection: bool,
}

fn objects_for_surface<'a>(
    packet: &'a InteractionIntegrityBetaPacket,
    surface_id_ref: &str,
) -> Vec<&'a ObjectInteractionStateRecord> {
    packet
        .object_states
        .iter()
        .filter(|row| row.surface_id_ref == surface_id_ref)
        .collect()
}

fn coordination_from_objects(
    objects: &[&ObjectInteractionStateRecord],
    knobs: DrillKnobs,
) -> CoordinationStateModel {
    let focus = objects.iter().find(|row| row.focused);
    let active = objects.iter().find(|row| row.active).or(focus);
    let selected: Vec<String> = if knobs.inspector_selection {
        focus
            .map(|row| vec![row.object_id_ref.clone()])
            .unwrap_or_default()
    } else {
        objects
            .iter()
            .filter(|row| row.selected)
            .map(|row| row.object_id_ref.clone())
            .collect()
    };
    let anchor = if knobs.inspector_selection {
        focus.map(|row| row.object_id_ref.clone())
    } else {
        objects
            .iter()
            .find(|row| row.selected)
            .map(|row| row.object_id_ref.clone())
    };
    CoordinationStateModel {
        focus_object_id_ref: focus.map(|row| row.object_id_ref.clone()),
        current_item_id_ref: focus.map(|row| row.object_id_ref.clone()),
        selection_object_id_refs: selected,
        anchor_object_id_ref: anchor,
        last_activated_object_id_ref: active.map(|row| row.object_id_ref.clone()),
        // The collapse drill keys identity by transient position rather than
        // modeling the five states distinctly.
        states_modeled_distinctly: !knobs.coordination_collapsed,
        activation_preserves_selection: !knobs.coordination_collapsed,
        identity_by_stable_id_not_index: true,
    }
}

fn a11y_cues_from_objects(objects: &[&ObjectInteractionStateRecord]) -> InteractionA11yCues {
    let set_size = objects.len() as u64;
    let position_in_set = objects
        .iter()
        .position(|row| row.focused)
        .map(|idx| idx as u64 + 1)
        .unwrap_or(1);
    let selected = objects.iter().filter(|row| row.selected).count() as u64;
    let blocked = objects.iter().any(|row| row.blocked);
    let read_only = objects.iter().any(|row| row.disabled);
    InteractionA11yCues {
        selected_count_narrated: true,
        selected_count_label: format!("{selected} of {set_size} selected"),
        position_in_set_narrated: true,
        position_in_set,
        set_size,
        position_in_set_label: format!("{position_in_set} of {set_size}"),
        blocked_row_cue_present: blocked,
        read_only_row_cue_present: read_only,
        roving_tabindex_narrated: true,
    }
}

fn async_rows(knobs: DrillKnobs) -> Vec<AsyncUpdateRow> {
    vec![
        AsyncUpdateRow {
            update_class: AsyncUpdateClass::StreamingInsert,
            preserves_focus_by_stable_id: true,
            preserves_selection_by_stable_id: true,
            preserves_anchor: true,
            steals_focus_from_active_task: knobs.async_steals_focus,
            focused_object_can_disappear: false,
            disappearance_resolution: DisappearanceResolution::NotApplicable,
            announces_focus_move_reason: false,
            user_impact_label:
                "Streamed rows append below without moving focus or selection off the active object."
                    .to_string(),
        },
        AsyncUpdateRow {
            update_class: AsyncUpdateClass::SortFilterRefresh,
            preserves_focus_by_stable_id: true,
            preserves_selection_by_stable_id: true,
            preserves_anchor: true,
            steals_focus_from_active_task: false,
            focused_object_can_disappear: true,
            disappearance_resolution: DisappearanceResolution::NearestSafeSibling,
            announces_focus_move_reason: true,
            user_impact_label:
                "When a filter removes the focused row, focus moves to the nearest visible sibling and announces why."
                    .to_string(),
        },
        AsyncUpdateRow {
            update_class: AsyncUpdateClass::BackgroundIndexing,
            preserves_focus_by_stable_id: true,
            preserves_selection_by_stable_id: true,
            preserves_anchor: true,
            steals_focus_from_active_task: false,
            focused_object_can_disappear: false,
            disappearance_resolution: DisappearanceResolution::NotApplicable,
            announces_focus_move_reason: false,
            user_impact_label:
                "Background indexing updates counts and badges without touching focus or selection."
                    .to_string(),
        },
        AsyncUpdateRow {
            update_class: AsyncUpdateClass::ExtensionViewReplacement,
            preserves_focus_by_stable_id: true,
            preserves_selection_by_stable_id: true,
            preserves_anchor: true,
            steals_focus_from_active_task: false,
            focused_object_can_disappear: true,
            disappearance_resolution: DisappearanceResolution::ParentNode,
            announces_focus_move_reason: true,
            user_impact_label:
                "If an extension view is replaced, focus falls back to the parent group and announces the move."
                    .to_string(),
        },
        AsyncUpdateRow {
            update_class: AsyncUpdateClass::NotificationBanner,
            preserves_focus_by_stable_id: true,
            preserves_selection_by_stable_id: true,
            preserves_anchor: true,
            steals_focus_from_active_task: false,
            focused_object_can_disappear: false,
            disappearance_resolution: DisappearanceResolution::NotApplicable,
            announces_focus_move_reason: false,
            user_impact_label:
                "Notifications and banners post to the activity center without stealing typing focus."
                    .to_string(),
        },
        AsyncUpdateRow {
            update_class: AsyncUpdateClass::MultiWindowUpdate,
            preserves_focus_by_stable_id: true,
            preserves_selection_by_stable_id: true,
            preserves_anchor: true,
            steals_focus_from_active_task: false,
            focused_object_can_disappear: false,
            disappearance_resolution: DisappearanceResolution::NotApplicable,
            announces_focus_move_reason: false,
            user_impact_label:
                "Updates from another window reconcile state by stable id without warping focus across windows."
                    .to_string(),
        },
    ]
}

fn focus_return_rows(surface_id_ref: &str, origin: &str, knobs: DrillKnobs) -> Vec<FocusReturnRow> {
    let parent = format!("{surface_id_ref}#parent");
    let sibling = format!("{surface_id_ref}#sibling");
    let mut rows = Vec::new();
    let triggers = [
        (
            FocusReturnTrigger::DialogConfirmCancel,
            "Returned to the invoking row after the dialog closed.",
        ),
        (
            FocusReturnTrigger::SheetDismiss,
            "Returned to the invoking row after the sheet was dismissed.",
        ),
        (
            FocusReturnTrigger::PaletteDismiss,
            "Returned to the invoking row after the palette closed.",
        ),
        (
            FocusReturnTrigger::PopoverDismiss,
            "Returned to the invoking control after the popover closed.",
        ),
        (
            FocusReturnTrigger::InlineRenameCommitCancel,
            "Returned to the renamed row after commit or cancel.",
        ),
        (
            FocusReturnTrigger::InspectorDismiss,
            "Returned to the inspected row after the inspector closed.",
        ),
        (
            FocusReturnTrigger::PaneClose,
            "Returned to the nearest pane after the pane closed.",
        ),
        (
            FocusReturnTrigger::SplitReflow,
            "Kept focus on the same object after the split reflowed.",
        ),
        (
            FocusReturnTrigger::ExtensionViewRemoval,
            "Returned to the parent group after the extension view was removed.",
        ),
        (
            FocusReturnTrigger::MissingDependencyPlaceholderReplacement,
            "Returned to the placeholder card's row after replacement.",
        ),
    ];
    for (idx, (trigger, announcement)) in triggers.iter().enumerate() {
        let to_body =
            knobs.focus_return_to_body && *trigger == FocusReturnTrigger::DialogConfirmCancel;
        rows.push(FocusReturnRow {
            trigger: *trigger,
            rule_id: format!("focus_return:{}:{}", trigger.as_str(), idx),
            origin_object_id_ref: origin.to_string(),
            invoking_control_id_ref: format!("control:{}:{}", surface_id_ref, trigger.as_str()),
            expected_return_target_id_ref: if to_body {
                "document:body".to_string()
            } else {
                origin.to_string()
            },
            fallback_return_target_id_ref: if matches!(
                trigger,
                FocusReturnTrigger::PaneClose | FocusReturnTrigger::ExtensionViewRemoval
            ) {
                parent.clone()
            } else {
                sibling.clone()
            },
            returns_to_invoker_or_safe_ancestor: !to_body,
            never_returns_to_document_body: !to_body,
            never_returns_to_offscreen_surface: true,
            never_warps_across_windows: true,
            preserves_selection_or_cursor_state: true,
            screen_reader_announcement: announcement.to_string(),
        });
    }
    rows
}

fn platform_conformance() -> Vec<PlatformConformanceRow> {
    let behaviors = || {
        vec![
            "roving tabindex".to_string(),
            "focus return".to_string(),
            "selected-count narration".to_string(),
        ]
    };
    vec![
        PlatformConformanceRow {
            profile: PlatformProfileClass::MacOs,
            profile_id: "macos_15_plus_universal".to_string(),
            covered: true,
            proof_ref: "ci:interaction-parity:macos".to_string(),
            named_behaviors: behaviors(),
        },
        PlatformConformanceRow {
            profile: PlatformProfileClass::Windows,
            profile_id: "windows_11_x86_64".to_string(),
            covered: true,
            proof_ref: "ci:interaction-parity:windows".to_string(),
            named_behaviors: behaviors(),
        },
        PlatformConformanceRow {
            profile: PlatformProfileClass::Linux,
            profile_id: "linux_gnome_wayland_x86_64".to_string(),
            covered: true,
            proof_ref: "ci:interaction-parity:linux".to_string(),
            named_behaviors: behaviors(),
        },
    ]
}

fn surface_projections(preview: bool) -> Vec<InteractionSurfaceProjectionInput> {
    InteractionTruthSurface::REQUIRED
        .iter()
        .map(|surface| {
            let surface_marker = if preview && *surface == InteractionTruthSurface::KeyboardHelp {
                LifecycleMarker::Preview
            } else {
                LifecycleMarker::Stable
            };
            InteractionSurfaceProjectionInput {
                surface: *surface,
                surface_marker,
                reads_shared_record: true,
            }
        })
        .collect()
}

fn routes(posture_ref: &str) -> Vec<EntryRouteRecord> {
    let route = |surface: AttentionRouteSurface, class: &str| EntryRouteRecord {
        surface,
        route_ref: format!("aureline://{class}/{posture_ref}"),
        keyboard_reachable: true,
        activates_same_item: true,
    };
    vec![
        route(AttentionRouteSurface::ActivityCenter, "activity-center"),
        route(AttentionRouteSurface::CommandPalette, "command-palette"),
        route(AttentionRouteSurface::StatusBar, "status-bar"),
        route(AttentionRouteSurface::MenuCommand, "menu-command"),
    ]
}

fn accessibility(
    recovery_routes: &[RecoveryRouteRecord],
    focus_order_index: u32,
    row_narration: &str,
) -> AccessibilityDisclosure {
    let layout_modes = LayoutMode::REQUIRED
        .iter()
        .map(|mode| LayoutModeDisclosure {
            mode: *mode,
            row_narration_available: true,
            recovery_affordances_reachable: true,
        })
        .collect();
    AccessibilityDisclosure {
        focus_order_index,
        tab_stop_count: 1,
        row_narration: row_narration.to_string(),
        action_labels: recovery_routes
            .iter()
            .map(|route| route.action_label.clone())
            .collect(),
        layout_modes,
    }
}

fn full_claim_ceiling() -> InteractionClaimCeiling {
    InteractionClaimCeiling {
        asserts_coordination_states_distinct: true,
        asserts_identity_survives_async_updates: true,
        asserts_focus_return_complete: true,
        asserts_keyboard_model_complete: true,
        asserts_async_never_steals_focus: true,
        asserts_accessibility_cues_complete: true,
        asserts_platform_conformance_complete: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn build_scenario(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
    scenario_id: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    surface_class: InteractionSurfaceClass,
    surface_id_ref: &'static str,
    claim_ceiling: InteractionClaimCeiling,
    knobs: DrillKnobs,
    expected_surface_class: InteractionSurfaceClass,
    expected_claim_class: StableClaimClass,
    expected_qualifies_stable: bool,
    expected_surface_marker: LifecycleMarker,
) -> InteractionParityScenario {
    let objects = objects_for_surface(packet, surface_id_ref);
    let focus_object = objects
        .iter()
        .find(|row| row.focused)
        .map(|row| row.object_id_ref.clone())
        .unwrap_or_else(|| format!("{surface_id_ref}#first"));
    let coordination = coordination_from_objects(&objects, knobs);
    let a11y_cues = a11y_cues_from_objects(&objects);
    let recovery_routes = required_recovery_routes(true);
    let accessibility_block = accessibility(&recovery_routes, 1, posture_label);
    let contributing_object_refs = objects
        .iter()
        .map(|row| row.object_id_ref.clone())
        .collect::<Vec<_>>();

    let keyboard_model = KeyboardModelRow {
        model_class: KeyboardModelClass::RovingTabindex,
        single_tab_stop: true,
        arrow_moves_current_item: true,
        selection_supported: !knobs.inspector_selection,
        space_toggles_selection: !knobs.inspector_selection,
        enter_triggers_default_action: true,
        default_action_discoverable: true,
        home_end_page_preserves_anchor: true,
        no_silent_destructive_activation: true,
    };

    let input = InteractionParityInput {
        record_id: scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: scenario_id.to_string(),
        posture_label: posture_label.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        surface_class,
        surface_id_ref: surface_id_ref.to_string(),
        coordination,
        async_updates: async_rows(knobs),
        focus_returns: focus_return_rows(surface_id_ref, &focus_object, knobs),
        keyboard_model,
        a11y_cues,
        platform_conformance: platform_conformance(),
        surface_projections: surface_projections(knobs.preview_surface),
        claim_ceiling,
        recovery_routes,
        routes: routes(scenario_id),
        accessibility: accessibility_block,
        available_without_account: true,
        available_without_managed_services: true,
        upstream: InteractionUpstream {
            interaction_packet_ref: packet_ref.to_string(),
            interaction_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_string(),
            contributing_object_refs,
        },
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };

    let record = InteractionParityRecord::build(input)
        .unwrap_or_else(|err| panic!("scenario {scenario_id} must build: {err}"));

    InteractionParityScenario {
        scenario_id,
        fixture_filename: format!("{scenario_id}.json"),
        expected_posture: scenario_id.to_string(),
        expected_surface_class,
        expected_claim_class,
        expected_qualifies_stable,
        expected_surface_marker,
        record,
    }
}

// ---------------------------------------------------------------------------
// Stable rows
// ---------------------------------------------------------------------------

fn git_change_tree(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    build_scenario(
        packet,
        packet_ref,
        "git_change_tree_stable",
        "Git change tree",
        "Git change tree keeps focus, current item, selection, and anchor distinct under streamed status refresh.",
        "The git change tree models focus, current item, selection, and anchor as distinct stable-id states, presents a roving-tabindex keyboard model, records focus-return targets for every transient surface, and never lets a background status refresh steal focus.",
        InteractionSurfaceClass::Tree,
        "surface:git:change-tree",
        full_claim_ceiling(),
        DrillKnobs::default(),
        InteractionSurfaceClass::Tree,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn search_results_list(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    build_scenario(
        packet,
        packet_ref,
        "search_results_virtualized_list_stable",
        "Search results list",
        "Search results preserve focus and selection by stable id across streamed inserts and filter refresh.",
        "The virtualized search-results list preserves focus and selection by stable object id across streamed inserts, sort/filter refresh, and background indexing, moves focus to the nearest safe sibling when the focused row is filtered out, and announces the reason.",
        InteractionSurfaceClass::VirtualizedList,
        "surface:search:results",
        full_claim_ceiling(),
        DrillKnobs::default(),
        InteractionSurfaceClass::VirtualizedList,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn review_queue_grid(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    build_scenario(
        packet,
        packet_ref,
        "review_queue_grid_stable",
        "Review queue grid",
        "The review queue grid narrates selected count, position-in-set, and blocked/read-only row cues across layouts.",
        "The review queue grid keeps the five coordination states distinct, narrates selected count, position-in-set, and blocked/read-only row cues across normal, high-contrast, and zoomed layouts, and returns focus to the invoking row after the publish sheet closes.",
        InteractionSurfaceClass::Grid,
        "surface:review:queue",
        full_claim_ceiling(),
        DrillKnobs::default(),
        InteractionSurfaceClass::Grid,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn command_palette_quick_open(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    build_scenario(
        packet,
        packet_ref,
        "command_palette_quick_open_stable",
        "Command palette / quick-open",
        "The command palette records a focus-return target before it steals focus and restores it on dismiss.",
        "The command palette / quick-open surface projects the search-result family, records a focus-return target before stealing focus, restores focus to the invoking control on dismiss, and never drops focus to the document body.",
        InteractionSurfaceClass::PaletteLike,
        "surface:search:results",
        full_claim_ceiling(),
        DrillKnobs::default(),
        InteractionSurfaceClass::PaletteLike,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn review_inspector_detail(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    build_scenario(
        packet,
        packet_ref,
        "review_inspector_detail_stable",
        "Review inspector / detail",
        "The review inspector returns focus to the inspected row and keeps activation from mutating selection.",
        "The review inspector / detail workflow binds the selected review object, keeps activation from mutating selection, returns focus to the inspected row when the inspector closes, and never warps focus across windows.",
        InteractionSurfaceClass::InspectorDetail,
        "surface:review:queue",
        full_claim_ceiling(),
        DrillKnobs {
            inspector_selection: true,
            ..DrillKnobs::default()
        },
        InteractionSurfaceClass::InspectorDetail,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

// ---------------------------------------------------------------------------
// Narrowed drills
// ---------------------------------------------------------------------------

fn focus_return_drop_to_body_drill(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    let ceiling = InteractionClaimCeiling {
        asserts_focus_return_complete: false,
        ..full_claim_ceiling()
    };
    build_scenario(
        packet,
        packet_ref,
        "focus_return_drop_to_body_drill",
        "Focus-return drop-to-body drill",
        "A dialog close drops focus to the document body, so the posture narrows below Stable.",
        "This drill closes a confirm dialog that drops focus to the document body instead of the invoking row, so the focus-return pillar fails and the posture is narrowed below Stable with a named reason.",
        InteractionSurfaceClass::Tree,
        "surface:git:change-tree",
        ceiling,
        DrillKnobs {
            focus_return_to_body: true,
            ..DrillKnobs::default()
        },
        InteractionSurfaceClass::Tree,
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn async_update_focus_theft_drill(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    let ceiling = InteractionClaimCeiling {
        asserts_async_never_steals_focus: false,
        ..full_claim_ceiling()
    };
    build_scenario(
        packet,
        packet_ref,
        "async_update_focus_theft_drill",
        "Async-update focus-theft drill",
        "A streamed insert steals focus from active typing, so the posture narrows below Stable.",
        "This drill lets a streamed insert steal focus from the active task, so the no-focus-theft pillar fails and the posture is narrowed below Stable with a named reason.",
        InteractionSurfaceClass::VirtualizedList,
        "surface:search:results",
        ceiling,
        DrillKnobs {
            async_steals_focus: true,
            ..DrillKnobs::default()
        },
        InteractionSurfaceClass::VirtualizedList,
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn coordination_collapse_drill(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    let ceiling = InteractionClaimCeiling {
        asserts_coordination_states_distinct: false,
        ..full_claim_ceiling()
    };
    build_scenario(
        packet,
        packet_ref,
        "coordination_collapse_drill",
        "Coordination-collapse drill",
        "Focus, current item, selection, and anchor are collapsed onto one value, so the posture narrows below Stable.",
        "This drill collapses focus, current item, selection, and anchor onto a single value and lets activation mutate selection, so the distinct-coordination pillar fails and the posture is narrowed below Stable with a named reason.",
        InteractionSurfaceClass::Grid,
        "surface:review:queue",
        ceiling,
        DrillKnobs {
            coordination_collapsed: true,
            ..DrillKnobs::default()
        },
        InteractionSurfaceClass::Grid,
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn preview_surface_inspector(
    packet: &InteractionIntegrityBetaPacket,
    packet_ref: &str,
) -> InteractionParityScenario {
    build_scenario(
        packet,
        packet_ref,
        "preview_surface_inspector",
        "Preview-surface inspector",
        "Every pillar holds, but the keyboard-help surface is still in preview, so the posture narrows to Preview.",
        "This inspector posture proves every interaction pillar, but binds a keyboard-help surface still in preview, so it is narrowed to Preview by its lowest binding surface marker instead of inheriting an adjacent green row.",
        InteractionSurfaceClass::InspectorDetail,
        "surface:review:queue",
        full_claim_ceiling(),
        DrillKnobs {
            inspector_selection: true,
            preview_surface: true,
            ..DrillKnobs::default()
        },
        InteractionSurfaceClass::InspectorDetail,
        StableClaimClass::Preview,
        false,
        LifecycleMarker::Preview,
    )
}
