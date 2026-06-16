//! Unit tests for the native-desktop matrix builder and validator.

use super::*;

fn satisfied(
    control: NativeDesktopControl,
    entry_id: &str,
    anchor: &str,
) -> NativeDesktopControlBinding {
    NativeDesktopControlBinding {
        control,
        status: NativeDesktopControlStatus::Satisfied,
        failure_mode: None,
        evidence_pack_ref: Some(format!("drill:{entry_id}:{}", control.as_str())),
        recovery_path_ref: control
            .requires_recovery_path()
            .then(|| format!("recovery:{entry_id}:{}", control.as_str())),
        durable_object_ref: control.requires_durable_object().then(|| anchor.to_owned()),
        narrowing_reason: None,
        note: None,
    }
}

fn full_bindings(entry_id: &str, anchor: &str) -> Vec<NativeDesktopControlBinding> {
    NativeDesktopControl::required_controls()
        .into_iter()
        .map(|control| satisfied(control, entry_id, anchor))
        .collect()
}

fn clean_descriptor(entry_id: &str, kind: NativeDesktopSurfaceKind) -> NativeDesktopDescriptor {
    NativeDesktopDescriptor {
        entry_id: entry_id.to_owned(),
        surface_kind: kind,
        descriptor_revision_ref: format!("{entry_id}:rev"),
        primary_label_ref: format!("{entry_id}:label"),
        channel_build_owner_ref: format!("{entry_id}:owner"),
        ownership_kind: NativeDesktopOwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: format!("{entry_id}:trust"),
        reopen_anchor_ref: format!("{entry_id}:anchor"),
        continuity_note: "preserves context".to_owned(),
        degraded_state_vocabulary: vec!["Reopen in this profile".to_owned()],
        claimed_platforms: NativeDesktopPlatform::all().to_vec(),
        evidence_freshness: NativeDesktopEvidenceFreshness::Fresh,
        evidence_captured_at: "2026-06-16T00:00:00Z".to_owned(),
        downgrade_rule_ref: "downgrade:rule".to_owned(),
        marketed: true,
        registered_on_native_desktop_harness: true,
    }
}

#[test]
fn seeded_matrix_is_clean_and_validates() {
    let report = seeded_native_desktop_matrix();
    assert!(report.report_clean, "seeded matrix must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_native_desktop_matrix(&report).expect("seeded matrix must validate");
}

#[test]
fn seeded_matrix_covers_every_kind_and_control() {
    let report = seeded_native_desktop_matrix();
    assert!(
        report.every_kind_present(),
        "every required kind must be present"
    );
    assert!(
        report.every_control_satisfied(),
        "every required control must be satisfied by at least one surface"
    );
    assert_eq!(
        report.required_surface_kinds.len(),
        NativeDesktopSurfaceKind::required_kinds().len()
    );
    assert_eq!(
        report.required_controls.len(),
        NativeDesktopControl::required_controls().len()
    );
}

#[test]
fn seeded_matrix_binds_every_control_on_every_surface() {
    let report = seeded_native_desktop_matrix();
    for entry in &report.entries {
        assert_eq!(
            entry.bindings.len(),
            NativeDesktopControl::required_controls().len(),
            "{} must bind every control",
            entry.descriptor.entry_id
        );
    }
}

#[test]
fn signal_controls_are_only_satisfied_on_signal_surfaces() {
    let report = seeded_native_desktop_matrix();
    for entry in &report.entries {
        for binding in &entry.bindings {
            let is_signal_control = matches!(
                binding.control,
                NativeDesktopControl::SignalDurability | NativeDesktopControl::NotificationPrivacy
            );
            if is_signal_control && binding.status == NativeDesktopControlStatus::Satisfied {
                assert!(
                    entry.descriptor.surface_kind.is_signal_surface(),
                    "{} satisfies a signal control without being a signal surface",
                    entry.descriptor.entry_id
                );
            }
        }
    }
}

#[test]
fn satisfied_recovery_controls_carry_recovery_paths() {
    let report = seeded_native_desktop_matrix();
    for entry in &report.entries {
        for binding in &entry.bindings {
            if binding.status == NativeDesktopControlStatus::Satisfied
                && binding.control.requires_recovery_path()
            {
                assert!(
                    binding.recovery_path_ref.is_some(),
                    "{} / {} must carry a recovery path",
                    entry.descriptor.entry_id,
                    binding.control.as_str()
                );
            }
            if binding.status == NativeDesktopControlStatus::Satisfied
                && binding.control.requires_durable_object()
            {
                assert!(
                    binding.durable_object_ref.is_some(),
                    "{} / signal_durability must carry a durable object",
                    entry.descriptor.entry_id
                );
            }
        }
    }
}

#[test]
fn failed_controls_emit_distinct_failure_classes() {
    let entry_id = "entry:test.failures";
    let mut bindings = full_bindings(entry_id, "anchor");
    // Fail every control and assert each yields its own distinct class.
    for binding in &mut bindings {
        binding.status = NativeDesktopControlStatus::Failed;
        binding.failure_mode = Some(binding.control.canonical_failure_mode());
        binding.evidence_pack_ref = None;
        binding.recovery_path_ref = None;
        binding.durable_object_ref = None;
    }
    let row = build_native_desktop_row(
        clean_descriptor(entry_id, NativeDesktopSurfaceKind::OsNotification),
        bindings,
    );
    let classes: std::collections::BTreeSet<&str> = row
        .blocking_findings
        .iter()
        .map(|finding| finding.class_token())
        .collect();
    for expected in [
        "trust_evaluation_bypassed",
        "hidden_handler_takeover",
        "wrong_target_no_recovery",
        "unavailable_path_silent_loss",
        "policy_block_unsafe",
        "transient_poll_signal",
        "privacy_unsafe_notification",
    ] {
        assert!(
            classes.contains(expected),
            "missing distinct failure class {expected}"
        );
    }
}

#[test]
fn declared_failure_mode_drift_is_a_blocker() {
    let entry_id = "entry:test.drift";
    let mut bindings = full_bindings(entry_id, "anchor");
    // A trust binding failing but declaring the wrong failure mode.
    bindings[0].status = NativeDesktopControlStatus::Failed;
    bindings[0].failure_mode = Some(NativeDesktopFailureMode::PrivacyUnsafeNotification);
    bindings[0].evidence_pack_ref = None;
    let row = build_native_desktop_row(
        clean_descriptor(entry_id, NativeDesktopSurfaceKind::ProtocolHandler),
        bindings,
    );
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "failure_mode_drift"));
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "trust_evaluation_bypassed"));
}

#[test]
fn narrowed_control_without_reason_is_a_blocker() {
    let entry_id = "entry:test.narrow";
    let mut bindings = full_bindings(entry_id, "anchor");
    bindings[5].status = NativeDesktopControlStatus::NotApplicable;
    bindings[5].evidence_pack_ref = None;
    bindings[5].durable_object_ref = None;
    bindings[5].narrowing_reason = None;
    let row = build_native_desktop_row(
        clean_descriptor(entry_id, NativeDesktopSurfaceKind::SystemOpen),
        bindings,
    );
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "missing_narrowing_reason"));
}

#[test]
fn satisfied_control_without_evidence_pack_is_a_blocker() {
    let entry_id = "entry:test.evidence";
    let mut bindings = full_bindings(entry_id, "anchor");
    bindings[0].evidence_pack_ref = None;
    let row = build_native_desktop_row(
        clean_descriptor(entry_id, NativeDesktopSurfaceKind::SystemOpen),
        bindings,
    );
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "missing_evidence_pack"));
}

#[test]
fn missing_required_control_is_a_blocker() {
    let entry_id = "entry:test.missing_control";
    let mut bindings = full_bindings(entry_id, "anchor");
    bindings.remove(0);
    let row = build_native_desktop_row(
        clean_descriptor(entry_id, NativeDesktopSurfaceKind::SystemOpen),
        bindings,
    );
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "missing_required_control"));
}

#[test]
fn surface_off_harness_and_missing_owner_are_blockers() {
    let entry_id = "entry:test.descriptor";
    let mut descriptor = clean_descriptor(entry_id, NativeDesktopSurfaceKind::SystemOpen);
    descriptor.registered_on_native_desktop_harness = false;
    descriptor.channel_build_owner_ref = String::new();
    descriptor.degraded_state_vocabulary = vec![];
    let row = build_native_desktop_row(descriptor, full_bindings(entry_id, "anchor"));
    let classes: std::collections::BTreeSet<&str> = row
        .blocking_findings
        .iter()
        .map(|finding| finding.class_token())
        .collect();
    assert!(classes.contains("surface_not_on_harness"));
    assert!(classes.contains("missing_channel_build_owner"));
    assert!(classes.contains("missing_degraded_state_vocabulary"));
}

#[test]
fn stale_evidence_on_marketed_surface_is_a_blocker_and_narrowable() {
    let entry_id = "entry:test.stale";
    let mut descriptor = clean_descriptor(entry_id, NativeDesktopSurfaceKind::SystemOpen);
    descriptor.evidence_freshness = NativeDesktopEvidenceFreshness::Stale;
    let row = build_native_desktop_row(descriptor, full_bindings(entry_id, "anchor"));
    let report = build_native_desktop_matrix(vec![row]);
    assert!(report
        .entries
        .iter()
        .flat_map(|entry| &entry.blocking_findings)
        .any(|finding| finding.class_token() == "stale_evidence_on_marketed_surface"));
    assert!(report
        .narrowable_marketed_entries
        .iter()
        .any(|narrowable| narrowable.entry_id == entry_id));
}

#[test]
fn validation_flags_missing_cross_link() {
    let mut report = seeded_native_desktop_matrix();
    report.cross_links.auth_recovery_ref = String::new();
    let errors = validate_native_desktop_matrix(&report).expect_err("must flag missing cross-link");
    assert!(errors.iter().any(|err| matches!(
        err,
        NativeDesktopValidationError::CrossLinkMissing { field } if field == "auth_recovery_ref"
    )));
}

#[test]
fn support_export_quotes_report_and_case_ids() {
    let report = seeded_native_desktop_matrix();
    let export =
        NativeDesktopSupportExport::from_report(NATIVE_DESKTOP_SUPPORT_EXPORT_ID, report.clone());
    assert!(export.case_ids.contains(&report.report_id));
    for entry in &report.entries {
        assert!(export.case_ids.contains(&entry.descriptor.entry_id));
        assert!(export
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn reopen_anchor_index_is_complete_and_sorted() {
    let report = seeded_native_desktop_matrix();
    assert_eq!(report.reopen_anchor_index.len(), report.entries.len());
    let mut sorted = report.reopen_anchor_index.clone();
    sorted.sort_by(|left, right| left.entry_id.cmp(&right.entry_id));
    assert_eq!(report.reopen_anchor_index, sorted);
    for anchor in &report.reopen_anchor_index {
        assert!(!anchor.reopen_anchor_ref.trim().is_empty());
    }
}

#[test]
fn compact_and_markdown_render_without_panicking() {
    let report = seeded_native_desktop_matrix();
    assert!(!report.compact_lines().is_empty());
    let markdown = report.render_markdown();
    assert!(markdown.contains("native-desktop integration and reopen matrix"));
    assert!(markdown.contains("Cross-links"));
}
