//! Canonical seed builders for the M5 dynamic-surface AT diagnostics report.
//!
//! These builders are the single producer of the checked-in support export and the
//! bridge/announcement/visual drill fixtures. The headless emitter and the inline tests
//! both call them so the in-code diagnostics, the artifact, and the fixtures never drift.

use super::*;

use crate::announcement_grammar::M5DurableFallbackSurface;

/// Stable packet id for the canonical (all-green) diagnostics report.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_REPORT_PACKET_ID: &str = "m5-at-diagnostics:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

/// Proof packet ref every governed surface carries.
const DIAGNOSTICS_PROOF_REF: &str = "evidence:at-diagnostics-conformance:m5";

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

/// Severity of a diagnostic class: reduced-motion is advisory, every other class blocks.
fn severity_for(class: M5AtDiagnosticClass) -> M5DiagnosticSeverity {
    match class {
        M5AtDiagnosticClass::ReducedMotionRegression => M5DiagnosticSeverity::Advisory,
        _ => M5DiagnosticSeverity::Blocking,
    }
}

/// The downgrade triggers every governed surface carries — broad enough that any
/// auto-narrowing class resolves to a present trigger.
fn standard_downgrade_triggers() -> Vec<M5DynamicSurfaceA11yDowngradeTrigger> {
    use M5DynamicSurfaceA11yDowngradeTrigger as D;
    vec![
        D::ProofStale,
        D::BridgeUnavailable,
        D::BridgePartialOrStale,
        D::FocusTeleported,
        D::FocusLost,
        D::LiveRegionSpam,
        D::NonVisualFidelityLost,
        D::LabelOrRoleDrift,
        D::PointerOrHoverDependence,
    ]
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SCHEMA_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_DOC_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_MATRIX_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SURFACE_DESCRIPTOR_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_ANNOUNCEMENT_GRAMMAR_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SCREEN_READER_CONTRACT_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_FOCUS_CONTRACT_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_VISUAL_ADAPTATION_CONTRACT_REF,
    ])
}

fn row_source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SURFACE_DESCRIPTOR_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SCREEN_READER_CONTRACT_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_FOCUS_CONTRACT_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_VISUAL_ADAPTATION_CONTRACT_REF,
    ])
}

/// Builds the full battery of green checks (one `pass` per diagnostic class) for a
/// surface family.
fn green_checks(family: M5SurfaceFamily) -> Vec<M5DiagnosticCheck> {
    M5AtDiagnosticClass::ALL
        .iter()
        .map(|&class| M5DiagnosticCheck {
            class,
            outcome: M5DiagnosticOutcome::Pass,
            severity: severity_for(class),
            detail_message_id: format!("diagnostic.{}.{}", family.as_str(), class.as_str()),
            evidence_ref: format!(
                "evidence:at-diagnostics:{}:{}",
                family.as_str(),
                class.as_str()
            ),
            focus_return_disposition: if class == M5AtDiagnosticClass::FocusReturnFailure {
                Some(A11yFocusReturnDisposition::ReturnedExact)
            } else {
                None
            },
        })
        .collect()
}

/// Sets the outcome of a single check (used by the drill builders).
fn set_check(
    surface: &mut M5SurfaceDiagnostics,
    class: M5AtDiagnosticClass,
    outcome: M5DiagnosticOutcome,
) {
    if let Some(check) = surface.checks.iter_mut().find(|c| c.class == class) {
        check.outcome = outcome;
    }
}

/// A healthy bridge probe with full semantic-node coverage.
fn healthy_probe(
    family: M5SurfaceFamily,
    bridge_kind: M5SurfaceBridgeKind,
    nodes: u32,
) -> M5SurfaceBridgeProbe {
    M5SurfaceBridgeProbe {
        bridge_kind,
        bridge_state: A11yBridgeState::BridgedActive,
        non_visual_fidelity: A11yNonVisualFidelity::FullAccessible,
        semantic_node_coverage: M5SemanticNodeCoverage {
            expected_nodes: nodes,
            present_nodes: nodes,
            missing_nodes: 0,
        },
        degradation_reason: M5BridgeDegradationReason::None,
        probe_message_id: format!("diagnostic.{}.bridge_probe", family.as_str()),
    }
}

/// A within-budget announcement check for a surface that coalesces its live region.
fn within_budget_announcements(family: M5SurfaceFamily) -> M5AnnouncementBudgetCheck {
    M5AnnouncementBudgetCheck {
        budget: M5CoalescingBudget {
            strategy: A11yCoalescingStrategy::DedupeSameMeaning,
            max_announcements_per_window: 6,
            window_seconds: 10,
            min_interval_ms: 250,
            suppress_unchanged_meaning: true,
        },
        observed_announcements_in_window: 2,
        observed_min_interval_ms: 600,
        within_budget: true,
        budget_message_id: format!("diagnostic.{}.announcement_budget", family.as_str()),
    }
}

fn green_visual_conformance(family: M5SurfaceFamily) -> M5VisualConformanceCheck {
    M5VisualConformanceCheck {
        high_zoom: M5DiagnosticOutcome::Pass,
        high_contrast: M5DiagnosticOutcome::Pass,
        reduced_motion: M5DiagnosticOutcome::Pass,
        conformance_message_id: format!("diagnostic.{}.visual_conformance", family.as_str()),
    }
}

/// A degraded-state disclosure mirroring its probe.
fn degraded_state_from_probe(
    family: M5SurfaceFamily,
    probe: &M5SurfaceBridgeProbe,
) -> M5DegradedStateDisclosure {
    let is_degraded = probe.bridge_state != A11yBridgeState::BridgedActive
        || probe.non_visual_fidelity != A11yNonVisualFidelity::FullAccessible
        || probe.degradation_reason.is_degraded();
    M5DegradedStateDisclosure {
        is_degraded,
        bridge_state: probe.bridge_state,
        non_visual_fidelity: probe.non_visual_fidelity,
        degradation_reason: probe.degradation_reason,
        disclosure_message_id: format!("diagnostic.{}.degraded_state", family.as_str()),
    }
}

/// Recomputes a surface's gate and degraded-state from its checks and probe, so a drill
/// mutation stays internally consistent without hand-maintaining the derived blocks.
fn reconcile_surface(surface: &mut M5SurfaceDiagnostics) {
    let blocking = surface.computed_blocking_classes();
    surface.gate = M5SurfaceReleaseGate {
        decision: if blocking.is_empty() {
            M5ReleaseGateDecision::Pass
        } else {
            M5ReleaseGateDecision::Blocked
        },
        blocking_finding_classes: blocking,
        gate_message_id: format!(
            "diagnostic.{}.release_gate",
            surface.surface_family.as_str()
        ),
    };
    surface.current_degraded_state =
        degraded_state_from_probe(surface.surface_family, &surface.bridge_probe);
    surface.visual_conformance.high_zoom = surface
        .check(M5AtDiagnosticClass::HighZoomRegression)
        .map(|c| c.outcome)
        .unwrap_or(M5DiagnosticOutcome::Pass);
    surface.visual_conformance.high_contrast = surface
        .check(M5AtDiagnosticClass::HighContrastRegression)
        .map(|c| c.outcome)
        .unwrap_or(M5DiagnosticOutcome::Pass);
    surface.visual_conformance.reduced_motion = surface
        .check(M5AtDiagnosticClass::ReducedMotionRegression)
        .map(|c| c.outcome)
        .unwrap_or(M5DiagnosticOutcome::Pass);
}

#[allow(clippy::too_many_arguments)]
fn green_surface(
    surface_family: M5SurfaceFamily,
    surface_label: &str,
    object_identity_ref: &str,
    bridge_kind: M5SurfaceBridgeKind,
    nodes: u32,
    durable_fallback: M5DurableFallbackRef,
    consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
) -> M5SurfaceDiagnostics {
    let probe = healthy_probe(surface_family, bridge_kind, nodes);
    let current_degraded_state = degraded_state_from_probe(surface_family, &probe);
    M5SurfaceDiagnostics {
        surface_id: format!("diagnostics:{}", surface_family.as_str()),
        surface_family,
        surface_label: surface_label.to_owned(),
        owner_role: "Accessibility owner".to_owned(),
        object_identity_ref: object_identity_ref.to_owned(),
        qualification: M5DynamicSurfaceA11yQualificationClass::Stable,
        bridge_probe: probe,
        checks: green_checks(surface_family),
        announcement_budget: within_budget_announcements(surface_family),
        visual_conformance: green_visual_conformance(surface_family),
        current_degraded_state,
        gate: M5SurfaceReleaseGate {
            decision: M5ReleaseGateDecision::Pass,
            blocking_finding_classes: Vec::new(),
            gate_message_id: format!("diagnostic.{}.release_gate", surface_family.as_str()),
        },
        durable_fallback,
        downgrade_triggers: standard_downgrade_triggers(),
        required_proof_packet_refs: strings(&[DIAGNOSTICS_PROOF_REF]),
        source_contract_refs: row_source_contract_refs(),
        consumer_surfaces,
    }
}

fn surfaces() -> Vec<M5SurfaceDiagnostics> {
    use M5DurableFallbackSurface as Surface;
    use M5DynamicSurfaceA11yConsumerSurface as Consumer;
    use M5SurfaceBridgeKind as Bridge;
    use M5SurfaceFamily as Family;

    vec![
        green_surface(
            Family::ShellRegion,
            "Shell zones and landmarks",
            "shell:zone-root",
            Bridge::UiAutomation,
            8,
            fallback(Surface::StatusDetail, "status-detail:shell-diagnostics"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        green_surface(
            Family::EditorCanvas,
            "Custom-rendered editor canvas",
            "editor:active-buffer",
            Bridge::NsAccessibility,
            20,
            fallback(Surface::StatusDetail, "status-detail:editor-diagnostics"),
            vec![Consumer::Editor, Consumer::SupportExport],
        ),
        green_surface(
            Family::TerminalCanvas,
            "Terminal / log canvas",
            "terminal:active-session",
            Bridge::AtSpi,
            14,
            fallback(Surface::ActivityRow, "activity-row:terminal-diagnostics"),
            vec![Consumer::Terminal, Consumer::SupportExport],
        ),
        green_surface(
            Family::DenseCollection,
            "Dense list / table / grid",
            "data-grid:active-view",
            Bridge::UiAutomation,
            18,
            fallback(
                Surface::SelectionSummary,
                "selection-summary:data-grid-diagnostics",
            ),
            vec![Consumer::DataGrid, Consumer::SupportExport],
        ),
        green_surface(
            Family::NotebookCell,
            "Notebook cell",
            "notebook:active-cell",
            Bridge::NsAccessibility,
            12,
            fallback(Surface::StatusDetail, "status-detail:notebook-diagnostics"),
            vec![Consumer::Notebook, Consumer::SupportExport],
        ),
        green_surface(
            Family::DataCell,
            "Data-surface cell",
            "data:active-cell",
            Bridge::AtSpi,
            10,
            fallback(
                Surface::SelectionSummary,
                "selection-summary:data-cell-diagnostics",
            ),
            vec![Consumer::DataGrid, Consumer::SupportExport],
        ),
        green_surface(
            Family::ReviewDiff,
            "Review / diff hunk surface",
            "review:active-diff",
            Bridge::UiAutomation,
            16,
            fallback(
                Surface::PatchReviewHeader,
                "patch-review-header:review-diagnostics",
            ),
            vec![Consumer::Review, Consumer::SupportExport],
        ),
        green_surface(
            Family::OverlaySheet,
            "Durable overlay / sheet",
            "overlay:active-sheet",
            Bridge::HeadlessInspector,
            6,
            fallback(Surface::BannerDetail, "banner-detail:overlay-diagnostics"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
    ]
}

fn conformance_review() -> M5DiagnosticsConformanceReview {
    M5DiagnosticsConformanceReview {
        every_protected_surface_has_diagnostics: true,
        bridge_state_and_missing_nodes_diagnosable: true,
        announcement_rate_and_coalescing_diagnosable: true,
        focus_return_failures_diagnosable: true,
        zoom_contrast_motion_regressions_diagnosable: true,
        announcement_spam_budgets_enforced: true,
        release_gate_fails_on_blocking_regressions: true,
        degraded_state_disclosed_not_hidden: true,
        diagnostics_reuse_descriptor_object_identity: true,
        support_export_carries_bridge_health_message_ids_focus_failures_degraded_state: true,
        claimed_surfaces_auto_narrow_when_bridge_or_proof_stale: true,
        per_surface_diagnostics_not_replaced_by_aggregate_dashboard: true,
    }
}

fn consumer_projection() -> M5DiagnosticsConsumerProjection {
    M5DiagnosticsConsumerProjection {
        shell_consumes_diagnostics: true,
        editor_surface_diagnosed: true,
        terminal_surface_diagnosed: true,
        data_grid_surface_diagnosed: true,
        notebook_surface_diagnosed: true,
        review_surface_diagnosed: true,
        support_export_consumes_diagnostics: true,
        help_documents_diagnostics: true,
        release_public_truth_gates_on_diagnostics: true,
        at_conformance_packets_reuse_diagnostics: true,
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
        release_packet_ref: "evidence:at-diagnostics-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:at-diagnostics-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

/// Builds the report-level release gate from the per-surface gates.
fn aggregate_release_gate(surfaces: &[M5SurfaceDiagnostics]) -> M5DiagnosticsReleaseGate {
    let mut blocked_surface_ids: Vec<String> = surfaces
        .iter()
        .filter(|s| s.gate.decision.blocks())
        .map(|s| s.surface_id.clone())
        .collect();
    blocked_surface_ids.sort();
    M5DiagnosticsReleaseGate {
        blocks_release: !blocked_surface_ids.is_empty(),
        blocked_surface_ids,
        gate_message_id: "diagnostic.release_gate".to_owned(),
    }
}

fn base_input(
    packet_id: &str,
    surfaces: Vec<M5SurfaceDiagnostics>,
) -> M5DynamicA11yDiagnosticsPacketInput {
    let release_gate = aggregate_release_gate(&surfaces);
    M5DynamicA11yDiagnosticsPacketInput {
        packet_id: packet_id.to_owned(),
        report_label: "M5 Dynamic-Surface Assistive-Tech Diagnostics".to_owned(),
        surfaces,
        shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        diagnostics_vocabulary_set: M5DiagnosticsVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        release_gate,
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical all-green diagnostics report packet.
///
/// This is the single producer of the checked-in support export: every protected surface
/// is bridged, within its announcement budget, and conformant under zoom/contrast/motion,
/// so the release gate passes.
pub fn seeded_m5_dynamic_a11y_diagnostics_report() -> M5DynamicA11yDiagnosticsPacket {
    M5DynamicA11yDiagnosticsPacket::new(base_input(
        M5_DYNAMIC_A11Y_DIAGNOSTICS_REPORT_PACKET_ID,
        surfaces(),
    ))
}

/// Mutates one surface in a fresh green catalog and returns the rebuilt packet.
fn drill<F: FnOnce(&mut M5SurfaceDiagnostics)>(
    packet_id: &str,
    family: M5SurfaceFamily,
    mutate: F,
) -> M5DynamicA11yDiagnosticsPacket {
    let mut surfaces = surfaces();
    if let Some(surface) = surfaces.iter_mut().find(|s| s.surface_family == family) {
        mutate(surface);
        reconcile_surface(surface);
    }
    M5DynamicA11yDiagnosticsPacket::new(base_input(packet_id, surfaces))
}

/// Builds a drill where the editor canvas's OS accessibility bridge is unavailable and
/// the surface auto-narrows from Stable to Beta, dropping its non-visual fidelity to
/// `degraded_accessible` while keeping every check, the disclosed degraded state, and the
/// `bridge_unavailable` trigger intact. The narrowing is honest, so the surface still
/// ships at the narrowed claim and the release gate stays green.
pub fn seeded_m5_dynamic_a11y_diagnostics_report_bridge_unavailable_narrowed(
) -> M5DynamicA11yDiagnosticsPacket {
    drill(
        "m5-at-diagnostics:bridge-unavailable-narrowed:0001",
        M5SurfaceFamily::EditorCanvas,
        |surface| {
            surface.qualification = M5DynamicSurfaceA11yQualificationClass::Beta;
            surface.bridge_probe.bridge_state = A11yBridgeState::Unavailable;
            surface.bridge_probe.non_visual_fidelity = A11yNonVisualFidelity::DegradedAccessible;
            surface.bridge_probe.degradation_reason =
                M5BridgeDegradationReason::PlatformBridgeUnavailable;
            surface.bridge_probe.semantic_node_coverage = M5SemanticNodeCoverage {
                expected_nodes: 20,
                present_nodes: 0,
                missing_nodes: 20,
            };
            set_check(
                surface,
                M5AtDiagnosticClass::BridgeHealth,
                M5DiagnosticOutcome::AutoNarrowed,
            );
            set_check(
                surface,
                M5AtDiagnosticClass::MissingSemanticNode,
                M5DiagnosticOutcome::AutoNarrowed,
            );
        },
    )
}

/// Builds a drill where the terminal canvas's bridge mapping has gone stale and partial
/// but the surface still over-claims Stable. The bridge-health and missing-semantic-node
/// checks are unhandled blocking regressions, so the per-surface gate and the report-level
/// gate both block release — the row a release/public-truth run fails on for a bridge
/// regression.
pub fn seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked(
) -> M5DynamicA11yDiagnosticsPacket {
    drill(
        "m5-at-diagnostics:bridge-regression-blocked:0001",
        M5SurfaceFamily::TerminalCanvas,
        |surface| {
            surface.bridge_probe.bridge_state = A11yBridgeState::Stale;
            surface.bridge_probe.non_visual_fidelity = A11yNonVisualFidelity::DegradedAccessible;
            surface.bridge_probe.degradation_reason = M5BridgeDegradationReason::StaleMapping;
            surface.bridge_probe.semantic_node_coverage = M5SemanticNodeCoverage {
                expected_nodes: 14,
                present_nodes: 5,
                missing_nodes: 9,
            };
            set_check(
                surface,
                M5AtDiagnosticClass::BridgeHealth,
                M5DiagnosticOutcome::Regressed,
            );
            set_check(
                surface,
                M5AtDiagnosticClass::MissingSemanticNode,
                M5DiagnosticOutcome::Regressed,
            );
        },
    )
}

/// Builds a drill where the dense-collection live region floods past its announcement
/// budget. The announcement-rate and coalescing checks are unhandled blocking
/// regressions, so the gate blocks release — the row a release/public-truth run fails on
/// for announcement spam. The bridge stays healthy, so no degraded bridge state is
/// claimed.
pub fn seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked(
) -> M5DynamicA11yDiagnosticsPacket {
    drill(
        "m5-at-diagnostics:announcement-spam-blocked:0001",
        M5SurfaceFamily::DenseCollection,
        |surface| {
            surface.announcement_budget.observed_announcements_in_window = 40;
            surface.announcement_budget.observed_min_interval_ms = 50;
            surface.announcement_budget.within_budget = false;
            set_check(
                surface,
                M5AtDiagnosticClass::AnnouncementRate,
                M5DiagnosticOutcome::Regressed,
            );
            set_check(
                surface,
                M5AtDiagnosticClass::CoalescingViolation,
                M5DiagnosticOutcome::Regressed,
            );
        },
    )
}

/// Builds a drill where the review/diff surface regresses under forced colors. The
/// high-contrast check is an unhandled blocking regression (the gate blocks release for a
/// contrast breakage); the reduced-motion check also regresses but is advisory, so it is
/// recorded without blocking — proving advisory findings do not gate while contrast
/// breakage does.
pub fn seeded_m5_dynamic_a11y_diagnostics_report_visual_regression_blocked(
) -> M5DynamicA11yDiagnosticsPacket {
    drill(
        "m5-at-diagnostics:visual-regression-blocked:0001",
        M5SurfaceFamily::ReviewDiff,
        |surface| {
            set_check(
                surface,
                M5AtDiagnosticClass::HighContrastRegression,
                M5DiagnosticOutcome::Regressed,
            );
            set_check(
                surface,
                M5AtDiagnosticClass::ReducedMotionRegression,
                M5DiagnosticOutcome::Regressed,
            );
        },
    )
}
