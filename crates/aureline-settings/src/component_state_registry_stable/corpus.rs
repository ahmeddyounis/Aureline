//! Deterministic claimed-stable matrix for component-state registry
//! certifications.
//!
//! Every record here is a genuine projection of **live** upstream contracts. The
//! corpus reads
//! [`aureline_design_system::seeded_component_state_registry`] for the hardened
//! registry's source-of-truth refs and id, the live extension
//! [`aureline_extensions::appearance_conformance::seeded_appearance_conformance_packet`]
//! for per-axis inheritance support, and the live
//! [`aureline_design_system::seeded_screenshot_diff_packet`] for the
//! launch-critical surface/state fixture matrix. The component-state vocabulary
//! binds to [`aureline_ui::components::ComponentStateClass`]. So a certification
//! record can never drift from the contracts the runtime actually publishes.
//!
//! Four postures pin the matrix:
//!
//! - `nominal` — every family covers the vocabulary and is token-driven; every
//!   normalized state is consistent across shell, review, settings, and support;
//!   every extension axis inherits fully or discloses its gap everywhere; every
//!   shell zone is token-driven; every launch-critical permutation has a
//!   conforming fixture. Qualifies **Stable**.
//! - `popover_family_in_preview` — every pillar holds, but the popover family
//!   still carries a Preview marker, so the posture narrows below Stable by its
//!   lowest family marker instead of inheriting an adjacent green row.
//! - `hardcoded_zoning_drill` — an adversarial posture where the editor-group
//!   zone hard-codes a chrome metric; the lane detects it and narrows the posture
//!   below Stable with a named reason.
//! - `extension_gap_undisclosed_drill` — an adversarial posture where one
//!   extension axis gap is not surfaced in the support export; the lane refuses
//!   the disclosure claim and narrows below Stable with a named reason.

use std::collections::BTreeMap;

use aureline_design_system::{
    seeded_component_state_registry, seeded_screenshot_diff_packet, LaunchSurfaceClass,
};
use aureline_extensions::appearance_conformance::{
    seeded_appearance_conformance_packet, AppearanceAxisClass, AppearanceSupportClass,
    APPEARANCE_AXES,
};
use aureline_ui::density::DensityClass;
use aureline_ui::themes::AccessibilityPostureClass;

use super::model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure,
    CertificationClaimCeiling, CertificationInput, CertificationUpstream,
    ComponentFamilyClass, ComponentFamilyRow, ComponentStateName,
    ComponentStateRegistryCertification, EntryRouteRecord, ExtensionInheritanceRow, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, NonColorCueClass, NormalizedStateRow,
    RegistryBinding, RegistrySurfaceClass, RequiredAffordanceClass, RouteSurface, ShellZoneClass,
    ShellZoneRow, StableClaimClass, StateFixtureRow, ZoneLayoutMode,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const REGISTRY_ID: &str = "component-state-registry:stable";
const REGISTRY_REVISION: u64 = 1;
const TAXONOMY_REF: &str = "aureline-ui:component_state_registry";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/component-state-registry";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/component-state-registry";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-component-state-registry";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-component-state-registry";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-component-state-registry";

/// One scenario in the claimed-stable certification matrix.
#[derive(Debug, Clone)]
pub struct ComponentStateRegistryScenario {
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
    /// Expected derived surface lifecycle marker (lowest family).
    pub expected_surface_marker: LifecycleMarker,
    record: ComponentStateRegistryCertification,
}

impl ComponentStateRegistryScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> ComponentStateRegistryCertification {
        self.record.clone()
    }
}

/// How one component family is overridden for a scenario.
#[derive(Debug, Clone, Copy)]
enum FamilyOverride {
    /// The family still carries a below-Stable marker.
    Marker(ComponentFamilyClass, LifecycleMarker),
}

/// How one shell zone is overridden for a scenario.
#[derive(Debug, Clone, Copy)]
enum ZoneOverride {
    /// The zone hard-codes a chrome metric (token-runtime divergence).
    Hardcoded(ShellZoneClass),
}

/// How one extension axis is overridden for a scenario.
#[derive(Debug, Clone, Copy)]
enum ExtensionOverride {
    /// The axis gap is not surfaced in the support export.
    UndisclosedInSupport(AppearanceAxisClass),
}

/// One whole-posture spec.
struct ScenarioSpec {
    scenario_id: &'static str,
    posture_id: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    family_override: Option<FamilyOverride>,
    zone_override: Option<ZoneOverride>,
    extension_override: Option<ExtensionOverride>,
    claim_ceiling: CertificationClaimCeiling,
}

fn full_claim_ceiling() -> CertificationClaimCeiling {
    CertificationClaimCeiling {
        asserts_registry_family_coverage: true,
        asserts_state_normalization: true,
        asserts_extension_gaps_disclosed: true,
        asserts_shell_zoning_token_driven: true,
        asserts_state_fixture_coverage: true,
        asserts_focus_and_screen_reader: true,
        asserts_no_hue_or_animation_only: true,
    }
}

/// The per-family supported state vocabulary. The union covers every required
/// canonical state without forcing every family to carry every state.
fn family_states(family: ComponentFamilyClass) -> Vec<ComponentStateName> {
    use ComponentStateName as S;
    let states: &[ComponentStateName] = match family {
        ComponentFamilyClass::CoreControl => &[
            S::Hover,
            S::Focus,
            S::Selected,
            S::Disabled,
            S::Loading,
            S::Warning,
            S::Error,
            S::Blocked,
            S::PolicyLocked,
        ],
        ComponentFamilyClass::DenseRow => &[
            S::Hover,
            S::Focus,
            S::Selected,
            S::Disabled,
            S::Stale,
            S::Partial,
            S::Degraded,
            S::Warning,
        ],
        ComponentFamilyClass::Tab => &[S::Hover, S::Focus, S::Selected, S::Disabled, S::Warning],
        ComponentFamilyClass::Tree => &[
            S::Hover,
            S::Focus,
            S::Selected,
            S::Disabled,
            S::Loading,
            S::Stale,
            S::Partial,
        ],
        ComponentFamilyClass::Palette => {
            &[S::Hover, S::Focus, S::Selected, S::Loading, S::Warming, S::Error]
        }
        ComponentFamilyClass::Popover => &[S::Hover, S::Focus, S::Disabled, S::Warning, S::Error],
        ComponentFamilyClass::Dialog => &[
            S::Focus,
            S::Disabled,
            S::Loading,
            S::Warning,
            S::Error,
            S::Blocked,
            S::PolicyLocked,
        ],
        ComponentFamilyClass::Banner => {
            &[S::Warning, S::Error, S::Blocked, S::Degraded, S::Partial]
        }
        ComponentFamilyClass::JobRow => &[
            S::Loading,
            S::Reconnecting,
            S::Warming,
            S::Partial,
            S::Stale,
            S::Recovering,
            S::Degraded,
            S::Error,
        ],
        ComponentFamilyClass::InlineNotice => {
            &[S::Warning, S::Error, S::Blocked, S::PolicyLocked, S::Degraded]
        }
    };
    states.to_vec()
}

fn family_label(family: ComponentFamilyClass) -> &'static str {
    match family {
        ComponentFamilyClass::CoreControl => "Core controls",
        ComponentFamilyClass::DenseRow => "Dense rows",
        ComponentFamilyClass::Tab => "Tabs",
        ComponentFamilyClass::Tree => "Trees",
        ComponentFamilyClass::Palette => "Palettes",
        ComponentFamilyClass::Popover => "Popovers",
        ComponentFamilyClass::Dialog => "Dialogs",
        ComponentFamilyClass::Banner => "Banners",
        ComponentFamilyClass::JobRow => "Job rows",
        ComponentFamilyClass::InlineNotice => "Inline notices",
    }
}

fn family_cues(family: ComponentFamilyClass) -> Vec<NonColorCueClass> {
    use NonColorCueClass as Cue;
    match family {
        ComponentFamilyClass::CoreControl | ComponentFamilyClass::DenseRow => {
            vec![Cue::LabelText, Cue::Icon, Cue::Border, Cue::FocusRing]
        }
        ComponentFamilyClass::Tab | ComponentFamilyClass::Tree => {
            vec![Cue::LabelText, Cue::Icon, Cue::FocusRing, Cue::SelectionMarker]
        }
        ComponentFamilyClass::Palette => {
            vec![Cue::LabelText, Cue::Icon, Cue::FocusRing, Cue::ProgressIndicator]
        }
        ComponentFamilyClass::Popover | ComponentFamilyClass::Dialog => {
            vec![Cue::LabelText, Cue::Icon, Cue::Border, Cue::LockOrShieldGlyph]
        }
        ComponentFamilyClass::Banner | ComponentFamilyClass::InlineNotice => {
            vec![Cue::LabelText, Cue::Icon, Cue::Border, Cue::Shape]
        }
        ComponentFamilyClass::JobRow => {
            vec![Cue::LabelText, Cue::Icon, Cue::ProgressIndicator, Cue::Border]
        }
    }
}

fn component_family_rows(family_override: Option<FamilyOverride>) -> Vec<ComponentFamilyRow> {
    ComponentFamilyClass::REQUIRED
        .into_iter()
        .map(|family| {
            let mut marker = LifecycleMarker::Stable;
            let mut waiver = None;
            if let Some(FamilyOverride::Marker(target, target_marker)) = family_override {
                if target == family {
                    marker = target_marker;
                    waiver = Some(format!("aureline://waiver/csr-{}-marker", family.as_str()));
                }
            }
            ComponentFamilyRow {
                family_class: family,
                display_label: family_label(family).to_owned(),
                supported_states: family_states(family),
                required_affordances: RequiredAffordanceClass::REQUIRED.to_vec(),
                non_color_cues: family_cues(family),
                accessibility_note: format!(
                    "{} render every state from the shared registry with a visible focus ring and a \
                     screen-reader label that names the state.",
                    family_label(family)
                ),
                token_driven: true,
                hardcoded_styling_absent: true,
                hue_only_forbidden: true,
                animation_only_forbidden: true,
                focus_visible_preserved: true,
                screen_reader_semantics_preserved: true,
                surface_marker: marker,
                waiver_ref: waiver,
                conforms: false,
            }
        })
        .collect()
}

fn normalized_state_rows() -> Vec<NormalizedStateRow> {
    use ComponentStateName as S;
    use NonColorCueClass as Cue;

    let spec: [(S, &str, &[Cue], &str, &str); 8] = [
        (
            S::Disabled,
            "Disabled",
            &[Cue::LabelText, Cue::Icon, Cue::Border],
            "The control is unavailable because the current context does not permit it.",
            "Name what would make the control available again.",
        ),
        (
            S::Blocked,
            "Blocked",
            &[Cue::LabelText, Cue::Icon, Cue::LockOrShieldGlyph],
            "The action is blocked by trust, permission, ownership, source, or capability.",
            "Open the least-surprising recovery or inspect route for the block source.",
        ),
        (
            S::PolicyLocked,
            "Policy locked",
            &[Cue::LabelText, Cue::Icon, Cue::LockOrShieldGlyph],
            "An administrator policy locks this action in the current workspace.",
            "Show the managing policy and the request-exception route.",
        ),
        (
            S::Reconnecting,
            "Reconnecting",
            &[Cue::LabelText, Cue::Icon, Cue::ProgressIndicator],
            "The live connection dropped and the surface is reconnecting.",
            "Offer retry now and a route to the connection diagnostics.",
        ),
        (
            S::Warming,
            "Warming",
            &[Cue::LabelText, Cue::Icon, Cue::ProgressIndicator],
            "The surface is preparing a warm restore before content is ready.",
            "Show progress and a route to cancel or inspect the warm restore.",
        ),
        (
            S::Partial,
            "Partial",
            &[Cue::LabelText, Cue::Icon, Cue::Border],
            "Only part of the content is available; the surface names what still works.",
            "Name the reduced scope and the route to the full result.",
        ),
        (
            S::Stale,
            "Stale",
            &[Cue::LabelText, Cue::Icon, Cue::Border],
            "The surface shows last-known-good content while a refresh lags.",
            "Show the freshness time and a refresh route.",
        ),
        (
            S::Recovering,
            "Recovering",
            &[Cue::LabelText, Cue::Icon, Cue::ProgressIndicator],
            "The surface is recovering after a failure and rebuilding its state.",
            "Show progress and a route to the recovery details.",
        ),
    ];

    spec.into_iter()
        .map(|(state, label, cues, reason, action)| NormalizedStateRow {
            state_name: state,
            taxonomy_ref: state.taxonomy(),
            display_label: label.to_owned(),
            non_color_cues: cues.to_vec(),
            hue_only_forbidden: true,
            animation_only_forbidden: true,
            consistent_across_surfaces: RegistrySurfaceClass::REQUIRED.to_vec(),
            narratable_reason: reason.to_owned(),
            action_path: action.to_owned(),
            screen_reader_label: format!("{label} state"),
            conforms: false,
        })
        .collect()
}

fn support_rank(support: AppearanceSupportClass) -> u8 {
    match support {
        AppearanceSupportClass::FullInheritance => 0,
        AppearanceSupportClass::ReducedSupport => 1,
        AppearanceSupportClass::UnsupportedPrivateStyling => 2,
        AppearanceSupportClass::UndisclosedGap => 3,
    }
}

/// Projects the worst per-axis support class observed across the live extension
/// appearance-conformance packet, so the certification mirrors what extensions
/// actually declare and the host actually probes.
fn worst_support_by_axis() -> BTreeMap<AppearanceAxisClass, AppearanceSupportClass> {
    let packet = seeded_appearance_conformance_packet();
    let mut map: BTreeMap<AppearanceAxisClass, AppearanceSupportClass> = BTreeMap::new();
    for row in &packet.rows {
        for axis in &row.axes {
            let entry = map
                .entry(axis.axis)
                .or_insert(AppearanceSupportClass::FullInheritance);
            if support_rank(axis.support_class) > support_rank(*entry) {
                *entry = axis.support_class;
            }
        }
    }
    map
}

fn extension_inheritance_rows(
    extension_override: Option<ExtensionOverride>,
) -> Vec<ExtensionInheritanceRow> {
    let worst = worst_support_by_axis();
    APPEARANCE_AXES
        .into_iter()
        .map(|axis| {
            let support = worst
                .get(&axis)
                .copied()
                .unwrap_or(AppearanceSupportClass::FullInheritance);
            let mut surfaced_in_support = true;
            let mut waiver = None;
            if let Some(ExtensionOverride::UndisclosedInSupport(target)) = extension_override {
                if target == axis {
                    surfaced_in_support = false;
                    waiver = Some(format!(
                        "aureline://waiver/csr-{}-undisclosed",
                        axis.as_str()
                    ));
                }
            }
            ExtensionInheritanceRow {
                axis,
                support_class: support,
                gap_disclosed_in_review: true,
                gap_surfaced_in_diagnostics: true,
                gap_surfaced_in_support_export: surfaced_in_support,
                caveat: format!(
                    "{} appearance: {}.",
                    axis.label(),
                    support.label()
                ),
                waiver_ref: waiver,
                conforms: false,
            }
        })
        .collect()
}

fn zone_label(zone: ShellZoneClass) -> &'static str {
    match zone {
        ShellZoneClass::NavigationRail => "shell.zone.navigation_rail",
        ShellZoneClass::PrimarySidebar => "shell.zone.primary_sidebar",
        ShellZoneClass::EditorGroup => "shell.zone.editor_group",
        ShellZoneClass::SecondaryPanel => "shell.zone.secondary_panel",
        ShellZoneClass::StatusStrip => "shell.zone.status_strip",
        ShellZoneClass::CompanionSheet => "shell.zone.companion_sheet",
    }
}

fn zone_metrics(zone: ShellZoneClass) -> (ZoneLayoutMode, u32, u32) {
    match zone {
        ShellZoneClass::NavigationRail => (ZoneLayoutMode::Docked, 48, 72),
        ShellZoneClass::PrimarySidebar => (ZoneLayoutMode::Sheet, 240, 420),
        ShellZoneClass::EditorGroup => (ZoneLayoutMode::Docked, 480, 4096),
        ShellZoneClass::SecondaryPanel => (ZoneLayoutMode::Sheet, 220, 520),
        ShellZoneClass::StatusStrip => (ZoneLayoutMode::Docked, 24, 36),
        ShellZoneClass::CompanionSheet => (ZoneLayoutMode::Sheet, 320, 640),
    }
}

fn shell_zone_rows(zone_override: Option<ZoneOverride>) -> Vec<ShellZoneRow> {
    ShellZoneClass::REQUIRED
        .into_iter()
        .map(|zone| {
            let (layout_mode, min_px, max_px) = zone_metrics(zone);
            let mut metrics_token_driven = true;
            let mut waiver = None;
            if let Some(ZoneOverride::Hardcoded(target)) = zone_override {
                if target == zone {
                    metrics_token_driven = false;
                    waiver = Some(format!("aureline://waiver/csr-{}-hardcoded", zone.as_str()));
                }
            }
            ShellZoneRow {
                zone_class: zone,
                slot_name: zone_label(zone).to_owned(),
                layout_mode,
                min_chrome_px: min_px,
                max_chrome_px: max_px,
                density_class: DensityClass::Standard,
                reduced_motion_posture: AccessibilityPostureClass::MotionReduced,
                metrics_token_driven,
                placeholder_token_driven: true,
                responsive_fallback_token_driven: true,
                waiver_ref: waiver,
                conforms: false,
            }
        })
        .collect()
}

/// Projects the launch-critical surface/state fixture matrix from the live
/// design-system screenshot-diff packet.
fn state_fixture_rows() -> Vec<StateFixtureRow> {
    let packet = seeded_screenshot_diff_packet();
    packet
        .rows
        .iter()
        .map(|row| {
            let slug = format!("{}-{}", row.surface_class.as_str(), row.state_class.as_str());
            StateFixtureRow {
                surface_class: row.surface_class,
                state_class: row.state_class,
                screenshot_ref: format!("aureline://artifact/csr-{slug}-screenshot"),
                fixture_ref: format!("aureline://fixture/csr-{slug}"),
                focus_visible_preserved: row.focus_visibility_present,
                screen_reader_semantics_preserved: true,
                transition_narrated: true,
                non_color_cue_present: !row.required_non_color_cues.is_empty(),
                hover_only_critical_action_absent: row.hover_only_critical_actions_absent,
                conforms: false,
            }
        })
        .collect()
}

fn binding() -> RegistryBinding {
    let upstream = seeded_component_state_registry();
    RegistryBinding {
        registry_id: REGISTRY_ID.to_owned(),
        registry_revision: REGISTRY_REVISION,
        value_ref: RegistryBinding::value_ref_for(REGISTRY_ID, REGISTRY_REVISION),
        taxonomy_ref: TAXONOMY_REF.to_owned(),
        source_refs: upstream.source_refs,
    }
}

fn upstream(contributing: Vec<String>) -> CertificationUpstream {
    CertificationUpstream {
        component_state_registry_ref: seeded_component_state_registry().registry_id,
        appearance_conformance_packet_ref: seeded_appearance_conformance_packet().packet_id,
        screenshot_diff_packet_ref: seeded_screenshot_diff_packet().packet_id,
        taxonomy_ref: TAXONOMY_REF.to_owned(),
        contributing_fixture_refs: contributing,
    }
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
        row_narration: "Component-state registry certification for the shared component vocabulary"
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
            route_ref: format!(
                "aureline://route/component-state-registry/{}",
                surface.as_str()
            ),
            keyboard_reachable: true,
            activates_same_record: true,
        })
        .collect()
}

fn build_scenario(spec: &ScenarioSpec) -> ComponentStateRegistryScenario {
    let state_fixtures = state_fixture_rows();
    let contributing: Vec<String> = state_fixtures
        .iter()
        .map(|row| row.fixture_ref.clone())
        .collect();
    let input = CertificationInput {
        record_id: spec.scenario_id.to_owned(),
        as_of: CORPUS_AS_OF.to_owned(),
        posture_id: spec.posture_id.to_owned(),
        posture_label: spec.posture_label.to_owned(),
        title: spec.title.to_owned(),
        summary: spec.summary.to_owned(),
        registry_binding: binding(),
        component_families: component_family_rows(spec.family_override),
        normalized_states: normalized_state_rows(),
        extension_inheritance: extension_inheritance_rows(spec.extension_override),
        shell_zones: shell_zone_rows(spec.zone_override),
        state_fixtures,
        claim_ceiling: spec.claim_ceiling,
        recovery_routes: required_recovery_routes(),
        routes: routes(),
        accessibility: accessibility(),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(contributing),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_owned(),
        support_export_ref: SUPPORT_EXPORT_REF.to_owned(),
        evidence_refs: vec![EVIDENCE_ARTIFACT_REF.to_owned(), EVIDENCE_FIXTURE_REF.to_owned()],
        narrative_refs: vec![NARRATIVE_REF.to_owned()],
    };
    let record = ComponentStateRegistryCertification::build(input)
        .unwrap_or_else(|err| panic!("scenario {} must build: {err}", spec.scenario_id));

    ComponentStateRegistryScenario {
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
pub fn component_state_registry_corpus() -> Vec<ComponentStateRegistryScenario> {
    let specs = [
        ScenarioSpec {
            scenario_id: "nominal",
            posture_id: "component_state_registry_nominal",
            posture_label: "Nominal component-state registry",
            title: "Component-state registry is certified across families, states, and surfaces",
            summary: "Every family covers the shared vocabulary and is token-driven; every \
                      normalized state is consistent across shell, review, settings, and support; \
                      every extension axis inherits fully or discloses its gap everywhere; every \
                      shell zone is token-driven; every launch-critical permutation has a \
                      conforming fixture.",
            family_override: None,
            zone_override: None,
            extension_override: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "popover_family_in_preview",
            posture_id: "component_state_registry_popover_preview",
            posture_label: "Popover family still in Preview",
            title: "A below-Stable family narrows the certification instead of inheriting green",
            summary: "Every pillar holds, but the popover family still carries a Preview marker; the \
                      posture is narrowed below Stable by its lowest family marker rather than \
                      inheriting an adjacent green row.",
            family_override: Some(FamilyOverride::Marker(
                ComponentFamilyClass::Popover,
                LifecycleMarker::Preview,
            )),
            zone_override: None,
            extension_override: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "hardcoded_zoning_drill",
            posture_id: "component_state_registry_hardcoded_zoning",
            posture_label: "Hard-coded shell-zoning drill",
            title: "A hard-coded shell zone narrows the certification with a named reason",
            summary: "An adversarial posture where the editor-group zone hard-codes a chrome metric \
                      instead of reading it from the token runtime; the lane detects the divergence \
                      and narrows the posture below Stable with a named reason.",
            family_override: None,
            zone_override: Some(ZoneOverride::Hardcoded(ShellZoneClass::EditorGroup)),
            extension_override: None,
            claim_ceiling: CertificationClaimCeiling {
                asserts_shell_zoning_token_driven: false,
                ..full_claim_ceiling()
            },
        },
        ScenarioSpec {
            scenario_id: "extension_gap_undisclosed_drill",
            posture_id: "component_state_registry_extension_gap_undisclosed",
            posture_label: "Extension gap undisclosed drill",
            title: "An undisclosed extension gap narrows the certification with a named reason",
            summary: "An adversarial posture where one extension appearance axis gap is not surfaced \
                      in the support export; the lane refuses the disclosure claim and narrows the \
                      posture below Stable with a named reason.",
            family_override: None,
            zone_override: None,
            extension_override: Some(ExtensionOverride::UndisclosedInSupport(
                AppearanceAxisClass::HostToken,
            )),
            claim_ceiling: CertificationClaimCeiling {
                asserts_extension_gaps_disclosed: false,
                ..full_claim_ceiling()
            },
        },
    ];
    let scenarios: Vec<ComponentStateRegistryScenario> = specs.iter().map(build_scenario).collect();
    debug_assert!(scenarios.iter().all(|scenario| {
        is_canonical_object_ref(&scenario.record.diagnostics_export_ref)
            && is_canonical_object_ref(&scenario.record.support_export_ref)
    }));
    debug_assert!(LaunchSurfaceClass::required().len() == 10);
    scenarios
}
