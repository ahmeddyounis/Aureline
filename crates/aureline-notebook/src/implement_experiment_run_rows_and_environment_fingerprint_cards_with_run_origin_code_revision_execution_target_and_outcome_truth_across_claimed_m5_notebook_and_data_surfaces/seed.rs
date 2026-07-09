//! Canonical seed builders for the experiment-run-row / environment-fingerprint controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code
//! components, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical experiment-run-row / environment-fingerprint packet.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_PACKET_ID: &str =
    "m5-experiment-run-row-environment-fingerprint-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn run_row_source_refs() -> Vec<String> {
    strings(&[
        M5_EXPERIMENT_RUN_ROW_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn fingerprint_source_refs() -> Vec<String> {
    strings(&[
        M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn run_row_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::RunOriginUnstated,
        M5ExperimentDowngradeTrigger::CodeRevisionUnstated,
        M5ExperimentDowngradeTrigger::ImportedRunUnmarked,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn fingerprint_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::EnvironmentFingerprintUnstated,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

/// Builds an experiment run row, deriving the origin class, the first-party claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn run_row(
    run_id: &str,
    run_label: &str,
    origin_kind: M5RunOriginKind,
    status_state: M5RunStatusState,
    status_label: &str,
    code_revision: &str,
    execution_origin_label: &str,
    run_window_label: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    run_actions: Vec<RunRowAction>,
    dispositions: Vec<M5ExperimentDisposition>,
) -> ExperimentRunRow {
    let disclosure = resolve_run_origin(origin_kind);
    ExperimentRunRow {
        component: M5ExperimentComponentFamily::ExperimentRunRow,
        run_id: run_id.to_owned(),
        run_label: run_label.to_owned(),
        origin_kind,
        status_state,
        status_label: status_label.to_owned(),
        origin_class: disclosure.origin_class,
        claims_first_party_origin: disclosure.is_first_party_origin,
        imported_note: if disclosure.needs_imported_note {
            "Imported from another tracker; provenance is only as complete as the import".to_owned()
        } else {
            String::new()
        },
        manual_attach_note: if disclosure.needs_manual_attach_note {
            "Manually attached to an external execution; Aureline did not launch this run"
                .to_owned()
        } else {
            String::new()
        },
        unknown_origin_note: if disclosure.needs_unknown_origin_note {
            "Run origin could not be resolved; do not treat it as a first-party run".to_owned()
        } else {
            String::new()
        },
        origin_and_status_note: format!(
            "Origin {}; status {}",
            disclosure.origin_class.as_str(),
            status_state.as_str()
        ),
        code_revision: code_revision.to_owned(),
        execution_origin_label: execution_origin_label.to_owned(),
        run_window_label: run_window_label.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        run_actions,
        dispositions,
        downgrade_triggers: run_row_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "run_id",
            "origin_kind",
            "status_state",
            "code_revision",
            "execution_origin_label",
            "origin_class",
            "context_note",
            "deep_link_kind",
        ]),
        source_contract_refs: run_row_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_run_origin_or_revision: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

/// Builds an environment fingerprint card, deriving the capture class, the captured claim, and
/// the required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn fingerprint_card(
    card_id: &str,
    card_label: &str,
    scope_class: M5FingerprintScopeClass,
    fingerprint_state: M5FingerprintState,
    interpreter_or_kernel_label: &str,
    toolchain_summary: &str,
    execution_target_label: &str,
    hardware_profile_label: &str,
    freshness_label: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    card_actions: Vec<FingerprintCardAction>,
    dispositions: Vec<M5ExperimentDisposition>,
) -> EnvironmentFingerprintCard {
    let disclosure = resolve_fingerprint_capture(fingerprint_state);
    EnvironmentFingerprintCard {
        component: M5ExperimentComponentFamily::EnvironmentFingerprintCard,
        card_id: card_id.to_owned(),
        card_label: card_label.to_owned(),
        scope_class,
        fingerprint_state,
        capture_class: disclosure.capture_class,
        claims_captured: disclosure.is_reliably_captured,
        partial_note: if disclosure.needs_partial_note {
            "Only part of this environment was captured; treat reproducibility as likely, not proven"
                .to_owned()
        } else {
            String::new()
        },
        uncaptured_note: if disclosure.needs_uncaptured_note {
            format!(
                "Environment is {} and not reliably captured; a rerun is needed to reproduce",
                fingerprint_state.as_str()
            )
        } else {
            String::new()
        },
        interpreter_or_kernel_label: interpreter_or_kernel_label.to_owned(),
        toolchain_summary: toolchain_summary.to_owned(),
        execution_target_label: execution_target_label.to_owned(),
        hardware_profile_label: hardware_profile_label.to_owned(),
        freshness_label: freshness_label.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        card_actions,
        dispositions,
        downgrade_triggers: fingerprint_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "card_id",
            "scope_class",
            "fingerprint_state",
            "interpreter_or_kernel_label",
            "toolchain_summary",
            "execution_target_label",
            "capture_class",
            "freshness_label",
            "deep_link_kind",
        ]),
        source_contract_refs: fingerprint_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_run_origin_or_revision: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

fn run_rows() -> Vec<ExperimentRunRow> {
    use DeepLinkKind as Link;
    use M5ExperimentDisposition as Disp;
    use M5RunOriginKind as Origin;
    use M5RunStatusState as Status;
    use RunRowAction as Action;

    vec![
        // 1. Notebook cell, succeeded → local run.
        run_row(
            "run-nb-2026-07-001",
            "Baseline sweep (notebook)",
            Origin::NotebookCell,
            Status::Succeeded,
            "Succeeded",
            "commit:9f3ac21",
            "Local kernel on this workstation",
            "Started 09:14, ended 09:52 (38m)",
            "Local run you launched from a notebook cell; compare or export it directly",
            Link::RunObject,
            "run:exp-2026-07-001",
            vec![
                Action::OpenRun,
                Action::CompareRuns,
                Action::ExportRun,
                Action::OpenDeepLink,
                Action::InspectFingerprint,
            ],
            vec![Disp::LocalRun, Disp::Reproducible],
        ),
        // 2. Script task, running → local run.
        run_row(
            "run-script-2026-07-014",
            "Nightly featurize (script)",
            Origin::ScriptTask,
            Status::Running,
            "Running",
            "commit:1c7de40",
            "Local runner on this workstation",
            "Started 22:00, still running",
            "Local run from a script task; still running, so results are provisional",
            Link::NotebookLocation,
            "notebook:featurize.ipynb#cell-3",
            vec![
                Action::OpenRun,
                Action::CompareRuns,
                Action::ExportRun,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalRun],
        ),
        // 3. Scheduled task, queued → managed run.
        run_row(
            "run-sched-2026-07-020",
            "Weekly retrain (scheduled)",
            Origin::ScheduledTask,
            Status::Queued,
            "Queued",
            "workspace:main@rev-204",
            "Managed scheduler",
            "Queued for 02:00 window",
            "Managed run Aureline schedules; queued, not yet started",
            Link::RunObject,
            "run:exp-2026-07-020",
            vec![
                Action::OpenRun,
                Action::CompareRuns,
                Action::ExportRun,
                Action::CopyRunId,
            ],
            vec![Disp::ManagedRun],
        ),
        // 4. Manual attach, failed → manually attached (needs manual note).
        run_row(
            "run-manual-2026-07-031",
            "Attached external eval",
            Origin::ManualAttach,
            Status::Failed,
            "Failed",
            "commit:unknown-attached",
            "External host (attached)",
            "Started 14:05, failed 14:07",
            "Manually attached run; Aureline did not launch it, so provenance is limited",
            Link::DocsAnchor,
            "docs:notebooks/attach-external-run",
            vec![
                Action::OpenRun,
                Action::CompareRuns,
                Action::ExportRun,
                Action::OpenDeepLink,
            ],
            vec![Disp::ManualAttach, Disp::NeedsRerun],
        ),
        // 5. Imported run, stale → imported (needs imported note).
        run_row(
            "run-import-2025-11-002",
            "Imported baseline (last quarter)",
            Origin::ImportedRun,
            Status::Stale,
            "Stale / superseded",
            "commit:imported-a13f",
            "Imported from external tracker",
            "Ran last quarter; imported this week",
            "Imported run; superseded and only as complete as the import, so check parity",
            Link::DatasetCatalogAnchor,
            "dataset:catalog/imported-baselines",
            vec![
                Action::OpenRun,
                Action::CompareRuns,
                Action::ExportRun,
                Action::OpenDeepLink,
            ],
            vec![Disp::ImportedRun, Disp::ContextIncomplete],
        ),
        // 6. Unknown origin, canceled → origin unknown (needs unknown note).
        run_row(
            "run-unknown-2026-07-040",
            "Unlabeled run",
            Origin::UnknownOrigin,
            Status::Canceled,
            "Canceled",
            "workspace:unresolved",
            "Origin unresolved",
            "Canceled before completion; timing unknown",
            "Origin could not be resolved; do not trust it in a comparison until clarified",
            Link::NoDeepLink,
            "",
            vec![Action::OpenRun, Action::CompareRuns, Action::ExportRun],
            vec![Disp::ContextIncomplete],
        ),
    ]
}

fn fingerprint_cards() -> Vec<EnvironmentFingerprintCard> {
    use DeepLinkKind as Link;
    use FingerprintCardAction as Action;
    use M5ExperimentDisposition as Disp;
    use M5FingerprintScopeClass as Scope;
    use M5FingerprintState as State;

    vec![
        // 1. Interpreter, captured complete → captured.
        fingerprint_card(
            "fp-interpreter-001",
            "Interpreter fingerprint",
            Scope::Interpreter,
            State::CapturedComplete,
            "CPython 3.11.7",
            "Standard library only; no third-party interpreter patches",
            "Local kernel on this workstation",
            "Hardware profile: 8-core CPU, no accelerator",
            "Captured at run start (fresh)",
            "Interpreter is fully captured; safe to reproduce",
            Link::RunObject,
            "run:exp-2026-07-001",
            vec![
                Action::InspectFingerprint,
                Action::ExportFingerprint,
                Action::OpenDeepLink,
                Action::PinEnvironment,
            ],
            vec![Disp::Reproducible],
        ),
        // 2. Kernel spec, captured partial → partially captured (needs partial note).
        fingerprint_card(
            "fp-kernelspec-002",
            "Kernel spec fingerprint",
            Scope::KernelSpec,
            State::CapturedPartial,
            "Python 3.11 (ipykernel)",
            "Kernel argv captured; env vars only partially captured",
            "Local kernel on this workstation",
            "Hardware profile: 8-core CPU, no accelerator",
            "Captured mid-run (partial)",
            "Kernel spec is only partially captured; reproducibility is likely, not proven",
            Link::NotebookLocation,
            "notebook:analysis.ipynb#kernel",
            vec![
                Action::InspectFingerprint,
                Action::ExportFingerprint,
                Action::OpenDeepLink,
            ],
            vec![Disp::LikelyReproducible],
        ),
        // 3. Packages, pinned → pinned.
        fingerprint_card(
            "fp-packages-003",
            "Package fingerprint",
            Scope::Packages,
            State::Pinned,
            "CPython 3.11.7",
            "Lockfile-pinned: numpy 1.26.4, pandas 2.2.1, scikit-learn 1.4.2",
            "Local kernel on this workstation",
            "Hardware profile: 8-core CPU, no accelerator",
            "Pinned to a lockfile (stable)",
            "Packages are pinned to a lockfile; safe to reproduce",
            Link::DatasetCatalogAnchor,
            "dataset:catalog/env-lockfiles",
            vec![
                Action::InspectFingerprint,
                Action::ExportFingerprint,
                Action::CompareEnvironments,
                Action::CopyFingerprintId,
            ],
            vec![Disp::Reproducible],
        ),
        // 4. Hardware accelerator, drifted → uncaptured (needs uncaptured note).
        fingerprint_card(
            "fp-accelerator-004",
            "Accelerator fingerprint",
            Scope::HardwareAccelerator,
            State::Drifted,
            "CPython 3.11.7",
            "CUDA toolkit summary captured at first run only",
            "GPU host (managed pool)",
            "Hardware profile: drifted from captured accelerator model",
            "Captured earlier; accelerator has since drifted",
            "Accelerator drifted from the captured fingerprint; rerun to reproduce",
            Link::DocsAnchor,
            "docs:notebooks/environment-drift",
            vec![
                Action::InspectFingerprint,
                Action::ExportFingerprint,
                Action::OpenDeepLink,
            ],
            vec![Disp::NeedsRerun],
        ),
        // 5. OS / platform, captured missing → uncaptured (needs uncaptured note).
        fingerprint_card(
            "fp-os-005",
            "OS / platform fingerprint",
            Scope::OsPlatform,
            State::CapturedMissing,
            "CPython 3.11.7",
            "Toolchain summary present; OS / platform capture missing",
            "Unrecorded host",
            "Hardware profile: unavailable on this build",
            "OS / platform capture missing",
            "OS and platform were not captured; do not assume this environment reproduces",
            Link::NoDeepLink,
            "",
            vec![Action::InspectFingerprint, Action::ExportFingerprint],
            vec![Disp::ContextIncomplete],
        ),
        // 6. Container image, unavailable → uncaptured (needs uncaptured note).
        fingerprint_card(
            "fp-container-006",
            "Container image fingerprint",
            Scope::ContainerImage,
            State::Unavailable,
            "CPython 3.11.7",
            "Container image digest unavailable on this deployment line",
            "Container runtime (digest unavailable)",
            "Hardware profile: unavailable on this build",
            "Container fingerprint unavailable",
            "Container image fingerprint is unavailable; capture it before trusting a rerun",
            Link::DocsAnchor,
            "docs:notebooks/container-fingerprint",
            vec![
                Action::InspectFingerprint,
                Action::ExportFingerprint,
                Action::OpenDeepLink,
            ],
            vec![Disp::ContextIncomplete],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::RunOriginUnstated,
        M5ExperimentDowngradeTrigger::CodeRevisionUnstated,
        M5ExperimentDowngradeTrigger::EnvironmentFingerprintUnstated,
        M5ExperimentDowngradeTrigger::ImportedRunUnmarked,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn experiment_review() -> ExperimentRunEnvironmentReview {
    ExperimentRunEnvironmentReview {
        run_row_shows_origin: true,
        run_row_shows_code_revision: true,
        run_row_offers_open_compare_export: true,
        fingerprint_card_shows_environment: true,
        fingerprint_card_offers_inspect_export: true,
        origin_and_capture_derived_never_asserted: true,
        unknown_origin_never_shown_as_first_party: true,
        uncaptured_never_shown_as_captured: true,
        comparison_never_implied_apples_to_apples_without_parity: true,
        every_next_step_names_stable_deep_link: true,
        no_component_widens_export_scope_or_exposes_raw_by_default: true,
        run_identity_and_revision_always_explicit: true,
        provenance_and_sensitivity_state_visible: true,
        cached_offline_local_only_state_visible: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ExperimentRunEnvironmentConsumerProjection {
    ExperimentRunEnvironmentConsumerProjection {
        experiment_run_dashboard_reads_single_source: true,
        environment_fingerprint_surface_reads_single_source: true,
        origin_and_status_visible_before_compare_or_share: true,
        environment_capture_visible_before_trust: true,
        support_export_shows_component_truth: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> ExperimentRunEnvironmentProofFreshness {
    ExperimentRunEnvironmentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF,
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_EXPERIMENT_RUN_ROW_SCHEMA_REF,
        M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical experiment-run-row / environment-fingerprint controls packet.
pub fn seeded_experiment_run_row_environment_fingerprint_controls(
) -> ExperimentRunRowEnvironmentFingerprintControlsPacket {
    ExperimentRunRowEnvironmentFingerprintControlsPacket::new(
        ExperimentRunRowEnvironmentFingerprintControlsPacketInput {
            packet_id: EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_PACKET_ID.to_owned(),
            surface_label:
                "M5 experiment run rows and environment fingerprint cards: run origin, commit/workspace revision, execution origin, outcome, and captured-environment truth across claimed notebook and data surfaces"
                    .to_owned(),
            run_rows: run_rows(),
            fingerprint_cards: fingerprint_cards(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
            experiment_review: experiment_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights an imported run row that must never read as a first-party run.
/// Every origin class, run origin kind, and status state stays covered so the fixture
/// validates on its own.
pub fn seeded_experiment_run_row_environment_fingerprint_controls_run_row_imported(
) -> ExperimentRunRowEnvironmentFingerprintControlsPacket {
    let mut packet = seeded_experiment_run_row_environment_fingerprint_controls();
    packet.packet_id =
        "m5-experiment-run-row-environment-fingerprint-controls:fixture:run-row-imported"
            .to_owned();
    packet.surface_label =
        "M5 experiment run rows: an imported run never reads as a first-party run".to_owned();
    packet
}

/// Scenario fixture: spotlights an uncaptured environment fingerprint card that must never
/// read as captured. Every capture class, fingerprint scope class, and fingerprint state stays
/// covered so the fixture validates on its own.
pub fn seeded_experiment_run_row_environment_fingerprint_controls_fingerprint_card_uncaptured(
) -> ExperimentRunRowEnvironmentFingerprintControlsPacket {
    let mut packet = seeded_experiment_run_row_environment_fingerprint_controls();
    packet.packet_id =
        "m5-experiment-run-row-environment-fingerprint-controls:fixture:fingerprint-card-uncaptured"
            .to_owned();
    packet.surface_label =
        "M5 environment fingerprint cards: an uncaptured environment never reads as captured"
            .to_owned();
    packet
}
