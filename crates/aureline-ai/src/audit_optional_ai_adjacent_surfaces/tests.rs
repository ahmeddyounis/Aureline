use super::*;

fn valid_notebook_row() -> SurfaceQualificationRow {
    SurfaceQualificationRow {
        surface_family: OptionalAiSurfaceFamily::Notebook,
        surface_label: "Notebook AI affordances".to_owned(),
        qualification: SurfaceQualificationLabel::Limited,
        has_own_qualification_packet: true,
        qualification_packet_ref: "fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/notebook_qualification.json".to_owned(),
        trust_boundary_explicit: true,
        route_evidence_explicit: true,
        export_support_parity_explicit: true,
        downgrade_rule_explicit: true,
        no_stable_inheritance_from_core: true,
        visible_below_stable_label: true,
        propagated_to_help_about: true,
        propagated_to_docs: true,
        propagated_to_release_packets: true,
        propagated_to_compat_reports: true,
        notebook_requirements: Some(NotebookRequirements {
            document_trust_labeled: true,
            kernel_trust_labeled: true,
            output_trust_labeled: true,
            kernel_state_consistent: true,
            debugger_support_labeled: true,
            output_sandbox_cues_present: true,
            no_generic_ai_affordances: true,
        }),
        voice_requirements: None,
        browser_companion_requirements: None,
        preview_designer_requirements: None,
        known_gaps_label: None,
    }
}

fn valid_voice_row() -> SurfaceQualificationRow {
    SurfaceQualificationRow {
        surface_family: OptionalAiSurfaceFamily::Voice,
        surface_label: "Voice and dictation input".to_owned(),
        qualification: SurfaceQualificationLabel::Experimental,
        has_own_qualification_packet: true,
        qualification_packet_ref: "fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/voice_qualification.json".to_owned(),
        trust_boundary_explicit: true,
        route_evidence_explicit: true,
        export_support_parity_explicit: true,
        downgrade_rule_explicit: true,
        no_stable_inheritance_from_core: true,
        visible_below_stable_label: true,
        propagated_to_help_about: true,
        propagated_to_docs: true,
        propagated_to_release_packets: true,
        propagated_to_compat_reports: true,
        notebook_requirements: None,
        voice_requirements: Some(VoiceRequirements {
            explicit_consent_required: true,
            capture_boundary_declared: true,
            local_vs_retained_transcript_explicit: true,
            disable_path_present: true,
            accessibility_safe_fallback_present: true,
            not_treated_as_plain_text_input: true,
        }),
        browser_companion_requirements: None,
        preview_designer_requirements: None,
        known_gaps_label: None,
    }
}

fn valid_browser_companion_row() -> SurfaceQualificationRow {
    SurfaceQualificationRow {
        surface_family: OptionalAiSurfaceFamily::BrowserCompanion,
        surface_label: "Browser companion AI actions".to_owned(),
        qualification: SurfaceQualificationLabel::Limited,
        has_own_qualification_packet: true,
        qualification_packet_ref: "fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/browser_companion_qualification.json".to_owned(),
        trust_boundary_explicit: true,
        route_evidence_explicit: true,
        export_support_parity_explicit: true,
        downgrade_rule_explicit: true,
        no_stable_inheritance_from_core: true,
        visible_below_stable_label: true,
        propagated_to_help_about: true,
        propagated_to_docs: true,
        propagated_to_release_packets: true,
        propagated_to_compat_reports: true,
        notebook_requirements: None,
        voice_requirements: None,
        browser_companion_requirements: Some(BrowserCompanionRequirements {
            scope_limited_to_review_docs_or_inspect: true,
            no_native_depth_authority: true,
            no_silent_write_back: true,
            no_hidden_runtime_mutation: true,
            scope_label_present: true,
        }),
        preview_designer_requirements: None,
        known_gaps_label: None,
    }
}

fn valid_preview_designer_row() -> SurfaceQualificationRow {
    SurfaceQualificationRow {
        surface_family: OptionalAiSurfaceFamily::PreviewDesigner,
        surface_label: "Preview and designer AI affordances".to_owned(),
        qualification: SurfaceQualificationLabel::Limited,
        has_own_qualification_packet: true,
        qualification_packet_ref: "fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/preview_designer_qualification.json".to_owned(),
        trust_boundary_explicit: true,
        route_evidence_explicit: true,
        export_support_parity_explicit: true,
        downgrade_rule_explicit: true,
        no_stable_inheritance_from_core: true,
        visible_below_stable_label: true,
        propagated_to_help_about: true,
        propagated_to_docs: true,
        propagated_to_release_packets: true,
        propagated_to_compat_reports: true,
        notebook_requirements: None,
        voice_requirements: None,
        browser_companion_requirements: None,
        preview_designer_requirements: Some(PreviewDesignerRequirements {
            scope_honest: true,
            no_native_depth_authority: true,
            no_silent_write_back: true,
            no_hidden_runtime_mutation: true,
            separately_proven_if_write_capable: true,
        }),
        known_gaps_label: None,
    }
}

fn valid_background_branch_row() -> SurfaceQualificationRow {
    SurfaceQualificationRow {
        surface_family: OptionalAiSurfaceFamily::BackgroundBranchAutomation,
        surface_label: "Background branch agent automation".to_owned(),
        qualification: SurfaceQualificationLabel::Limited,
        has_own_qualification_packet: true,
        qualification_packet_ref: "fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/background_branch_automation_qualification.json".to_owned(),
        trust_boundary_explicit: true,
        route_evidence_explicit: true,
        export_support_parity_explicit: true,
        downgrade_rule_explicit: true,
        no_stable_inheritance_from_core: true,
        visible_below_stable_label: true,
        propagated_to_help_about: true,
        propagated_to_docs: true,
        propagated_to_release_packets: true,
        propagated_to_compat_reports: true,
        notebook_requirements: None,
        voice_requirements: None,
        browser_companion_requirements: None,
        preview_designer_requirements: None,
        known_gaps_label: None,
    }
}

fn valid_downgrade_automation() -> DowngradeAutomationBlock {
    DowngradeAutomationBlock {
        triggers: vec![
            DowngradeTriggerClass::PacketFreshnessExpired,
            DowngradeTriggerClass::RouteTruthRegressed,
            DowngradeTriggerClass::SupportExportParityMissing,
        ],
        fires_on_packet_freshness_expiry: true,
        fires_on_route_truth_regression: true,
        fires_on_support_export_parity_missing: true,
        downgrade_target_label: "Experimental or Unsupported".to_owned(),
        downgrade_propagates_to_product_copy: true,
        downgrade_propagates_to_docs_help: true,
        downgrade_propagates_to_release_packets: true,
        downgrade_propagates_to_compat_reports: true,
    }
}

fn valid_propagation_state() -> PropagationStateBlock {
    PropagationStateBlock {
        help_about_surfaces_consume_family_specific_state: true,
        docs_consume_family_specific_state: true,
        marketplace_consume_family_specific_state: true,
        cli_headless_inspect_consume_family_specific_state: true,
        support_export_consume_family_specific_state: true,
        no_optimistic_available_in_build_collapse: true,
    }
}

fn valid_packet() -> OptionalAiAdjacentSurfaceAuditPacket {
    OptionalAiAdjacentSurfaceAuditPacket {
        record_kind: OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_RECORD_KIND.to_owned(),
        schema_version: OPTIONAL_AI_ADJACENT_SURFACE_AUDIT_SCHEMA_VERSION,
        packet_id: "optional-ai-adjacent-surface-audit:stable:0001".to_owned(),
        display_label: "Optional AI-adjacent surface qualification audit".to_owned(),
        policy_epoch_ref: "policy-epoch:stable:0004".to_owned(),
        core_ai_graduation_packet_ref: "artifacts/ai/m4/publish_stable_ai_graduation_packets/graduation_state.json".to_owned(),
        surface_rows: vec![
            valid_notebook_row(),
            valid_voice_row(),
            valid_browser_companion_row(),
            valid_preview_designer_row(),
            valid_background_branch_row(),
        ],
        downgrade_automation: valid_downgrade_automation(),
        propagation_state: valid_propagation_state(),
        source_contract_refs: vec![
            "docs/ai/m4/audit-optional-ai-adjacent-surfaces.md".to_owned(),
            "schemas/ai/optional-ai-surface-qualification.schema.json".to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-01T09:11:29Z".to_owned(),
    }
}

#[test]
fn valid_packet_passes_validation() {
    let packet = valid_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {violations:?}"
    );
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = valid_packet();
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::WrongRecordKind { .. })));
}

#[test]
fn wrong_schema_version_is_rejected() {
    let mut packet = valid_packet();
    packet.schema_version = 99;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::WrongSchemaVersion { .. })));
}

#[test]
fn empty_surface_rows_is_rejected() {
    let mut packet = valid_packet();
    packet.surface_rows.clear();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::NoSurfaceRows)));
}

#[test]
fn missing_required_family_is_rejected() {
    let mut packet = valid_packet();
    packet
        .surface_rows
        .retain(|r| r.surface_family != OptionalAiSurfaceFamily::Voice);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AuditViolation::MissingRequiredFamily {
            family: OptionalAiSurfaceFamily::Voice
        }
    )));
}

#[test]
fn below_stable_without_visible_label_is_rejected() {
    let mut packet = valid_packet();
    packet.surface_rows[0].visible_below_stable_label = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::MissingBelowStableLabel { .. })));
}

#[test]
fn stable_without_own_packet_is_rejected() {
    let mut packet = valid_packet();
    packet.surface_rows[0].qualification = SurfaceQualificationLabel::Stable;
    packet.surface_rows[0].visible_below_stable_label = false;
    packet.surface_rows[0].has_own_qualification_packet = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::StableWithoutOwnPacket { .. })));
}

#[test]
fn stable_inherited_from_core_is_rejected() {
    let mut packet = valid_packet();
    packet.surface_rows[0].no_stable_inheritance_from_core = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::StableInheritedFromCore { .. })));
}

#[test]
fn notebook_missing_requirements_is_rejected() {
    let mut packet = valid_packet();
    packet.surface_rows[0].notebook_requirements = None;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::NotebookRequirementsMissing { .. })));
}

#[test]
fn notebook_unsatisfied_requirements_is_rejected() {
    let mut packet = valid_packet();
    if let Some(ref mut req) = packet.surface_rows[0].notebook_requirements {
        req.output_sandbox_cues_present = false;
    }
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::NotebookRequirementsUnsatisfied { .. })));
}

#[test]
fn voice_missing_requirements_is_rejected() {
    let mut packet = valid_packet();
    let voice_idx = packet
        .surface_rows
        .iter()
        .position(|r| r.surface_family == OptionalAiSurfaceFamily::Voice)
        .unwrap();
    packet.surface_rows[voice_idx].voice_requirements = None;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::VoiceRequirementsMissing { .. })));
}

#[test]
fn voice_unsatisfied_requirements_is_rejected() {
    let mut packet = valid_packet();
    let voice_idx = packet
        .surface_rows
        .iter()
        .position(|r| r.surface_family == OptionalAiSurfaceFamily::Voice)
        .unwrap();
    if let Some(ref mut req) = packet.surface_rows[voice_idx].voice_requirements {
        req.explicit_consent_required = false;
    }
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::VoiceRequirementsUnsatisfied { .. })));
}

#[test]
fn browser_companion_missing_requirements_is_rejected() {
    let mut packet = valid_packet();
    let idx = packet
        .surface_rows
        .iter()
        .position(|r| r.surface_family == OptionalAiSurfaceFamily::BrowserCompanion)
        .unwrap();
    packet.surface_rows[idx].browser_companion_requirements = None;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::BrowserCompanionRequirementsMissing { .. })));
}

#[test]
fn browser_companion_unsatisfied_requirements_is_rejected() {
    let mut packet = valid_packet();
    let idx = packet
        .surface_rows
        .iter()
        .position(|r| r.surface_family == OptionalAiSurfaceFamily::BrowserCompanion)
        .unwrap();
    if let Some(ref mut req) = packet.surface_rows[idx].browser_companion_requirements {
        req.no_silent_write_back = false;
    }
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::BrowserCompanionRequirementsUnsatisfied { .. })));
}

#[test]
fn preview_designer_missing_requirements_is_rejected() {
    let mut packet = valid_packet();
    let idx = packet
        .surface_rows
        .iter()
        .position(|r| r.surface_family == OptionalAiSurfaceFamily::PreviewDesigner)
        .unwrap();
    packet.surface_rows[idx].preview_designer_requirements = None;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::PreviewDesignerRequirementsMissing { .. })));
}

#[test]
fn preview_designer_unsatisfied_requirements_is_rejected() {
    let mut packet = valid_packet();
    let idx = packet
        .surface_rows
        .iter()
        .position(|r| r.surface_family == OptionalAiSurfaceFamily::PreviewDesigner)
        .unwrap();
    if let Some(ref mut req) = packet.surface_rows[idx].preview_designer_requirements {
        req.no_hidden_runtime_mutation = false;
    }
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::PreviewDesignerRequirementsUnsatisfied { .. })));
}

#[test]
fn downgrade_not_firing_on_freshness_expiry_is_rejected() {
    let mut packet = valid_packet();
    packet.downgrade_automation.fires_on_packet_freshness_expiry = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::DowngradeNotFiringOnFreshnessExpiry)));
}

#[test]
fn downgrade_not_firing_on_route_truth_regression_is_rejected() {
    let mut packet = valid_packet();
    packet.downgrade_automation.fires_on_route_truth_regression = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::DowngradeNotFiringOnRouteTruthRegression)));
}

#[test]
fn downgrade_not_propagating_is_rejected() {
    let mut packet = valid_packet();
    packet.downgrade_automation.downgrade_propagates_to_product_copy = false;
    packet.downgrade_automation.downgrade_propagates_to_docs_help = false;
    packet.downgrade_automation.downgrade_propagates_to_release_packets = false;
    packet.downgrade_automation.downgrade_propagates_to_compat_reports = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::DowngradeNotPropagating)));
}

#[test]
fn optimistic_collapse_is_rejected() {
    let mut packet = valid_packet();
    packet
        .propagation_state
        .no_optimistic_available_in_build_collapse = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AuditViolation::OptimisticAvailableInBuildCollapse)));
}

#[test]
fn consumer_not_consuming_family_specific_state_is_rejected() {
    let mut packet = valid_packet();
    packet
        .propagation_state
        .support_export_consume_family_specific_state = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AuditViolation::ConsumerNotConsumingFamilySpecificState {
            consumer_label
        } if consumer_label == "support_export"
    )));
}

#[test]
fn checked_artifact_parses_and_validates() {
    let packet = OptionalAiAdjacentSurfaceAuditPacket::from_checked_artifact()
        .expect("checked artifact must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked artifact has violations: {violations:?}"
    );
}

#[test]
fn qualification_label_below_stable_requires_visible_label() {
    for label in [
        SurfaceQualificationLabel::Limited,
        SurfaceQualificationLabel::Preview,
        SurfaceQualificationLabel::Experimental,
        SurfaceQualificationLabel::Unsupported,
    ] {
        assert!(
            label.requires_visible_below_stable_label(),
            "{:?} should require a visible below-stable label",
            label.as_str()
        );
    }
    assert!(
        !SurfaceQualificationLabel::Stable.requires_visible_below_stable_label(),
        "Stable should not require a visible below-stable label"
    );
}

#[test]
fn all_required_families_are_covered_by_valid_packet() {
    let packet = valid_packet();
    for family in OptionalAiSurfaceFamily::required_families() {
        assert!(
            packet
                .surface_rows
                .iter()
                .any(|r| r.surface_family == family),
            "required family {:?} is not covered",
            family.as_str()
        );
    }
}
