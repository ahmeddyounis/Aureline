//! Canonical seed builders for the M5 accessibility-surface descriptor catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so
//! the in-code catalog, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical descriptor catalog.
pub const M5_SURFACE_DESCRIPTOR_CATALOG_PACKET_ID: &str =
    "m5-accessibility-surface-descriptors:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn region(
    region_id: &str,
    role_class: A11ySemanticRoleClass,
    label: &str,
    is_landmark: bool,
) -> M5SurfaceRegion {
    M5SurfaceRegion {
        region_id: region_id.to_owned(),
        role_class,
        label: label.to_owned(),
        is_landmark,
    }
}

fn focus_stop(
    order_index: u32,
    region_id: &str,
    role_class: A11ySemanticRoleClass,
    focusable: bool,
) -> M5SurfaceFocusStop {
    M5SurfaceFocusStop {
        order_index,
        region_id: region_id.to_owned(),
        role_class,
        focusable,
    }
}

fn label_model(
    label_model_id: &str,
    name_source: M5SurfaceNameSource,
    state_label_classes: Vec<M5SurfaceStateLabelClass>,
    fallback_durability: A11yFallbackDurability,
    non_visual_fidelity: A11yNonVisualFidelity,
) -> M5SurfaceLabelModel {
    M5SurfaceLabelModel {
        label_model_id: label_model_id.to_owned(),
        name_source,
        state_label_classes,
        fallback_durability,
        non_visual_fidelity,
    }
}

fn native_role_hint(uia: &str, axc: &str, atspi: &str) -> M5SurfaceNativeRoleHint {
    M5SurfaceNativeRoleHint {
        ui_automation: Some(uia.to_owned()),
        ns_accessibility: Some(axc.to_owned()),
        at_spi: Some(atspi.to_owned()),
    }
}

/// A healthy, bridged-active mapping with no disclosed degradation.
fn bridged_active(
    bridge_kind: M5SurfaceBridgeKind,
    non_visual_fidelity: A11yNonVisualFidelity,
    native_role_hint: M5SurfaceNativeRoleHint,
) -> M5SurfaceBridgeMapping {
    M5SurfaceBridgeMapping {
        bridge_kind,
        bridge_state: A11yBridgeState::BridgedActive,
        non_visual_fidelity,
        native_role_hint,
        degradation_reason: M5BridgeDegradationReason::None,
    }
}

#[allow(clippy::too_many_arguments)]
fn descriptor(
    surface_id: &str,
    surface_family: M5SurfaceFamily,
    surface_label: &str,
    qualification: M5DynamicSurfaceA11yQualificationClass,
    primary_role_class: A11ySemanticRoleClass,
    regions: Vec<M5SurfaceRegion>,
    label_model: M5SurfaceLabelModel,
    focus_order: M5SurfaceFocusOrder,
    motion_zoom: M5SurfaceMotionZoomPosture,
    bridge_mapping: M5SurfaceBridgeMapping,
    live_announcement: M5SurfaceLiveAnnouncement,
    downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    required_proof_packet_refs: &[&str],
    source_contract_refs: &[&str],
    consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
) -> M5AccessibilitySurfaceDescriptor {
    M5AccessibilitySurfaceDescriptor {
        surface_id: surface_id.to_owned(),
        surface_family,
        surface_label: surface_label.to_owned(),
        owner_role: "Accessibility owner".to_owned(),
        qualification,
        primary_role_class,
        regions,
        label_model,
        focus_order,
        motion_zoom,
        bridge_mapping,
        live_announcement,
        downgrade_triggers,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        source_contract_refs: strings(source_contract_refs),
        consumer_surfaces,
    }
}

fn descriptors() -> Vec<M5AccessibilitySurfaceDescriptor> {
    use A11ySemanticRoleClass as R;
    use M5DynamicSurfaceA11yConsumerSurface as S;
    use M5DynamicSurfaceA11yDowngradeTrigger as D;

    vec![
        // Shell zones / landmark regions.
        descriptor(
            "surface:shell.zone-frame",
            M5SurfaceFamily::ShellRegion,
            "Shell zone frame",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::LandmarkRegion,
            vec![
                region(
                    "region:shell.primary-side",
                    R::LandmarkRegion,
                    "Primary side dock",
                    true,
                ),
                region(
                    "region:shell.workbench",
                    R::LandmarkRegion,
                    "Workbench",
                    true,
                ),
                region(
                    "region:shell.status-bar",
                    R::StatusRegion,
                    "Status bar",
                    true,
                ),
            ],
            label_model(
                "label:shell.zone-frame",
                M5SurfaceNameSource::MessageIdSource,
                vec![
                    M5SurfaceStateLabelClass::Freshness,
                    M5SurfaceStateLabelClass::TrustOrPolicy,
                ],
                A11yFallbackDurability::OnFocus,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:shell.zone-frame".to_owned(),
                stops: vec![
                    focus_stop(0, "region:shell.primary-side", R::LandmarkRegion, true),
                    focus_stop(1, "region:shell.workbench", R::LandmarkRegion, true),
                    focus_stop(2, "region:shell.status-bar", R::StatusRegion, true),
                ],
                async_return_disposition: A11yFocusReturnDisposition::ReturnedExact,
                return_fallback_durability: A11yFallbackDurability::OnFocus,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::CrossfadeReplacedWithInstant,
                high_zoom: M5HighZoomPosture::ReflowsToSingleColumn,
                behavior_changes_under_reduced_motion: true,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("Group", "AXGroup", "ROLE_PANEL"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Polite,
                coalescing: A11yCoalescingStrategy::DedupeSameMeaning,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::LabelOrRoleDrift,
                D::PointerOrHoverDependence,
                D::ProofStale,
            ],
            &["evidence:accessibility-surface-descriptor-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_SHELL_BRIDGE_CONTRACT_REF,
            ],
            vec![S::Shell, S::SupportExport, S::Help],
        ),
        // Editor content canvas.
        descriptor(
            "surface:editor.content-canvas",
            M5SurfaceFamily::EditorCanvas,
            "Editor content canvas",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::TextDocument,
            vec![
                region(
                    "region:editor.document",
                    R::TextDocument,
                    "Editor document",
                    true,
                ),
                region(
                    "region:editor.gutter",
                    R::StructureGroup,
                    "Editor gutter",
                    false,
                ),
                region(
                    "region:editor.diagnostics",
                    R::StatusRegion,
                    "Inline diagnostics",
                    false,
                ),
            ],
            label_model(
                "label:editor.content-canvas",
                M5SurfaceNameSource::DocumentOrSymbolLabel,
                vec![
                    M5SurfaceStateLabelClass::Severity,
                    M5SurfaceStateLabelClass::SelectionScope,
                    M5SurfaceStateLabelClass::TrustOrPolicy,
                ],
                A11yFallbackDurability::DurableSurfaceOnly,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:editor.content-canvas".to_owned(),
                stops: vec![
                    focus_stop(0, "region:editor.document", R::TextDocument, true),
                    focus_stop(1, "region:editor.gutter", R::StructureGroup, true),
                    focus_stop(2, "region:editor.diagnostics", R::StatusRegion, true),
                ],
                async_return_disposition: A11yFocusReturnDisposition::ReturnedExact,
                return_fallback_durability: A11yFallbackDurability::OnFocus,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::AnimationDisabledStatePreserved,
                high_zoom: M5HighZoomPosture::ScrollsWithoutClipping,
                behavior_changes_under_reduced_motion: true,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("Document", "AXTextArea", "ROLE_DOCUMENT_TEXT"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Polite,
                coalescing: A11yCoalescingStrategy::LastMeaningWinsWithCount,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::LabelOrRoleDrift,
                D::FocusLost,
                D::ProofStale,
            ],
            &["evidence:screen-reader-label-model-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF,
            ],
            vec![S::Editor, S::SupportExport, S::Review],
        ),
        // Terminal / log canvas.
        descriptor(
            "surface:terminal.log-canvas",
            M5SurfaceFamily::TerminalCanvas,
            "Terminal log canvas",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::LiveLogRegion,
            vec![
                region(
                    "region:terminal.scrollback",
                    R::LiveLogRegion,
                    "Terminal scrollback",
                    true,
                ),
                region(
                    "region:terminal.prompt",
                    R::InteractiveControl,
                    "Terminal prompt",
                    false,
                ),
            ],
            label_model(
                "label:terminal.log-canvas",
                M5SurfaceNameSource::GeneratedSummary,
                vec![
                    M5SurfaceStateLabelClass::LiveRegion,
                    M5SurfaceStateLabelClass::Severity,
                ],
                A11yFallbackDurability::Coalesced,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:terminal.log-canvas".to_owned(),
                stops: vec![
                    focus_stop(0, "region:terminal.scrollback", R::LiveLogRegion, true),
                    focus_stop(1, "region:terminal.prompt", R::InteractiveControl, true),
                ],
                async_return_disposition:
                    A11yFocusReturnDisposition::ReturnedCurrentBatchOrDetailOwner,
                return_fallback_durability: A11yFallbackDurability::OnFocus,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::MotionIndependentAlready,
                high_zoom: M5HighZoomPosture::ScrollsWithoutClipping,
                behavior_changes_under_reduced_motion: false,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("Document", "AXTextArea", "ROLE_TERMINAL"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Polite,
                coalescing: A11yCoalescingStrategy::StartAndTerminalOnly,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::LiveRegionSpam,
                D::AnnouncementMeaningLost,
                D::ProofStale,
            ],
            &["evidence:live-announcement-class-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_SHELL_BRIDGE_CONTRACT_REF,
            ],
            vec![S::Terminal, S::SupportExport],
        ),
        // Dense list / table / data-grid collection.
        descriptor(
            "surface:data.dense-collection",
            M5SurfaceFamily::DenseCollection,
            "Dense data collection",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::DataGridCell,
            vec![
                region("region:data.grid", R::StructureGroup, "Data grid", true),
                region(
                    "region:data.row-header",
                    R::DataGridCell,
                    "Row header",
                    false,
                ),
                region(
                    "region:data.summary",
                    R::StatusRegion,
                    "Collection summary",
                    false,
                ),
            ],
            label_model(
                "label:data.dense-collection",
                M5SurfaceNameSource::RowOrCellIdentity,
                vec![
                    M5SurfaceStateLabelClass::SelectionScope,
                    M5SurfaceStateLabelClass::Virtualization,
                ],
                A11yFallbackDurability::Coalesced,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:data.dense-collection".to_owned(),
                stops: vec![
                    focus_stop(0, "region:data.grid", R::DataGridCell, true),
                    focus_stop(1, "region:data.summary", R::StatusRegion, true),
                ],
                async_return_disposition: A11yFocusReturnDisposition::ReturnedNearestSafeAncestor,
                return_fallback_durability: A11yFallbackDurability::OnFocus,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::MotionIndependentAlready,
                high_zoom: M5HighZoomPosture::ScrollsWithoutClipping,
                behavior_changes_under_reduced_motion: false,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("DataGrid", "AXTable", "ROLE_TABLE"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Polite,
                coalescing: A11yCoalescingStrategy::LastMeaningWinsWithCount,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::NonVisualFidelityLost,
                D::PointerOrHoverDependence,
                D::ProofStale,
            ],
            &["evidence:dense-surface-non-visual-summary-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF,
            ],
            vec![S::DataGrid, S::Review, S::SupportExport],
        ),
        // Notebook cell (input + output).
        descriptor(
            "surface:notebook.cell",
            M5SurfaceFamily::NotebookCell,
            "Notebook cell",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::NotebookCell,
            vec![
                region("region:notebook.input", R::NotebookCell, "Cell input", true),
                region(
                    "region:notebook.output",
                    R::StatusRegion,
                    "Cell output",
                    false,
                ),
            ],
            label_model(
                "label:notebook.cell",
                M5SurfaceNameSource::DocumentOrSymbolLabel,
                vec![
                    M5SurfaceStateLabelClass::Severity,
                    M5SurfaceStateLabelClass::Freshness,
                ],
                A11yFallbackDurability::DurableSurfaceOnly,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:notebook.cell".to_owned(),
                stops: vec![
                    focus_stop(0, "region:notebook.input", R::NotebookCell, true),
                    focus_stop(1, "region:notebook.output", R::StatusRegion, true),
                ],
                async_return_disposition:
                    A11yFocusReturnDisposition::ReturnedCurrentBatchOrDetailOwner,
                return_fallback_durability: A11yFallbackDurability::OnFocus,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::NoAnimation,
                high_zoom: M5HighZoomPosture::ReflowsToSingleColumn,
                behavior_changes_under_reduced_motion: true,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("Group", "AXGroup", "ROLE_SECTION"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Polite,
                coalescing: A11yCoalescingStrategy::LastMeaningWinsWithCount,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::FocusLost,
                D::AnnouncementMeaningLost,
                D::ProofStale,
            ],
            &["evidence:focus-return-contract-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF,
            ],
            vec![S::Notebook, S::SupportExport],
        ),
        // Data-surface cell.
        descriptor(
            "surface:data.cell",
            M5SurfaceFamily::DataCell,
            "Data surface cell",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::DataGridCell,
            vec![
                region("region:data.cell-body", R::DataGridCell, "Cell body", true),
                region(
                    "region:data.cell-state",
                    R::StatusRegion,
                    "Cell state",
                    false,
                ),
            ],
            label_model(
                "label:data.cell",
                M5SurfaceNameSource::RowOrCellIdentity,
                vec![
                    M5SurfaceStateLabelClass::Support,
                    M5SurfaceStateLabelClass::Freshness,
                ],
                A11yFallbackDurability::Coalesced,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:data.cell".to_owned(),
                stops: vec![focus_stop(
                    0,
                    "region:data.cell-body",
                    R::DataGridCell,
                    true,
                )],
                async_return_disposition: A11yFocusReturnDisposition::ReturnedNearestSafeAncestor,
                return_fallback_durability: A11yFallbackDurability::OnFocus,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::MotionIndependentAlready,
                high_zoom: M5HighZoomPosture::ContentScalesWithContainer,
                behavior_changes_under_reduced_motion: false,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("DataItem", "AXCell", "ROLE_TABLE_CELL"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Polite,
                coalescing: A11yCoalescingStrategy::DedupeSameMeaning,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::NonVisualFidelityLost,
                D::LabelOrRoleDrift,
                D::ProofStale,
            ],
            &["evidence:accessibility-surface-descriptor-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_SHELL_BRIDGE_CONTRACT_REF,
            ],
            vec![S::DataGrid, S::SupportExport],
        ),
        // Review / diff hunk surface.
        descriptor(
            "surface:review.diff-hunk",
            M5SurfaceFamily::ReviewDiff,
            "Review diff hunk",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::StructureGroup,
            vec![
                region("region:review.hunk", R::StructureGroup, "Diff hunk", true),
                region(
                    "region:review.gutter",
                    R::StructureGroup,
                    "Change gutter",
                    false,
                ),
                region(
                    "region:review.comments",
                    R::StatusRegion,
                    "Review comments",
                    false,
                ),
            ],
            label_model(
                "label:review.diff-hunk",
                M5SurfaceNameSource::DocumentOrSymbolLabel,
                vec![
                    M5SurfaceStateLabelClass::Severity,
                    M5SurfaceStateLabelClass::SelectionScope,
                ],
                A11yFallbackDurability::DurableSurfaceOnly,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:review.diff-hunk".to_owned(),
                stops: vec![
                    focus_stop(0, "region:review.hunk", R::StructureGroup, true),
                    focus_stop(1, "region:review.comments", R::StatusRegion, true),
                ],
                async_return_disposition: A11yFocusReturnDisposition::ReturnedExact,
                return_fallback_durability: A11yFallbackDurability::OnFocus,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::MotionIndependentAlready,
                high_zoom: M5HighZoomPosture::ReflowsToSingleColumn,
                behavior_changes_under_reduced_motion: false,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("Group", "AXGroup", "ROLE_SECTION"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Polite,
                coalescing: A11yCoalescingStrategy::DedupeSameMeaning,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::LabelOrRoleDrift,
                D::FocusLost,
                D::ProofStale,
            ],
            &["evidence:screen-reader-label-model-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF,
            ],
            vec![S::Review, S::SupportExport],
        ),
        // Durable overlay / sheet / modal surface.
        descriptor(
            "surface:shell.overlay-sheet",
            M5SurfaceFamily::OverlaySheet,
            "Durable overlay sheet",
            M5DynamicSurfaceA11yQualificationClass::Stable,
            R::StructureGroup,
            vec![
                region(
                    "region:overlay.dialog",
                    R::StructureGroup,
                    "Overlay dialog",
                    true,
                ),
                region(
                    "region:overlay.actions",
                    R::InteractiveControl,
                    "Overlay actions",
                    false,
                ),
            ],
            label_model(
                "label:shell.overlay-sheet",
                M5SurfaceNameSource::MessageIdSource,
                vec![
                    M5SurfaceStateLabelClass::TrustOrPolicy,
                    M5SurfaceStateLabelClass::Severity,
                ],
                A11yFallbackDurability::Immediate,
                A11yNonVisualFidelity::FullAccessible,
            ),
            M5SurfaceFocusOrder {
                focus_contract_id: "focus:shell.overlay-sheet".to_owned(),
                stops: vec![
                    focus_stop(0, "region:overlay.dialog", R::StructureGroup, true),
                    focus_stop(1, "region:overlay.actions", R::InteractiveControl, true),
                ],
                async_return_disposition: A11yFocusReturnDisposition::ReturnedPlaceholderAnnounced,
                return_fallback_durability: A11yFallbackDurability::DurableSurfaceOnly,
            },
            M5SurfaceMotionZoomPosture {
                reduced_motion: M5ReducedMotionPosture::CrossfadeReplacedWithInstant,
                high_zoom: M5HighZoomPosture::ContentScalesWithContainer,
                behavior_changes_under_reduced_motion: true,
                behavior_changes_under_high_zoom: true,
            },
            bridged_active(
                M5SurfaceBridgeKind::UiAutomation,
                A11yNonVisualFidelity::FullAccessible,
                native_role_hint("Window", "AXSheet", "ROLE_DIALOG"),
            ),
            M5SurfaceLiveAnnouncement {
                politeness: A11yAnnouncementPoliteness::Assertive,
                coalescing: A11yCoalescingStrategy::FocusedSurfaceOnly,
            },
            vec![
                D::BridgePartialOrStale,
                D::BridgeUnavailable,
                D::FocusTeleported,
                D::FocusLost,
                D::ProofStale,
            ],
            &["evidence:focus-return-contract-conformance:m5"],
            &[
                M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF,
                M5_SURFACE_DESCRIPTOR_VISUAL_ADAPTATION_CONTRACT_REF,
            ],
            vec![S::Shell, S::Presentation, S::SupportExport],
        ),
    ]
}

fn conformance_review() -> M5SurfaceDescriptorConformanceReview {
    M5SurfaceDescriptorConformanceReview {
        every_claimed_surface_has_descriptor: true,
        descriptors_expose_roles_and_regions: true,
        descriptors_carry_screen_reader_label_model: true,
        descriptors_carry_focus_order_metadata: true,
        focus_never_teleports_or_vanishes_on_async_update: true,
        reduced_motion_and_high_zoom_declared_when_behavior_changes: true,
        bridge_health_disclosed_not_hidden: true,
        one_descriptor_contract_not_per_surface_handwiring: true,
        descriptors_reused_in_diagnostics_support_docs_and_proof: true,
        claimed_descriptors_auto_narrow_when_bridge_or_proof_stale: true,
        no_pixel_only_or_pointer_only_source_of_truth: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> M5SurfaceDescriptorConsumerProjection {
    M5SurfaceDescriptorConsumerProjection {
        shell_consumes_descriptors: true,
        editor_consumes_descriptors: true,
        terminal_consumes_descriptors: true,
        notebook_consumes_descriptors: true,
        data_grid_consumes_descriptors: true,
        review_consumes_descriptors: true,
        diagnostics_reuse_descriptors: true,
        support_export_reuses_descriptors: true,
        docs_help_reuse_descriptors: true,
        at_conformance_packets_reuse_descriptors: true,
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
        release_packet_ref: "evidence:surface-descriptor-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:surface-descriptor-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SURFACE_DESCRIPTOR_SCHEMA_REF,
        M5_SURFACE_DESCRIPTOR_DOC_REF,
        M5_SURFACE_DESCRIPTOR_MATRIX_REF,
        M5_SURFACE_DESCRIPTOR_TREE_CONTRACT_REF,
        M5_SURFACE_DESCRIPTOR_FOCUS_CONTRACT_REF,
        M5_SURFACE_DESCRIPTOR_VISUAL_ADAPTATION_CONTRACT_REF,
        M5_SURFACE_DESCRIPTOR_SHELL_BRIDGE_CONTRACT_REF,
    ])
}

fn base_input() -> M5SurfaceDescriptorCatalogPacketInput {
    M5SurfaceDescriptorCatalogPacketInput {
        packet_id: M5_SURFACE_DESCRIPTOR_CATALOG_PACKET_ID.to_owned(),
        catalog_label: "M5 Accessibility-Surface Descriptors and Bridge Mappings".to_owned(),
        descriptors: descriptors(),
        shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        descriptor_vocabulary_set: M5SurfaceDescriptorVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable descriptor catalog packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_surface_descriptor_catalog() -> M5SurfaceDescriptorCatalogPacket {
    M5SurfaceDescriptorCatalogPacket::new(base_input())
}

/// Builds a narrowed variant where the terminal canvas's OS accessibility bridge
/// has gone partial, proving the descriptor discloses the degradation and narrows
/// its claim rather than implying silent screen-reader completeness.
pub fn seeded_m5_surface_descriptor_catalog_bridge_degraded() -> M5SurfaceDescriptorCatalogPacket {
    let mut input = base_input();
    input.packet_id = "m5-accessibility-surface-descriptors:bridge-degraded:0001".to_owned();
    for descriptor in &mut input.descriptors {
        if descriptor.surface_family == M5SurfaceFamily::TerminalCanvas {
            descriptor.qualification = M5DynamicSurfaceA11yQualificationClass::Preview;
            descriptor.bridge_mapping.bridge_state = A11yBridgeState::Partial;
            descriptor.bridge_mapping.non_visual_fidelity =
                A11yNonVisualFidelity::DegradedAccessible;
            descriptor.bridge_mapping.degradation_reason =
                M5BridgeDegradationReason::PartialTreeMapping;
            descriptor.label_model.non_visual_fidelity = A11yNonVisualFidelity::DegradedAccessible;
        }
    }
    M5SurfaceDescriptorCatalogPacket::new(input)
}

/// Builds a narrowed variant where the editor canvas's assistive-tech proof has
/// gone stale, proving the descriptor narrows from Stable to Beta and keeps the
/// surface visible with its proof-stale trigger intact.
pub fn seeded_m5_surface_descriptor_catalog_proof_stale_narrowed(
) -> M5SurfaceDescriptorCatalogPacket {
    let mut input = base_input();
    input.packet_id = "m5-accessibility-surface-descriptors:proof-stale-narrowed:0001".to_owned();
    for descriptor in &mut input.descriptors {
        if descriptor.surface_family == M5SurfaceFamily::EditorCanvas {
            descriptor.qualification = M5DynamicSurfaceA11yQualificationClass::Beta;
        }
    }
    M5SurfaceDescriptorCatalogPacket::new(input)
}
