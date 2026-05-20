//! Protected tests for the crash-loop recovery center beta evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::crash_loop_center::{
    load_signal, CrashLoopRecoveryCenterBeta, CrashLoopSignal, EvidenceDataClass,
    EvidenceEntryClass, FaultDomainClass, RecoveryChoiceClass, RestoreClass,
    SessionSensitivityClass, CRASH_LOOP_RECOVERY_CENTER_RECORD_KIND, CRASH_LOOP_RECOVERY_DOC_REF,
    CRASH_LOOP_RECOVERY_SCHEMA_REF, CRASH_LOOP_RECOVERY_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    signal_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/recovery/m3/crash_loop_center")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_signals() -> Vec<CrashLoopSignal> {
    load_manifest()
        .signal_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_signal(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

fn find_signal(signal_id: &str) -> CrashLoopSignal {
    load_signals()
        .into_iter()
        .find(|signal| signal.signal_id == signal_id)
        .unwrap_or_else(|| panic!("signal {signal_id} exists"))
}

#[test]
fn every_signal_routes_into_a_visible_bounded_center_without_user_state_deletion() {
    let beta = CrashLoopRecoveryCenterBeta::new();
    let signals = load_signals();
    assert_eq!(signals.len(), 5);

    for signal in &signals {
        let center = beta
            .evaluate(signal)
            .unwrap_or_else(|err| panic!("{} failed: {err}", signal.signal_id));

        assert_eq!(center.record_kind, CRASH_LOOP_RECOVERY_CENTER_RECORD_KIND);
        // Acceptance: repeated crashes no longer leave an invisible loop.
        assert!(center.silent_restart_suppressed);
        assert!(center.is_bounded_recovery_surface());
        // Acceptance: crash id, build id, restore class, and fault domain stay visible.
        assert_eq!(center.crash_id, signal.crash_id);
        assert_eq!(center.build_id, signal.build_id);
        assert_eq!(center.restore_class, signal.restore_class);
        assert_eq!(center.suspected_fault_domain, signal.suspected_fault_domain);
        assert_eq!(center.last_reopen_mode, signal.last_reopen_mode);
        assert!(!center.crash_id.is_empty());
        assert!(!center.build_id.is_empty());
        // Guardrail: never destructive, never deletes user-owned state.
        assert!(!center.destructive_cleanup_suggested);
        for choice in &center.recovery_choices {
            assert!(!choice.deletes_user_owned_state);
            assert!(choice.narrower_than_full_reset);
            assert!(choice.accessibility.keyboard_complete);
            assert!(!choice.accessibility.screen_reader_label.is_empty());
        }
        // Accessibility: focus order is unique across choices and entries.
        let focus_orders = center
            .recovery_choices
            .iter()
            .map(|choice| choice.accessibility.focus_order)
            .chain(
                center
                    .evidence_entry_points
                    .iter()
                    .map(|entry| entry.accessibility.focus_order),
            )
            .collect::<Vec<_>>();
        let unique = focus_orders.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(
            unique.len(),
            focus_orders.len(),
            "focus orders must be unique"
        );
    }
}

#[test]
fn every_center_offers_the_required_bounded_choice_set_without_a_generic_retry() {
    let beta = CrashLoopRecoveryCenterBeta::new();
    for signal in load_signals() {
        let center = beta.evaluate(&signal).expect("signal evaluates");
        for required in [
            RecoveryChoiceClass::EnterSafeMode,
            RecoveryChoiceClass::OpenWithoutRestore,
            RecoveryChoiceClass::OpenLogs,
            RecoveryChoiceClass::ExportCrashManifest,
            RecoveryChoiceClass::ReportIssue,
        ] {
            assert!(
                center.choice(required).is_some(),
                "{} missing required choice {:?}",
                signal.signal_id,
                required
            );
        }
    }
}

#[test]
fn safe_mode_and_open_without_restore_honor_no_silent_rerun_for_privileged_or_mutating_sessions() {
    let beta = CrashLoopRecoveryCenterBeta::new();

    // Privileged/remote session: re-entry requires review and never silently re-runs.
    let privileged = beta
        .evaluate(&find_signal("crash_loop_center.startup_extension_suspect"))
        .expect("signal evaluates");
    assert_eq!(
        privileged.signal_ref,
        "crash_loop_center.startup_extension_suspect"
    );
    for class in [
        RecoveryChoiceClass::EnterSafeMode,
        RecoveryChoiceClass::OpenWithoutRestore,
    ] {
        let choice = privileged.choice(class).expect("re-entry choice exists");
        assert!(
            choice.command.no_silent_rerun,
            "{class:?} must not re-run silently"
        );
        assert!(
            choice.command.requires_review,
            "{class:?} must require review"
        );
        assert!(choice.command.requires_explicit_confirmation);
    }

    // Read-only session: re-entry still never silent, but does not force review.
    let read_only = beta
        .evaluate(&find_signal("crash_loop_center.restore_replay_unsafe"))
        .expect("signal evaluates");
    let safe_mode = read_only
        .choice(RecoveryChoiceClass::EnterSafeMode)
        .expect("safe mode choice exists");
    assert!(safe_mode.command.no_silent_rerun);
    assert!(safe_mode.command.requires_explicit_confirmation);
    assert!(!safe_mode.command.requires_review);
}

#[test]
fn suspected_extension_and_profile_changes_get_targeted_reversible_disable_choices() {
    let beta = CrashLoopRecoveryCenterBeta::new();

    let extension = beta
        .evaluate(&find_signal("crash_loop_center.startup_extension_suspect"))
        .expect("signal evaluates");
    let disable_ext = extension
        .choice(RecoveryChoiceClass::DisableRecentlyChangedExtension)
        .expect("disable-extension choice exists");
    assert_eq!(
        disable_ext.targets_recent_change_ref.as_deref(),
        Some("change.extension.alpha-linter.update")
    );
    assert!(!disable_ext.deletes_user_owned_state);

    let profile = beta
        .evaluate(&find_signal("crash_loop_center.reopen_profile_suspect"))
        .expect("signal evaluates");
    let disable_profile_or_layout = profile
        .recovery_choices
        .iter()
        .filter(|choice| {
            choice.choice_class == RecoveryChoiceClass::DisableRecentlyChangedProfileOrLayout
        })
        .collect::<Vec<_>>();
    assert_eq!(
        disable_profile_or_layout.len(),
        2,
        "both the profile and layout suspects get a targeted disable choice"
    );
    let targets = disable_profile_or_layout
        .iter()
        .filter_map(|choice| choice.targets_recent_change_ref.clone())
        .collect::<BTreeSet<_>>();
    assert!(targets.contains("change.profile.web-team.switch"));
    assert!(targets.contains("change.layout.split-grid"));

    // A signal with no narrowed suspect offers no disable choice.
    let unknown = beta
        .evaluate(&find_signal("crash_loop_center.runtime_host_unknown"))
        .expect("signal evaluates");
    assert_eq!(unknown.suspected_fault_domain, FaultDomainClass::Unknown);
    assert!(unknown
        .choice(RecoveryChoiceClass::DisableRecentlyChangedExtension)
        .is_none());
    assert!(unknown
        .choice(RecoveryChoiceClass::DisableRecentlyChangedProfileOrLayout)
        .is_none());
}

#[test]
fn recovered_drafts_and_rollbackable_state_keep_distinct_evidence_entry_points() {
    let beta = CrashLoopRecoveryCenterBeta::new();
    let center = beta
        .evaluate(&find_signal("crash_loop_center.restore_replay_unsafe"))
        .expect("signal evaluates");

    assert_eq!(center.restore_class, RestoreClass::EvidenceOnly);
    let classes = center
        .evidence_entry_points
        .iter()
        .map(|entry| entry.entry_class)
        .collect::<BTreeSet<_>>();
    assert!(classes.contains(&EvidenceEntryClass::RecoveredDraft));
    assert!(classes.contains(&EvidenceEntryClass::RollbackableState));
    assert!(classes.contains(&EvidenceEntryClass::LocalHistoryTimeline));
    // Entry points are distinct from the generic recovery choices.
    for entry in &center.evidence_entry_points {
        assert!(!entry.preserves.is_empty());
        assert!(entry.accessibility.keyboard_complete);
        assert!(!entry.command.command_id.is_empty());
    }
}

#[test]
fn support_packet_keeps_identity_visible_and_excludes_private_content() {
    let beta = CrashLoopRecoveryCenterBeta::new();
    let signal = find_signal("crash_loop_center.startup_extension_suspect");
    let center = beta.evaluate(&signal).expect("signal evaluates");
    let packet = beta.support_packet("support:crash-loop-center:startup", &center);

    assert_eq!(
        packet.record_kind,
        CRASH_LOOP_RECOVERY_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(packet.doc_ref, CRASH_LOOP_RECOVERY_DOC_REF);
    assert_eq!(packet.schema_ref, CRASH_LOOP_RECOVERY_SCHEMA_REF);
    // Acceptance: crash and build identity remain in the support packet.
    assert_eq!(packet.crash_id, signal.crash_id);
    assert_eq!(packet.build_id, signal.build_id);
    assert_eq!(packet.restore_class, signal.restore_class);
    assert_eq!(packet.suspected_fault_domain, signal.suspected_fault_domain);
    assert!(packet.is_export_safe());
    assert!(packet.raw_private_material_excluded);
    assert!(packet.ambient_authority_excluded);
    assert!(!packet.destructive_cleanup_suggested);
    assert!(!packet.evidence_refs.is_empty());
    // The plaintext view stays metadata-only and screen-reader-legible.
    let rendered = packet.render_support_summary();
    assert!(rendered.contains(&signal.crash_id));
    assert!(rendered.contains(&signal.build_id));
    assert!(rendered.contains("enter_safe_mode"));
}

#[test]
fn malformed_signal_is_rejected_with_explicit_violations() {
    let beta = CrashLoopRecoveryCenterBeta::new();
    let mut signal = find_signal("crash_loop_center.runtime_host_unknown");
    signal.crash_id.clear();
    signal.build_id.clear();
    signal.doctor_finding_ref = "finding.not.namespaced".to_owned();
    signal.evidence[0].data_class = EvidenceDataClass::SecretBearing;

    let report = beta
        .evaluate(&signal)
        .expect_err("malformed signal must be rejected");
    assert!(report.contains("crash_loop.crash_id_missing"));
    assert!(report.contains("crash_loop.build_id_missing"));
    assert!(report.contains("crash_loop.doctor_finding_ref_missing"));
    assert!(report.contains("crash_loop.evidence_private_data_class"));
}

#[test]
fn budget_breach_must_be_proven_before_opening_the_center() {
    let beta = CrashLoopRecoveryCenterBeta::new();
    let mut signal = find_signal("crash_loop_center.startup_extension_suspect");
    // A budget-breach trigger without an exhausted budget must not open the center.
    signal.strike_count = 1;
    signal.strike_budget = 3;

    let report = beta
        .evaluate(&signal)
        .expect_err("unproven breach must be rejected");
    assert!(report.contains("crash_loop.budget_breach_not_proven"));
}

#[test]
fn explicit_user_request_opens_the_center_without_a_budget_breach() {
    let beta = CrashLoopRecoveryCenterBeta::new();
    let signal = find_signal("crash_loop_center.explicit_user_request");
    assert!(signal.strike_count < signal.strike_budget);
    let center = beta
        .evaluate(&signal)
        .expect("explicit request opens the center");
    assert!(center.silent_restart_suppressed);
    // The changed setting is offered as a reversible disable suspect.
    assert!(center
        .choice(RecoveryChoiceClass::DisableRecentlyChangedProfileOrLayout)
        .is_some());
}

#[test]
fn vocabulary_tokens_round_trip_fixture_strings() {
    let restore: RestoreClass = serde_yaml::from_str("evidence_only").expect("token parses");
    assert_eq!(restore.as_str(), "evidence_only");
    let domain: FaultDomainClass =
        serde_yaml::from_str("workspace_profile_or_layout").expect("token parses");
    assert_eq!(domain.as_str(), "workspace_profile_or_layout");
    let session: SessionSensitivityClass =
        serde_yaml::from_str("privileged_or_remote").expect("token parses");
    assert!(session.requires_no_silent_rerun());
    let choice: RecoveryChoiceClass =
        serde_yaml::from_str("enter_safe_mode").expect("token parses");
    assert!(choice.is_session_reentry());
    assert_eq!(choice.command_id(), "command.recovery.enter_safe_mode");
}
