use super::*;

const PACKET_ID: &str = "m5-assistive-selection-parity:test:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn roving(kind: FocusModelKind) -> RovingFocusModel {
    RovingFocusModel {
        kind,
        single_tabstop: true,
        focus_tracked_by_stable_identity: true,
        focus_visible_indicator: true,
        arrow_key_navigation: true,
        navigation_bound_label: "Arrow keys clamp at the first and last item".to_owned(),
    }
}

fn command(kind: AssistiveCommandKind, announcement: &str) -> AssistiveCommand {
    AssistiveCommand {
        kind,
        keyboard_binding: "Shift+ArrowDown".to_owned(),
        accessible_name: "Distinct accessible command name".to_owned(),
        announcement: announcement.to_owned(),
        keyboard_reachable: true,
        screen_reader_reachable: true,
        pointer_only: false,
    }
}

fn full_commands() -> Vec<AssistiveCommand> {
    vec![
        command(AssistiveCommandKind::SelectCurrent, "Focused row selected"),
        command(
            AssistiveCommandKind::ExtendRange,
            "Range extended by one row",
        ),
        command(AssistiveCommandKind::ClearSelection, "Selection cleared"),
        command(
            AssistiveCommandKind::InspectHiddenCount,
            "3 selected rows are hidden outside the current filter",
        ),
        command(
            AssistiveCommandKind::OpenBatchReview,
            "Batch review opened for the current selection",
        ),
    ]
}

fn announcement() -> SelectionAnnouncement {
    SelectionAnnouncement {
        politeness: LiveRegionPoliteness::Polite,
        announces_selection_count: true,
        announces_hidden_selected_count: true,
        announces_batch_review_open: true,
        sample_announcement: "12 selected; 3 outside the current filter".to_owned(),
    }
}

fn resilience(event: FocusChurnEvent, outcome: FocusDurabilityOutcome) -> FocusChurnResilience {
    FocusChurnResilience {
        event,
        outcome,
        selection_preserved: true,
        focus_not_stolen: true,
        change_announced: true,
        detail_label: "focus and selection hold by stable id across the event".to_owned(),
    }
}

fn offscreen(count: u64) -> OffscreenSelectionDurability {
    OffscreenSelectionDurability {
        selection_survives_virtualization_recycle: true,
        offscreen_members_tracked_by_identity: true,
        hidden_selected_count_exposed_to_at: true,
        offscreen_selected_count: count,
        detail_label: "offscreen selections are tracked by stable id".to_owned(),
    }
}

fn profile(
    profile_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
    data_mode: CollectionDataMode,
    focus_model: FocusModelKind,
    churn: Vec<FocusChurnResilience>,
    offscreen_count: u64,
) -> AssistiveSelectionProfile {
    AssistiveSelectionProfile {
        profile_id: profile_id.to_owned(),
        surface,
        view_kind,
        data_mode,
        label_summary: "Assistive selection profile".to_owned(),
        focus_model: roving(focus_model),
        commands: full_commands(),
        announcement: announcement(),
        churn_resilience: churn,
        offscreen_durability: offscreen(offscreen_count),
        evidence_refs: refs(&[&format!("evidence:{profile_id}")]),
    }
}

fn baseline_profiles() -> Vec<AssistiveSelectionProfile> {
    vec![
        profile(
            "p:pipeline",
            DenseCollectionSurface::PipelineRunList,
            CollectionViewKind::List,
            CollectionDataMode::Streaming,
            FocusModelKind::RovingTabindex,
            vec![
                resilience(
                    FocusChurnEvent::StreamingInsert,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                ),
                resilience(
                    FocusChurnEvent::BackgroundRefresh,
                    FocusDurabilityOutcome::FocusHeldByIdentity,
                ),
            ],
            0,
        ),
        profile(
            "p:review",
            DenseCollectionSurface::ReviewQueue,
            CollectionViewKind::Queue,
            CollectionDataMode::FilteredSorted,
            FocusModelKind::AriaActivedescendant,
            vec![resilience(
                FocusChurnEvent::SortOrFilterChange,
                FocusDurabilityOutcome::FocusHeldByIdentity,
            )],
            3,
        ),
        profile(
            "p:incident",
            DenseCollectionSurface::IncidentList,
            CollectionViewKind::List,
            CollectionDataMode::Streaming,
            FocusModelKind::RovingTabindex,
            vec![resilience(
                FocusChurnEvent::StreamingInsert,
                FocusDurabilityOutcome::FocusHeldByIdentity,
            )],
            0,
        ),
        profile(
            "p:graph",
            DenseCollectionSurface::GraphList,
            CollectionViewKind::Tree,
            CollectionDataMode::Virtualized,
            FocusModelKind::AriaActivedescendant,
            vec![resilience(
                FocusChurnEvent::VirtualizationRecycle,
                FocusDurabilityOutcome::FocusReanchoredVisible,
            )],
            5,
        ),
        profile(
            "p:marketplace",
            DenseCollectionSurface::MarketplaceResults,
            CollectionViewKind::Table,
            CollectionDataMode::Virtualized,
            FocusModelKind::RovingTabindex,
            vec![resilience(
                FocusChurnEvent::VirtualizationRecycle,
                FocusDurabilityOutcome::FocusHeldByIdentity,
            )],
            188,
        ),
        profile(
            "p:admin",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            CollectionDataMode::FilteredSorted,
            FocusModelKind::AriaActivedescendant,
            vec![resilience(
                FocusChurnEvent::SortOrFilterChange,
                FocusDurabilityOutcome::FocusReanchoredVisible,
            )],
            6,
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
    ])
}

fn baseline_packet() -> AssistiveSelectionParityPacket {
    AssistiveSelectionParityPacket::new(AssistiveSelectionParityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "Test assistive selection parity packet".to_owned(),
        profiles: baseline_profiles(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn baseline_packet_validates() {
    assert!(baseline_packet().validate().is_empty());
}

#[test]
fn pointer_only_command_is_rejected() {
    let mut packet = baseline_packet();
    packet.profiles[0].commands[0].pointer_only = true;
    let violations = packet.validate();
    assert!(violations.contains(&AssistiveSelectionParityViolation::PointerOnlyControlPresent));
    assert!(violations.contains(&AssistiveSelectionParityViolation::ProfileIncomplete));
}

#[test]
fn non_keyboard_command_is_rejected() {
    let mut packet = baseline_packet();
    packet.profiles[0].commands[1].keyboard_reachable = false;
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::PointerOnlyControlPresent));
}

#[test]
fn missing_required_command_is_rejected() {
    let mut packet = baseline_packet();
    packet.profiles[0]
        .commands
        .retain(|command| command.kind != AssistiveCommandKind::OpenBatchReview);
    let violations = packet.validate();
    assert!(violations.contains(&AssistiveSelectionParityViolation::RequiredCommandMissing));
}

#[test]
fn inspect_hidden_count_must_name_hidden_population() {
    let mut packet = baseline_packet();
    for command in &mut packet.profiles[1].commands {
        if command.kind == AssistiveCommandKind::InspectHiddenCount {
            command.announcement = "Inspecting the selection".to_owned();
        }
    }
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::ProfileIncomplete));
    // The command itself is invalid because it does not expose the count.
    let command = AssistiveCommand {
        kind: AssistiveCommandKind::InspectHiddenCount,
        keyboard_binding: "Alt+H".to_owned(),
        accessible_name: "Inspect".to_owned(),
        announcement: "Inspecting the selection".to_owned(),
        keyboard_reachable: true,
        screen_reader_reachable: true,
        pointer_only: false,
    };
    assert!(!command.exposes_hidden_count_when_inspecting());
    assert!(!command.is_valid());
}

#[test]
fn focus_not_by_stable_identity_is_rejected() {
    let mut packet = baseline_packet();
    packet.profiles[0]
        .focus_model
        .focus_tracked_by_stable_identity = false;
    let violations = packet.validate();
    assert!(violations.contains(&AssistiveSelectionParityViolation::FocusNotByStableIdentity));
    assert!(violations.contains(&AssistiveSelectionParityViolation::ProfileIncomplete));
}

#[test]
fn stolen_focus_on_churn_is_rejected() {
    let mut packet = baseline_packet();
    packet.profiles[0].churn_resilience[0].focus_not_stolen = false;
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::FocusStolenOnChurn));
}

#[test]
fn corrupted_selection_on_churn_is_rejected() {
    let mut packet = baseline_packet();
    packet.profiles[0].churn_resilience[0].selection_preserved = false;
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::SelectionNotDurable));
}

#[test]
fn offscreen_selection_must_survive_virtualization() {
    let mut packet = baseline_packet();
    packet.profiles[4]
        .offscreen_durability
        .selection_survives_virtualization_recycle = false;
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::SelectionNotDurable));
}

#[test]
fn hidden_selected_count_must_be_exposed() {
    let mut packet = baseline_packet();
    packet.profiles[1]
        .announcement
        .announces_hidden_selected_count = false;
    let violations = packet.validate();
    assert!(violations.contains(&AssistiveSelectionParityViolation::HiddenSelectedCountNotExposed));
}

#[test]
fn offscreen_hidden_count_must_be_exposed() {
    let mut packet = baseline_packet();
    packet.profiles[4]
        .offscreen_durability
        .hidden_selected_count_exposed_to_at = false;
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::HiddenSelectedCountNotExposed));
}

#[test]
fn generic_navigation_label_is_rejected() {
    let mut model = roving(FocusModelKind::RovingTabindex);
    model.navigation_bound_label = "focus".to_owned();
    assert!(!model.is_valid());
}

#[test]
fn both_focus_models_required() {
    let mut packet = baseline_packet();
    for profile in &mut packet.profiles {
        profile.focus_model.kind = FocusModelKind::RovingTabindex;
    }
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::RequiredFocusModelMissing));
}

#[test]
fn missing_required_surface_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .profiles
        .retain(|profile| profile.surface != DenseCollectionSurface::ProviderAdminTable);
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_view_kind_is_rejected() {
    let mut packet = baseline_packet();
    for profile in &mut packet.profiles {
        if profile.view_kind == CollectionViewKind::Tree {
            profile.view_kind = CollectionViewKind::List;
        }
    }
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::RequiredViewKindMissing));
}

#[test]
fn missing_data_mode_is_rejected() {
    let mut packet = baseline_packet();
    for profile in &mut packet.profiles {
        if profile.data_mode == CollectionDataMode::Virtualized {
            profile.data_mode = CollectionDataMode::Streaming;
        }
    }
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::RequiredDataModeMissing));
}

#[test]
fn missing_churn_event_is_rejected() {
    let mut packet = baseline_packet();
    // Drop every virtualization-recycle resilience entry across the packet.
    for profile in &mut packet.profiles {
        profile
            .churn_resilience
            .retain(|resilience| resilience.event != FocusChurnEvent::VirtualizationRecycle);
    }
    // Ensure at least one resilience entry remains per profile so other coverage
    // checks do not mask this one.
    for profile in &mut packet.profiles {
        if profile.churn_resilience.is_empty() {
            profile.churn_resilience.push(resilience(
                FocusChurnEvent::BackgroundRefresh,
                FocusDurabilityOutcome::FocusHeldByIdentity,
            ));
        }
    }
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::RequiredChurnEventMissing));
}

#[test]
fn missing_offscreen_case_is_rejected() {
    let mut packet = baseline_packet();
    for profile in &mut packet.profiles {
        profile.offscreen_durability.offscreen_selected_count = 0;
    }
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::OffscreenSelectionCaseMissing));
}

#[test]
fn missing_focus_reanchor_case_is_rejected() {
    let mut packet = baseline_packet();
    for profile in &mut packet.profiles {
        for resilience in &mut profile.churn_resilience {
            if resilience.outcome == FocusDurabilityOutcome::FocusReanchoredVisible {
                resilience.outcome = FocusDurabilityOutcome::FocusHeldByIdentity;
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::FocusReanchorCaseMissing));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = baseline_packet();
    packet.source_contract_refs = refs(&["schemas/collections/unrelated.schema.json"]);
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::MissingSourceContracts));
}

#[test]
fn guardrails_must_hold() {
    let mut packet = baseline_packet();
    packet.guardrails.no_pointer_only_selection_controls = false;
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_projection_must_hold() {
    let mut packet = baseline_packet();
    packet
        .consumer_projection
        .accessibility_evidence_reuses_records = false;
    assert!(packet
        .validate()
        .contains(&AssistiveSelectionParityViolation::ConsumerProjectionIncomplete));
}

#[test]
fn reconstruction_recovers_parity_truth() {
    let packet = baseline_packet();
    let reconstructions = packet.reconstructions();
    assert_eq!(reconstructions.len(), packet.profiles.len());

    let marketplace = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.profile_id == "p:marketplace")
        .expect("marketplace reconstruction present");
    assert_eq!(marketplace.offscreen_selected_count, 188);
    assert!(marketplace.all_commands_keyboard_reachable);
    assert!(marketplace.hidden_selected_count_exposed);
    assert!(marketplace.focus_and_selection_durable);
    assert_eq!(marketplace.command_tokens.len(), 5);

    let graph = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.profile_id == "p:graph")
        .expect("graph reconstruction present");
    assert_eq!(graph.focus_model_token, "aria_activedescendant");
    assert!(graph
        .churn_event_tokens
        .contains(&"virtualization_recycle".to_owned()));
}

#[test]
fn export_is_metadata_safe() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("bearer "));
    assert!(packet.validate().is_empty());
}

#[test]
fn record_kind_and_schema_version_are_pinned() {
    let packet = baseline_packet();
    assert_eq!(packet.record_kind, ASSISTIVE_SELECTION_PARITY_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        ASSISTIVE_SELECTION_PARITY_SCHEMA_VERSION
    );
}

#[test]
fn round_trips_through_json() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let parsed: AssistiveSelectionParityPacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_in_export_validates() {
    let packet = current_m5_assistive_selection_parity_export()
        .expect("checked-in assistive selection parity export parses and validates");
    assert_eq!(
        packet.packet_id,
        "m5-assistive-selection-parity:stable:0001"
    );
    assert!(packet.validate().is_empty());
    for required in REQUIRED_PARITY_SURFACES {
        assert!(packet.represented_surfaces().contains(&required));
    }
    for required in CollectionViewKind::ALL {
        assert!(packet.represented_view_kinds().contains(&required));
    }
    for required in REQUIRED_PARITY_DATA_MODES {
        assert!(packet.represented_data_modes().contains(&required));
    }
    for required in FocusModelKind::ALL {
        assert!(packet.represented_focus_models().contains(&required));
    }
    for required in REQUIRED_CHURN_EVENTS {
        assert!(packet.demonstrated_churn_events().contains(&required));
    }
    assert!(packet.offscreen_selection_profile_count() >= 1);
}
