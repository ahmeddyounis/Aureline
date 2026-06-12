//! Inline unit tests for the M5 command-parity audit.

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_command_parity_audit();
    validate_m5_command_parity_audit(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_claims_every_required_channel() {
    let report = seeded_m5_command_parity_audit();
    assert!(report.every_required_channel_claimed());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_command_parity_audit();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    for row in &report.rows {
        assert!(
            row.blocking_findings.is_empty(),
            "row {} carried blocking findings: {:?}",
            row.descriptor.command_id,
            row.blocking_findings
        );
    }
}

#[test]
fn seeded_audit_covers_every_feature_family() {
    let report = seeded_m5_command_parity_audit();
    let families: BTreeSet<M5FeatureFamily> = report
        .rows
        .iter()
        .map(|row| row.descriptor.feature_family)
        .collect();
    for expected in [
        M5FeatureFamily::Notebook,
        M5FeatureFamily::DataApi,
        M5FeatureFamily::Profiler,
        M5FeatureFamily::TraceReplay,
        M5FeatureFamily::DocsBrowser,
        M5FeatureFamily::TemplateScaffold,
        M5FeatureFamily::ReviewPipeline,
        M5FeatureFamily::Preview,
        M5FeatureFamily::Companion,
        M5FeatureFamily::Incident,
        M5FeatureFamily::Sync,
        M5FeatureFamily::Offboarding,
        M5FeatureFamily::SecretBroker,
        M5FeatureFamily::Infrastructure,
    ] {
        assert!(
            families.contains(&expected),
            "feature family {} is not registered",
            expected.as_str()
        );
    }
}

#[test]
fn high_risk_commands_are_marked_and_typed() {
    let report = seeded_m5_command_parity_audit();
    assert!(report.high_risk_command_count > 0);
    for row in &report.rows {
        if row.high_risk {
            assert_eq!(
                row.descriptor.disabled_reason_mode,
                M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
                "high-risk command {} must require a typed disabled reason",
                row.descriptor.command_id
            );
        }
    }
}

#[test]
fn help_anchor_index_covers_every_command() {
    let report = seeded_m5_command_parity_audit();
    assert_eq!(report.help_anchor_index.len(), report.rows.len());
    for row in &report.rows {
        let entry = report
            .help_anchor_index
            .iter()
            .find(|entry| entry.command_id == row.descriptor.command_id)
            .expect("every command must have a help-anchor entry");
        assert_eq!(entry.help_anchor_ref, row.descriptor.help_anchor_ref);
        assert!(!entry.help_anchor_ref.is_empty());
    }
}

#[test]
fn unknown_high_risk_gap_blocks_when_high_risk() {
    let descriptor = M5CommandDescriptor {
        command_id: "cmd:sync.push_workspace_state".to_owned(),
        feature_family: M5FeatureFamily::Sync,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        discoverability_summary: "summary".to_owned(),
        help_anchor_ref: "help:anchor:sync:push".to_owned(),
        search_keywords: vec!["sync".to_owned()],
        category_refs: vec!["category:sync".to_owned()],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::IrreversiblePublishPreview,
        approval_posture_class: "approval_required_human_confirm".to_owned(),
        capability_scope_class: M5CapabilityScope::IrreversiblePublish,
        mutability_class: M5MutabilityClass::ExternalPublish,
        disabled_reason_mode: M5DisabledReasonMode::TypedReasonRequiredWhenUnavailable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        automation_labels: vec!["approval_required".to_owned()],
        canonical_aliases: vec![],
        origin_class: "core".to_owned(),
        source_ref: None,
        publisher_ref: None,
        invocation_schema_ref: "schemas/commands/command_invocation_session.schema.json".to_owned(),
        result_schema_ref: "schemas/commands/command_result_packet.schema.json".to_owned(),
        promoted_to_stable_graph: true,
    };
    let channels = vec![M5ChannelProjection {
        channel: M5DiscoveryChannel::CommandPalette,
        coverage_status: M5CoverageStatus::UnknownHighRiskGap,
        projected_command_id: None,
        projected_label_ref: None,
        projected_lifecycle_label: None,
        projected_preview_class: None,
        projected_approval_posture_class: None,
        projected_disabled_reason_mode: None,
        projected_automation_suitability: None,
        projected_automation_labels: vec![],
        projected_help_anchor_ref: None,
        projected_aliases: vec![],
        narrowing_reason: None,
        note: None,
    }];
    let row = build_m5_command_parity_row(descriptor, channels);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::UnknownHighRiskGap { .. })));
}

#[test]
fn custom_pane_only_blocks_as_pointer_only() {
    let descriptor = M5CommandDescriptor {
        command_id: "cmd:notebook.run_all_cells".to_owned(),
        feature_family: M5FeatureFamily::Notebook,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        discoverability_summary: "summary".to_owned(),
        help_anchor_ref: "help:anchor".to_owned(),
        search_keywords: vec!["notebook".to_owned()],
        category_refs: vec!["category:notebook".to_owned()],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::NoPreviewRequired,
        approval_posture_class: "no_approval_required".to_owned(),
        capability_scope_class: M5CapabilityScope::ReversibleLocalRead,
        mutability_class: M5MutabilityClass::ReadOnly,
        disabled_reason_mode: M5DisabledReasonMode::AlwaysInvokable,
        automation_suitability: M5AutomationSuitability::FullyAutomatable,
        automation_labels: vec!["recipe_safe".to_owned()],
        canonical_aliases: vec![],
        origin_class: "core".to_owned(),
        source_ref: None,
        publisher_ref: None,
        invocation_schema_ref: "schemas/commands/command_invocation_session.schema.json".to_owned(),
        result_schema_ref: "schemas/commands/command_result_packet.schema.json".to_owned(),
        promoted_to_stable_graph: true,
    };
    let channels = vec![M5ChannelProjection {
        channel: M5DiscoveryChannel::CommandPalette,
        coverage_status: M5CoverageStatus::CustomPaneOnly,
        projected_command_id: None,
        projected_label_ref: None,
        projected_lifecycle_label: None,
        projected_preview_class: None,
        projected_approval_posture_class: None,
        projected_disabled_reason_mode: None,
        projected_automation_suitability: None,
        projected_automation_labels: vec![],
        projected_help_anchor_ref: None,
        projected_aliases: vec![],
        narrowing_reason: None,
        note: None,
    }];
    let row = build_m5_command_parity_row(descriptor, channels);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::PointerOnlyAffordance { .. })));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5CommandDescriptor {
        command_id: "cmd:sync.push_workspace_state".to_owned(),
        feature_family: M5FeatureFamily::Sync,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        discoverability_summary: "summary".to_owned(),
        help_anchor_ref: "  ".to_owned(),
        search_keywords: vec![],
        category_refs: vec!["category:sync".to_owned()],
        lifecycle_label: M5LifecycleLabel::Beta,
        preview_class: M5PreviewClass::IrreversiblePublishPreview,
        approval_posture_class: "approval_required_human_confirm".to_owned(),
        capability_scope_class: M5CapabilityScope::IrreversiblePublish,
        mutability_class: M5MutabilityClass::ExternalPublish,
        disabled_reason_mode: M5DisabledReasonMode::AlwaysInvokable,
        automation_suitability: M5AutomationSuitability::HumanOnly,
        automation_labels: vec!["approval_required".to_owned()],
        canonical_aliases: vec![],
        origin_class: "core".to_owned(),
        source_ref: None,
        publisher_ref: None,
        invocation_schema_ref: "schemas/commands/command_invocation_session.schema.json".to_owned(),
        result_schema_ref: "schemas/commands/command_result_packet.schema.json".to_owned(),
        promoted_to_stable_graph: false,
    };
    let row = build_m5_command_parity_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = row
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_help_anchor"));
    assert!(tokens.contains("missing_search_metadata"));
    assert!(tokens.contains("missing_disabled_reason_mode"));
    assert!(tokens.contains("command_not_promoted"));
}

#[test]
fn automation_suitability_drift_blocks() {
    let mut report = seeded_m5_command_parity_audit();
    // Force the trace/replay AI channel to project a wider authority.
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.descriptor.command_id == "cmd:trace_replay.replay_session")
        .expect("trace replay row present");
    let projection = row
        .channels
        .iter_mut()
        .find(|projection| projection.channel == M5DiscoveryChannel::AiAutomation)
        .expect("ai automation channel present");
    projection.projected_automation_suitability = Some(M5AutomationSuitability::FullyAutomatable);
    let rebuilt = build_m5_command_parity_row(row.descriptor.clone(), row.channels.clone());
    assert!(rebuilt.blocking_findings.iter().any(|f| matches!(
        f,
        M5ParityBlockingFinding::AutomationSuitabilityDrift { .. }
    )));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_command_parity_audit();
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.descriptor.command_id == "cmd:data_api.send_request")
        .expect("data api row present");
    let projection = row
        .channels
        .iter_mut()
        .find(|projection| projection.channel == M5DiscoveryChannel::KeybindingHelp)
        .expect("keybinding channel present");
    projection.narrowing_reason = None;
    let rebuilt = build_m5_command_parity_row(row.descriptor.clone(), row.channels.clone());
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::MissingNarrowingReason { .. })));
}

#[test]
fn missing_required_channel_blocks_validation() {
    let mut report = seeded_m5_command_parity_audit();
    report.rows[0]
        .channels
        .retain(|projection| projection.channel != M5DiscoveryChannel::CliHeadless);
    let result = validate_m5_command_parity_audit(&report);
    assert!(result.is_err());
}

#[test]
fn support_export_quotes_every_command_id() {
    let report = seeded_m5_command_parity_audit();
    let export = M5CommandParitySupportExport::from_report(
        M5_COMMAND_PARITY_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(export.case_ids.contains(&row.descriptor.command_id));
        assert!(export
            .case_ids
            .contains(&row.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn render_markdown_is_deterministic() {
    let report = seeded_m5_command_parity_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
