//! Conformance dump for the M5 assistive-selection-parity packet — roving
//! dense-collection focus, keyboard/screen-reader selection commands, live-region
//! announcements, focus/selection churn resilience, and offscreen-selection
//! durability across query-backed dense surfaces.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_collections::ship_keyboard_assistive_selection_parity_and_roving_tabindex_focus::*;
use aureline_collections::{CollectionDataMode, CollectionViewKind, DenseCollectionSurface};

const PACKET_ID: &str = "m5-assistive-selection-parity:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn command(
    kind: AssistiveCommandKind,
    keyboard_binding: &str,
    accessible_name: &str,
    announcement: &str,
) -> AssistiveCommand {
    AssistiveCommand {
        kind,
        keyboard_binding: keyboard_binding.to_owned(),
        accessible_name: accessible_name.to_owned(),
        announcement: announcement.to_owned(),
        keyboard_reachable: true,
        screen_reader_reachable: true,
        pointer_only: false,
    }
}

/// The full required command set, parameterized by the noun a surface uses so the
/// announcements read naturally per surface.
fn full_command_set(noun: &str) -> Vec<AssistiveCommand> {
    vec![
        command(
            AssistiveCommandKind::SelectCurrent,
            "Space",
            &format!("Toggle selection of the focused {noun}"),
            &format!("Focused {noun} selected"),
        ),
        command(
            AssistiveCommandKind::ExtendRange,
            "Shift+ArrowDown",
            &format!("Extend selection to the next {noun}"),
            &format!("Range extended to include the next {noun}"),
        ),
        command(
            AssistiveCommandKind::ClearSelection,
            "Escape",
            "Clear the entire selection",
            "Selection cleared",
        ),
        command(
            AssistiveCommandKind::InspectHiddenCount,
            "Alt+H",
            "Announce items selected outside the current view",
            &format!("Some selected {noun} items are hidden outside the current filter"),
        ),
        command(
            AssistiveCommandKind::OpenBatchReview,
            "Shift+Enter",
            "Open batch review for the current selection",
            "Batch review opened for the current selection",
        ),
    ]
}

fn announcement(politeness: LiveRegionPoliteness, sample: &str) -> SelectionAnnouncement {
    SelectionAnnouncement {
        politeness,
        announces_selection_count: true,
        announces_hidden_selected_count: true,
        announces_batch_review_open: true,
        sample_announcement: sample.to_owned(),
    }
}

fn resilience(
    event: FocusChurnEvent,
    outcome: FocusDurabilityOutcome,
    detail: &str,
) -> FocusChurnResilience {
    FocusChurnResilience {
        event,
        outcome,
        selection_preserved: true,
        focus_not_stolen: true,
        change_announced: true,
        detail_label: detail.to_owned(),
    }
}

fn offscreen(count: u64, detail: &str) -> OffscreenSelectionDurability {
    OffscreenSelectionDurability {
        selection_survives_virtualization_recycle: true,
        offscreen_members_tracked_by_identity: true,
        hidden_selected_count_exposed_to_at: true,
        offscreen_selected_count: count,
        detail_label: detail.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn profile(
    profile_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
    data_mode: CollectionDataMode,
    label: &str,
    focus_model: RovingFocusModel,
    commands: Vec<AssistiveCommand>,
    announcement: SelectionAnnouncement,
    churn_resilience: Vec<FocusChurnResilience>,
    offscreen_durability: OffscreenSelectionDurability,
) -> AssistiveSelectionProfile {
    AssistiveSelectionProfile {
        profile_id: profile_id.to_owned(),
        surface,
        view_kind,
        data_mode,
        label_summary: label.to_owned(),
        focus_model,
        commands,
        announcement,
        churn_resilience,
        offscreen_durability,
        evidence_refs: refs(&[&format!("evidence:a11y:{profile_id}")]),
    }
}

fn roving(kind: FocusModelKind, navigation_bound_label: &str) -> RovingFocusModel {
    RovingFocusModel {
        kind,
        single_tabstop: true,
        focus_tracked_by_stable_identity: true,
        focus_visible_indicator: true,
        arrow_key_navigation: true,
        navigation_bound_label: navigation_bound_label.to_owned(),
    }
}

fn profiles() -> Vec<AssistiveSelectionProfile> {
    vec![
        // Pipeline run list: roving tabindex over a flat list, focus held by id
        // through a background refresh.
        profile(
            "profile:pipeline-run-list:0001",
            DenseCollectionSurface::PipelineRunList,
            CollectionViewKind::List,
            CollectionDataMode::Streaming,
            "Pipeline run list keyboard selection with roving tabindex over streaming rows",
            roving(
                FocusModelKind::RovingTabindex,
                "Arrow keys clamp at the first and last run; Home and End jump to the ends",
            ),
            full_command_set("run"),
            announcement(
                LiveRegionPoliteness::Polite,
                "3 runs selected; focus on Run 7 of 412",
            ),
            vec![
                resilience(
                    FocusChurnEvent::StreamingInsert,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "new runs stream in above the focused run; focus stays on the same run by id",
                ),
                resilience(
                    FocusChurnEvent::BackgroundRefresh,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "a background refresh re-keys rows; focus and selection hold by stable id",
                ),
            ],
            offscreen(0, "all selected runs are within the loaded window"),
        ),
        // Review queue: aria-activedescendant queue, hidden-selected members
        // survive a filter change.
        profile(
            "profile:review-queue:0001",
            DenseCollectionSurface::ReviewQueue,
            CollectionViewKind::Queue,
            CollectionDataMode::FilteredSorted,
            "Review queue keyboard selection with aria-activedescendant and hidden-selected exposure",
            roving(
                FocusModelKind::AriaActivedescendant,
                "Arrow keys clamp at the queue ends; the container keeps a single tabstop",
            ),
            full_command_set("review item"),
            announcement(
                LiveRegionPoliteness::Polite,
                "12 selected; 3 outside the current filter; focus on review item 5",
            ),
            vec![
                resilience(
                    FocusChurnEvent::SortOrFilterChange,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "re-sorting the queue keeps focus and selection on the same items by id",
                ),
                resilience(
                    FocusChurnEvent::BackgroundRefresh,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "a refresh leaves hidden-selected members intact and announced",
                ),
            ],
            offscreen(3, "3 selected review items are outside the current filter"),
        ),
        // Incident list: roving tabindex over a live streaming list.
        profile(
            "profile:incident-list:0001",
            DenseCollectionSurface::IncidentList,
            CollectionViewKind::List,
            CollectionDataMode::Streaming,
            "Incident list keyboard selection that holds focus while new incidents stream in",
            roving(
                FocusModelKind::RovingTabindex,
                "Arrow keys clamp at the incident-list ends; Home and End jump to the ends",
            ),
            full_command_set("incident"),
            announcement(
                LiveRegionPoliteness::Assertive,
                "2 incidents selected; focus on incident INC-204",
            ),
            vec![
                resilience(
                    FocusChurnEvent::StreamingInsert,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "new incidents prepend above the focus; the roving tabstop never jumps to the top",
                ),
                resilience(
                    FocusChurnEvent::SortOrFilterChange,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "severity re-sort keeps focus and selection by stable incident id",
                ),
            ],
            offscreen(0, "all selected incidents are within the loaded window"),
        ),
        // Graph tree: virtualized tree, focus re-anchors when the focused node is
        // collapsed out of view, with offscreen selection held by id.
        profile(
            "profile:graph-list:0001",
            DenseCollectionSurface::GraphList,
            CollectionViewKind::Tree,
            CollectionDataMode::Virtualized,
            "Reference graph tree keyboard selection with focus re-anchor on virtualization recycle",
            roving(
                FocusModelKind::AriaActivedescendant,
                "Arrow keys walk visible tree order; Left collapses, Right expands; ends clamp",
            ),
            full_command_set("node"),
            announcement(
                LiveRegionPoliteness::Polite,
                "8 nodes selected; 5 collapsed out of view; focus on Node B",
            ),
            vec![
                resilience(
                    FocusChurnEvent::VirtualizationRecycle,
                    FocusDurabilityOutcome::FocusReanchoredVisible,
                    "the focused node scrolled out and its DOM recycled; focus re-anchors on the nearest visible node and is announced",
                ),
                resilience(
                    FocusChurnEvent::SortOrFilterChange,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "re-rooting the tree keeps selection on the same nodes by stable id",
                ),
            ],
            offscreen(5, "5 selected nodes are collapsed or scrolled out of the virtualized view"),
        ),
        // Marketplace results: virtualized table with a large offscreen selection.
        profile(
            "profile:marketplace-results:0001",
            DenseCollectionSurface::MarketplaceResults,
            CollectionViewKind::Table,
            CollectionDataMode::Virtualized,
            "Marketplace results keyboard selection with a large offscreen-selected population",
            roving(
                FocusModelKind::RovingTabindex,
                "Arrow keys move the cell-row tabstop; ends clamp; PageUp and PageDown page rows",
            ),
            full_command_set("extension"),
            announcement(
                LiveRegionPoliteness::Polite,
                "190 selected; 188 offscreen in the virtualized table; focus on Extension row 12",
            ),
            vec![
                resilience(
                    FocusChurnEvent::VirtualizationRecycle,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "scrolling recycles row DOM; the focused row and 188 offscreen selections hold by id",
                ),
                resilience(
                    FocusChurnEvent::StreamingInsert,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "a new page of results appends; focus and offscreen selection are untouched",
                ),
            ],
            offscreen(188, "188 selected extensions are offscreen in the virtualized table"),
        ),
        // Provider/admin table: filtered/sorted table, focus re-anchors after a
        // filter change drops the focused row.
        profile(
            "profile:provider-admin-table:0001",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            CollectionDataMode::FilteredSorted,
            "Provider/admin table keyboard selection with focus re-anchor after a filter drop",
            roving(
                FocusModelKind::AriaActivedescendant,
                "Arrow keys move the active row; ends clamp; Ctrl+Home and Ctrl+End jump to the ends",
            ),
            full_command_set("provider row"),
            announcement(
                LiveRegionPoliteness::Polite,
                "18 selected; 6 outside the current filter; focus on provider row 3",
            ),
            vec![
                resilience(
                    FocusChurnEvent::SortOrFilterChange,
                    FocusDurabilityOutcome::FocusReanchoredVisible,
                    "a filter change dropped the focused row; focus re-anchors on the next visible row and is announced",
                ),
                resilience(
                    FocusChurnEvent::BackgroundRefresh,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                    "a refresh keeps the 6 hidden-selected rows tracked by stable id",
                ),
            ],
            offscreen(6, "6 selected provider rows are outside the current filter"),
        ),
    ]
}

fn guardrails() -> AssistiveSelectionGuardrails {
    AssistiveSelectionGuardrails {
        no_pointer_only_selection_controls: true,
        focus_not_stolen_by_streaming_or_virtualization: true,
        selection_survives_sort_filter_virtualization_by_identity: true,
        hidden_selected_count_exposed_to_assistive_tech: true,
        roving_focus_tracked_by_stable_identity: true,
        broad_action_review_keyboard_reachable: true,
    }
}

fn consumer_projection() -> AssistiveSelectionConsumerProjection {
    AssistiveSelectionConsumerProjection {
        product_renders_assistive_selection: true,
        diagnostics_reconstructs_parity: true,
        support_export_reuses_records: true,
        accessibility_evidence_reuses_records: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        ASSISTIVE_SELECTION_PARITY_SCHEMA_REF,
        ASSISTIVE_SELECTION_PARITY_DOC_REF,
        ASSISTIVE_SELECTION_PARITY_ARTIFACT_REF,
        "schemas/collections/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.schema.json",
        "schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json",
    ])
}

fn packet() -> AssistiveSelectionParityPacket {
    AssistiveSelectionParityPacket::new(AssistiveSelectionParityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Keyboard Assistive Selection Parity And Roving Focus".to_owned(),
        profiles: profiles(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
