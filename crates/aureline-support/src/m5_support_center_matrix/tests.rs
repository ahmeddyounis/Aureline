use super::*;

fn packet() -> M5SupportCenterMatrix {
    current_m5_support_center_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_SUPPORT_CENTER_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_SUPPORT_CENTER_MATRIX_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_module_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.rows.len(), SupportModule::ALL.len());
    for module in SupportModule::ALL {
        assert!(
            packet.row_for(module).is_some(),
            "missing row for module {}",
            module.as_str()
        );
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(
            row.published_readiness,
            row.effective_readiness(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.module_publication,
            row.computed_publication(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.downgrade_path,
            row.computed_downgrade_path(),
            "{}",
            row.row_id
        );
    }
}

#[test]
fn every_row_reuses_at_least_one_inspector_with_evidence() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            !row.inspectors.is_empty(),
            "{} reuses no inspector",
            row.row_id
        );
        for binding in &row.inspectors {
            assert!(
                binding.is_well_formed(),
                "{} inspector {} is incomplete",
                row.row_id,
                binding.inspector.as_str()
            );
        }
    }
}

#[test]
fn every_row_carries_required_evidence_refs() {
    let packet = packet();
    for row in &packet.rows {
        assert!(row.has_required_evidence(), "{}", row.row_id);
    }
}

#[test]
fn high_risk_modules_exclude_always() {
    let packet = packet();
    for row in &packet.rows {
        if row.touches_high_risk() {
            assert_eq!(
                row.redaction_default,
                RedactionDefault::ExcludedAlways,
                "{} touches high risk without excluding always",
                row.row_id
            );
        }
    }
}

#[test]
fn sharing_modules_reuse_export_consent() {
    let packet = packet();
    for row in &packet.rows {
        if row.offers_sharing_export() {
            assert!(
                row.reuses_inspector(Inspector::ExportConsent),
                "{} shares off-machine without export_consent",
                row.row_id
            );
        }
    }
}

#[test]
fn withheld_modules_offer_nothing() {
    let packet = packet();
    for row in packet.withheld_rows() {
        assert_eq!(
            row.published_readiness,
            ModuleReadiness::Unavailable,
            "{}",
            row.row_id
        );
        assert!(
            row.offered_actions.is_empty(),
            "{} is withheld but still offers actions",
            row.row_id
        );
    }
}

#[test]
fn narrowed_modules_offer_recovery_and_caveats() {
    let packet = packet();
    for row in &packet.rows {
        if row.module_publication.is_narrowed() {
            assert!(row.downgrade_path.is_offered(), "{}", row.row_id);
            assert!(!row.caveats.is_empty(), "{}", row.row_id);
            assert!(!row.stale_or_missing_fields.is_empty(), "{}", row.row_id);
        }
    }
}

#[test]
fn published_modules_are_whole() {
    let packet = packet();
    let operational = packet
        .published_rows()
        .filter(|r| r.published_readiness == ModuleReadiness::Operational)
        .count();
    assert!(
        operational >= 2,
        "fixture needs at least two operational modules to prove the gate is not a blanket downgrade"
    );
    for row in packet.published_rows() {
        if row.published_readiness == ModuleReadiness::Operational {
            assert_eq!(row.declared_readiness, ModuleReadiness::Operational);
            assert_eq!(row.evidence_freshness, EvidenceFreshness::Current);
            assert_eq!(row.inspector_ceiling(), ModuleReadiness::Operational);
            assert_eq!(row.consent_ceiling(), ModuleReadiness::Operational);
            assert!(row.downgrade_reasons.is_empty());
            assert!(row.caveats.is_empty());
            assert!(!row.downgrade_path.is_offered());
            assert!(!row.is_downgraded());
            assert!(!row.offered_actions.is_empty());
        }
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in MatrixConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_rows_gate_consistent,
        packet.all_rows_gate_consistent()
    );
    assert_eq!(projection.published_count, packet.published_rows().count());
    assert_eq!(projection.narrowed_count, packet.narrowed_rows().count());
    assert_eq!(projection.withheld_count, packet.withheld_rows().count());
    for (row, export) in packet.rows.iter().zip(projection.rows.iter()) {
        assert_eq!(export.published_readiness, row.published_readiness.as_str());
        assert_eq!(export.published, row.is_published());
        assert_eq!(export.downgraded, row.is_downgraded());
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("support:m5:support-center-matrix", "2026-06-16T13:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.matrix_packet_id_ref, packet.packet_id);
    assert!(export.raw_private_material_excluded);
}

#[test]
fn published_readinesses_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ModuleReadiness> =
        packet.rows.iter().map(|r| r.published_readiness).collect();
    for label in ModuleReadiness::ALL {
        assert!(
            present.contains(&label),
            "no row publishes {}",
            label.as_str()
        );
    }
}

#[test]
fn publications_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ModulePublication> =
        packet.rows.iter().map(|r| r.module_publication).collect();
    for publication in ModulePublication::ALL {
        assert!(
            present.contains(&publication),
            "no row exercises {}",
            publication.as_str()
        );
    }
}

#[test]
fn evidence_freshness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> =
        packet.rows.iter().map(|r| r.evidence_freshness).collect();
    for state in EvidenceFreshness::ALL {
        assert!(
            present.contains(&state),
            "no row exercises {}",
            state.as_str()
        );
    }
}

#[test]
fn downgrade_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SupportModuleDowngradePath> =
        packet.rows.iter().map(|r| r.downgrade_path).collect();
    for path in SupportModuleDowngradePath::ALL {
        assert!(
            present.contains(&path),
            "no row exercises {}",
            path.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SupportModuleDowngradeReason> = packet
        .rows
        .iter()
        .flat_map(|r| r.downgrade_reasons.iter().copied())
        .collect();
    for reason in SupportModuleDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no row exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn inspector_availabilities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<InspectorAvailability> = packet
        .rows
        .iter()
        .flat_map(|r| r.inspectors.iter().map(|b| b.availability))
        .collect();
    for availability in InspectorAvailability::ALL {
        assert!(
            present.contains(&availability),
            "no inspector exercises {}",
            availability.as_str()
        );
    }
}

#[test]
fn consent_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ConsentState> = packet
        .rows
        .iter()
        .flat_map(|r| r.export_modes.iter().map(|b| b.consent))
        .collect();
    for state in ConsentState::ALL {
        assert!(
            present.contains(&state),
            "no export mode exercises {}",
            state.as_str()
        );
    }
}

#[test]
fn inspectors_data_classes_export_modes_are_exhaustive() {
    let packet = packet();
    let inspectors: BTreeSet<Inspector> = packet
        .rows
        .iter()
        .flat_map(|r| r.inspectors.iter().map(|b| b.inspector))
        .collect();
    for inspector in Inspector::ALL {
        assert!(
            inspectors.contains(&inspector),
            "no row reuses {}",
            inspector.as_str()
        );
    }
    let classes: BTreeSet<DataClass> = packet
        .rows
        .iter()
        .flat_map(|r| r.data_classes.iter().copied())
        .collect();
    for class in DataClass::ALL {
        assert!(
            classes.contains(&class),
            "no row touches {}",
            class.as_str()
        );
    }
    let modes: BTreeSet<ExportMode> = packet
        .rows
        .iter()
        .flat_map(|r| r.export_modes.iter().map(|b| b.mode))
        .collect();
    for mode in ExportMode::ALL {
        assert!(modes.contains(&mode), "no row offers {}", mode.as_str());
    }
    let redactions: BTreeSet<RedactionDefault> =
        packet.rows.iter().map(|r| r.redaction_default).collect();
    for redaction in RedactionDefault::ALL {
        assert!(
            redactions.contains(&redaction),
            "no row defaults to {}",
            redaction.as_str()
        );
    }
}

#[test]
fn evidence_stale_module_refreshes_evidence() {
    let packet = packet();
    let row = packet
        .row_for(SupportModule::SafeMode)
        .expect("safe-mode row");
    assert_eq!(row.evidence_freshness, EvidenceFreshness::Aging);
    assert_eq!(row.published_readiness, ModuleReadiness::Degraded);
    assert_eq!(row.module_publication, ModulePublication::Narrowed);
    assert_eq!(
        row.downgrade_path,
        SupportModuleDowngradePath::RefreshEvidence
    );
    assert_eq!(
        row.downgrade_reasons,
        vec![SupportModuleDowngradeReason::EvidenceStale]
    );
}

#[test]
fn degraded_inspector_module_restores_inspector() {
    let packet = packet();
    let row = packet.row_for(SupportModule::Bisect).expect("bisect row");
    assert!(row.has_degraded_inspector());
    assert_eq!(row.published_readiness, ModuleReadiness::Degraded);
    assert_eq!(
        row.downgrade_path,
        SupportModuleDowngradePath::RestoreInspector
    );
    assert!(row
        .downgrade_reasons
        .contains(&SupportModuleDowngradeReason::InspectorDegraded));
}

#[test]
fn unsatisfied_consent_module_resolves_consent() {
    let packet = packet();
    let row = packet
        .row_for(SupportModule::Language)
        .expect("language row");
    assert!(row.has_unsatisfied_consent());
    assert_eq!(
        row.downgrade_path,
        SupportModuleDowngradePath::ResolveConsent
    );
    assert!(row
        .downgrade_reasons
        .contains(&SupportModuleDowngradeReason::ConsentUnsatisfied));
}

#[test]
fn unavailable_inspector_module_is_withheld() {
    let packet = packet();
    let row = packet.row_for(SupportModule::Network).expect("network row");
    assert!(row.has_unavailable_inspector());
    assert_eq!(row.published_readiness, ModuleReadiness::Unavailable);
    assert_eq!(row.module_publication, ModulePublication::Withheld);
    assert_eq!(
        row.downgrade_path,
        SupportModuleDowngradePath::WithholdModule
    );
    assert!(row.offered_actions.is_empty());
}

#[test]
fn validate_flags_overstated_readiness() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.effective_readiness() != ModuleReadiness::Operational)
    {
        row.published_readiness = ModuleReadiness::Operational;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterMatrixViolation::OverstatedReadiness { .. }
        )));
    }
}

#[test]
fn validate_flags_high_risk_not_excluded() {
    let mut packet = packet();
    if let Some(row) = packet.rows.iter_mut().find(|r| r.touches_high_risk()) {
        row.redaction_default = RedactionDefault::EmbeddedMetadataOnly;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterMatrixViolation::HighRiskNotExcluded { .. }
        )));
    }
}

#[test]
fn validate_flags_sharing_without_consent_inspector() {
    let mut packet = packet();
    if let Some(row) = packet.rows.iter_mut().find(|r| r.offers_sharing_export()) {
        row.inspectors
            .retain(|b| b.inspector != Inspector::ExportConsent);
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterMatrixViolation::SharingWithoutConsentInspector { .. }
        )));
    }
}

#[test]
fn validate_flags_withheld_module_offering_actions() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.module_publication == ModulePublication::Withheld)
    {
        row.offered_actions.push("inspect anyway".to_owned());
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterMatrixViolation::WithheldModuleOffersActions { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != MatrixConsumerSurface::CliHeadless);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5SupportCenterMatrixViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_stops_narrowing() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.narrows_on_downgrade = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterMatrixViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_publication_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.module_publication != ModulePublication::Withheld)
    {
        row.module_publication = ModulePublication::Withheld;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterMatrixViolation::PublicationMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_module() {
    let mut packet = packet();
    packet.rows.retain(|r| r.module != SupportModule::Doctor);
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5SupportCenterMatrixViolation::MissingModule { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_modules = packet.summary.total_modules.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5SupportCenterMatrixViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(SupportModule::AiUsage.as_str(), "ai_usage");
    assert_eq!(
        SupportModule::IssueReportCrashIntake.as_str(),
        "issue_report_crash_intake"
    );
    assert_eq!(
        SupportModule::SupportBundleExportPreview.as_str(),
        "support_bundle_export_preview"
    );
    assert_eq!(ModuleReadiness::InspectOnly.as_str(), "inspect_only");
    assert_eq!(EvidenceFreshness::Missing.as_str(), "missing");
    assert_eq!(Inspector::ExportConsent.as_str(), "export_consent");
    assert_eq!(InspectorAvailability::Unavailable.as_str(), "unavailable");
    assert_eq!(DataClass::HighRisk.as_str(), "high_risk");
    assert_eq!(RedactionDefault::ExcludedAlways.as_str(), "excluded_always");
    assert_eq!(ExportMode::FormalSupport.as_str(), "formal_support");
    assert_eq!(
        ConsentState::RequiredNotGranted.as_str(),
        "required_not_granted"
    );
    assert_eq!(ModulePublication::Withheld.as_str(), "withheld");
    assert_eq!(
        SupportModuleDowngradeReason::ConsentUnsatisfied.as_str(),
        "consent_unsatisfied"
    );
    assert_eq!(SupportModuleDowngradePath::NoneNeeded.as_str(), "none");
    assert_eq!(
        MatrixConsumerSurface::FormalSupportHandoff.as_str(),
        "formal_support_handoff"
    );
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        EvidenceFreshness::Aging.readiness_ceiling(),
        ModuleReadiness::Degraded
    );
    assert_eq!(
        EvidenceFreshness::Expired.readiness_ceiling(),
        ModuleReadiness::InspectOnly
    );
    assert_eq!(
        EvidenceFreshness::Missing.readiness_ceiling(),
        ModuleReadiness::Unavailable
    );
    assert_eq!(
        InspectorAvailability::Degraded.readiness_ceiling(),
        ModuleReadiness::Degraded
    );
    assert_eq!(
        InspectorAvailability::Unavailable.readiness_ceiling(),
        ModuleReadiness::Unavailable
    );
    assert_eq!(
        ConsentState::RequiredNotGranted.readiness_ceiling(),
        ModuleReadiness::Degraded
    );
    assert_eq!(
        ConsentState::Blocked.readiness_ceiling(),
        ModuleReadiness::InspectOnly
    );
}
