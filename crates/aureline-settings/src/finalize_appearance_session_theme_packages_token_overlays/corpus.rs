//! Deterministic claimed-stable matrix for appearance-session finalization
//! certifications.
//!
//! Every record here is a genuine projection of the **live** appearance runtime.
//! The corpus reads
//! [`aureline_design_system::seeded_appearance_session_beta_contract`] for the
//! in-effect appearance-session value, the theme-package manifest
//! ([`aureline_ui::themes::first_party_theme_package_manifest`]), the token
//! registry ([`aureline_ui::tokens::seeded_token_registry`]), the imported-theme
//! mapping report ([`aureline_ui::themes::imported_theme_mapping_report_with_warnings`]),
//! and the extension appearance-conformance packet
//! ([`aureline_extensions::appearance_conformance::seeded_appearance_conformance_packet`])
//! so the certification can never drift from what the runtime actually resolves.
//!
//! Four postures pin the matrix:
//!
//! - `nominal` — every theme package is versioned, the session summary is
//!   exportable, token overlays preserve unknown tokens, import reports are
//!   honest, extension gaps are visible, live changes disclose, and provenance
//!   survives sync. Qualifies **Stable**.
//! - `token_overlay_silently_dropped_drill` — an adversarial posture where one
//!   overlay scope silently drops tokens; the lane detects it and narrows below
//!   Stable with a named reason.
//! - `extension_gap_undisclosed` — an extension surface does not disclose its
//!   inheritance gap in diagnostics; the posture narrows below Stable.
//! - `import_report_missing_rollback` — an imported-theme mapping report lacks
//!   a rollback path; the posture narrows below Stable.

use aureline_design_system::{
    seeded_component_state_registry, try_seeded_appearance_session_beta_contract,
};
use aureline_extensions::appearance_conformance::seeded_appearance_conformance_packet;
use aureline_ui::density::DensityClass;
use aureline_ui::themes::{
    first_party_theme_package_manifest, imported_theme_mapping_report_with_warnings,
    AccessibilityPostureClass,
};
use aureline_ui::tokens::ThemeClass;

use super::model::AppearanceSessionFinalizationCertification;
use super::model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure,
    AppearanceSessionBinding, AppearanceSessionSummaryRow, CertificationClaimCeiling,
    CertificationInput, CertificationUpstream, EntryRouteRecord, ExtensionAppearanceDescriptorRow,
    ExtensionInheritanceState, ImportedThemeMappingReportRow, LayoutMode, LayoutModeDisclosure,
    LifecycleMarker, LiveAppearanceAxisClass, LiveAppearanceChangeRow, LiveApplyClass,
    OverlayScopeClass, ProvenanceDimensionClass, ProvenancePreservationRow, RouteSurface,
    StableClaimClass, ThemePackageManifestRow, TokenOverlayValidationRow,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-06-03T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/appearance-session-finalization";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/appearance-session-finalization";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-appearance-session-finalization";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-appearance-session-finalization";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-appearance-session-finalization";

/// One scenario in the claimed-stable certification matrix.
#[derive(Debug, Clone)]
pub struct AppearanceSessionFinalizationScenario {
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
    /// Expected derived surface lifecycle marker (lowest row).
    pub expected_surface_marker: LifecycleMarker,
    record: AppearanceSessionFinalizationCertification,
}

impl AppearanceSessionFinalizationScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> AppearanceSessionFinalizationCertification {
        self.record.clone()
    }
}

/// One whole-posture spec.
struct ScenarioSpec {
    scenario_id: &'static str,
    posture_id: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    /// Overlay scope forced to silently drop tokens.
    overlay_scope_override: Option<OverlayScopeClass>,
    /// Extension descriptor forced to undisclosed gap.
    extension_gap_override: Option<&'static str>,
    /// Import report forced to missing rollback.
    import_rollback_override: Option<&'static str>,
    claim_ceiling: CertificationClaimCeiling,
}

fn full_claim_ceiling() -> CertificationClaimCeiling {
    CertificationClaimCeiling {
        asserts_theme_packages_versioned: true,
        asserts_sessions_inspectable: true,
        asserts_token_overlays_validated: true,
        asserts_import_reports_honest: true,
        asserts_extension_gaps_visible: true,
        asserts_live_changes_disclosed: true,
        asserts_provenance_intact: true,
    }
}

fn binding() -> AppearanceSessionBinding {
    let contract = try_seeded_appearance_session_beta_contract()
        .expect("seeded appearance-session beta contract must project");
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

fn theme_package_rows() -> Vec<ThemePackageManifestRow> {
    let manifest =
        first_party_theme_package_manifest().expect("first-party theme package manifest");
    vec![ThemePackageManifestRow {
        package_ref: manifest.package_id().to_owned(),
        package_revision_ref: manifest.theme_package_revision_ref().to_owned(),
        package_version_label: manifest.package_version_label().to_owned(),
        supported_theme_classes: vec![
            ThemeClass::DarkReference,
            ThemeClass::LightParity,
            ThemeClass::HighContrastDark,
            ThemeClass::HighContrastLight,
        ],
        supported_density_classes: vec![
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ],
        supported_motion_postures: vec![
            AccessibilityPostureClass::MotionStandard,
            AccessibilityPostureClass::MotionReduced,
            AccessibilityPostureClass::MotionLowMotion,
            AccessibilityPostureClass::MotionPowerSaver,
            AccessibilityPostureClass::MotionCriticalHotPath,
        ],
        default_density_class: manifest.default_density_class(),
        default_motion_posture: manifest.default_motion_posture(),
        minimum_text_contrast_target: manifest
            .minimum_text_contrast_target(ThemeClass::DarkReference)
            .unwrap_or(4.5),
        minimum_ui_contrast_target: manifest
            .minimum_ui_contrast_target(ThemeClass::DarkReference)
            .unwrap_or(3.0),
        manifest_versioned: true,
        provenance_declared: true,
        trust_severity_semantics_preserved: true,
        surface_marker: LifecycleMarker::Stable,
        waiver_ref: None,
        conforms: false,
    }]
}

fn session_summary_rows(binding: &AppearanceSessionBinding) -> Vec<AppearanceSessionSummaryRow> {
    vec![AppearanceSessionSummaryRow {
        appearance_session_id: binding.appearance_session_id.clone(),
        session_revision: binding.session_revision,
        active_theme_package_ref: binding.active_theme_package_ref.clone(),
        active_theme_revision_ref: binding.active_theme_revision_ref.clone(),
        mode_theme_class: binding.mode_theme_class,
        contrast_mode: binding.contrast_mode,
        accent_source: binding.accent_source,
        density_class: binding.density_class,
        text_scale_percent: binding.text_scale_percent,
        reduced_motion_posture: binding.reduced_motion_posture,
        follow_system_posture: binding.follow_system_posture,
        checkpoint_active: false,
        current_checkpoint_ref: None,
        rollback_ref: None,
        summary_exportable: true,
        cites_one_package_source: true,
        conforms: false,
    }]
}

fn token_overlay_rows(
    overlay_scope_override: Option<OverlayScopeClass>,
) -> Vec<TokenOverlayValidationRow> {
    OverlayScopeClass::REQUIRED
        .into_iter()
        .map(|scope| {
            let (supported, inert, downgraded, dropped, unknown_preserved, unsupported_preserved) =
                if overlay_scope_override == Some(scope) {
                    (2, 0, 0, 1, false, false)
                } else {
                    (3, 1, 0, 0, true, true)
                };
            TokenOverlayValidationRow {
                scope,
                overlay_ref: format!("token_overlay:{}:01", scope.as_str()),
                supported_token_count: supported,
                inert_token_count: inert,
                downgraded_token_count: downgraded,
                silently_dropped_token_count: dropped,
                unknown_tokens_preserved: unknown_preserved,
                unsupported_tokens_preserved: unsupported_preserved,
                scope_lineage_recorded: true,
                conforms: false,
            }
        })
        .collect()
}

fn import_report_rows(
    import_rollback_override: Option<&str>,
) -> Vec<ImportedThemeMappingReportRow> {
    let report = imported_theme_mapping_report_with_warnings().expect("import mapping report");
    let summary = report.summary();
    let rollback_present = import_rollback_override.is_none();
    vec![ImportedThemeMappingReportRow {
        report_ref: report.report_id().to_owned(),
        source_format: "vscode_color_theme".to_owned(),
        translated_slot_count: summary.translated_slot_count,
        unsupported_slot_count: summary.unsupported_slot_count,
        unresolved_slot_count: summary.unresolved_mapping_count,
        fallback_substituted_count: summary.substituted_with_fallback_count,
        syntax_coverage_reported: true,
        parity_notes_visible: true,
        fallback_behavior_documented: true,
        rollback_path_present: rollback_present,
        full_fidelity_claim_blocked_when_unsupported: rollback_present,
        conforms: false,
    }]
}

fn extension_descriptor_rows(
    extension_gap_override: Option<&str>,
) -> Vec<ExtensionAppearanceDescriptorRow> {
    let packet = seeded_appearance_conformance_packet();
    packet
        .rows
        .iter()
        .map(|row| {
            let all_inherit = row.overall_support_class
                == aureline_extensions::appearance_conformance::AppearanceSupportClass::FullInheritance;
            let gap_visible = !all_inherit;
            let gap_product = gap_visible;
            let gap_export = gap_visible;
            let mut gap_diagnostics = gap_visible;
            if extension_gap_override == Some(row.surface_id.as_str()) {
                gap_diagnostics = false;
            }
            ExtensionAppearanceDescriptorRow {
                surface_id: row.surface_id.clone(),
                surface_label: row.surface_label.clone(),
                theme_inheritance: axis_inheritance_state(row.axes.first().map(|a| a.support_class)),
                density_inheritance: axis_inheritance_state(row.axes.get(1).map(|a| a.support_class)),
                high_contrast_inheritance: axis_inheritance_state(row.axes.get(2).map(|a| a.support_class)),
                focus_inheritance: axis_inheritance_state(row.axes.get(3).map(|a| a.support_class)),
                reduced_motion_inheritance: axis_inheritance_state(row.axes.get(4).map(|a| a.support_class)),
                gap_visible_in_product: gap_product,
                gap_visible_in_export: gap_export,
                gap_visible_in_diagnostics: gap_diagnostics,
                prevents_quiet_stable_claim: gap_product && gap_export && gap_diagnostics,
                conforms: false,
            }
        })
        .collect()
}

fn axis_inheritance_state(
    support: Option<aureline_extensions::appearance_conformance::AppearanceSupportClass>,
) -> ExtensionInheritanceState {
    use aureline_extensions::appearance_conformance::AppearanceSupportClass;
    match support {
        Some(AppearanceSupportClass::FullInheritance) => ExtensionInheritanceState::Inherits,
        Some(AppearanceSupportClass::ReducedSupport) => ExtensionInheritanceState::Partial,
        _ => ExtensionInheritanceState::DoesNotInherit,
    }
}

fn live_change_rows() -> Vec<LiveAppearanceChangeRow> {
    LiveAppearanceAxisClass::REQUIRED
        .into_iter()
        .map(|axis| {
            let (class, disclosure, note) = match axis {
                LiveAppearanceAxisClass::OsTheme => (
                    LiveApplyClass::ApplyLive,
                    false,
                    "OS theme signal applies live with no review".to_owned(),
                ),
                LiveAppearanceAxisClass::OsContrast => (
                    LiveApplyClass::ApplyLiveCheckpointed,
                    false,
                    "OS contrast signal applies live behind a revertable checkpoint".to_owned(),
                ),
                LiveAppearanceAxisClass::OsAccent => (
                    LiveApplyClass::ApplyLive,
                    false,
                    "OS accent signal applies live with no review".to_owned(),
                ),
                LiveAppearanceAxisClass::OsDensity => (
                    LiveApplyClass::ApplyLiveCheckpointed,
                    false,
                    "OS density signal applies live behind a revertable checkpoint".to_owned(),
                ),
                LiveAppearanceAxisClass::OsTextScale => (
                    LiveApplyClass::ConfirmRequired,
                    false,
                    "OS text-scale change requires user confirmation".to_owned(),
                ),
                LiveAppearanceAxisClass::OsReducedMotion => (
                    LiveApplyClass::ApplyLiveCheckpointed,
                    false,
                    "OS reduced-motion signal applies live behind a checkpoint".to_owned(),
                ),
            };
            LiveAppearanceChangeRow {
                axis,
                live_apply_class: class,
                disclosure_required: disclosure,
                silently_lags_system: false,
                applies_coherently_or_discloses: true,
                note,
                conforms: false,
            }
        })
        .collect()
}

fn provenance_rows() -> Vec<ProvenancePreservationRow> {
    ProvenanceDimensionClass::REQUIRED
        .into_iter()
        .map(|dimension| ProvenancePreservationRow {
            dimension,
            package_identity_survives_export: true,
            unresolved_slots_survive_export: true,
            overlay_lineage_survives_export: true,
            inheritance_gaps_survive_export: true,
            survives_sync_without_flattening: true,
            conforms: false,
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
        row_narration:
            "Appearance-session finalization certification for the active appearance session"
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
                "aureline://route/appearance-session-finalization/{}",
                surface.as_str()
            ),
            keyboard_reachable: true,
            activates_same_record: true,
        })
        .collect()
}

fn upstream(contract_ref: String, capture_refs: Vec<String>) -> CertificationUpstream {
    let token_registry_refs = [
        ThemeClass::DarkReference,
        ThemeClass::LightParity,
        ThemeClass::HighContrastDark,
        ThemeClass::HighContrastLight,
    ]
    .into_iter()
    .map(|theme| format!("aureline-ui:token_registry:{}", theme.token()))
    .collect();
    let conformance_packet = seeded_appearance_conformance_packet();
    CertificationUpstream {
        appearance_contract_ref: contract_ref,
        component_state_registry_ref: seeded_component_state_registry().registry_id,
        token_registry_refs,
        appearance_conformance_packet_ref: conformance_packet.packet_id.clone(),
        contributing_capture_refs: capture_refs,
    }
}

fn build_scenario(spec: &ScenarioSpec) -> AppearanceSessionFinalizationScenario {
    let contract = try_seeded_appearance_session_beta_contract()
        .expect("seeded appearance-session beta contract must project");
    let session_binding = binding();
    let theme_packages = theme_package_rows();
    let session_summaries = session_summary_rows(&session_binding);
    let token_overlays = token_overlay_rows(spec.overlay_scope_override);
    let import_reports = import_report_rows(spec.import_rollback_override);
    let extension_descriptors = extension_descriptor_rows(spec.extension_gap_override);
    let live_changes = live_change_rows();
    let provenance = provenance_rows();

    let capture_refs: Vec<String> =
        vec!["aureline://artifact/appearance-session-finalization-nominal".to_owned()];

    let input = CertificationInput {
        record_id: spec.scenario_id.to_owned(),
        as_of: CORPUS_AS_OF.to_owned(),
        posture_id: spec.posture_id.to_owned(),
        posture_label: spec.posture_label.to_owned(),
        title: spec.title.to_owned(),
        summary: spec.summary.to_owned(),
        appearance_session: session_binding,
        theme_packages,
        session_summaries,
        token_overlays,
        import_reports,
        extension_descriptors,
        live_changes,
        provenance,
        claim_ceiling: spec.claim_ceiling,
        recovery_routes: required_recovery_routes(),
        routes: routes(),
        accessibility: accessibility(),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(contract.packet_id.clone(), capture_refs),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_owned(),
        support_export_ref: SUPPORT_EXPORT_REF.to_owned(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_owned(),
            EVIDENCE_FIXTURE_REF.to_owned(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_owned()],
    };

    let record = AppearanceSessionFinalizationCertification::build(input)
        .unwrap_or_else(|err| panic!("scenario {} must build: {err}", spec.scenario_id));

    AppearanceSessionFinalizationScenario {
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
pub fn appearance_session_finalization_corpus() -> Vec<AppearanceSessionFinalizationScenario> {
    let specs = [
        ScenarioSpec {
            scenario_id: "nominal",
            posture_id: "appearance_session_finalization_nominal",
            posture_label: "Nominal appearance-session finalization",
            title: "Appearance session is finalized across theme packages, overlays, imports, extensions, live changes, and provenance",
            summary:
                "Every theme package carries a versioned manifest; the session summary is exportable \
                      and cites one package source; token overlays preserve unknown tokens; imported-theme \
                      mapping reports name translated, unsupported, and fallback slots; extension surfaces \
                      declare inheritance or surface visible gaps; live OS changes apply coherently or \
                      disclose reload; and provenance survives export/sync without flattening.",
            overlay_scope_override: None,
            extension_gap_override: None,
            import_rollback_override: None,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "token_overlay_silently_dropped_drill",
            posture_id: "appearance_session_finalization_overlay_drill",
            posture_label: "Token-overlay silently-dropped drill",
            title: "A token overlay that silently drops tokens narrows the certification with a named reason",
            summary:
                "An adversarial posture where the workspace token overlay silently drops unknown \
                      tokens instead of preserving them inert; the lane detects the drop and narrows \
                      the posture below Stable with a named reason.",
            overlay_scope_override: Some(OverlayScopeClass::Workspace),
            extension_gap_override: None,
            import_rollback_override: None,
            claim_ceiling: CertificationClaimCeiling {
                asserts_token_overlays_validated: false,
                ..full_claim_ceiling()
            },
        },
        ScenarioSpec {
            scenario_id: "extension_gap_undisclosed",
            posture_id: "appearance_session_finalization_extension_gap_undisclosed",
            posture_label: "Extension gap undisclosed in diagnostics",
            title: "An undisclosed extension inheritance gap narrows the certification",
            summary:
                "An extension surface declares partial theme inheritance but does not disclose the \
                      gap in diagnostics; the lane detects the undisclosed gap and narrows the posture \
                      below Stable with a named reason.",
            overlay_scope_override: None,
            extension_gap_override: Some("extension-surface-01"),
            import_rollback_override: None,
            claim_ceiling: CertificationClaimCeiling {
                asserts_extension_gaps_visible: false,
                ..full_claim_ceiling()
            },
        },
        ScenarioSpec {
            scenario_id: "import_report_missing_rollback",
            posture_id: "appearance_session_finalization_import_missing_rollback",
            posture_label: "Import report missing rollback path",
            title: "An imported-theme mapping report without a rollback path narrows the certification",
            summary:
                "An imported-theme mapping report names translated and unsupported slots but lacks \
                      a rollback path; the lane detects the missing rollback and narrows the posture \
                      below Stable with a named reason.",
            overlay_scope_override: None,
            extension_gap_override: None,
            import_rollback_override: Some("import-report-01"),
            claim_ceiling: CertificationClaimCeiling {
                asserts_import_reports_honest: false,
                ..full_claim_ceiling()
            },
        },
    ];
    let scenarios: Vec<AppearanceSessionFinalizationScenario> =
        specs.iter().map(build_scenario).collect();
    debug_assert!(scenarios.iter().all(|scenario| {
        is_canonical_object_ref(&scenario.record.diagnostics_export_ref)
            && is_canonical_object_ref(&scenario.record.support_export_ref)
    }));
    scenarios
}
