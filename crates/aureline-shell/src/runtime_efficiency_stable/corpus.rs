//! Deterministic claimed-stable matrix for runtime-efficiency records.
//!
//! Every record here is a genuine projection of the live efficiency runtime in
//! [`crate::efficiency`] (the power/thermal policy, the workload-budget
//! decisions, and the hidden-pane render audit) and the suspend-resume /
//! power-posture page in [`crate::runtime_adaptation`]. The corpus drives a real
//! [`crate::efficiency::EfficiencyStateRuntime`] into each posture's state,
//! reads the resulting workload-budget and render-visibility decisions, projects
//! the suspend-resume continuity from the seeded runtime-adaptation page, and
//! reconciles each posture through the governed
//! [`RuntimeEfficiencyRecord::build`] builder, so a record can never drift from
//! what ships.
//!
//! The matrix materializes all five runtime-efficiency states and spans Stable
//! and narrowed rows:
//!
//! - Nominal (AC power), EfficiencyAware (battery saver), ThermalConstrained
//!   (thermal clamp), ProtectCore (critical battery), and Recovery
//!   (suspend/resume) postures qualify **Stable**.
//! - A foreground-latency regression drill narrows below Stable with a named
//!   reason because a protected path exceeds its published band.
//! - A hidden-pane render-leak drill narrows below Stable because a hidden pane
//!   keeps painting and polling off-screen.
//! - A low-disk ProtectCore posture whose diagnostics-review binding surface is
//!   still in preview narrows to **Preview** by its lowest binding surface
//!   marker instead of inheriting an adjacent green row.

use crate::efficiency::{
    EfficiencyDurabilityInvariants, EfficiencyPressureSource, EfficiencyState,
    EfficiencyStateRuntime, HiddenPaneRenderAudit, ProtectedSurfaceClass, RenderVisibilityDecision,
    RenderVisibilityInput, VisibilityState, WorkloadBudgetDecision, WorkloadFamily,
    RENDER_VISIBILITY_DECISION_RECORD_KIND,
};
use crate::notification_attention_stable::model::{
    AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, RecoveryRouteRecord, StableClaimClass,
};
use crate::runtime_adaptation::{seeded_runtime_adaptation_page, SuspendResumeRow};

use super::model::{
    required_recovery_routes, EfficiencyClaimCeiling, EfficiencySurfaceProjectionInput,
    EfficiencyTruthSurface, EfficiencyUpstream, GovernorReasonClass, PlatformConformanceRow,
    PlatformProfileClass, ProtectedForegroundPath, ProtectedPathRow, QueueGovernorDisclosure,
    ResumeOwner, RuntimeEfficiencyInput, RuntimeEfficiencyRecord, ShedWorkRow,
    SuspendResumeContinuity,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/runtime-efficiency-adaptation";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/runtime-efficiency-adaptation";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-runtime-efficiency-adaptation";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-runtime-efficiency-adaptation";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-runtime-efficiency-adaptation";

/// One scenario in the claimed-stable runtime-efficiency matrix.
#[derive(Debug, Clone)]
pub struct RuntimeEfficiencyScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Posture token pinned for the scenario.
    pub expected_posture: String,
    /// Expected runtime-efficiency state.
    pub expected_state: EfficiencyState,
    /// Expected governor reason.
    pub expected_governor: GovernorReasonClass,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest binding surface).
    pub expected_surface_marker: LifecycleMarker,
    record: RuntimeEfficiencyRecord,
}

impl RuntimeEfficiencyScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> RuntimeEfficiencyRecord {
        self.record.clone()
    }
}

/// The claimed-stable runtime-efficiency matrix, in canonical order.
pub fn runtime_efficiency_corpus() -> Vec<RuntimeEfficiencyScenario> {
    let page = seeded_runtime_adaptation_page();
    crate::runtime_adaptation::validate_runtime_adaptation_page(&page)
        .expect("seeded runtime-adaptation page must validate");
    let page_ref = page.shared_contract_ref.clone();

    vec![
        nominal_ac_power(&page_ref),
        battery_saver_efficiency_aware(&page_ref),
        thermal_constrained(&page_ref),
        critical_battery_protect_core(&page_ref),
        suspend_resume_recovery(&page, &page_ref),
        foreground_latency_regression_drill(&page_ref),
        hidden_pane_render_leak_drill(&page_ref),
        low_disk_preview_surface(&page_ref),
    ]
}

// ---------------------------------------------------------------------------
// Projection helpers
// ---------------------------------------------------------------------------

const EDIT_BAND_MS: u32 = 50;
const SAVE_BAND_MS: u32 = 250;
const NAV_BAND_MS: u32 = 150;
const QUICK_OPEN_BAND_MS: u32 = 200;
const PALETTE_BAND_MS: u32 = 120;

/// Builds the five protected-path latency rows from observed p99 values in the
/// canonical order edit / save / nav / quick-open / palette.
fn protected_paths(observed: [u32; 5]) -> Vec<ProtectedPathRow> {
    let bands = [
        (ProtectedForegroundPath::EditTyping, EDIT_BAND_MS),
        (ProtectedForegroundPath::SaveDocument, SAVE_BAND_MS),
        (ProtectedForegroundPath::DirectNavigation, NAV_BAND_MS),
        (ProtectedForegroundPath::QuickOpen, QUICK_OPEN_BAND_MS),
        (ProtectedForegroundPath::CommandPalette, PALETTE_BAND_MS),
    ];
    bands
        .iter()
        .zip(observed.iter())
        .map(|((path, band), observed_p99)| {
            let within = observed_p99 <= band;
            ProtectedPathRow {
                path: *path,
                published_band_ms: *band,
                observed_p99_ms: *observed_p99,
                within_band: within,
                preserved_under_posture: true,
            }
        })
        .collect()
}

/// Drives a real efficiency runtime into `state` for the given pressure source.
fn runtime_in_state(
    state: EfficiencyState,
    source: EfficiencyPressureSource,
    reason: &str,
) -> EfficiencyStateRuntime {
    let mut runtime = EfficiencyStateRuntime::new();
    if state != EfficiencyState::Nominal {
        runtime.transition_to(state, source, reason.to_string(), CORPUS_AS_OF);
    }
    runtime
}

/// Projects a workload-budget decision into a shed-work row.
fn shed_row(
    decision: &WorkloadBudgetDecision,
    resume_owner: ResumeOwner,
    resume_condition: &str,
) -> ShedWorkRow {
    ShedWorkRow {
        workload_id: decision.workload_id.clone(),
        work_class: decision.work_class.clone(),
        queue_lane: decision.queue_lane.clone(),
        action: decision.action.clone(),
        changes_behavior: decision.changed_behavior(),
        shed_before_foreground: true,
        user_impact_label: decision.capability_row.user_impact_label.clone(),
        resume_owner,
        resume_condition: resume_condition.to_string(),
    }
}

/// The render-visibility decisions a posture commits: a focused editor and a
/// background terminal keep painting; hidden / collapsed / occluded panes are
/// suppressed by the runtime.
fn render_decisions(runtime: &EfficiencyStateRuntime) -> Vec<RenderVisibilityDecision> {
    vec![
        runtime.decide_render(RenderVisibilityInput {
            surface_id: "surface:editor:focused".to_string(),
            surface_class: ProtectedSurfaceClass::EditorViewport,
            visibility_state: VisibilityState::VisibleFocused,
            requested_paint_count: 8,
            requested_animation_tick_count: 0,
            correctness_polling_required: true,
        }),
        runtime.decide_render(RenderVisibilityInput {
            surface_id: "surface:terminal:background".to_string(),
            surface_class: ProtectedSurfaceClass::TerminalViewport,
            visibility_state: VisibilityState::VisibleBackground,
            requested_paint_count: 3,
            requested_animation_tick_count: 0,
            correctness_polling_required: true,
        }),
        runtime.decide_render(RenderVisibilityInput {
            surface_id: "surface:preview:hidden".to_string(),
            surface_class: ProtectedSurfaceClass::PreviewViewport,
            visibility_state: VisibilityState::HiddenTab,
            requested_paint_count: 6,
            requested_animation_tick_count: 4,
            correctness_polling_required: false,
        }),
        runtime.decide_render(RenderVisibilityInput {
            surface_id: "surface:graph:collapsed".to_string(),
            surface_class: ProtectedSurfaceClass::GraphPanel,
            visibility_state: VisibilityState::CollapsedSplit,
            requested_paint_count: 3,
            requested_animation_tick_count: 2,
            correctness_polling_required: false,
        }),
        runtime.decide_render(RenderVisibilityInput {
            surface_id: "surface:diff:occluded".to_string(),
            surface_class: ProtectedSurfaceClass::DiffReviewViewport,
            visibility_state: VisibilityState::OccludedWindow,
            requested_paint_count: 5,
            requested_animation_tick_count: 0,
            correctness_polling_required: true,
        }),
    ]
}

fn clean_render_audit(runtime: &EfficiencyStateRuntime) -> HiddenPaneRenderAudit {
    HiddenPaneRenderAudit::from_decisions(&render_decisions(runtime))
}

/// A render audit where one hidden pane leaks committed paint and a background
/// poll — the violation the leak drill must narrow on.
fn leaky_render_audit(runtime: &EfficiencyStateRuntime) -> HiddenPaneRenderAudit {
    let mut decisions = render_decisions(runtime);
    decisions.push(RenderVisibilityDecision {
        record_kind: RENDER_VISIBILITY_DECISION_RECORD_KIND.to_string(),
        event_id: "render_visibility_decision".to_string(),
        surface_id: "surface:preview:hidden-leak".to_string(),
        surface_class: ProtectedSurfaceClass::PreviewViewport.as_str().to_string(),
        visibility_state: VisibilityState::HiddenTab.as_str().to_string(),
        efficiency_state: runtime.current_state().as_str().to_string(),
        paint_suppressed: false,
        animation_suppressed: false,
        polling_mode: "off_screen_poll_leaking".to_string(),
        committed_paint_count: 1,
        hidden_pane_work: 2,
        offscreen_suppression_eligible: 0,
        hidden_pane_violation: true,
    });
    HiddenPaneRenderAudit::from_decisions(&decisions)
}

fn surface_inputs(diagnostics_marker: LifecycleMarker) -> Vec<EfficiencySurfaceProjectionInput> {
    vec![
        EfficiencySurfaceProjectionInput {
            surface: EfficiencyTruthSurface::ShellStatusStrip,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
        EfficiencySurfaceProjectionInput {
            surface: EfficiencyTruthSurface::DiagnosticsReview,
            surface_marker: diagnostics_marker,
            reads_shared_record: true,
        },
        EfficiencySurfaceProjectionInput {
            surface: EfficiencyTruthSurface::CliInspect,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
        EfficiencySurfaceProjectionInput {
            surface: EfficiencyTruthSurface::HelpAbout,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
        EfficiencySurfaceProjectionInput {
            surface: EfficiencyTruthSurface::SupportExport,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
    ]
}

fn routes_for(posture: &str) -> Vec<EntryRouteRecord> {
    AttentionRouteSurface::REQUIRED
        .iter()
        .map(|surface| EntryRouteRecord {
            surface: *surface,
            route_ref: format!(
                "aureline://efficiency-posture/{posture}/{}",
                surface.as_str()
            ),
            keyboard_reachable: true,
            activates_same_item: true,
        })
        .collect()
}

fn accessibility_for(
    focus_order_index: u32,
    routes: &[RecoveryRouteRecord],
) -> AccessibilityDisclosure {
    AccessibilityDisclosure {
        focus_order_index,
        tab_stop_count: routes.len() as u32 + 1,
        row_narration:
            "Runtime efficiency posture, owned by the runtime power manager; paused background work \
             and its resume owner are listed below."
                .to_string(),
        action_labels: routes.iter().map(|route| route.action_label.clone()).collect(),
        layout_modes: LayoutMode::REQUIRED
            .iter()
            .map(|mode| LayoutModeDisclosure {
                mode: *mode,
                row_narration_available: true,
                recovery_affordances_reachable: true,
            })
            .collect(),
    }
}

fn platform_rows(downgrades: &[&str]) -> Vec<PlatformConformanceRow> {
    let downgrade_tokens: Vec<String> = downgrades.iter().map(|d| (*d).to_string()).collect();
    let mapping = [
        (PlatformProfileClass::MacOs, "macos_15_plus_universal"),
        (PlatformProfileClass::Windows, "windows_11_23h2_plus_x86_64"),
        (
            PlatformProfileClass::Linux,
            "linux_ubuntu_24_04_gnome_wayland_x86_64",
        ),
    ];
    mapping
        .iter()
        .map(|(profile, profile_id)| PlatformConformanceRow {
            profile: *profile,
            profile_id: (*profile_id).to_string(),
            covered: true,
            proof_ref: format!("efficiency-lab:{}:{profile_id}", profile.as_str()),
            named_downgrade_behaviors: downgrade_tokens.clone(),
        })
        .collect()
}

fn full_claim_ceiling() -> EfficiencyClaimCeiling {
    EfficiencyClaimCeiling {
        asserts_efficiency_state_materialized: true,
        asserts_background_shed_before_foreground: true,
        asserts_foreground_within_latency_bands: true,
        asserts_hidden_panes_quiescent: true,
        asserts_governor_reason_surfaced: true,
        asserts_durable_state_preserved: true,
        asserts_platform_conformance_complete: true,
    }
}

fn paused_lane_tokens(shed_work: &[ShedWorkRow]) -> Vec<String> {
    let mut lanes: Vec<String> = shed_work
        .iter()
        .filter(|row| row.changes_behavior)
        .map(|row| row.queue_lane.clone())
        .collect();
    lanes.sort();
    lanes.dedup();
    lanes
}

fn contributing_decision_refs(shed_work: &[ShedWorkRow]) -> Vec<String> {
    shed_work
        .iter()
        .map(|row| format!("workload-decision:{}", row.workload_id))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn scenario(
    scenario_id: &'static str,
    posture: &'static str,
    posture_label: &str,
    title: &str,
    summary: &str,
    state: EfficiencyState,
    governor: QueueGovernorDisclosure,
    shed_work: Vec<ShedWorkRow>,
    observed_latency: [u32; 5],
    hidden_pane_audit: HiddenPaneRenderAudit,
    suspend_resume: Option<SuspendResumeContinuity>,
    diagnostics_marker: LifecycleMarker,
    claim_ceiling: EfficiencyClaimCeiling,
    resumable: bool,
    overridable: bool,
    downgrades: &[&str],
    page_ref: &str,
    expected_state: EfficiencyState,
    expected_governor: GovernorReasonClass,
    expected_claim_class: StableClaimClass,
    expected_qualifies_stable: bool,
    expected_surface_marker: LifecycleMarker,
) -> RuntimeEfficiencyScenario {
    let recovery_routes = required_recovery_routes(resumable, overridable);
    let accessibility = accessibility_for(3, &recovery_routes);
    let routes = routes_for(posture);
    let protected = protected_paths(observed_latency);
    let upstream = EfficiencyUpstream {
        efficiency_snapshot_ref: format!("efficiency-snapshot:{posture}:01"),
        runtime_adaptation_page_ref: page_ref.to_string(),
        contributing_decision_refs: contributing_decision_refs(&shed_work),
    };
    let input = RuntimeEfficiencyInput {
        record_id: scenario_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture.to_string(),
        posture_label: posture_label.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        efficiency_state: state,
        governor,
        shed_work,
        protected_paths: protected,
        hidden_pane_audit,
        durability: EfficiencyDurabilityInvariants::default(),
        suspend_resume,
        platform_conformance: platform_rows(downgrades),
        surface_projections: surface_inputs(diagnostics_marker),
        claim_ceiling,
        recovery_routes,
        routes,
        accessibility,
        available_without_account: true,
        available_without_managed_services: true,
        upstream,
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_string(),
            EVIDENCE_FIXTURE_REF.to_string(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    let record = RuntimeEfficiencyRecord::build(input)
        .unwrap_or_else(|err| panic!("scenario {scenario_id} must build: {err}"));
    RuntimeEfficiencyScenario {
        scenario_id,
        fixture_filename: format!("{scenario_id}.json"),
        expected_posture: posture.to_string(),
        expected_state,
        expected_governor,
        expected_claim_class,
        expected_qualifies_stable,
        expected_surface_marker,
        record,
    }
}

fn governor(
    reason: GovernorReasonClass,
    paused_lane_tokens: Vec<String>,
    resume_owner: ResumeOwner,
    surfaced: bool,
) -> QueueGovernorDisclosure {
    QueueGovernorDisclosure {
        reason,
        reason_label: reason.label().to_string(),
        paused_lane_tokens,
        resume_owner,
        resume_owner_label: resume_owner.label().to_string(),
        surfaced_in_status_strip: surfaced,
        surfaced_in_diagnostics: surfaced,
        not_generic_slowness: true,
        not_stale_masquerade: true,
    }
}

// ---------------------------------------------------------------------------
// Stable scenarios
// ---------------------------------------------------------------------------

fn nominal_ac_power(page_ref: &str) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::Nominal,
        EfficiencyPressureSource::AcPower,
        "ac power",
    );
    let audit = clean_render_audit(&runtime);
    let governor = governor(
        GovernorReasonClass::NoneNominal,
        vec![],
        ResumeOwner::AutomaticOnPressureClear,
        false,
    );
    scenario(
        "nominal_ac_power_stable",
        "nominal-ac-power",
        "Nominal — AC power",
        "Nominal runtime efficiency on AC power",
        "On AC power the shell runs ordinary published budgets, sheds no background work, and keeps every protected foreground path well within band.",
        EfficiencyState::Nominal,
        governor,
        vec![],
        [16, 70, 45, 60, 30],
        audit,
        None,
        LifecycleMarker::Stable,
        full_claim_ceiling(),
        false,
        false,
        &["none"],
        page_ref,
        EfficiencyState::Nominal,
        GovernorReasonClass::NoneNominal,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn battery_saver_efficiency_aware(page_ref: &str) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::EfficiencyAware,
        EfficiencyPressureSource::OsBatterySaver,
        "os battery saver active",
    );
    let source = EfficiencyPressureSource::OsBatterySaver;
    let prefetch =
        runtime.decide_workload(WorkloadFamily::SpeculativePrefetch, source, CORPUS_AS_OF);
    let warmup = runtime.decide_workload(WorkloadFamily::AiWarmup, source, CORPUS_AS_OF);
    let indexing = runtime.decide_workload(WorkloadFamily::IndexingRefresh, source, CORPUS_AS_OF);
    let shed = vec![
        shed_row(
            &prefetch,
            ResumeOwner::AutomaticOnPressureClear,
            "Prefetch resumes when battery saver clears.",
        ),
        shed_row(
            &warmup,
            ResumeOwner::AutomaticOnPressureClear,
            "AI warmups resume when battery saver clears.",
        ),
        shed_row(
            &indexing,
            ResumeOwner::AutomaticOnPressureClear,
            "Indexing returns to full cadence when battery saver clears.",
        ),
    ];
    let audit = clean_render_audit(&runtime);
    let governor = governor(
        GovernorReasonClass::BatterySaver,
        paused_lane_tokens(&shed),
        ResumeOwner::AutomaticOnPressureClear,
        true,
    );
    scenario(
        "battery_saver_efficiency_aware_stable",
        "battery-saver-efficiency-aware",
        "Efficiency aware — battery saver",
        "Efficiency-aware adaptation under OS battery saver",
        "Under battery saver the shell throttles prefetch and indexing and defers AI warmups before any foreground regression, names the battery-saver reason in the status strip and diagnostics, and keeps editing, save, navigation, quick-open, and the palette within band.",
        EfficiencyState::EfficiencyAware,
        governor,
        shed,
        [20, 90, 60, 80, 40],
        audit,
        None,
        LifecycleMarker::Stable,
        full_claim_ceiling(),
        true,
        true,
        &["speculative_prefetch_throttled", "ai_warmup_deferred", "indexing_refresh_throttled"],
        page_ref,
        EfficiencyState::EfficiencyAware,
        GovernorReasonClass::BatterySaver,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn thermal_constrained(page_ref: &str) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::ThermalConstrained,
        EfficiencyPressureSource::ThermalPressure,
        "host thermal pressure",
    );
    let source = EfficiencyPressureSource::ThermalPressure;
    let warmup = runtime.decide_workload(WorkloadFamily::AiWarmup, source, CORPUS_AS_OF);
    let preview = runtime.decide_workload(WorkloadFamily::PreviewRefresh, source, CORPUS_AS_OF);
    let graph = runtime.decide_workload(WorkloadFamily::GraphEnrichment, source, CORPUS_AS_OF);
    let shed = vec![
        shed_row(
            &warmup,
            ResumeOwner::AutomaticOnPressureClear,
            "AI warmups resume when the host cools.",
        ),
        shed_row(
            &preview,
            ResumeOwner::AutomaticOnPressureClear,
            "Preview refresh returns to full cadence when the host cools.",
        ),
        shed_row(
            &graph,
            ResumeOwner::AutomaticOnPressureClear,
            "Graph enrichment resumes when the host cools.",
        ),
    ];
    let audit = clean_render_audit(&runtime);
    let governor = governor(
        GovernorReasonClass::ThermalClamp,
        paused_lane_tokens(&shed),
        ResumeOwner::AutomaticOnPressureClear,
        true,
    );
    scenario(
        "thermal_constrained_stable",
        "thermal-constrained",
        "Thermal constrained — thermal clamp",
        "Thermal-constrained adaptation under host thermal pressure",
        "Under thermal pressure the shell pauses AI warmups and graph enrichment and throttles preview refresh, marks any stale preview snapshot explicitly, and keeps every protected foreground path within band while naming the thermal-clamp reason.",
        EfficiencyState::ThermalConstrained,
        governor,
        shed,
        [24, 110, 70, 95, 48],
        audit,
        None,
        LifecycleMarker::Stable,
        full_claim_ceiling(),
        true,
        true,
        &["ai_warmup_paused", "graph_enrichment_paused", "preview_refresh_throttled"],
        page_ref,
        EfficiencyState::ThermalConstrained,
        GovernorReasonClass::ThermalClamp,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn critical_battery_protect_core(page_ref: &str) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::ProtectCore,
        EfficiencyPressureSource::CriticalBattery,
        "critical battery",
    );
    let source = EfficiencyPressureSource::CriticalBattery;
    let upload = runtime.decide_workload(WorkloadFamily::UploadTransfer, source, CORPUS_AS_OF);
    let extension = runtime.decide_workload(WorkloadFamily::ExtensionPolling, source, CORPUS_AS_OF);
    let indexing = runtime.decide_workload(WorkloadFamily::IndexingRefresh, source, CORPUS_AS_OF);
    let shed = vec![
        shed_row(
            &upload,
            ResumeOwner::UserRestorePower,
            "Uploads resume after you plug in.",
        ),
        shed_row(
            &extension,
            ResumeOwner::UserRestorePower,
            "Extension background polling resumes after you plug in.",
        ),
        shed_row(
            &indexing,
            ResumeOwner::UserRestorePower,
            "Indexing resumes after you plug in.",
        ),
    ];
    let audit = clean_render_audit(&runtime);
    let governor = governor(
        GovernorReasonClass::CriticalBattery,
        paused_lane_tokens(&shed),
        ResumeOwner::UserRestorePower,
        true,
    );
    scenario(
        "critical_battery_protect_core_stable",
        "critical-battery-protect-core",
        "Protect core — critical battery",
        "Protect-core adaptation at critical battery",
        "At critical battery only core interaction is funded: uploads are denied, extension polling and indexing pause, the user is told to restore power to resume, and editing, save, and navigation stay within band with no loss of local durable state.",
        EfficiencyState::ProtectCore,
        governor,
        shed,
        [28, 130, 85, 110, 55],
        audit,
        None,
        LifecycleMarker::Stable,
        full_claim_ceiling(),
        true,
        false,
        &["upload_transfer_denied", "extension_polling_paused", "indexing_refresh_paused"],
        page_ref,
        EfficiencyState::ProtectCore,
        GovernorReasonClass::CriticalBattery,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn suspend_resume_recovery(
    page: &crate::runtime_adaptation::RuntimeAdaptationPage,
    page_ref: &str,
) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::Recovery,
        EfficiencyPressureSource::PressureCleared,
        "resume from sleep; deferred work staging",
    );
    let source = EfficiencyPressureSource::PressureCleared;
    let warmup = runtime.decide_workload(WorkloadFamily::AiWarmup, source, CORPUS_AS_OF);
    let prefetch =
        runtime.decide_workload(WorkloadFamily::SpeculativePrefetch, source, CORPUS_AS_OF);
    let shed = vec![
        shed_row(
            &warmup,
            ResumeOwner::RemoteReconnect,
            "AI warmups stage back after reconnect completes.",
        ),
        shed_row(
            &prefetch,
            ResumeOwner::AutomaticOnPressureClear,
            "Prefetch stages back automatically after resume.",
        ),
    ];
    let audit = clean_render_audit(&runtime);
    let continuity = suspend_resume_continuity(page);
    let governor = governor(
        GovernorReasonClass::SuspendResume,
        paused_lane_tokens(&shed),
        ResumeOwner::RemoteReconnect,
        true,
    );
    scenario(
        "suspend_resume_recovery_stable",
        "suspend-resume-recovery",
        "Recovery — resume from sleep",
        "Recovery adaptation after a suspend/resume cycle",
        "After waking from sleep local editing and the palette are immediately responsive, deferred work stages back in order, remote authority is held until reconnect, cached views are labeled rather than shown as fresh, and the resume summary names what is staging.",
        EfficiencyState::Recovery,
        governor,
        shed,
        [22, 95, 62, 82, 42],
        audit,
        Some(continuity),
        LifecycleMarker::Stable,
        full_claim_ceiling(),
        true,
        false,
        &["deferred_work_staged_resume", "remote_authority_held_pending_reconnect", "cached_view_labeled_stale"],
        page_ref,
        EfficiencyState::Recovery,
        GovernorReasonClass::SuspendResume,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
    )
}

fn suspend_resume_continuity(
    page: &crate::runtime_adaptation::RuntimeAdaptationPage,
) -> SuspendResumeContinuity {
    let row: &SuspendResumeRow = page
        .suspend_resume_rows
        .iter()
        .find(|row| row.case_id == "shell:runtime-adaptation:suspend-resume:macos-wake:01")
        .expect("runtime-adaptation page must carry the macOS wake suspend-resume row");
    SuspendResumeContinuity {
        case_id_ref: row.case_id.clone(),
        event_token: row.event.as_str().to_string(),
        continuity_summary_tokens: row
            .continuity_summary
            .iter()
            .map(|summary| summary.as_str().to_string())
            .collect(),
        local_work_continues: row.local_work_continues,
        privileged_or_mutating_work_paused: row.privileged_or_mutating_work_paused,
        no_silent_rerun_or_authority_reuse: row.no_silent_rerun_or_authority_reuse,
        user_visible_resume_summary_required: row.user_visible_resume_summary_required,
        recovery_action_tokens: row.recovery_action_tokens.clone(),
    }
}

// ---------------------------------------------------------------------------
// Narrowed drills
// ---------------------------------------------------------------------------

fn foreground_latency_regression_drill(page_ref: &str) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::EfficiencyAware,
        EfficiencyPressureSource::OsBatterySaver,
        "os battery saver active",
    );
    let source = EfficiencyPressureSource::OsBatterySaver;
    let prefetch =
        runtime.decide_workload(WorkloadFamily::SpeculativePrefetch, source, CORPUS_AS_OF);
    let shed = vec![shed_row(
        &prefetch,
        ResumeOwner::AutomaticOnPressureClear,
        "Prefetch resumes when battery saver clears.",
    )];
    let audit = clean_render_audit(&runtime);
    let governor = governor(
        GovernorReasonClass::BatterySaver,
        paused_lane_tokens(&shed),
        ResumeOwner::AutomaticOnPressureClear,
        true,
    );
    let mut ceiling = full_claim_ceiling();
    // The product cannot honestly assert the foreground latency band on this row.
    ceiling.asserts_foreground_within_latency_bands = false;
    scenario(
        "foreground_latency_regression_drill",
        "foreground-latency-regression",
        "Drill — foreground latency regression",
        "Drill: typing exceeds its published latency band under battery saver",
        "An adversarial drill where battery-saver throttling lets typing p99 exceed its published band; the row narrows below Stable with a named reason rather than claiming efficiency by quietly slowing active editing.",
        EfficiencyState::EfficiencyAware,
        governor,
        shed,
        [80, 90, 60, 80, 40],
        audit,
        None,
        LifecycleMarker::Stable,
        ceiling,
        true,
        true,
        &["speculative_prefetch_throttled"],
        page_ref,
        EfficiencyState::EfficiencyAware,
        GovernorReasonClass::BatterySaver,
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn hidden_pane_render_leak_drill(page_ref: &str) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::ThermalConstrained,
        EfficiencyPressureSource::ThermalPressure,
        "host thermal pressure",
    );
    let source = EfficiencyPressureSource::ThermalPressure;
    let preview = runtime.decide_workload(WorkloadFamily::PreviewRefresh, source, CORPUS_AS_OF);
    let shed = vec![shed_row(
        &preview,
        ResumeOwner::AutomaticOnPressureClear,
        "Preview refresh returns to full cadence when the host cools.",
    )];
    let audit = leaky_render_audit(&runtime);
    let governor = governor(
        GovernorReasonClass::ThermalClamp,
        paused_lane_tokens(&shed),
        ResumeOwner::AutomaticOnPressureClear,
        true,
    );
    let mut ceiling = full_claim_ceiling();
    // The product cannot honestly assert hidden-pane quiescence on this row.
    ceiling.asserts_hidden_panes_quiescent = false;
    scenario(
        "hidden_pane_render_leak_drill",
        "hidden-pane-render-leak",
        "Drill — hidden-pane render leak",
        "Drill: a hidden preview pane keeps painting and polling off-screen",
        "An adversarial drill where a hidden preview pane keeps committing paint and a background poll while the row claims thermal protection; the hidden-pane audit fails and the row narrows below Stable with a named reason.",
        EfficiencyState::ThermalConstrained,
        governor,
        shed,
        [24, 110, 70, 95, 48],
        audit,
        None,
        LifecycleMarker::Stable,
        ceiling,
        true,
        true,
        &["preview_refresh_throttled"],
        page_ref,
        EfficiencyState::ThermalConstrained,
        GovernorReasonClass::ThermalClamp,
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
    )
}

fn low_disk_preview_surface(page_ref: &str) -> RuntimeEfficiencyScenario {
    let runtime = runtime_in_state(
        EfficiencyState::ProtectCore,
        EfficiencyPressureSource::PolicyCap,
        "low local disk headroom",
    );
    let source = EfficiencyPressureSource::PolicyCap;
    let upload = runtime.decide_workload(WorkloadFamily::UploadTransfer, source, CORPUS_AS_OF);
    let indexing = runtime.decide_workload(WorkloadFamily::IndexingRefresh, source, CORPUS_AS_OF);
    let shed = vec![
        shed_row(
            &upload,
            ResumeOwner::UserResume,
            "Uploads resume after you free disk space.",
        ),
        shed_row(
            &indexing,
            ResumeOwner::UserResume,
            "Indexing resumes after you free disk space.",
        ),
    ];
    let audit = clean_render_audit(&runtime);
    let governor = governor(
        GovernorReasonClass::LowDisk,
        paused_lane_tokens(&shed),
        ResumeOwner::UserResume,
        true,
    );
    scenario(
        "low_disk_protect_core_preview_surface",
        "low-disk-protect-core",
        "Protect core — low disk (preview review surface)",
        "Protect-core adaptation under low disk with a preview diagnostics surface",
        "Low local disk headroom pauses uploads and indexing to protect core editing; the posture proves every pillar, but its diagnostics-review binding surface is still in preview, so the row is narrowed to Preview by its lowest binding surface marker rather than inheriting an adjacent green row.",
        EfficiencyState::ProtectCore,
        governor,
        shed,
        [26, 120, 80, 100, 50],
        audit,
        None,
        LifecycleMarker::Preview,
        full_claim_ceiling(),
        true,
        false,
        &["upload_transfer_paused", "indexing_refresh_paused"],
        page_ref,
        EfficiencyState::ProtectCore,
        GovernorReasonClass::LowDisk,
        StableClaimClass::Preview,
        false,
        LifecycleMarker::Preview,
    )
}
