//! Deterministic claimed-stable matrix for window-topology restore records.
//!
//! Every record here is a genuine projection of the live workspace-management
//! page in [`crate::windows`] and the restore-provenance contract in
//! [`crate::restore`]. The corpus reads
//! [`crate::windows::seeded_windows_beta_page`] for the contributing case ids and
//! the live topology vocabulary, then reconciles each window reopen through the
//! governed [`WindowTopologyRestoreRecord::build`] builder, so a record can never
//! drift from the live windows projection or the restore contract.
//!
//! Five postures pin the matrix:
//!
//! - `exact_single_window` — a primary window with a clean editor split that
//!   reopens exactly, with workspace authority and window topology separated and
//!   recovery chrome fully reachable. Qualifies **Stable**.
//! - `mixed_dpi_multi_monitor_compatible` — a window reopened after a mixed-DPI
//!   dock change; bounds and scale are adjusted and the editor panes hydrate
//!   live, so the reopen is Compatible with a recorded display-topology
//!   downgrade. Qualifies **Stable**.
//! - `monitor_removed_placeholder_backed` — a window reopened after an external
//!   display detached and its M5 session runtimes did not survive; notebook,
//!   query-console, profiler, pipeline, preview-route, docs, incident, and
//!   terminal panes keep their slots with in-place placeholder cards and
//!   restore-no-rerun guards. Qualifies **Stable** by proving placeholder
//!   honesty across the expanded surface set.
//! - `help_about_preview_surface` — the exact reopen, but the Help/About binding
//!   surface marker is Preview, so the reopen narrows below Stable by its lowest
//!   binding surface marker instead of inheriting an adjacent green row.
//! - `authority_topology_leak_drill` — an adversarial reopen where window
//!   topology leaked into shared workspace authority; the lane detects the fusion
//!   and narrows the reopen below Stable with a named reason.

use crate::notification_attention_stable::model::{
    AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, RecoveryRouteRecord, StableClaimClass,
};
use crate::restore::provenance::RESTORE_PROVENANCE_RECORD_KIND;
use crate::windows::{
    seeded_windows_beta_page, RestoreAdjustmentClass, SplitAxisClass, TopologyChangeClass,
    WindowRoleClass, WINDOWS_BETA_SHARED_CONTRACT_REF,
};

use super::model::{
    required_recovery_routes, AuthoritySeparation, DisplayTopologyProvenance, PaneHydrationClass,
    PanePlaceholderState, PaneRecoveryAction, PaneSlot, PaneSubstitutionReason, PaneSurfaceClass,
    PaneTree, PaneTreeNode, RecoveryChromeAssurance, RestoreDowngrade, RestoreDowngradeReason,
    RestoreFidelityClass, RestoreProvenance, RestoreSurfaceProjectionInput, RestoreTruthSurface,
    WindowLocalTopologyClass, WindowRestoreClaimCeiling, WindowRestoreUpstream,
    WindowTopologyRestoreInput, WindowTopologyRestoreRecord, WorkspaceAuthorityClass,
    PANE_TREE_SCHEMA_VERSION,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/window-topology-restore";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/window-topology-restore";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-window-topology-restore";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-window-topology-restore";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-window-topology-restore";

/// One scenario in the claimed-stable window-restore matrix.
#[derive(Debug, Clone)]
pub struct WindowTopologyRestoreScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Posture token pinned for the scenario.
    pub expected_posture: String,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest binding surface).
    pub expected_surface_marker: LifecycleMarker,
    /// Expected resulting restore fidelity.
    pub expected_fidelity: RestoreFidelityClass,
    record: WindowTopologyRestoreRecord,
}

impl WindowTopologyRestoreScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> WindowTopologyRestoreRecord {
        self.record.clone()
    }
}

/// The claimed-stable window-restore matrix, in canonical order.
pub fn window_topology_restore_corpus() -> Vec<WindowTopologyRestoreScenario> {
    vec![
        exact_single_window(),
        mixed_dpi_multi_monitor_compatible(),
        monitor_removed_placeholder_backed(),
        help_about_preview_surface(),
        authority_topology_leak_drill(),
    ]
}

// ---------------------------------------------------------------------------
// Shared builders
// ---------------------------------------------------------------------------

fn workspace_authority_classes() -> Vec<WorkspaceAuthorityClass> {
    WorkspaceAuthorityClass::REQUIRED.to_vec()
}

fn window_local_topology_classes() -> Vec<WindowLocalTopologyClass> {
    WindowLocalTopologyClass::REQUIRED.to_vec()
}

fn separation(centralized: bool, window_local: bool) -> AuthoritySeparation {
    AuthoritySeparation {
        workspace_authority_ref: "aureline://workspace-authority/payments".to_string(),
        workspace_authority_classes: workspace_authority_classes(),
        window_local_topology_classes: window_local_topology_classes(),
        workspace_authority_centralized: centralized,
        topology_window_local: window_local,
    }
}

fn editor_slot(pane_id: &str, title: &str) -> PaneSlot {
    PaneSlot {
        pane_id: pane_id.to_string(),
        surface_class: PaneSurfaceClass::Editor,
        title_hint: Some(title.to_string()),
        hydration: PaneHydrationClass::HydratedLive,
        placeholder_state: None,
        substitution_reason: None,
        runtime_survived: true,
        command_rerun_forbidden: false,
        authority_reacquire_forbidden: false,
        reopen_target_ref: format!("aureline://pane/{pane_id}"),
        evidence_ref: None,
        recovery_actions: Vec::new(),
        note: format!(
            "Editor {title} re-read truthfully from durable buffer state; no side effect."
        ),
    }
}

fn diff_slot(pane_id: &str, title: &str) -> PaneSlot {
    PaneSlot {
        pane_id: pane_id.to_string(),
        surface_class: PaneSurfaceClass::Diff,
        title_hint: Some(title.to_string()),
        hydration: PaneHydrationClass::HydratedLive,
        placeholder_state: None,
        substitution_reason: None,
        runtime_survived: true,
        command_rerun_forbidden: false,
        authority_reacquire_forbidden: false,
        reopen_target_ref: format!("aureline://pane/{pane_id}"),
        evidence_ref: None,
        recovery_actions: Vec::new(),
        note: format!(
            "Diff/review {title} re-read truthfully from the change object; no side effect."
        ),
    }
}

fn placeholder_slot(
    pane_id: &str,
    surface_class: PaneSurfaceClass,
    title: &str,
    placeholder_state: PanePlaceholderState,
    substitution_reason: PaneSubstitutionReason,
    recovery_actions: Vec<PaneRecoveryAction>,
    note: &str,
) -> PaneSlot {
    PaneSlot {
        pane_id: pane_id.to_string(),
        surface_class,
        title_hint: Some(title.to_string()),
        hydration: PaneHydrationClass::PlaceholderCard,
        placeholder_state: Some(placeholder_state),
        substitution_reason: Some(substitution_reason),
        runtime_survived: false,
        command_rerun_forbidden: true,
        authority_reacquire_forbidden: true,
        reopen_target_ref: format!("aureline://pane/{pane_id}"),
        evidence_ref: Some(format!("aureline://restore-evidence/{pane_id}")),
        recovery_actions,
        note: note.to_string(),
    }
}

fn surface_inputs(help_about_marker: LifecycleMarker) -> Vec<RestoreSurfaceProjectionInput> {
    vec![
        RestoreSurfaceProjectionInput {
            surface: RestoreTruthSurface::DesktopRestoreReview,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
        RestoreSurfaceProjectionInput {
            surface: RestoreTruthSurface::CliInspect,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
        RestoreSurfaceProjectionInput {
            surface: RestoreTruthSurface::HelpAbout,
            surface_marker: help_about_marker,
            reads_shared_record: true,
        },
        RestoreSurfaceProjectionInput {
            surface: RestoreTruthSurface::DiagnosticsSupportExport,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
    ]
}

fn entry_routes(window_id: &str) -> Vec<EntryRouteRecord> {
    AttentionRouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!("aureline://restore-window/{window_id}#{}", surface.as_str()),
            keyboard_reachable: true,
            activates_same_item: true,
        })
        .collect()
}

fn accessibility(routes: &[RecoveryRouteRecord]) -> AccessibilityDisclosure {
    AccessibilityDisclosure {
        focus_order_index: 0,
        tab_stop_count: routes.len() as u32 + 1,
        row_narration:
            "Window restore review, owned by the desktop shell; lists restore fidelity, pane \
             placeholder states, and recovery actions."
                .to_string(),
        action_labels: routes.iter().map(|r| r.action_label.clone()).collect(),
        layout_modes: LayoutMode::REQUIRED
            .into_iter()
            .map(|mode| LayoutModeDisclosure {
                mode,
                row_narration_available: true,
                recovery_affordances_reachable: true,
            })
            .collect(),
    }
}

fn full_claim_ceiling() -> WindowRestoreClaimCeiling {
    WindowRestoreClaimCeiling {
        asserts_authority_topology_separated: true,
        asserts_pane_tree_versioned: true,
        asserts_skeleton_first_hydrate_second: true,
        asserts_no_silent_rerun: true,
        asserts_provenance_export_safe: true,
        asserts_recovery_chrome_reachable: true,
    }
}

fn full_recovery_chrome() -> RecoveryChromeAssurance {
    RecoveryChromeAssurance {
        title_context_visible: true,
        restore_details_reachable: true,
        command_palette_reachable: true,
        keyboard_focus_reachable: true,
        activity_center_visible: true,
    }
}

fn upstream(case_refs: Vec<String>) -> WindowRestoreUpstream {
    WindowRestoreUpstream {
        windows_page_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_string(),
        restore_provenance_kind_ref: RESTORE_PROVENANCE_RECORD_KIND.to_string(),
        contributing_case_refs: case_refs,
    }
}

/// Pulls real case ids from the live windows page so the corpus is a genuine
/// projection of the shipped workspace-management projection.
fn live_case_refs(predicate: impl Fn(&str) -> bool) -> Vec<String> {
    let page = seeded_windows_beta_page();
    let mut refs: Vec<String> = Vec::new();
    for split in &page.split_intents {
        if predicate(&split.case_id) {
            refs.push(split.case_id.clone());
        }
    }
    for detach in &page.detach_intents {
        if predicate(&detach.case_id) {
            refs.push(detach.case_id.clone());
        }
    }
    for outcome in &page.restore_outcomes {
        if predicate(&outcome.case_id) {
            refs.push(outcome.case_id.clone());
        }
    }
    refs
}

fn finish(
    scenario_id: &'static str,
    expected_claim_class: StableClaimClass,
    expected_qualifies_stable: bool,
    expected_surface_marker: LifecycleMarker,
    expected_fidelity: RestoreFidelityClass,
    input: WindowTopologyRestoreInput,
) -> WindowTopologyRestoreScenario {
    let record = WindowTopologyRestoreRecord::build(input)
        .unwrap_or_else(|err| panic!("{scenario_id} must build: {err}"));
    WindowTopologyRestoreScenario {
        scenario_id,
        fixture_filename: format!("{scenario_id}.json"),
        expected_posture: record.posture_id.clone(),
        expected_claim_class,
        expected_qualifies_stable,
        expected_surface_marker,
        expected_fidelity,
        record,
    }
}

// ---------------------------------------------------------------------------
// Scenarios
// ---------------------------------------------------------------------------

fn exact_single_window() -> WindowTopologyRestoreScenario {
    let window_id = "win-workspace-primary-01";
    let routes = required_recovery_routes(false, false, false);
    let pane_tree = PaneTree {
        pane_tree_schema_version: PANE_TREE_SCHEMA_VERSION,
        root: PaneTreeNode::Split {
            axis: SplitAxisClass::Vertical,
            first_weight: 1,
            second_weight: 1,
            first: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-editor-handler".to_string(),
            }),
            second: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-editor-router".to_string(),
            }),
        },
        slots: vec![
            editor_slot("pane-editor-handler", "handler.rs"),
            editor_slot("pane-editor-router", "router.rs"),
        ],
    };
    let input = WindowTopologyRestoreInput {
        record_id: "exact_single_window".to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: "exact_single_window".to_string(),
        posture_label: "Single window, exact reopen".to_string(),
        title: "Primary window reopened exactly".to_string(),
        summary: "A single primary window with a vertical editor split reopened exactly: the pane \
                  tree, focus history, and bounds were restored and both editor panes re-read \
                  truthfully from durable buffer state."
            .to_string(),
        window_id: window_id.to_string(),
        window_role: WindowRoleClass::PrimaryWorkspace,
        authority: separation(true, true),
        pane_tree,
        display_topology: DisplayTopologyProvenance::default(),
        restore_provenance: RestoreProvenance {
            restore_provenance_ref: "aureline://restore-provenance/exact-single-window".to_string(),
            resulting_fidelity: RestoreFidelityClass::Exact,
            downgrade: None,
            compare_ref: None,
            export_ref: None,
            summary: "Exact restore; no topology adjustment, no placeholder, no downgrade."
                .to_string(),
        },
        recovery_chrome: full_recovery_chrome(),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(window_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(live_case_refs(|id| id.contains("split"))),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        "exact_single_window",
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        RestoreFidelityClass::Exact,
        input,
    )
}

fn mixed_dpi_multi_monitor_compatible() -> WindowTopologyRestoreScenario {
    let window_id = "win-workspace-secondary-01";
    let routes = required_recovery_routes(false, false, false);
    let pane_tree = PaneTree {
        pane_tree_schema_version: PANE_TREE_SCHEMA_VERSION,
        root: PaneTreeNode::Split {
            axis: SplitAxisClass::Vertical,
            first_weight: 2,
            second_weight: 1,
            first: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-editor-payments".to_string(),
            }),
            second: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-diff-review".to_string(),
            }),
        },
        slots: vec![
            editor_slot("pane-editor-payments", "payments.rs"),
            diff_slot("pane-diff-review", "payments-feature.diff"),
        ],
    };
    let input = WindowTopologyRestoreInput {
        record_id: "mixed_dpi_multi_monitor_compatible".to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: "mixed_dpi_multi_monitor_compatible".to_string(),
        posture_label: "Mixed-DPI dock, compatible reopen".to_string(),
        title: "Secondary window reopened after a mixed-DPI dock change".to_string(),
        summary: "A secondary window reopened after a mixed-DPI dock change: scale was normalized \
                  and bounds were clamped into the current safe visible region before focus and \
                  keyboard routing, while both re-readable panes hydrated live. The reopen is \
                  Compatible with a recorded display-topology downgrade."
            .to_string(),
        window_id: window_id.to_string(),
        window_role: WindowRoleClass::SecondaryWorkspace,
        authority: separation(true, true),
        pane_tree,
        display_topology: DisplayTopologyProvenance {
            topology_change_classes: vec![
                TopologyChangeClass::ScaleChanged,
                TopologyChangeClass::SafeBoundsChanged,
            ],
            adjustments: vec![
                RestoreAdjustmentClass::ScaleNormalized,
                RestoreAdjustmentClass::SnappedToSafeBounds,
            ],
            layout_adjusted_note_required: true,
        },
        restore_provenance: RestoreProvenance {
            restore_provenance_ref: "aureline://restore-provenance/mixed-dpi-compatible"
                .to_string(),
            resulting_fidelity: RestoreFidelityClass::Compatible,
            downgrade: Some(RestoreDowngrade {
                from_fidelity: RestoreFidelityClass::Exact,
                to_fidelity: RestoreFidelityClass::Compatible,
                reason: RestoreDowngradeReason::DisplayTopologyChange,
                note: "Scale was normalized and bounds were clamped into the current safe visible \
                       region; the user sees a layout-adjusted note in the restore history."
                    .to_string(),
            }),
            compare_ref: Some("aureline://restore-compare/mixed-dpi-compatible".to_string()),
            export_ref: Some("aureline://restore-export/mixed-dpi-compatible".to_string()),
            summary: "Compatible restore; bounds and scale adjusted for a mixed-DPI dock change."
                .to_string(),
        },
        recovery_chrome: full_recovery_chrome(),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(window_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(live_case_refs(|id| {
            id.contains("mixed-dpi") || id.contains("dock")
        })),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        "mixed_dpi_multi_monitor_compatible",
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        RestoreFidelityClass::Compatible,
        input,
    )
}

fn monitor_removed_placeholder_backed() -> WindowTopologyRestoreScenario {
    let window_id = "win-workspace-secondary-02";
    let routes = required_recovery_routes(true, true, true);
    // A nested split tree: editor on the left; the right side preserves the
    // exact M5 surface layout even though the backing runtimes did not survive
    // the restart.
    let pane_tree = PaneTree {
        pane_tree_schema_version: PANE_TREE_SCHEMA_VERSION,
        root: PaneTreeNode::Split {
            axis: SplitAxisClass::Vertical,
            first_weight: 2,
            second_weight: 5,
            first: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-editor-deploy".to_string(),
            }),
            second: Box::new(PaneTreeNode::Split {
                axis: SplitAxisClass::Horizontal,
                first_weight: 1,
                second_weight: 1,
                first: Box::new(PaneTreeNode::Split {
                    axis: SplitAxisClass::Vertical,
                    first_weight: 1,
                    second_weight: 1,
                    first: Box::new(PaneTreeNode::Leaf {
                        pane_id: "pane-terminal-deploy".to_string(),
                    }),
                    second: Box::new(PaneTreeNode::Leaf {
                        pane_id: "pane-pipeline-deploy".to_string(),
                    }),
                }),
                second: Box::new(PaneTreeNode::Split {
                    axis: SplitAxisClass::Vertical,
                    first_weight: 1,
                    second_weight: 1,
                    first: Box::new(PaneTreeNode::Split {
                        axis: SplitAxisClass::Horizontal,
                        first_weight: 1,
                        second_weight: 1,
                        first: Box::new(PaneTreeNode::Leaf {
                            pane_id: "pane-query-console-orders".to_string(),
                        }),
                        second: Box::new(PaneTreeNode::Leaf {
                            pane_id: "pane-notebook-explore".to_string(),
                        }),
                    }),
                    second: Box::new(PaneTreeNode::Split {
                        axis: SplitAxisClass::Horizontal,
                        first_weight: 1,
                        second_weight: 1,
                        first: Box::new(PaneTreeNode::Split {
                            axis: SplitAxisClass::Horizontal,
                            first_weight: 1,
                            second_weight: 1,
                            first: Box::new(PaneTreeNode::Leaf {
                                pane_id: "pane-preview-route-checkout".to_string(),
                            }),
                            second: Box::new(PaneTreeNode::Leaf {
                                pane_id: "pane-docs-runtime-playbook".to_string(),
                            }),
                        }),
                        second: Box::new(PaneTreeNode::Split {
                            axis: SplitAxisClass::Horizontal,
                            first_weight: 1,
                            second_weight: 1,
                            first: Box::new(PaneTreeNode::Leaf {
                                pane_id: "pane-profiler-startup".to_string(),
                            }),
                            second: Box::new(PaneTreeNode::Leaf {
                                pane_id: "pane-incident-sev1".to_string(),
                            }),
                        }),
                    }),
                }),
            }),
        },
        slots: vec![
            editor_slot("pane-editor-deploy", "deploy.rs"),
            placeholder_slot(
                "pane-terminal-deploy",
                PaneSurfaceClass::Terminal,
                "deploy shell",
                PanePlaceholderState::TranscriptRestored,
                PaneSubstitutionReason::RuntimeDidNotSurvive,
                vec![
                    PaneRecoveryAction::RerunExplicitly,
                    PaneRecoveryAction::CompareEvidence,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RemovePane,
                ],
                "Terminal transcript restored; the deploy command was not rerun. Rerun requires an \
                 explicit user action.",
            ),
            placeholder_slot(
                "pane-pipeline-deploy",
                PaneSurfaceClass::Pipeline,
                "deploy pipeline",
                PanePlaceholderState::SessionEnded,
                PaneSubstitutionReason::RuntimeDidNotSurvive,
                vec![
                    PaneRecoveryAction::RerunExplicitly,
                    PaneRecoveryAction::CompareEvidence,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RemovePane,
                ],
                "Pipeline run details, logs, and artifact pointers are preserved, but rerun and \
                 cancel authority were not silently restored.",
            ),
            placeholder_slot(
                "pane-query-console-orders",
                PaneSurfaceClass::QueryConsole,
                "orders query console",
                PanePlaceholderState::ReconnectAvailable,
                PaneSubstitutionReason::ExpiredRemoteSession,
                vec![
                    PaneRecoveryAction::ReconnectSession,
                    PaneRecoveryAction::CompareEvidence,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RemovePane,
                ],
                "The saved query console preserved target and result hints, but the remote session \
                 expired and live execution has not resumed.",
            ),
            placeholder_slot(
                "pane-notebook-explore",
                PaneSurfaceClass::Notebook,
                "explore.ipynb",
                PanePlaceholderState::RerunRequired,
                PaneSubstitutionReason::RuntimeDidNotSurvive,
                vec![
                    PaneRecoveryAction::RerunExplicitly,
                    PaneRecoveryAction::CompareEvidence,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RemovePane,
                ],
                "Notebook kernel did not survive; cell outputs are shown from the last snapshot and \
                 rerun requires an explicit user action.",
            ),
            placeholder_slot(
                "pane-preview-route-checkout",
                PaneSurfaceClass::Preview,
                "checkout preview",
                PanePlaceholderState::ContextUnavailable,
                PaneSubstitutionReason::RuntimeDidNotSurvive,
                vec![
                    PaneRecoveryAction::RerunExplicitly,
                    PaneRecoveryAction::CompareEvidence,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RemovePane,
                ],
                "The preview runtime did not survive restart; the route, title, and evidence are \
                 preserved but live rendering is withheld until an explicit restart.",
            ),
            placeholder_slot(
                "pane-docs-runtime-playbook",
                PaneSurfaceClass::Docs,
                "runtime playbook",
                PanePlaceholderState::ContextUnavailable,
                PaneSubstitutionReason::ExpiredManagedSession,
                vec![
                    PaneRecoveryAction::ReconnectSession,
                    PaneRecoveryAction::OpenWithout,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RemovePane,
                ],
                "The docs pane preserved its slot and route context, but the managed recall route \
                 expired and the pane reopens as a truthful placeholder instead of faking fresh docs.",
            ),
            placeholder_slot(
                "pane-profiler-startup",
                PaneSurfaceClass::ProfilerCapture,
                "startup capture",
                PanePlaceholderState::SessionEnded,
                PaneSubstitutionReason::RuntimeDidNotSurvive,
                vec![
                    PaneRecoveryAction::CompareEvidence,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RerunExplicitly,
                    PaneRecoveryAction::RemovePane,
                ],
                "Profiler capture evidence remains inspectable, but the live capture session ended \
                 and replay is not silently resumed.",
            ),
            placeholder_slot(
                "pane-incident-sev1",
                PaneSurfaceClass::IncidentWorkspace,
                "incident sev-1",
                PanePlaceholderState::ReconnectAvailable,
                PaneSubstitutionReason::ExpiredManagedSession,
                vec![
                    PaneRecoveryAction::ReconnectSession,
                    PaneRecoveryAction::CompareEvidence,
                    PaneRecoveryAction::ExportEvidence,
                    PaneRecoveryAction::RemovePane,
                ],
                "The incident workspace preserved timeline and evidence hints, but shared-control \
                 authority expired and requires an explicit reconnect review.",
            ),
        ],
    };
    let input = WindowTopologyRestoreInput {
        record_id: "monitor_removed_placeholder_backed".to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: "monitor_removed_placeholder_backed".to_string(),
        posture_label: "Monitor removed, placeholder-backed recovery".to_string(),
        title: "Secondary window reopened with placeholder-backed recovery".to_string(),
        summary: "A secondary window reopened after an external display detached and its session \
                  runtimes did not survive: the window was remapped to the primary display and \
                  clamped to the safe visible region, the editor pane hydrated live, and the \
                  terminal, pipeline, query-console, notebook, preview-route, docs, profiler, \
                  and incident panes kept their slots with in-place placeholder cards and \
                  restore-no-rerun guards."
            .to_string(),
        window_id: window_id.to_string(),
        window_role: WindowRoleClass::SecondaryWorkspace,
        authority: separation(true, true),
        pane_tree,
        display_topology: DisplayTopologyProvenance {
            topology_change_classes: vec![
                TopologyChangeClass::DisplayRemoved,
                TopologyChangeClass::SafeBoundsChanged,
            ],
            adjustments: vec![
                RestoreAdjustmentClass::MovedToPrimaryDisplay,
                RestoreAdjustmentClass::SnappedToSafeBounds,
                RestoreAdjustmentClass::LayoutOnlyFallback,
            ],
            layout_adjusted_note_required: true,
        },
        restore_provenance: RestoreProvenance {
            restore_provenance_ref: "aureline://restore-provenance/monitor-removed-placeholder"
                .to_string(),
            resulting_fidelity: RestoreFidelityClass::PlaceholderBacked,
            downgrade: Some(RestoreDowngrade {
                from_fidelity: RestoreFidelityClass::Compatible,
                to_fidelity: RestoreFidelityClass::PlaceholderBacked,
                reason: RestoreDowngradeReason::RuntimeDidNotSurvive,
                note: "An external display detached and the terminal, pipeline, query-console, \
                       notebook, preview-route, docs, profiler, and incident runtimes did not \
                       survive; the structure was preserved with placeholder cards rather than \
                       collapsing panes."
                    .to_string(),
            }),
            compare_ref: Some("aureline://restore-compare/monitor-removed-placeholder".to_string()),
            export_ref: Some("aureline://restore-export/monitor-removed-placeholder".to_string()),
            summary: "Placeholder-backed recovery; live surfaces preserved as placeholders with \
                      restore-no-rerun guards."
                .to_string(),
        },
        recovery_chrome: full_recovery_chrome(),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(window_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(live_case_refs(|id| {
            id.contains("display-detach") || id.contains("detach")
        })),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        "monitor_removed_placeholder_backed",
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        RestoreFidelityClass::PlaceholderBacked,
        input,
    )
}

fn help_about_preview_surface() -> WindowTopologyRestoreScenario {
    let window_id = "win-workspace-primary-02";
    let routes = required_recovery_routes(false, false, false);
    let pane_tree = PaneTree {
        pane_tree_schema_version: PANE_TREE_SCHEMA_VERSION,
        root: PaneTreeNode::Split {
            axis: SplitAxisClass::Vertical,
            first_weight: 1,
            second_weight: 1,
            first: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-editor-main".to_string(),
            }),
            second: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-explorer-tree".to_string(),
            }),
        },
        slots: vec![
            editor_slot("pane-editor-main", "main.rs"),
            PaneSlot {
                pane_id: "pane-explorer-tree".to_string(),
                surface_class: PaneSurfaceClass::Explorer,
                title_hint: Some("Explorer".to_string()),
                hydration: PaneHydrationClass::HydratedLive,
                placeholder_state: None,
                substitution_reason: None,
                runtime_survived: true,
                command_rerun_forbidden: false,
                authority_reacquire_forbidden: false,
                reopen_target_ref: "aureline://pane/pane-explorer-tree".to_string(),
                evidence_ref: None,
                recovery_actions: Vec::new(),
                note: "Explorer tree re-read truthfully from the workspace VFS; no side effect."
                    .to_string(),
            },
        ],
    };
    let input = WindowTopologyRestoreInput {
        record_id: "help_about_preview_surface".to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: "help_about_preview_surface".to_string(),
        posture_label: "Exact reopen, Help/About surface in preview".to_string(),
        title: "Exact reopen narrowed by a preview Help/About surface".to_string(),
        summary: "The window reopened exactly, but the Help/About binding surface is still in \
                  preview, so the reopen narrows below Stable by its lowest binding surface marker \
                  rather than inheriting an adjacent green row."
            .to_string(),
        window_id: window_id.to_string(),
        window_role: WindowRoleClass::PrimaryWorkspace,
        authority: separation(true, true),
        pane_tree,
        display_topology: DisplayTopologyProvenance::default(),
        restore_provenance: RestoreProvenance {
            restore_provenance_ref: "aureline://restore-provenance/help-about-preview".to_string(),
            resulting_fidelity: RestoreFidelityClass::Exact,
            downgrade: None,
            compare_ref: None,
            export_ref: None,
            summary:
                "Exact restore; the reopen itself is clean but a binding surface is in preview."
                    .to_string(),
        },
        recovery_chrome: full_recovery_chrome(),
        surface_projections: surface_inputs(LifecycleMarker::Preview),
        // The reopen still proves every pillar; only the surface marker narrows
        // it, so the claim ceiling may continue to assert each pillar.
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(window_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(live_case_refs(|id| id.contains("split"))),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        "help_about_preview_surface",
        StableClaimClass::Preview,
        false,
        LifecycleMarker::Preview,
        RestoreFidelityClass::Exact,
        input,
    )
}

fn authority_topology_leak_drill() -> WindowTopologyRestoreScenario {
    let window_id = "win-workspace-primary-03";
    let routes = required_recovery_routes(false, false, false);
    let pane_tree = PaneTree {
        pane_tree_schema_version: PANE_TREE_SCHEMA_VERSION,
        root: PaneTreeNode::Split {
            axis: SplitAxisClass::Vertical,
            first_weight: 1,
            second_weight: 1,
            first: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-editor-a".to_string(),
            }),
            second: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-editor-b".to_string(),
            }),
        },
        slots: vec![
            editor_slot("pane-editor-a", "a.rs"),
            editor_slot("pane-editor-b", "b.rs"),
        ],
    };
    let input = WindowTopologyRestoreInput {
        record_id: "authority_topology_leak_drill".to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: "authority_topology_leak_drill".to_string(),
        posture_label: "Authority/topology fusion drill".to_string(),
        title: "Reopen narrowed because topology leaked into shared authority".to_string(),
        summary: "An adversarial reopen where window-local topology was persisted into shared \
                  workspace authority instead of staying window-local. The lane detects the fusion \
                  and narrows the reopen below Stable with a named reason instead of papering over \
                  the gap."
            .to_string(),
        window_id: window_id.to_string(),
        window_role: WindowRoleClass::PrimaryWorkspace,
        // Topology is not window-local: the separation pillar fails.
        authority: separation(true, false),
        pane_tree,
        display_topology: DisplayTopologyProvenance::default(),
        restore_provenance: RestoreProvenance {
            restore_provenance_ref: "aureline://restore-provenance/authority-topology-leak"
                .to_string(),
            resulting_fidelity: RestoreFidelityClass::Exact,
            downgrade: None,
            compare_ref: None,
            export_ref: None,
            summary: "Exact pane restore, but the persisted model fused topology into shared \
                      authority."
                .to_string(),
        },
        recovery_chrome: full_recovery_chrome(),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        // The claim ceiling must not assert the separation it cannot prove.
        claim_ceiling: WindowRestoreClaimCeiling {
            asserts_authority_topology_separated: false,
            ..full_claim_ceiling()
        },
        recovery_routes: routes.clone(),
        routes: entry_routes(window_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(live_case_refs(|id| id.contains("split"))),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        "authority_topology_leak_drill",
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
        RestoreFidelityClass::Exact,
        input,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_builds_and_spans_stable_and_narrowed() {
        let corpus = window_topology_restore_corpus();
        assert_eq!(corpus.len(), 5);
        let stable = corpus
            .iter()
            .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
            .count();
        assert!(stable >= 3, "matrix must include several Stable rows");
        assert!(stable < corpus.len(), "matrix must include narrowed rows");
    }

    #[test]
    fn placeholder_scenario_keeps_every_session_pane_honest() {
        let scenario = monitor_removed_placeholder_backed();
        let record = scenario.record();
        for slot in &record.pane_tree.slots {
            if slot.surface_class.is_session_scoped() {
                assert!(
                    slot.is_placeholder(),
                    "{} must be placeholder",
                    slot.pane_id
                );
                assert!(slot.command_rerun_forbidden && slot.authority_reacquire_forbidden);
                assert!(!slot.runtime_survived);
            }
        }
        assert!(record.pillars.no_silent_rerun_or_reacquire);
    }

    #[test]
    fn placeholder_scenario_covers_the_m5_restore_surface_set() {
        let scenario = monitor_removed_placeholder_backed();
        let record = scenario.record();
        let seen = record
            .pane_tree
            .slots
            .iter()
            .map(|slot| slot.surface_class)
            .collect::<std::collections::BTreeSet<_>>();
        for required in [
            PaneSurfaceClass::Terminal,
            PaneSurfaceClass::Notebook,
            PaneSurfaceClass::QueryConsole,
            PaneSurfaceClass::Pipeline,
            PaneSurfaceClass::Preview,
            PaneSurfaceClass::Docs,
            PaneSurfaceClass::ProfilerCapture,
            PaneSurfaceClass::IncidentWorkspace,
        ] {
            assert!(
                seen.contains(&required),
                "placeholder scenario must cover {}",
                required.as_str()
            );
        }
    }
}
