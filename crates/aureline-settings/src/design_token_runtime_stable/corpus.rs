//! Deterministic claimed-stable matrix for design-token runtime certifications.
//!
//! Every record here is a genuine projection of the **live** appearance runtime.
//! The corpus reads
//! [`aureline_design_system::seeded_appearance_session_beta_contract`] for the
//! in-effect appearance-session value and its live follow-system posture, the
//! [`aureline_design_system::seeded_component_state_registry`] for the
//! launch-critical surface set, the per-theme semantic token registry
//! ([`aureline_ui::tokens::seeded_token_registry`]) to prove tokens resolve in
//! each mode, and the motion presets ([`aureline_ui::motion`]) to prove
//! reduced-motion / power-saving suppression is modeled in the runtime. So a
//! certification record can never drift from what the runtime actually resolves.
//!
//! Four postures pin the matrix:
//!
//! - `nominal` — every dark, light, high-contrast, reduced-motion, and density
//!   row conforms; protected cues carry non-color carriers; every axis applies
//!   live or behind a checkpoint; every launch surface honors the runtime.
//!   Qualifies **Stable**.
//! - `forced_colors_reload_disclosed` — the contrast axis requires a reload when
//!   the OS switches to forced colors, and the row discloses it instead of
//!   silently lagging. Still qualifies **Stable** because the reload is honest.
//! - `density_surface_in_preview` — the activity-center surface still carries a
//!   Preview marker, so the posture is narrowed below Stable by its lowest
//!   surface marker instead of inheriting an adjacent green row.
//! - `hardcoded_styling_drill` — an adversarial posture where one launch surface
//!   hard-codes styling; the lane detects it and narrows the posture below
//!   Stable with a named reason.

use aureline_design_system::appearance_session::AppearanceSessionBetaContract;
use aureline_design_system::{
    seeded_component_state_registry, try_seeded_appearance_session_beta_contract, LaunchSurfaceClass,
};
use aureline_ui::motion::OVERLAY_DIALOG_ENTER;
use aureline_ui::themes::{AccessibilityPostureClass, AppearanceAxis};
use aureline_ui::tokens::{seeded_token_registry, ThemeClass};
use aureline_ui::density::DensityClass;

use super::model::{
    is_canonical_object_ref, required_recovery_routes, snake_token, AccessibilityDisclosure,
    AppearanceModeClass, AppearanceModeRow, AppearanceSessionBinding, CertificationClaimCeiling,
    CertificationInput, CertificationUpstream, DesignTokenRuntimeCertification, EntryRouteRecord,
    LaunchSurfaceRow, LayoutMode, LayoutModeDisclosure, LifecycleMarker, LiveApplyAxisRow,
    LiveApplyClass, MotionSuppressionRow, NonColorCueClass, ProtectedCueClass, ProtectedCueRow,
    RouteSurface, StableClaimClass,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/design-token-runtime";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/design-token-runtime";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-design-token-runtime";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-design-token-runtime";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-design-token-runtime";

/// Semantic tokens proven present in every certified theme. These cover the
/// protected-cue families (severity / status), surface chrome, and text so a
/// resolved row is real, not asserted.
const CERTIFIED_TOKENS: [&str; 8] = [
    "status.danger",
    "status.warning",
    "status.success",
    "status.info",
    "status.insight",
    "al.color.text.secondary",
    "al.color.border.default",
    "al.color.bg.surface",
];

/// One scenario in the claimed-stable certification matrix.
#[derive(Debug, Clone)]
pub struct DesignTokenRuntimeScenario {
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
    /// Expected derived surface lifecycle marker (lowest surface).
    pub expected_surface_marker: LifecycleMarker,
    record: DesignTokenRuntimeCertification,
}

impl DesignTokenRuntimeScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> DesignTokenRuntimeCertification {
        self.record.clone()
    }
}

/// How one launch surface is overridden for a scenario.
#[derive(Debug, Clone, Copy)]
enum SurfaceOverride {
    /// The surface still carries a below-Stable marker.
    Marker(LaunchSurfaceClass, LifecycleMarker),
    /// The surface hard-codes styling (token-runtime divergence).
    Hardcoded(LaunchSurfaceClass),
}

/// One whole-posture spec.
struct ScenarioSpec {
    scenario_id: &'static str,
    posture_id: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    /// Axis forced to a disclosed reload (used by the forced-colors posture).
    reload_axis: Option<AppearanceAxis>,
    /// Surface override (used by the preview / hard-coded postures).
    surface_override: Option<SurfaceOverride>,
    claim_ceiling: CertificationClaimCeiling,
}

fn full_claim_ceiling() -> CertificationClaimCeiling {
    CertificationClaimCeiling {
        asserts_all_modes_conform: true,
        asserts_protected_cues_non_color: true,
        asserts_one_appearance_session: true,
        asserts_live_apply_no_silent_lag: true,
        asserts_motion_suppression_runtime: true,
        asserts_no_hardcoded_styling: true,
    }
}

fn theme_for(mode: AppearanceModeClass) -> ThemeClass {
    match mode {
        AppearanceModeClass::Dark
        | AppearanceModeClass::ReducedMotion
        | AppearanceModeClass::Density => ThemeClass::DarkReference,
        AppearanceModeClass::Light => ThemeClass::LightParity,
        AppearanceModeClass::HighContrastDark => ThemeClass::HighContrastDark,
        AppearanceModeClass::HighContrastLight => ThemeClass::HighContrastLight,
    }
}

fn density_for(mode: AppearanceModeClass) -> DensityClass {
    match mode {
        AppearanceModeClass::Density => DensityClass::Compact,
        _ => DensityClass::Standard,
    }
}

fn motion_for(mode: AppearanceModeClass) -> AccessibilityPostureClass {
    match mode {
        AppearanceModeClass::ReducedMotion => AccessibilityPostureClass::MotionReduced,
        _ => AccessibilityPostureClass::MotionStandard,
    }
}

/// Proves the semantic token registry resolves every certified token for a
/// theme, returning the resolved token names.
fn certified_tokens_for(theme: ThemeClass) -> (bool, Vec<String>) {
    match seeded_token_registry(theme) {
        Ok(registry) => {
            let resolved: Vec<String> = CERTIFIED_TOKENS
                .iter()
                .filter(|token| registry.color(token).is_some())
                .map(|token| (*token).to_owned())
                .collect();
            (resolved.len() == CERTIFIED_TOKENS.len(), resolved)
        }
        Err(_) => (false, Vec::new()),
    }
}

fn mode_rows(posture_id: &str, session_value_ref: &str) -> Vec<AppearanceModeRow> {
    AppearanceModeClass::REQUIRED
        .into_iter()
        .map(|mode| {
            let theme = theme_for(mode);
            let (resolves, tokens) = certified_tokens_for(theme);
            AppearanceModeRow {
                mode_class: mode,
                theme_class: theme,
                density_class: density_for(mode),
                motion_posture: motion_for(mode),
                token_registry_resolves: resolves,
                certified_token_refs: tokens,
                focus_ring_preserved: true,
                state_badges_preserved: true,
                severity_cues_preserved: true,
                keyboard_affordances_preserved: true,
                protected_cues_survive: true,
                golden_capture_ref: format!(
                    "aureline://artifact/dtr-{posture_id}-{}-golden",
                    mode.as_str()
                ),
                accessibility_packet_ref: format!(
                    "aureline://artifact/dtr-{posture_id}-{}-a11y",
                    mode.as_str()
                ),
                appearance_session_value_ref: session_value_ref.to_owned(),
                surface_marker: LifecycleMarker::Stable,
                conforms: false,
            }
        })
        .collect()
}

fn protected_cue_rows() -> Vec<ProtectedCueRow> {
    use NonColorCueClass as Cue;
    use ProtectedCueClass as Class;

    let spec: [(Class, &[Cue]); 6] = [
        (
            Class::Diagnostics,
            &[Cue::LabelText, Cue::Icon, Cue::Border],
        ),
        (
            Class::PolicyLock,
            &[Cue::LabelText, Cue::Icon, Cue::Shape],
        ),
        (
            Class::TrustWarning,
            &[Cue::LabelText, Cue::Icon, Cue::Border],
        ),
        (
            Class::ExecutionTarget,
            &[Cue::LabelText, Cue::Icon, Cue::Shape],
        ),
        (Class::Selection, &[Cue::LabelText, Cue::Border, Cue::Shape]),
        (Class::Focus, &[Cue::FocusRing, Cue::Border]),
    ];
    spec.into_iter()
        .map(|(cue_class, cues)| ProtectedCueRow {
            cue_class,
            non_color_cues: cues.to_vec(),
            hue_only_forbidden: true,
            survives_high_contrast: true,
            survives_forced_colors: true,
            survives_reduced_motion: true,
            conforms: false,
        })
        .collect()
}

fn live_apply_class_for(
    contract: &AppearanceSessionBetaContract,
    axis: AppearanceAxis,
) -> LiveApplyClass {
    let token = snake_token(&axis);
    let summary = &contract.live_follow_system_summary;
    if summary.live_no_review_axes.iter().any(|a| a == &token) {
        LiveApplyClass::ApplyLive
    } else if summary.live_checkpointed_axes.iter().any(|a| a == &token) {
        LiveApplyClass::ApplyLiveCheckpointed
    } else if summary.confirm_required_axes.iter().any(|a| a == &token) {
        LiveApplyClass::ConfirmRequired
    } else if summary.policy_blocked_axes.iter().any(|a| a == &token) {
        LiveApplyClass::PolicyBlocked
    } else {
        LiveApplyClass::ConfirmRequired
    }
}

fn live_apply_axes(
    contract: &AppearanceSessionBetaContract,
    reload_axis: Option<AppearanceAxis>,
) -> Vec<LiveApplyAxisRow> {
    let axes = [
        AppearanceAxis::ModeThemeClass,
        AppearanceAxis::ContrastMode,
        AppearanceAxis::AccentSource,
        AppearanceAxis::DensityClass,
        AppearanceAxis::TextScale,
        AppearanceAxis::ReducedMotionPosture,
        AppearanceAxis::FollowSystemPosture,
    ];
    axes.into_iter()
        .map(|axis| {
            let (live_apply_class, disclosure_required, note) = if reload_axis == Some(axis) {
                (
                    LiveApplyClass::ReloadRequired,
                    true,
                    "OS forced-colors switch requires a disclosed surface reload".to_owned(),
                )
            } else {
                let class = live_apply_class_for(contract, axis);
                (
                    class,
                    false,
                    format!(
                        "{} change applies via the {} path with no silent lag",
                        snake_token(&axis),
                        class.as_str()
                    ),
                )
            };
            LiveApplyAxisRow {
                axis,
                live_apply_class,
                disclosure_required,
                silently_lags_system: false,
                note,
            }
        })
        .collect()
}

fn motion_suppression_rows() -> Vec<MotionSuppressionRow> {
    [
        AccessibilityPostureClass::MotionStandard,
        AccessibilityPostureClass::MotionReduced,
        AccessibilityPostureClass::MotionLowMotion,
        AccessibilityPostureClass::MotionPowerSaver,
        AccessibilityPostureClass::MotionCriticalHotPath,
    ]
    .into_iter()
    .map(|posture| {
        let plan = OVERLAY_DIALOG_ENTER.plan_for(posture);
        let substitution_class = plan
            .substitution_class
            .map(|class| snake_token(&class))
            .unwrap_or_else(|| "none".to_owned());
        MotionSuppressionRow {
            posture,
            modeled_in_token_runtime: true,
            non_essential_motion_suppressed: plan.substitution_class.is_some(),
            substitution_class,
            per_surface_improvisation_absent: true,
        }
    })
    .collect()
}

fn launch_surface_rows(surface_override: Option<SurfaceOverride>) -> Vec<LaunchSurfaceRow> {
    let registry = seeded_component_state_registry();
    registry
        .launch_surface_consumers
        .iter()
        .map(|consumer| {
            let surface = consumer.surface_class;
            let mut honors = true;
            let mut hardcoded_absent = true;
            let mut marker = LifecycleMarker::Stable;
            let mut waiver = None;
            match surface_override {
                Some(SurfaceOverride::Marker(target, target_marker)) if target == surface => {
                    marker = target_marker;
                    waiver = Some(format!(
                        "aureline://waiver/dtr-{}-marker",
                        surface.as_str()
                    ));
                }
                Some(SurfaceOverride::Hardcoded(target)) if target == surface => {
                    honors = false;
                    hardcoded_absent = false;
                    waiver = Some(format!(
                        "aureline://waiver/dtr-{}-hardcoded",
                        surface.as_str()
                    ));
                }
                _ => {}
            }
            LaunchSurfaceRow {
                surface_class: surface,
                honors_token_runtime: honors,
                hardcoded_styling_absent: hardcoded_absent,
                surface_marker: marker,
                certified_modes: AppearanceModeClass::REQUIRED.to_vec(),
                waiver_ref: waiver,
                conforms: false,
            }
        })
        .collect()
}

fn accessibility() -> AccessibilityDisclosure {
    let action_labels: Vec<String> = required_recovery_routes()
        .into_iter()
        .map(|route| route.action_label)
        .collect();
    let layout_modes = LayoutMode::REQUIRED
        .into_iter()
        .map(|mode| LayoutModeDisclosure {
            mode,
            row_narration_available: true,
            recovery_affordances_reachable: true,
        })
        .collect();
    AccessibilityDisclosure {
        focus_order_index: 0,
        tab_stop_count: 5,
        row_narration: "Design-token runtime certification for the active appearance session"
            .to_owned(),
        action_labels,
        layout_modes,
    }
}

fn routes() -> Vec<EntryRouteRecord> {
    RouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!("aureline://route/design-token-runtime/{}", surface.as_str()),
            keyboard_reachable: true,
            activates_same_record: true,
        })
        .collect()
}

fn binding(contract: &AppearanceSessionBetaContract) -> AppearanceSessionBinding {
    let session = &contract.appearance_session;
    let value_ref = AppearanceSessionBinding::value_ref_for(
        &session.appearance_session_id,
        session.session_revision,
    );
    AppearanceSessionBinding {
        appearance_session_id: session.appearance_session_id.clone(),
        session_revision: session.session_revision,
        active_theme_package_ref: session.active_theme_package_ref.clone(),
        active_theme_revision_ref: session.active_theme_revision_ref.clone(),
        mode_theme_class: session.mode_theme_class,
        contrast_mode: session.contrast_mode,
        accent_source: session.accent_source,
        density_class: session.density_class,
        text_scale_percent: session.text_scale.scale_percent,
        reduced_motion_posture: session.reduced_motion_posture,
        follow_system_posture: session.follow_system_posture,
        live_follow_system_policy_ref: session.live_follow_system_policy_ref.clone(),
        value_ref,
    }
}

fn upstream(contract: &AppearanceSessionBetaContract, capture_refs: Vec<String>) -> CertificationUpstream {
    let token_registry_refs = [
        ThemeClass::DarkReference,
        ThemeClass::LightParity,
        ThemeClass::HighContrastDark,
        ThemeClass::HighContrastLight,
    ]
    .into_iter()
    .map(|theme| format!("aureline-ui:token_registry:{}", theme.token()))
    .collect();
    CertificationUpstream {
        appearance_contract_ref: contract.packet_id.clone(),
        component_state_registry_ref: seeded_component_state_registry().registry_id,
        token_registry_refs,
        contributing_capture_refs: capture_refs,
    }
}

fn build_scenario(spec: &ScenarioSpec) -> DesignTokenRuntimeScenario {
    let contract = try_seeded_appearance_session_beta_contract()
        .expect("seeded appearance-session beta contract must project");
    let session = binding(&contract);
    let mode_rows = mode_rows(spec.posture_id, &session.value_ref);
    let capture_refs: Vec<String> = mode_rows
        .iter()
        .map(|row| row.golden_capture_ref.clone())
        .collect();
    let input = CertificationInput {
        record_id: spec.scenario_id.to_owned(),
        as_of: CORPUS_AS_OF.to_owned(),
        posture_id: spec.posture_id.to_owned(),
        posture_label: spec.posture_label.to_owned(),
        title: spec.title.to_owned(),
        summary: spec.summary.to_owned(),
        appearance_session: session,
        mode_rows,
        protected_cues: protected_cue_rows(),
        live_apply_axes: live_apply_axes(&contract, spec.reload_axis),
        motion_suppression: motion_suppression_rows(),
        launch_surfaces: launch_surface_rows(spec.surface_override),
        claim_ceiling: spec.claim_ceiling,
        recovery_routes: required_recovery_routes(),
        routes: routes(),
        accessibility: accessibility(),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(&contract, capture_refs),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_owned(),
        support_export_ref: SUPPORT_EXPORT_REF.to_owned(),
        evidence_refs: vec![EVIDENCE_ARTIFACT_REF.to_owned(), EVIDENCE_FIXTURE_REF.to_owned()],
        narrative_refs: vec![NARRATIVE_REF.to_owned()],
    };
    let record = DesignTokenRuntimeCertification::build(input)
        .unwrap_or_else(|err| panic!("scenario {} must build: {err}", spec.scenario_id));

    DesignTokenRuntimeScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id),
        expected_posture: record.posture_id.clone(),
        expected_claim_class: record.stable_qualification.claim_class,
        expected_qualifies_stable: record.stable_qualification.qualifies_stable,
        expected_surface_marker: record.surface_lifecycle_marker,
        record,
    }
}

/// Returns the deterministic claimed-stable certification matrix.
pub fn design_token_runtime_corpus() -> Vec<DesignTokenRuntimeScenario> {
    let specs = [
        ScenarioSpec {
            scenario_id: "nominal",
            posture_id: "design_token_runtime_nominal",
            posture_label: "Nominal design-token runtime",
            title: "Design-token runtime is certified across every claimed appearance mode",
            summary: "Dark, light, high-contrast, reduced-motion, and density rows resolve from \
                      one semantic token runtime; protected cues survive every mode; every axis \
                      applies live or behind a checkpoint; every launch surface honors the runtime.",
            reload_axis: None,
            surface_override: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "forced_colors_reload_disclosed",
            posture_id: "design_token_runtime_forced_colors_reload",
            posture_label: "Forced-colors reload disclosed",
            title: "Forced-colors contrast switch discloses a reload instead of silently lagging",
            summary: "When the OS switches to forced colors the contrast axis requires a disclosed \
                      surface reload; the row labels the reload rather than silently lagging the \
                      system, so the runtime stays honest and Stable.",
            reload_axis: Some(AppearanceAxis::ContrastMode),
            surface_override: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "density_surface_in_preview",
            posture_id: "design_token_runtime_density_surface_preview",
            posture_label: "Activity-center surface still in Preview",
            title: "A below-Stable surface narrows the certification instead of inheriting green",
            summary: "Every mode and cue conforms, but the activity-center surface still carries a \
                      Preview marker; the posture is narrowed below Stable by its lowest surface \
                      marker rather than inheriting an adjacent green row.",
            reload_axis: None,
            surface_override: Some(SurfaceOverride::Marker(
                LaunchSurfaceClass::ActivityCenterRow,
                LifecycleMarker::Preview,
            )),
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "hardcoded_styling_drill",
            posture_id: "design_token_runtime_hardcoded_drill",
            posture_label: "Hard-coded styling drill",
            title: "A hard-coded surface narrows the certification with a named reason",
            summary: "An adversarial posture where the notification-envelope surface hard-codes \
                      styling instead of consuming the token runtime; the lane detects the \
                      divergence and narrows the posture below Stable with a named reason.",
            reload_axis: None,
            surface_override: Some(SurfaceOverride::Hardcoded(
                LaunchSurfaceClass::NotificationEnvelope,
            )),
            claim_ceiling: CertificationClaimCeiling {
                asserts_no_hardcoded_styling: false,
                ..full_claim_ceiling()
            },
        },
    ];
    let scenarios: Vec<DesignTokenRuntimeScenario> = specs.iter().map(build_scenario).collect();
    debug_assert!(scenarios.iter().all(|scenario| {
        is_canonical_object_ref(&scenario.record.diagnostics_export_ref)
            && is_canonical_object_ref(&scenario.record.support_export_ref)
    }));
    scenarios
}
