//! Cross-surface warm-startup / warm-restore / first-useful-work drill corpus.
//!
//! ## Why a corpus, not a single seeded record
//!
//! The model in [`crate::warm_continuity::model`] proves the useful-chrome-first,
//! no-silent-rerun, bounded-preference, zone-owned-truth, and reachable-fallback
//! invariants in isolation. The stable-claim grade is about the shell,
//! diagnostics, support exports, Help/About, and docs *agreeing* on what one warm
//! cycle means across the failure cases the acceptance criteria name. This corpus
//! mints one [`WarmContinuityScenario`] per named drill and pins each rendered
//! [`WarmContinuityRecord`] bit-for-bit on disk under
//! `fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful/`, so a
//! regression in the startup-order rule, the restore-provenance split, the
//! no-rerun gate, the bounded-preference rule, the zone-identity rule, or the
//! responsive-fallback rule fails the fixture-replay test instead of shipping
//! silently.
//!
//! The drills deliberately exercise every required regression case — sleep /
//! resume, display-topology change, missing extension, expired remote session,
//! and revoked authorization — alongside the clean warm relaunch and crash
//! recovery baselines, and they cover every window class, every landing route,
//! every restore-provenance class, every collapse target, every zone-owned cue,
//! and every side-effectful surface class.

use super::model::{
    CollapseTargetClass, CollapsedSurface, DowngradeTriggerToken, EntryCauseClass,
    LandingDecisionInput, LandingRouteClass, LandingRouteReasonClass, NoRerunSurface,
    RememberedPreference, ResponsiveFallbackInput, RestoreClassToken, RestoreItem,
    RestoreProvenanceClass, RestoreProvenanceInput, RestoreSurfaceClass, ShellZoneToken,
    SideEffectfulSurfaceClass, StartupMilestoneClass, StartupMilestoneInput, StartupTrace,
    WarmContinuityInput, WarmContinuityRecord, WindowClassToken, ZoneIdentityInput, ZoneOwnedCue,
    ZoneOwnedCueClass,
};

/// Stable `as_of` instant the whole corpus is evaluated against. Pinned so the
/// on-disk fixtures stay deterministic.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

/// Stable record-id prefix shared by every scenario.
pub const CORPUS_RECORD_ID_PREFIX: &str = "warm_continuity:m4.stable.corpus.";

/// One drill. Surfaces under review MUST reproduce the same record projection
/// bit-for-bit; the test in
/// `crates/aureline-shell/tests/warm_continuity_fixtures.rs` pins each scenario
/// against the on-disk fixture.
#[derive(Clone)]
pub struct WarmContinuityScenario {
    /// Stable identifier, quoted in the matrix, the report, and the doc.
    pub scenario_id: &'static str,
    /// Stable human-readable label.
    pub scenario_label: &'static str,
    /// One-sentence narrative the report and matrix quote.
    pub narrative: &'static str,
    /// On-disk fixture filename (relative to the corpus fixture dir).
    pub fixture_filename: &'static str,
    /// Expected entry cause.
    pub expected_entry_cause: EntryCauseClass,
    /// Expected restore class.
    pub expected_restore_class: RestoreClassToken,
    /// Expected window class.
    pub expected_window_class: WindowClassToken,
    /// Expected selected landing route.
    pub expected_landing_route: LandingRouteClass,
    /// Expected honesty-marker value.
    pub expected_honesty_marker_present: bool,
    /// Expected restored-exactly count.
    pub expected_restored_exactly_count: u32,
    /// Expected restored-partially count.
    pub expected_restored_partially_count: u32,
    /// Expected needs-review count.
    pub expected_needs_review_count: u32,
    input: WarmContinuityInput,
}

impl WarmContinuityScenario {
    /// Build the rendered record for this scenario. The corpus inputs are
    /// deterministic and honest, so a build failure is a bug.
    pub fn record(&self) -> WarmContinuityRecord {
        WarmContinuityRecord::build(self.input.clone())
            .expect("warm-continuity corpus scenario must build")
    }
}

// --------------------------------------------------------------------------- //
// Compact constructors
// --------------------------------------------------------------------------- //

fn ms(
    milestone: StartupMilestoneClass,
    ordinal: u32,
    reached_before_deep_discovery: bool,
    detail: &str,
) -> StartupMilestoneInput {
    StartupMilestoneInput {
        milestone,
        ordinal,
        reached_before_deep_discovery,
        keyboard_reachable: true,
        detail: detail.to_owned(),
    }
}

/// The standard skeleton-first → hydrate-second startup trace. Useful chrome,
/// command entry, and a stable focus target all land before deep discovery.
fn standard_startup() -> StartupTrace {
    StartupTrace {
        milestones: vec![
            ms(
                StartupMilestoneClass::ShellChromePainted,
                0,
                true,
                "Canonical zones painted with placeholders before any indexing.",
            ),
            ms(
                StartupMilestoneClass::CommandEntryReady,
                1,
                true,
                "Command palette accepts input immediately.",
            ),
            ms(
                StartupMilestoneClass::StableFocusTarget,
                2,
                true,
                "Focus rests on a stable, keyboard-reachable target.",
            ),
            ms(
                StartupMilestoneClass::LayoutSkeletonRestored,
                3,
                true,
                "Windows, pane trees, tabs, and status surfaces restored as skeletons.",
            ),
            ms(
                StartupMilestoneClass::PriorFramePreserved,
                4,
                true,
                "The prior useful editor frame stays painted while hydration continues.",
            ),
            ms(
                StartupMilestoneClass::HydrationContinuing,
                5,
                false,
                "Indexing, remote reconnect, and provider hydration continue in the background.",
            ),
            ms(
                StartupMilestoneClass::FirstUsefulWorkRouted,
                6,
                true,
                "First-useful-work routing chose the next meaningful action.",
            ),
        ],
    }
}

#[allow(clippy::too_many_arguments)]
fn item(
    object_ref: &str,
    surface_class: RestoreSurfaceClass,
    provenance: RestoreProvenanceClass,
    downgrade_trigger: Option<DowngradeTriggerToken>,
    detail: &str,
    user_authored: bool,
) -> RestoreItem {
    RestoreItem {
        object_ref: object_ref.to_owned(),
        surface_class,
        provenance,
        downgrade_trigger,
        detail: detail.to_owned(),
        user_authored,
    }
}

fn no_rerun(
    surface_class: SideEffectfulSurfaceClass,
    skeleton_ref: &str,
    requires_fresh_authorization: bool,
    requires_review: bool,
    resume_route_ref: &str,
    detail: &str,
) -> NoRerunSurface {
    NoRerunSurface {
        surface_class,
        skeleton_ref: skeleton_ref.to_owned(),
        auto_rerun_forbidden: true,
        requires_fresh_authorization,
        requires_review,
        resume_route_ref: resume_route_ref.to_owned(),
        detail: detail.to_owned(),
    }
}

fn cue(cue: ZoneOwnedCueClass, owning_zone: ShellZoneToken, updated: bool) -> ZoneOwnedCue {
    ZoneOwnedCue {
        cue,
        owning_zone,
        rendered_zone: owning_zone,
        label_or_placeholder_updated: updated,
    }
}

fn collapsed(
    surface_ref: &str,
    source_zone: ShellZoneToken,
    collapsed_to: CollapseTargetClass,
    reopen_route_ref: &str,
    last_meaningful_state_ref: &str,
) -> CollapsedSurface {
    CollapsedSurface {
        surface_ref: surface_ref.to_owned(),
        source_zone,
        collapsed_to,
        reopen_route_ref: reopen_route_ref.to_owned(),
        last_meaningful_state_ref: last_meaningful_state_ref.to_owned(),
        keyboard_reachable: true,
        approved_to_move: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn landing(
    selected_route: LandingRouteClass,
    route_reason: LandingRouteReasonClass,
    target_ref: &str,
    candidate_routes: Vec<LandingRouteClass>,
    remembered_preference: Option<RememberedPreference>,
    detail: &str,
) -> LandingDecisionInput {
    LandingDecisionInput {
        selected_route,
        route_reason,
        target_ref: target_ref.to_owned(),
        keyboard_reachable: true,
        destructive: false,
        candidate_routes,
        remembered_preference,
        detail: detail.to_owned(),
    }
}

fn bounded_pref(preference_ref: &str) -> RememberedPreference {
    RememberedPreference {
        preference_ref: preference_ref.to_owned(),
        influences_routing: true,
        widens_workspace_trust: false,
        installs_packages: false,
        applies_workflow_bundle: false,
        suppresses_required_checkpoint: false,
    }
}

fn record_id(scenario_id: &str) -> String {
    format!("{CORPUS_RECORD_ID_PREFIX}{scenario_id}")
}

// --------------------------------------------------------------------------- //
// The corpus
// --------------------------------------------------------------------------- //

/// The full ordered drill corpus.
pub fn warm_continuity_corpus() -> Vec<WarmContinuityScenario> {
    vec![
        warm_relaunch_exact(),
        crash_recovery_drafts(),
        sleep_resume(),
        display_topology_change(),
        missing_extension(),
        expired_remote_session(),
        revoked_authorization(),
    ]
}

fn warm_relaunch_exact() -> WarmContinuityScenario {
    let input = WarmContinuityInput {
        record_id: record_id("warm_relaunch_exact"),
        as_of: CORPUS_AS_OF.to_owned(),
        entry_cause: EntryCauseClass::WarmRelaunch,
        title: "Warm relaunch restored your last session exactly.".to_owned(),
        summary:
            "Useful chrome painted first; layout and editors restored exactly; routed to your \
                  prior active editor."
                .to_owned(),
        startup: standard_startup(),
        restore: RestoreProvenanceInput {
            restore_class: RestoreClassToken::ExactRestore,
            items: vec![
                item(
                    "aureline://window/primary",
                    RestoreSurfaceClass::Window,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Primary window restored exactly.",
                    false,
                ),
                item(
                    "aureline://pane_tree/primary",
                    RestoreSurfaceClass::PaneTree,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Pane tree restored exactly.",
                    false,
                ),
                item(
                    "aureline://tab_group/main",
                    RestoreSurfaceClass::TabGroup,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Tab group restored exactly.",
                    false,
                ),
                item(
                    "aureline://editor/router_ts",
                    RestoreSurfaceClass::Editor,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Active editor restored exactly with cursor and scroll.",
                    true,
                ),
                item(
                    "aureline://non_mutating_context/outline",
                    RestoreSurfaceClass::NonMutatingContext,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Outline context restored exactly.",
                    false,
                ),
            ],
            no_rerun_surfaces: vec![],
        },
        landing: landing(
            LandingRouteClass::PriorActiveEditor,
            LandingRouteReasonClass::RememberedPreference,
            "aureline://editor/router_ts",
            vec![
                LandingRouteClass::PriorActiveEditor,
                LandingRouteClass::ChangedFilesView,
                LandingRouteClass::Readme,
            ],
            Some(bounded_pref(
                "aureline://preference/first_useful_prior_editor",
            )),
            "Routed to the prior active editor per your remembered preference.",
        ),
        zone_identity: ZoneIdentityInput {
            cues: vec![
                cue(
                    ZoneOwnedCueClass::Breadcrumb,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::TrustBadge,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::ExecutionTargetCue,
                    ShellZoneToken::StatusBar,
                    false,
                ),
            ],
        },
        responsive: ResponsiveFallbackInput {
            window_class: WindowClassToken::StandardDesktop,
            collapsed_surfaces: vec![],
        },
        diagnostics_export_ref: "aureline://diagnostics/warm_relaunch_exact".to_owned(),
        support_export_ref: "aureline://support_export/warm_relaunch_exact".to_owned(),
        evidence_refs: vec!["aureline://trace/warm_relaunch_exact".to_owned()],
        narrative_refs: vec![
            "aureline://doc/harden_shell_startup_warm_restore_and_first_useful".to_owned(),
        ],
    };
    WarmContinuityScenario {
        scenario_id: "warm_relaunch_exact",
        scenario_label: "Warm relaunch, exact restore",
        narrative:
            "A clean warm relaunch paints useful chrome first and restores the last session \
                    exactly, routing to the prior active editor.",
        fixture_filename: "warm_relaunch_exact.json",
        expected_entry_cause: EntryCauseClass::WarmRelaunch,
        expected_restore_class: RestoreClassToken::ExactRestore,
        expected_window_class: WindowClassToken::StandardDesktop,
        expected_landing_route: LandingRouteClass::PriorActiveEditor,
        expected_honesty_marker_present: false,
        expected_restored_exactly_count: 5,
        expected_restored_partially_count: 0,
        expected_needs_review_count: 0,
        input,
    }
}

fn crash_recovery_drafts() -> WarmContinuityScenario {
    let input = WarmContinuityInput {
        record_id: record_id("crash_recovery_drafts"),
        as_of: CORPUS_AS_OF.to_owned(),
        entry_cause: EntryCauseClass::CrashRecovery,
        title: "Recovered your unsaved work after an unexpected exit.".to_owned(),
        summary: "Layout came back; a draft was recovered and needs saving; terminals, tasks, the \
                  debugger, and notebook cells are paused, not rerun."
            .to_owned(),
        startup: standard_startup(),
        restore: RestoreProvenanceInput {
            restore_class: RestoreClassToken::RecoveredDrafts,
            items: vec![
                item(
                    "aureline://window/primary",
                    RestoreSurfaceClass::Window,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Primary window restored exactly.",
                    false,
                ),
                item(
                    "aureline://pane_tree/primary",
                    RestoreSurfaceClass::PaneTree,
                    RestoreProvenanceClass::RestoredPartially,
                    Some(DowngradeTriggerToken::SchemaTranslationRequired),
                    "Pane tree translated from an older schema; layout approximated.",
                    false,
                ),
                item(
                    "aureline://editor/draft_main_rs",
                    RestoreSurfaceClass::Editor,
                    RestoreProvenanceClass::RestoredPartially,
                    Some(DowngradeTriggerToken::ManualRepairRequired),
                    "Recovered draft from the crash journal; save to confirm.",
                    true,
                ),
                item(
                    "aureline://status_surface/scm",
                    RestoreSurfaceClass::StatusSurface,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Source-control status surface restored exactly.",
                    false,
                ),
            ],
            no_rerun_surfaces: vec![
                no_rerun(
                    SideEffectfulSurfaceClass::Terminal,
                    "aureline://terminal/build",
                    false,
                    false,
                    "aureline://command/terminal.reopen",
                    "Terminal restored as a skeleton; nothing rerun.",
                ),
                no_rerun(
                    SideEffectfulSurfaceClass::Task,
                    "aureline://task/test_watch",
                    false,
                    false,
                    "aureline://command/task.rerun",
                    "Task paused; rerun only on your action.",
                ),
                no_rerun(
                    SideEffectfulSurfaceClass::DebugSession,
                    "aureline://debug_session/api",
                    false,
                    true,
                    "aureline://command/debug.relaunch",
                    "Debug session not re-attached; relaunch requires review.",
                ),
                no_rerun(
                    SideEffectfulSurfaceClass::NotebookCell,
                    "aureline://notebook_cell/eda_cell_7",
                    false,
                    false,
                    "aureline://command/notebook.run_cell",
                    "Notebook cell outputs preserved; not re-executed.",
                ),
            ],
        },
        landing: landing(
            LandingRouteClass::ChangedFilesView,
            LandingRouteReasonClass::ChangedFilesPending,
            "aureline://changed_files_view/workspace",
            vec![
                LandingRouteClass::ChangedFilesView,
                LandingRouteClass::PriorActiveEditor,
                LandingRouteClass::ReviewPacket,
            ],
            None,
            "Routed to changed files so you can review recovered drafts first.",
        ),
        zone_identity: ZoneIdentityInput {
            cues: vec![
                cue(
                    ZoneOwnedCueClass::TrustBadge,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::StatusSummary,
                    ShellZoneToken::StatusBar,
                    true,
                ),
            ],
        },
        responsive: ResponsiveFallbackInput {
            window_class: WindowClassToken::StandardDesktop,
            collapsed_surfaces: vec![],
        },
        diagnostics_export_ref: "aureline://diagnostics/crash_recovery_drafts".to_owned(),
        support_export_ref: "aureline://support_export/crash_recovery_drafts".to_owned(),
        evidence_refs: vec!["aureline://trace/crash_recovery_drafts".to_owned()],
        narrative_refs: vec![
            "aureline://doc/harden_shell_startup_warm_restore_and_first_useful".to_owned(),
        ],
    };
    WarmContinuityScenario {
        scenario_id: "crash_recovery_drafts",
        scenario_label: "Crash recovery, recovered drafts",
        narrative: "After an abnormal exit, layout and a recovered draft come back with honest \
                    provenance and every side-effectful session is paused, not rerun.",
        fixture_filename: "crash_recovery_drafts.json",
        expected_entry_cause: EntryCauseClass::CrashRecovery,
        expected_restore_class: RestoreClassToken::RecoveredDrafts,
        expected_window_class: WindowClassToken::StandardDesktop,
        expected_landing_route: LandingRouteClass::ChangedFilesView,
        expected_honesty_marker_present: true,
        expected_restored_exactly_count: 2,
        expected_restored_partially_count: 2,
        expected_needs_review_count: 0,
        input,
    }
}

fn sleep_resume() -> WarmContinuityScenario {
    let input = WarmContinuityInput {
        record_id: record_id("sleep_resume"),
        as_of: CORPUS_AS_OF.to_owned(),
        entry_cause: EntryCauseClass::SleepResume,
        title: "Resumed after sleep with your editors intact.".to_owned(),
        summary: "Editors and layout resumed; the provider connection and a remote action need a \
                  fresh handshake before they run again."
            .to_owned(),
        startup: standard_startup(),
        restore: RestoreProvenanceInput {
            restore_class: RestoreClassToken::CompatibleRestore,
            items: vec![
                item(
                    "aureline://window/primary",
                    RestoreSurfaceClass::Window,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Primary window restored exactly.",
                    false,
                ),
                item(
                    "aureline://editor/service_go",
                    RestoreSurfaceClass::Editor,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Active editor resumed exactly.",
                    true,
                ),
                item(
                    "aureline://non_mutating_context/symbol_index",
                    RestoreSurfaceClass::NonMutatingContext,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Symbol index context restored exactly.",
                    false,
                ),
            ],
            no_rerun_surfaces: vec![
                no_rerun(
                    SideEffectfulSurfaceClass::ProviderMutation,
                    "aureline://provider_mutation/ai_apply",
                    true,
                    false,
                    "aureline://command/provider.reconnect",
                    "Provider mutation paused; reconnect needs fresh authorization.",
                ),
                no_rerun(
                    SideEffectfulSurfaceClass::RemoteAction,
                    "aureline://remote_action/deploy",
                    false,
                    true,
                    "aureline://command/remote.review_action",
                    "Remote action not replayed; resuming requires review.",
                ),
            ],
        },
        landing: landing(
            LandingRouteClass::PriorActiveEditor,
            LandingRouteReasonClass::PriorActiveEditorPresent,
            "aureline://editor/service_go",
            vec![
                LandingRouteClass::PriorActiveEditor,
                LandingRouteClass::ChangedFilesView,
            ],
            None,
            "Routed to the editor you were in before the machine slept.",
        ),
        zone_identity: ZoneIdentityInput {
            cues: vec![
                cue(
                    ZoneOwnedCueClass::WorkspaceIdentity,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::ExecutionTargetCue,
                    ShellZoneToken::StatusBar,
                    true,
                ),
            ],
        },
        responsive: ResponsiveFallbackInput {
            window_class: WindowClassToken::ExpandedDesktop,
            collapsed_surfaces: vec![],
        },
        diagnostics_export_ref: "aureline://diagnostics/sleep_resume".to_owned(),
        support_export_ref: "aureline://support_export/sleep_resume".to_owned(),
        evidence_refs: vec!["aureline://trace/sleep_resume".to_owned()],
        narrative_refs: vec![
            "aureline://doc/harden_shell_startup_warm_restore_and_first_useful".to_owned(),
        ],
    };
    WarmContinuityScenario {
        scenario_id: "sleep_resume",
        scenario_label: "Sleep / resume",
        narrative: "After sleep, editors and layout resume while the provider and remote action \
                    require a fresh handshake, never a silent replay.",
        fixture_filename: "sleep_resume.json",
        expected_entry_cause: EntryCauseClass::SleepResume,
        expected_restore_class: RestoreClassToken::CompatibleRestore,
        expected_window_class: WindowClassToken::ExpandedDesktop,
        expected_landing_route: LandingRouteClass::PriorActiveEditor,
        expected_honesty_marker_present: true,
        expected_restored_exactly_count: 3,
        expected_restored_partially_count: 0,
        expected_needs_review_count: 0,
        input,
    }
}

fn display_topology_change() -> WarmContinuityScenario {
    let input = WarmContinuityInput {
        record_id: record_id("display_topology_change"),
        as_of: CORPUS_AS_OF.to_owned(),
        entry_cause: EntryCauseClass::DisplayTopologyChange,
        title: "Reflowed to one screen after a display change.".to_owned(),
        summary: "Your content is intact; the layout reflowed to a single compact display and two \
                  side surfaces moved to overflow and a sheet, both reopenable."
            .to_owned(),
        startup: standard_startup(),
        restore: RestoreProvenanceInput {
            restore_class: RestoreClassToken::LayoutOnly,
            items: vec![
                item(
                    "aureline://window/primary",
                    RestoreSurfaceClass::Window,
                    RestoreProvenanceClass::RestoredPartially,
                    Some(DowngradeTriggerToken::UnsupportedDisplayTopology),
                    "Second monitor is gone; window reflowed to the remaining display.",
                    false,
                ),
                item(
                    "aureline://pane_tree/primary",
                    RestoreSurfaceClass::PaneTree,
                    RestoreProvenanceClass::RestoredPartially,
                    Some(DowngradeTriggerToken::UnsupportedDisplayTopology),
                    "Pane tree reflowed to fit the compact layout.",
                    false,
                ),
                item(
                    "aureline://layout/main",
                    RestoreSurfaceClass::Layout,
                    RestoreProvenanceClass::RestoredPartially,
                    Some(DowngradeTriggerToken::UnsupportedDisplayTopology),
                    "Layout downgraded to a single-display arrangement.",
                    false,
                ),
                item(
                    "aureline://editor/view_tsx",
                    RestoreSurfaceClass::Editor,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Active editor content restored exactly.",
                    true,
                ),
            ],
            no_rerun_surfaces: vec![],
        },
        landing: landing(
            LandingRouteClass::PostEntryHandoffCard,
            LandingRouteReasonClass::PostEntryHandoffPending,
            "aureline://post_entry_handoff_card/reflow",
            vec![
                LandingRouteClass::PostEntryHandoffCard,
                LandingRouteClass::PriorActiveEditor,
            ],
            None,
            "Showed a handoff card explaining the reflow and how to restore panels.",
        ),
        zone_identity: ZoneIdentityInput {
            cues: vec![
                cue(
                    ZoneOwnedCueClass::Breadcrumb,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::TrustBadge,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::ExecutionTargetCue,
                    ShellZoneToken::StatusBar,
                    false,
                ),
            ],
        },
        responsive: ResponsiveFallbackInput {
            window_class: WindowClassToken::CompactDesktop,
            collapsed_surfaces: vec![
                collapsed(
                    "aureline://surface/right_inspector",
                    ShellZoneToken::RightInspector,
                    CollapseTargetClass::Overflow,
                    "aureline://command/inspector.reopen",
                    "aureline://state/right_inspector_last",
                ),
                collapsed(
                    "aureline://surface/bottom_panel",
                    ShellZoneToken::BottomPanel,
                    CollapseTargetClass::Sheet,
                    "aureline://command/bottom_panel.reopen",
                    "aureline://state/bottom_panel_last",
                ),
            ],
        },
        diagnostics_export_ref: "aureline://diagnostics/display_topology_change".to_owned(),
        support_export_ref: "aureline://support_export/display_topology_change".to_owned(),
        evidence_refs: vec!["aureline://trace/display_topology_change".to_owned()],
        narrative_refs: vec![
            "aureline://doc/harden_shell_startup_warm_restore_and_first_useful".to_owned(),
        ],
    };
    WarmContinuityScenario {
        scenario_id: "display_topology_change",
        scenario_label: "Display-topology change",
        narrative: "When a monitor disappears, content stays intact and the layout reflows to a \
                    compact display with side surfaces in reopenable overflow and sheet states.",
        fixture_filename: "display_topology_change.json",
        expected_entry_cause: EntryCauseClass::DisplayTopologyChange,
        expected_restore_class: RestoreClassToken::LayoutOnly,
        expected_window_class: WindowClassToken::CompactDesktop,
        expected_landing_route: LandingRouteClass::PostEntryHandoffCard,
        expected_honesty_marker_present: true,
        expected_restored_exactly_count: 1,
        expected_restored_partially_count: 3,
        expected_needs_review_count: 0,
        input,
    }
}

fn missing_extension() -> WarmContinuityScenario {
    let input = WarmContinuityInput {
        record_id: record_id("missing_extension"),
        as_of: CORPUS_AS_OF.to_owned(),
        entry_cause: EntryCauseClass::MissingExtensionFallback,
        title: "Opened with a missing extension flagged for review.".to_owned(),
        summary: "Your editors restored; a surface that needed a missing extension is held for \
                  review, and you landed on the project README."
            .to_owned(),
        startup: standard_startup(),
        restore: RestoreProvenanceInput {
            restore_class: RestoreClassToken::CompatibleRestore,
            items: vec![
                item(
                    "aureline://window/primary",
                    RestoreSurfaceClass::Window,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Primary window restored exactly.",
                    false,
                ),
                item(
                    "aureline://editor/readme_md",
                    RestoreSurfaceClass::Editor,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Editor restored exactly.",
                    true,
                ),
                item(
                    "aureline://non_mutating_context/extension_panel",
                    RestoreSurfaceClass::NonMutatingContext,
                    RestoreProvenanceClass::NeedsReview,
                    Some(DowngradeTriggerToken::MissingExtensionDependency),
                    "A panel needs an extension that is not installed; held as a placeholder.",
                    false,
                ),
            ],
            no_rerun_surfaces: vec![],
        },
        landing: landing(
            LandingRouteClass::Readme,
            LandingRouteReasonClass::FirstRunReadme,
            "aureline://readme/project",
            vec![
                LandingRouteClass::Readme,
                LandingRouteClass::PriorActiveEditor,
                LandingRouteClass::PostEntryHandoffCard,
            ],
            None,
            "Routed to the README while the missing-extension panel waits for review.",
        ),
        zone_identity: ZoneIdentityInput {
            cues: vec![
                cue(
                    ZoneOwnedCueClass::TrustBadge,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::StatusSummary,
                    ShellZoneToken::StatusBar,
                    true,
                ),
            ],
        },
        responsive: ResponsiveFallbackInput {
            window_class: WindowClassToken::StandardDesktop,
            collapsed_surfaces: vec![],
        },
        diagnostics_export_ref: "aureline://diagnostics/missing_extension".to_owned(),
        support_export_ref: "aureline://support_export/missing_extension".to_owned(),
        evidence_refs: vec!["aureline://trace/missing_extension".to_owned()],
        narrative_refs: vec![
            "aureline://doc/harden_shell_startup_warm_restore_and_first_useful".to_owned(),
        ],
    };
    WarmContinuityScenario {
        scenario_id: "missing_extension",
        scenario_label: "Missing extension",
        narrative: "A missing extension does not collapse the layout: editors restore and the \
                    affected surface is held as a reviewable placeholder.",
        fixture_filename: "missing_extension.json",
        expected_entry_cause: EntryCauseClass::MissingExtensionFallback,
        expected_restore_class: RestoreClassToken::CompatibleRestore,
        expected_window_class: WindowClassToken::StandardDesktop,
        expected_landing_route: LandingRouteClass::Readme,
        expected_honesty_marker_present: true,
        expected_restored_exactly_count: 2,
        expected_restored_partially_count: 0,
        expected_needs_review_count: 1,
        input,
    }
}

fn expired_remote_session() -> WarmContinuityScenario {
    let input = WarmContinuityInput {
        record_id: record_id("expired_remote_session"),
        as_of: CORPUS_AS_OF.to_owned(),
        entry_cause: EntryCauseClass::ExpiredRemoteSession,
        title: "Remote session expired; review what to resume.".to_owned(),
        summary:
            "The remote workspace session expired, so live state is evidence-only; remote and \
                  collaboration actions need a fresh handshake before resuming."
                .to_owned(),
        startup: standard_startup(),
        restore: RestoreProvenanceInput {
            restore_class: RestoreClassToken::EvidenceOnly,
            items: vec![
                item(
                    "aureline://non_mutating_context/remote_workspace",
                    RestoreSurfaceClass::NonMutatingContext,
                    RestoreProvenanceClass::NeedsReview,
                    Some(DowngradeTriggerToken::MissingRemoteSession),
                    "Remote workspace context is evidence-only until the session is renewed.",
                    false,
                ),
                item(
                    "aureline://status_surface/remote_status",
                    RestoreSurfaceClass::StatusSurface,
                    RestoreProvenanceClass::RestoredPartially,
                    Some(DowngradeTriggerToken::MissingRemoteAuthority),
                    "Remote status surface shows last-known evidence, not live state.",
                    false,
                ),
            ],
            no_rerun_surfaces: vec![
                no_rerun(
                    SideEffectfulSurfaceClass::RemoteAction,
                    "aureline://remote_action/sync",
                    true,
                    false,
                    "aureline://command/remote.reauthorize",
                    "Remote sync paused; resuming needs fresh authorization.",
                ),
                no_rerun(
                    SideEffectfulSurfaceClass::CollaborationControl,
                    "aureline://collaboration_control/session",
                    false,
                    true,
                    "aureline://command/collab.rejoin",
                    "Collaboration control not auto-rejoined; rejoin requires review.",
                ),
            ],
        },
        landing: landing(
            LandingRouteClass::ReviewPacket,
            LandingRouteReasonClass::ReviewRequested,
            "aureline://review_packet/remote_resume",
            vec![
                LandingRouteClass::ReviewPacket,
                LandingRouteClass::PostEntryHandoffCard,
            ],
            None,
            "Routed to a review packet describing exactly what expired and how to resume.",
        ),
        zone_identity: ZoneIdentityInput {
            cues: vec![
                cue(
                    ZoneOwnedCueClass::TrustBadge,
                    ShellZoneToken::TitleContextBar,
                    true,
                ),
                cue(
                    ZoneOwnedCueClass::ExecutionTargetCue,
                    ShellZoneToken::StatusBar,
                    true,
                ),
            ],
        },
        responsive: ResponsiveFallbackInput {
            window_class: WindowClassToken::StandardDesktop,
            collapsed_surfaces: vec![],
        },
        diagnostics_export_ref: "aureline://diagnostics/expired_remote_session".to_owned(),
        support_export_ref: "aureline://support_export/expired_remote_session".to_owned(),
        evidence_refs: vec!["aureline://trace/expired_remote_session".to_owned()],
        narrative_refs: vec![
            "aureline://doc/harden_shell_startup_warm_restore_and_first_useful".to_owned(),
        ],
    };
    WarmContinuityScenario {
        scenario_id: "expired_remote_session",
        scenario_label: "Expired remote session",
        narrative:
            "An expired remote session yields evidence-only live state and gates remote and \
                    collaboration resumes behind a fresh handshake and a review packet.",
        fixture_filename: "expired_remote_session.json",
        expected_entry_cause: EntryCauseClass::ExpiredRemoteSession,
        expected_restore_class: RestoreClassToken::EvidenceOnly,
        expected_window_class: WindowClassToken::StandardDesktop,
        expected_landing_route: LandingRouteClass::ReviewPacket,
        expected_honesty_marker_present: true,
        expected_restored_exactly_count: 0,
        expected_restored_partially_count: 1,
        expected_needs_review_count: 1,
        input,
    }
}

fn revoked_authorization() -> WarmContinuityScenario {
    let input = WarmContinuityInput {
        record_id: record_id("revoked_authorization"),
        as_of: CORPUS_AS_OF.to_owned(),
        entry_cause: EntryCauseClass::RevokedAuthorization,
        title: "Authorization was revoked; local work is safe.".to_owned(),
        summary: "Your local editors restored; provider authority was revoked, so provider \
                  mutations are held for fresh authorization and review."
            .to_owned(),
        startup: standard_startup(),
        restore: RestoreProvenanceInput {
            restore_class: RestoreClassToken::CompatibleRestore,
            items: vec![
                item(
                    "aureline://window/primary",
                    RestoreSurfaceClass::Window,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Primary window restored exactly.",
                    false,
                ),
                item(
                    "aureline://editor/notes_md",
                    RestoreSurfaceClass::Editor,
                    RestoreProvenanceClass::RestoredExactly,
                    None,
                    "Local editor restored exactly.",
                    true,
                ),
                item(
                    "aureline://non_mutating_context/provider_panel",
                    RestoreSurfaceClass::NonMutatingContext,
                    RestoreProvenanceClass::NeedsReview,
                    Some(DowngradeTriggerToken::MissingRemoteAuthority),
                    "Provider panel needs re-authorization before it shows live data.",
                    false,
                ),
            ],
            no_rerun_surfaces: vec![no_rerun(
                SideEffectfulSurfaceClass::ProviderMutation,
                "aureline://provider_mutation/commit_push",
                true,
                true,
                "aureline://command/provider.reauthorize",
                "Provider mutation held; resuming needs fresh authorization and review.",
            )],
        },
        landing: landing(
            LandingRouteClass::PostEntryHandoffCard,
            LandingRouteReasonClass::PostEntryHandoffPending,
            "aureline://post_entry_handoff_card/reauthorize",
            vec![
                LandingRouteClass::PostEntryHandoffCard,
                LandingRouteClass::PriorActiveEditor,
            ],
            None,
            "Showed a handoff card to re-authorize without touching your local work.",
        ),
        zone_identity: ZoneIdentityInput {
            cues: vec![
                cue(
                    ZoneOwnedCueClass::Breadcrumb,
                    ShellZoneToken::TitleContextBar,
                    false,
                ),
                cue(
                    ZoneOwnedCueClass::TrustBadge,
                    ShellZoneToken::TitleContextBar,
                    true,
                ),
            ],
        },
        responsive: ResponsiveFallbackInput {
            window_class: WindowClassToken::CompactDesktop,
            collapsed_surfaces: vec![collapsed(
                "aureline://surface/left_sidebar",
                ShellZoneToken::LeftSidebar,
                CollapseTargetClass::Overlay,
                "aureline://command/sidebar.reopen",
                "aureline://state/left_sidebar_last",
            )],
        },
        diagnostics_export_ref: "aureline://diagnostics/revoked_authorization".to_owned(),
        support_export_ref: "aureline://support_export/revoked_authorization".to_owned(),
        evidence_refs: vec!["aureline://trace/revoked_authorization".to_owned()],
        narrative_refs: vec![
            "aureline://doc/harden_shell_startup_warm_restore_and_first_useful".to_owned(),
        ],
    };
    WarmContinuityScenario {
        scenario_id: "revoked_authorization",
        scenario_label: "Revoked authorization",
        narrative: "Revoked authority never blocks local work: editors restore and provider \
                    mutations are held for fresh authorization and review.",
        fixture_filename: "revoked_authorization.json",
        expected_entry_cause: EntryCauseClass::RevokedAuthorization,
        expected_restore_class: RestoreClassToken::CompatibleRestore,
        expected_window_class: WindowClassToken::CompactDesktop,
        expected_landing_route: LandingRouteClass::PostEntryHandoffCard,
        expected_honesty_marker_present: true,
        expected_restored_exactly_count: 2,
        expected_restored_partially_count: 0,
        expected_needs_review_count: 1,
        input,
    }
}
