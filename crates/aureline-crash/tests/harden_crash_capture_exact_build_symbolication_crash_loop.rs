//! Protected tests for hardened crash capture, exact-build symbolication,
//! crash-loop detection, and evidence preview/export.
//!
//! These tests exercise the M4 stable lane owned by
//! `aureline_crash::harden_crash_capture_exact_build_symbolication_crash_loop`.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_crash::{
    detect_crash_loop, export_evidence, preview_evidence, CrashIncidentTrail,
    CrashIncidentTrailInputs, CrashLoopDetectionState, CrashLoopScenarioClass, CrashLoopSignal,
    CrashLoopSignalInputs, EvidenceExportInputs, EvidenceExportPacket, EvidencePreview,
    EvidencePreviewInputs, ExportRedactionClass, HardenedCrashCaptureEvaluator,
    RecoveryLadderHookClass, SymbolicationState, EVIDENCE_EXPORT_PACKET_RECORD_KIND,
    EVIDENCE_PREVIEW_RECORD_KIND, HARDEN_CRASH_CAPTURE_DOC_REF, HARDEN_CRASH_CAPTURE_SCHEMA_REF,
    HARDEN_CRASH_CAPTURE_SCHEMA_VERSION,
};

const GENERATED_AT: &str = "2026-06-02T10:00:00Z";
const ALPHA_CHANNEL_REF: &str = "alpha-channel:preview:design-partner-linux";
const SUPPORT_BUNDLE_MANIFEST_REF: &str =
    "support.bundle.manifest.alpha_preview.renderer_panic.local_review";
const SUPPORT_PREVIEW_SNAPSHOT_REF: &str =
    "preview-snapshot:support-bundle:alpha-preview:renderer-panic";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root()
        .join("fixtures")
        .join("support")
        .join("m4")
        .join("harden_crash_capture_exact_build_symbolication_crash_loop")
}

fn load_json<T>(name: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn incident_trail_fixture() -> CrashIncidentTrail {
    let envelope = load_fixture_json::<aureline_crash::CrashEnvelope>(
        "fixtures/support/incident_trail_alpha/crash_envelope.json",
    );
    let dump_manifest = load_fixture_json::<aureline_crash::CrashDumpManifest>(
        "fixtures/support/incident_trail_alpha/crash_dump_manifest.json",
    );
    let report = Some(load_fixture_json::<aureline_crash::SymbolicationReport>(
        "fixtures/support/incident_trail_alpha/symbolication_report_exact.json",
    ));

    CrashIncidentTrail::from_inputs(CrashIncidentTrailInputs {
        incident_trail_id: "crash-incident-trail:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        alpha_channel_ref: ALPHA_CHANNEL_REF.into(),
        crash_envelope: envelope,
        crash_dump_manifest: dump_manifest,
        symbolication_report: report,
        support_bundle_manifest_ref: Some(SUPPORT_BUNDLE_MANIFEST_REF.into()),
        support_preview_snapshot_ref: Some(SUPPORT_PREVIEW_SNAPSHOT_REF.into()),
    })
}

fn load_fixture_json<T>(path_suffix: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let path = repo_root().join(path_suffix);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn exact_build_identity() -> String {
    "build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:a1b2c3d4".into()
}

fn make_trails(count: usize, symbolication_state: SymbolicationState) -> Vec<CrashIncidentTrail> {
    make_trails_for_fault_domain(count, symbolication_state, "fd.renderer.crash_loop_alpha")
}

fn make_trails_for_fault_domain(
    count: usize,
    symbolication_state: SymbolicationState,
    fault_domain_id: &str,
) -> Vec<CrashIncidentTrail> {
    let mut trails = Vec::with_capacity(count);
    for i in 0..count {
        let mut trail = incident_trail_fixture();
        trail.incident_trail_id =
            format!("crash-incident-trail:alpha-preview:renderer-panic:{i:04}");
        trail.symbolication_state = symbolication_state;
        trail.primary_exact_build_identity_ref = exact_build_identity();
        trail.fault_domain_id = fault_domain_id.into();
        trails.push(trail);
    }
    trails
}

// ---------------------------------------------------------------------------
// Crash-loop detection
// ---------------------------------------------------------------------------

#[test]
fn no_loop_when_trails_are_within_budget() {
    let trails = make_trails(1, SymbolicationState::Exact);
    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 3,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert_eq!(signal.detection_state, CrashLoopDetectionState::NoLoop);
    assert_eq!(signal.strike_count, 1);
    assert!(!signal.is_confirmed_or_escalating());
}

#[test]
fn emerging_loop_when_two_crashes_below_budget() {
    let trails = make_trails(2, SymbolicationState::Exact);
    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 3,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert_eq!(signal.detection_state, CrashLoopDetectionState::Emerging);
    assert_eq!(signal.strike_count, 2);
}

#[test]
fn confirmed_loop_when_budget_is_met() {
    let trails = make_trails(3, SymbolicationState::Exact);
    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 3,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert_eq!(signal.detection_state, CrashLoopDetectionState::Confirmed);
    assert_eq!(signal.strike_count, 3);
    assert!(signal.is_confirmed_or_escalating());
    assert!(signal.is_exact_build_consistent());
}

#[test]
fn escalating_loop_with_recovery_attempts() {
    let mut trails = make_trails(5, SymbolicationState::Exact);
    // Enable recovery actions on a few trails so recovery_attempt_count > 0
    for trail in trails.iter_mut().take(3) {
        for action in &mut trail.next_safe_actions {
            if action.action_ref.starts_with("recovery_action:") {
                action.enabled = true;
            }
        }
    }

    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 2,
        escalation_threshold: 4,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert_eq!(signal.detection_state, CrashLoopDetectionState::Escalating);
    assert!(signal.recovery_attempt_count > 0);
}

#[test]
fn build_mismatch_flagged_when_trails_differ() {
    let mut trails = make_trails(3, SymbolicationState::Exact);
    trails[2].symbolication_state = SymbolicationState::BuildMismatch;

    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 2,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert!(signal.any_build_mismatch_observed);
    assert!(!signal.is_exact_build_consistent());
}

#[test]
fn empty_trails_produce_no_signal() {
    let inputs = CrashLoopSignalInputs {
        incident_trails: vec![],
        strike_budget: 2,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    assert!(detect_crash_loop(&inputs).is_none());
}

#[test]
fn mismatched_identity_produces_no_signal() {
    let trails = make_trails(3, SymbolicationState::Exact);
    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 2,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: "build-id:mismatch".into(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    assert!(detect_crash_loop(&inputs).is_none());
}

#[test]
fn signal_carries_safe_hooks_by_default() {
    let trails = make_trails(3, SymbolicationState::Exact);
    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 2,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.renderer.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert!(
        signal.available_hooks.iter().any(|h| {
            h.hook_class == RecoveryLadderHookClass::SafeModeMinimalProfile
                && h.preserves_user_state
        }),
        "safe_mode hook must preserve user state"
    );
    assert!(
        signal.available_hooks.iter().any(|h| {
            h.hook_class == RecoveryLadderHookClass::ExportEvidence && h.preserves_user_state
        }),
        "export_evidence hook must preserve user state"
    );
}

// ---------------------------------------------------------------------------
// Evidence preview
// ---------------------------------------------------------------------------

#[test]
fn preview_lists_included_and_omitted_items() {
    let trail = incident_trail_fixture();
    let preview = preview_evidence(&EvidencePreviewInputs {
        preview_id: "crash-evidence-preview:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        incident_trail: trail,
        crash_loop_signal: None,
        redaction_class: ExportRedactionClass::MetadataSafeDefault,
    });

    assert_eq!(preview.record_kind, EVIDENCE_PREVIEW_RECORD_KIND);
    assert_eq!(preview.schema_version, HARDEN_CRASH_CAPTURE_SCHEMA_VERSION);
    assert!(!preview.raw_dump_exported);
    assert!(
        preview
            .included_items
            .iter()
            .any(|i| i.item_kind == "crash_envelope"),
        "preview must include crash_envelope"
    );
    assert!(
        preview
            .omitted_items
            .iter()
            .any(|i| i.item_kind == "raw_dump_body"),
        "preview must omit raw_dump_body"
    );
    assert_eq!(preview.doc_ref, HARDEN_CRASH_CAPTURE_DOC_REF);
    assert_eq!(preview.schema_ref, HARDEN_CRASH_CAPTURE_SCHEMA_REF);
}

#[test]
fn preview_with_crash_loop_signal_embeds_signal() {
    let trail = incident_trail_fixture();
    let signal = make_crash_loop_signal();
    let preview = preview_evidence(&EvidencePreviewInputs {
        preview_id: "crash-evidence-preview:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        incident_trail: trail,
        crash_loop_signal: Some(signal),
        redaction_class: ExportRedactionClass::MetadataSafeDefault,
    });

    assert!(preview.crash_loop_signal_ref.is_some());
    assert!(
        preview
            .included_items
            .iter()
            .any(|i| i.item_kind == "crash_loop_signal"),
        "preview must include crash_loop_signal when provided"
    );
}

// ---------------------------------------------------------------------------
// Evidence export packet
// ---------------------------------------------------------------------------

#[test]
fn export_packet_is_metadata_safe_by_construction() {
    let trail = incident_trail_fixture();
    let packet = export_evidence(&EvidenceExportInputs {
        packet_id: "crash-evidence-export:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        incident_trail: trail,
        crash_loop_signal: None,
        repair_transaction_id: None,
        redaction_class: ExportRedactionClass::MetadataSafeDefault,
    });

    assert_eq!(packet.record_kind, EVIDENCE_EXPORT_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, HARDEN_CRASH_CAPTURE_SCHEMA_VERSION);
    assert!(!packet.raw_dump_exported);
    assert!(packet.raw_private_material_excluded);
    assert!(packet.ambient_authority_excluded);
    assert!(!packet.export_items.is_empty());
    assert!(!packet.chain_of_custody.is_empty());
    assert_eq!(packet.doc_ref, HARDEN_CRASH_CAPTURE_DOC_REF);
    assert_eq!(packet.schema_ref, HARDEN_CRASH_CAPTURE_SCHEMA_REF);
}

#[test]
fn export_packet_links_repair_transaction_id() {
    let trail = incident_trail_fixture();
    let packet = export_evidence(&EvidenceExportInputs {
        packet_id: "crash-evidence-export:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        incident_trail: trail,
        crash_loop_signal: None,
        repair_transaction_id: Some("repair-tx:alpha:0001".into()),
        redaction_class: ExportRedactionClass::MetadataSafeDefault,
    });

    assert_eq!(
        packet.repair_transaction_id.as_deref(),
        Some("repair-tx:alpha:0001")
    );
    assert_eq!(packet.chain_of_custody.len(), 2);
    assert!(packet
        .chain_of_custody
        .iter()
        .any(|e| e.action == "repair_transaction_linked"));
}

#[test]
fn serde_round_trip_preserves_export_packet() {
    let trail = incident_trail_fixture();
    let packet = export_evidence(&EvidenceExportInputs {
        packet_id: "crash-evidence-export:alpha-preview:renderer-panic:0001".into(),
        generated_at: GENERATED_AT.into(),
        incident_trail: trail,
        crash_loop_signal: None,
        repair_transaction_id: None,
        redaction_class: ExportRedactionClass::MetadataSafeDefault,
    });

    let json = serde_json::to_string(&packet).expect("serialize");
    let restored: EvidenceExportPacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored, packet);
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

#[test]
fn evaluator_accepts_valid_crash_loop_signal() {
    let signal = load_json::<CrashLoopSignal>("crash_loop_signal.json");
    let evaluator = HardenedCrashCaptureEvaluator::new();
    let report = evaluator.validate_crash_loop_signal(&signal);
    assert!(report.is_valid(), "violations: {:?}", report.violations);
}

#[test]
fn evaluator_rejects_signal_with_zero_strikes() {
    let mut signal = load_json::<CrashLoopSignal>("crash_loop_signal.json");
    signal.strike_count = 0;

    let evaluator = HardenedCrashCaptureEvaluator::new();
    let report = evaluator.validate_crash_loop_signal(&signal);
    assert!(!report.is_valid());
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id.contains("strike_count_zero")));
}

#[test]
fn evaluator_rejects_signal_without_safe_hooks() {
    let mut signal = load_json::<CrashLoopSignal>("crash_loop_signal.json");
    for hook in &mut signal.available_hooks {
        hook.preserves_user_state = false;
    }

    let evaluator = HardenedCrashCaptureEvaluator::new();
    let report = evaluator.validate_crash_loop_signal(&signal);
    assert!(!report.is_valid());
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id.contains("no_safe_hook")));
}

#[test]
fn evaluator_accepts_valid_evidence_preview() {
    let preview = load_json::<EvidencePreview>("evidence_preview.json");
    let evaluator = HardenedCrashCaptureEvaluator::new();
    let report = evaluator.validate_evidence_preview(&preview);
    assert!(report.is_valid(), "violations: {:?}", report.violations);
}

#[test]
fn evaluator_rejects_preview_with_raw_dump_exported() {
    let mut preview = load_json::<EvidencePreview>("evidence_preview.json");
    preview.raw_dump_exported = true;

    let evaluator = HardenedCrashCaptureEvaluator::new();
    let report = evaluator.validate_evidence_preview(&preview);
    assert!(!report.is_valid());
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id.contains("raw_dump_exported")));
}

#[test]
fn evaluator_accepts_valid_export_packet() {
    let packet = load_json::<EvidenceExportPacket>("evidence_export_packet.json");
    let evaluator = HardenedCrashCaptureEvaluator::new();
    let report = evaluator.validate_evidence_export_packet(&packet);
    assert!(report.is_valid(), "violations: {:?}", report.violations);
}

#[test]
fn evaluator_rejects_packet_with_empty_export_items() {
    let mut packet = load_json::<EvidenceExportPacket>("evidence_export_packet.json");
    packet.export_items.clear();

    let evaluator = HardenedCrashCaptureEvaluator::new();
    let report = evaluator.validate_evidence_export_packet(&packet);
    assert!(!report.is_valid());
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id.contains("export_items_empty")));
}

// ---------------------------------------------------------------------------
// Boundary refs
// ---------------------------------------------------------------------------

#[test]
fn boundary_schema_and_doc_refs_exist_on_disk() {
    let schema_path = repo_root().join(HARDEN_CRASH_CAPTURE_SCHEMA_REF);
    let doc_path = repo_root().join(HARDEN_CRASH_CAPTURE_DOC_REF);

    assert!(
        schema_path.exists(),
        "schema must exist: {}",
        schema_path.display()
    );
    assert!(doc_path.exists(), "doc must exist: {}", doc_path.display());
}

// ---------------------------------------------------------------------------
// Seeded scenarios
// ---------------------------------------------------------------------------

#[test]
fn all_scenario_classes_have_stable_tokens() {
    let all = CrashLoopScenarioClass::all();
    assert_eq!(all.len(), 5);
    for scenario in &all {
        assert!(!scenario.as_str().is_empty());
    }
}

#[test]
fn extension_scenario_includes_disable_hook() {
    let mut trails = make_trails_for_fault_domain(
        3,
        SymbolicationState::Exact,
        "fd.extension_host.crash_loop_alpha",
    );
    // Disable restore actions so the classifier does not route into restore-replay.
    for trail in trails.iter_mut() {
        trail.next_safe_actions.iter_mut().for_each(|a| {
            if a.action_ref.contains("open_without_restore") {
                a.enabled = false;
            }
        });
    }

    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 2,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.extension_host.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert_eq!(
        signal.scenario_class,
        CrashLoopScenarioClass::ExtensionHostRestartBudgetExceeded
    );
    assert!(
        signal
            .available_hooks
            .iter()
            .any(|h| { h.hook_class == RecoveryLadderHookClass::DisableRecentExtension }),
        "extension-host scenario must offer disable_recent_extension hook"
    );
}

#[test]
fn restore_replay_scenario_includes_cache_reset_hook() {
    let mut trails =
        make_trails_for_fault_domain(4, SymbolicationState::Exact, "fd.restore.crash_loop_alpha");
    for trail in trails.iter_mut() {
        trail.next_safe_actions.iter_mut().for_each(|a| {
            if a.action_ref.contains("open_without_restore") {
                a.enabled = true;
            }
        });
    }

    let inputs = CrashLoopSignalInputs {
        incident_trails: trails,
        strike_budget: 2,
        escalation_threshold: 5,
        primary_exact_build_identity_ref: exact_build_identity(),
        fault_domain_id: "fd.restore.crash_loop_alpha".into(),
    };

    let signal = detect_crash_loop(&inputs).expect("signal produced");
    assert!(
        signal
            .available_hooks
            .iter()
            .any(|h| { h.hook_class == RecoveryLadderHookClass::ResetEphemeralCache }),
        "restore-replay scenario must offer reset_ephemeral_cache hook"
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_crash_loop_signal() -> CrashLoopSignal {
    load_json("crash_loop_signal.json")
}
