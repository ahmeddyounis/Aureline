//! Canonical seed builders for the M5 focus-and-selection contract.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code contract, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical focus-and-selection contract.
pub const M5_FOCUS_SELECTION_CONTRACT_PACKET_ID: &str = "m5-focus-selection:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

/// Proof packet ref every governed zone carries.
const FOCUS_PROOF_REF: &str = "evidence:focus-return-conformance:m5";

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

fn focus_return(
    return_target_ref: &str,
    primary: A11yFocusReturnDisposition,
    safe_fallback: A11yFocusReturnDisposition,
    announces_return: bool,
) -> M5FocusReturnRule {
    M5FocusReturnRule {
        return_target_ref: return_target_ref.to_owned(),
        primary_disposition: primary,
        safe_fallback_disposition: safe_fallback,
        announces_return,
    }
}

/// Builds a stable-item-identity rule that preserves focus and selection across every
/// async class the model requires.
fn stable_identity(
    identity_strategy: M5IdentityStrategy,
    model: M5FocusInteractionModel,
) -> M5StableIdentityRule {
    M5StableIdentityRule {
        identity_strategy,
        preserves_focus: true,
        preserves_selection: true,
        preserved_across: model.required_async_classes().to_vec(),
    }
}

fn roving(navigation_keys: Vec<M5CollectionNavKey>) -> M5RovingTabindexRule {
    M5RovingTabindexRule {
        single_tab_stop: true,
        navigation_keys,
        multi_selection_narrowing_announced: true,
    }
}

/// The downgrade triggers every governed zone carries: both bridge degradation paths,
/// stale proof, and the two focus-safety regressions the contract guards against.
fn standard_downgrade_triggers() -> Vec<M5DynamicSurfaceA11yDowngradeTrigger> {
    use M5DynamicSurfaceA11yDowngradeTrigger as D;
    vec![
        D::ProofStale,
        D::BridgeUnavailable,
        D::BridgePartialOrStale,
        D::FocusTeleported,
        D::FocusLost,
    ]
}

#[allow(clippy::too_many_arguments)]
fn zone(
    zone_id: &str,
    zone_kind: M5FocusZoneKind,
    label: &str,
    keyboard_complete_claim: bool,
    focus_return: M5FocusReturnRule,
    identity_strategy: M5IdentityStrategy,
    roving_tabindex: Option<M5RovingTabindexRule>,
    durable_fallback: M5DurableFallbackRef,
    consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
) -> M5FocusZoneContract {
    let interaction_model = zone_kind.interaction_model();
    M5FocusZoneContract {
        zone_id: zone_id.to_owned(),
        zone_kind,
        interaction_model,
        label: label.to_owned(),
        owner_role: "Accessibility owner".to_owned(),
        qualification: M5DynamicSurfaceA11yQualificationClass::Stable,
        non_visual_fidelity: A11yNonVisualFidelity::FullAccessible,
        keyboard_complete_claim,
        focus_return,
        stable_identity: stable_identity(identity_strategy, interaction_model),
        roving_tabindex,
        durable_fallback,
        downgrade_triggers: standard_downgrade_triggers(),
        required_proof_packet_refs: strings(&[FOCUS_PROOF_REF]),
        source_contract_refs: strings(&[
            M5_FOCUS_SELECTION_FOCUS_CONTRACT_REF,
            M5_FOCUS_SELECTION_SCREEN_READER_CONTRACT_REF,
        ]),
        consumer_surfaces,
    }
}

fn zones() -> Vec<M5FocusZoneContract> {
    use A11yFocusReturnDisposition as Return;
    use M5CollectionNavKey as Key;
    use M5DurableFallbackSurface as Surface;
    use M5DynamicSurfaceA11yConsumerSurface as Consumer;
    use M5FocusZoneKind as Zone;
    use M5IdentityStrategy as Id;

    vec![
        // Transient overlays: return focus to the exact invoker; when it is gone, fall
        // back to a nearest safe ancestor or an announced re-entry point.
        zone(
            "focus-zone:modal-dialog",
            Zone::ModalDialog,
            "Modal dialog",
            true,
            focus_return(
                "surface:invoker.modal-dialog",
                Return::ReturnedExact,
                Return::ReturnedNearestSafeAncestor,
                false,
            ),
            Id::StableKey,
            None,
            fallback(Surface::StatusDetail, "status-detail:dialog-return"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        zone(
            "focus-zone:sheet",
            Zone::Sheet,
            "Sheet / drawer",
            true,
            focus_return(
                "surface:invoker.sheet",
                Return::ReturnedExact,
                Return::ReturnedNearestSafeAncestor,
                false,
            ),
            Id::StableKey,
            None,
            fallback(Surface::StatusDetail, "status-detail:sheet-return"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        zone(
            "focus-zone:command-palette",
            Zone::CommandPalette,
            "Command palette",
            true,
            focus_return(
                "surface:invoker.command-palette",
                Return::ReturnedExact,
                Return::ReturnedPlaceholderAnnounced,
                true,
            ),
            Id::StableKey,
            None,
            fallback(Surface::SelectionSummary, "selection-summary:palette"),
            vec![Consumer::Shell, Consumer::Editor, Consumer::SupportExport],
        ),
        zone(
            "focus-zone:popover",
            Zone::Popover,
            "Popover / menu",
            true,
            focus_return(
                "surface:invoker.popover",
                Return::ReturnedExact,
                Return::ReturnedNearestSafeAncestor,
                false,
            ),
            Id::StableKey,
            None,
            fallback(Surface::StatusDetail, "status-detail:popover-return"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        zone(
            "focus-zone:rename-field",
            Zone::RenameField,
            "Inline rename field",
            true,
            focus_return(
                "surface:invoker.rename-field",
                Return::ReturnedExact,
                Return::ReturnedCurrentBatchOrDetailOwner,
                false,
            ),
            Id::PathOrUri,
            None,
            fallback(Surface::SelectionSummary, "selection-summary:rename"),
            vec![Consumer::Editor, Consumer::DataGrid, Consumer::SupportExport],
        ),
        zone(
            "focus-zone:inspector-promotion",
            Zone::InspectorPromotion,
            "Inspector promotion",
            true,
            focus_return(
                "surface:invoker.inspector-promotion",
                Return::ReturnedExact,
                Return::ReturnedCurrentBatchOrDetailOwner,
                false,
            ),
            Id::StableKey,
            None,
            fallback(Surface::SelectionSummary, "selection-summary:inspector"),
            vec![Consumer::Shell, Consumer::Review, Consumer::SupportExport],
        ),
        // Dense collections: roving single tab stop, predictable navigation, identity
        // preserved across virtualization / refresh / streaming / filter / sort.
        zone(
            "focus-zone:dense-collection",
            Zone::DenseCollection,
            "Dense collection (list / grid / tree)",
            true,
            focus_return(
                "surface:collection.focused-row",
                Return::ReturnedExact,
                Return::ReturnedNearestSafeAncestor,
                false,
            ),
            Id::PathOrUri,
            Some(roving(vec![
                Key::ArrowUpDown,
                Key::ArrowLeftRight,
                Key::HomeEnd,
                Key::PageUpDown,
                Key::TypeAhead,
            ])),
            fallback(Surface::SelectionSummary, "selection-summary:collection"),
            vec![Consumer::DataGrid, Consumer::Review, Consumer::SupportExport],
        ),
        zone(
            "focus-zone:streamed-list",
            Zone::StreamedList,
            "Streamed-insert list",
            true,
            focus_return(
                "surface:streamed-list.focused-row",
                Return::ReturnedExact,
                Return::ReturnedCurrentBatchOrDetailOwner,
                false,
            ),
            Id::ContentHash,
            Some(roving(vec![Key::ArrowUpDown, Key::HomeEnd, Key::PageUpDown])),
            fallback(Surface::ActivityRow, "activity-row:streamed-list"),
            vec![Consumer::Terminal, Consumer::AiSurfaces, Consumer::SupportExport],
        ),
        // Persistent shell zone: focus restored across layout adjustment and refresh.
        zone(
            "focus-zone:shell-zone",
            Zone::ShellZone,
            "Shell layout zone",
            true,
            focus_return(
                "surface:shell.active-zone",
                Return::ReturnedExact,
                Return::ReturnedNearestSafeAncestor,
                false,
            ),
            Id::StableKey,
            None,
            fallback(Surface::StatusDetail, "status-detail:shell-zone"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        // Multi-window restore: focus and identity restored across window restore.
        zone(
            "focus-zone:multi-window-layout",
            Zone::MultiWindowLayout,
            "Multi-window restore / layout",
            true,
            focus_return(
                "surface:multi-window.restored-focus",
                Return::ReturnedExact,
                Return::ReturnedNearestSafeAncestor,
                false,
            ),
            Id::StableKey,
            None,
            fallback(Surface::StatusDetail, "status-detail:multi-window"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        // Follow / presentation: context preserved, focus returned on exit.
        zone(
            "focus-zone:follow-presentation",
            Zone::FollowPresentation,
            "Follow / presentation mode",
            true,
            focus_return(
                "surface:presentation.follow-anchor",
                Return::ReturnedExact,
                Return::ReturnedPlaceholderAnnounced,
                true,
            ),
            Id::StableKey,
            None,
            fallback(Surface::StatusDetail, "status-detail:follow-presentation"),
            vec![Consumer::Presentation, Consumer::SupportExport],
        ),
    ]
}

fn conformance_review() -> M5FocusSelectionConformanceReview {
    M5FocusSelectionConformanceReview {
        transient_surfaces_declare_explicit_focus_return: true,
        focus_never_teleports_or_vanishes_on_async_update: true,
        safe_fallback_when_invoking_object_gone: true,
        focus_and_selection_preserved_by_stable_item_identity: true,
        no_row_index_based_focus_loss_or_selection_drift: true,
        dense_collections_use_roving_single_tab_stop: true,
        predictable_arrow_home_end_page_navigation: true,
        no_silent_multi_selection_narrowing: true,
        overlays_return_to_safe_working_context: true,
        keyboard_complete_requires_focus_return_and_stable_identity: true,
        claimed_zones_auto_narrow_when_bridge_or_proof_stale: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> M5FocusSelectionConsumerProjection {
    M5FocusSelectionConsumerProjection {
        shell_returns_focus_on_overlay_teardown: true,
        search_palette_returns_focus_to_invoker: true,
        review_preserves_row_identity_across_refresh: true,
        data_grid_uses_roving_tabindex: true,
        notifications_route_focus_return_targets: true,
        presentation_follow_mode_preserves_context: true,
        multi_window_restore_preserves_identity: true,
        support_export_reuses_contract: true,
        docs_help_reuse_contract: true,
        at_conformance_packets_reuse_contract: true,
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
        release_packet_ref: "evidence:focus-return-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:focus-return-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_FOCUS_SELECTION_SCHEMA_REF,
        M5_FOCUS_SELECTION_DOC_REF,
        M5_FOCUS_SELECTION_MATRIX_REF,
        M5_FOCUS_SELECTION_SURFACE_DESCRIPTOR_REF,
        M5_FOCUS_SELECTION_FOCUS_CONTRACT_REF,
        M5_FOCUS_SELECTION_SCREEN_READER_CONTRACT_REF,
    ])
}

fn base_input() -> M5FocusSelectionContractPacketInput {
    M5FocusSelectionContractPacketInput {
        packet_id: M5_FOCUS_SELECTION_CONTRACT_PACKET_ID.to_owned(),
        contract_label: "M5 Focus-Return and Stable-Selection Contract".to_owned(),
        zones: zones(),
        shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        focus_vocabulary_set: M5FocusSelectionVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable focus-and-selection contract packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_focus_selection_contract() -> M5FocusSelectionContractPacket {
    M5FocusSelectionContractPacket::new(base_input())
}

/// Builds a narrowed variant where the dense-collection zone's assistive-tech proof
/// has gone stale, proving the zone narrows from Stable to Beta and drops its
/// keyboard-complete claim while keeping its focus-return rule, stable-item-identity
/// rule, roving tabindex, and `proof_stale` trigger intact — the narrowing is a
/// disclosed claim change, never a hidden zone.
pub fn seeded_m5_focus_selection_contract_proof_stale_narrowed() -> M5FocusSelectionContractPacket {
    let mut input = base_input();
    input.packet_id = "m5-focus-selection:proof-stale-narrowed:0001".to_owned();
    for zone in &mut input.zones {
        if zone.zone_kind == M5FocusZoneKind::DenseCollection {
            zone.qualification = M5DynamicSurfaceA11yQualificationClass::Beta;
            // A non-stable zone cannot carry the public keyboard-complete claim.
            zone.keyboard_complete_claim = false;
        }
    }
    M5FocusSelectionContractPacket::new(input)
}

/// Builds a narrowed variant where the multi-window-layout zone's OS accessibility
/// bridge is unavailable, proving the zone narrows from Stable to Preview, drops its
/// non-visual fidelity to `degraded_accessible`, and drops its keyboard-complete claim
/// while keeping its stable-item-identity rule, safe focus-return fallback, and
/// `bridge_unavailable` trigger — restored windows still preserve item identity rather
/// than degrading into row-index focus loss.
pub fn seeded_m5_focus_selection_contract_bridge_unavailable_narrowed(
) -> M5FocusSelectionContractPacket {
    let mut input = base_input();
    input.packet_id = "m5-focus-selection:bridge-unavailable-narrowed:0001".to_owned();
    for zone in &mut input.zones {
        if zone.zone_kind == M5FocusZoneKind::MultiWindowLayout {
            zone.qualification = M5DynamicSurfaceA11yQualificationClass::Preview;
            zone.non_visual_fidelity = A11yNonVisualFidelity::DegradedAccessible;
            zone.keyboard_complete_claim = false;
        }
    }
    M5FocusSelectionContractPacket::new(input)
}
