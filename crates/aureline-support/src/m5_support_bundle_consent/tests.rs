use super::*;

fn packet() -> M5SupportBundleConsent {
    current_m5_support_bundle_consent().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_SUPPORT_BUNDLE_CONSENT_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_SUPPORT_BUNDLE_CONSENT_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_sheets() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_sheet_has_all_four_data_classes() {
    let packet = packet();
    for sheet in &packet.sheets {
        for data_class in ConsentDataClass::ALL {
            assert!(
                sheet.class_row(data_class).is_some(),
                "{} missing data class {}",
                sheet.sheet_id,
                data_class.as_str()
            );
        }
    }
}

#[test]
fn every_sheet_carries_one_step_explainability() {
    // Consent is answerable from the Support Center and the CLI / headless export review in one step.
    let packet = packet();
    for sheet in &packet.sheets {
        assert!(
            sheet.has_one_step_explainability(),
            "{} lacks one-step explainability",
            sheet.sheet_id
        );
        assert!(!sheet.explain_entrypoint_ref.trim().is_empty());
        assert!(!sheet.cli_object_ref.trim().is_empty());
    }
}

#[test]
fn every_sheet_shows_a_retention_note_and_schema_version() {
    let packet = packet();
    for sheet in &packet.sheets {
        assert!(
            !sheet.retention_note.trim().is_empty(),
            "{}",
            sheet.sheet_id
        );
        assert!(
            !sheet.schema_version_label.trim().is_empty(),
            "{}",
            sheet.sheet_id
        );
    }
}

#[test]
fn local_save_is_first_class_on_every_sheet() {
    // The headline acceptance criterion: local-save is a first-class path, never a hidden fallback.
    let packet = packet();
    assert!(packet.all_local_save_first_class());
    for sheet in &packet.sheets {
        assert!(
            sheet.local_save_destinations().next().is_some(),
            "{} offers no enabled local-save path",
            sheet.sheet_id
        );
        assert!(
            sheet.local_save_is_first_class(),
            "{} buries the local-save path beneath a send path",
            sheet.sheet_id
        );
        assert!(sheet.local_save_first_class);
    }
    assert_eq!(
        packet.summary.local_save_first_class_sheets,
        packet.sheets.len()
    );
}

#[test]
fn secret_bearing_class_stays_excluded_and_unexportable() {
    // High-risk content is excluded by default, never offers an off-machine-exportable toggle, and is
    // never included in a sheet that would send it.
    let packet = packet();
    for sheet in &packet.sheets {
        let high = sheet
            .class_row(ConsentDataClass::HighRisk)
            .expect("high-risk row");
        assert!(
            high.default_inclusion.is_excluded_by_default(),
            "{} includes secret-bearing content by default",
            sheet.sheet_id
        );
        assert!(
            !high
                .redaction_toggle
                .allowed_modes
                .iter()
                .any(|m| m.is_exportable_off_machine()),
            "{} offers an off-machine toggle for secret-bearing content",
            sheet.sheet_id
        );
        if let Some(dest) = sheet.selected_destination() {
            if dest.leaves_machine
                && sheet.effective_presentation() != ConsentPresentation::SendBlocked
            {
                assert_eq!(
                    high.included_count, 0,
                    "{} would send secret-bearing content",
                    sheet.sheet_id
                );
            }
        }
    }
}

#[test]
fn every_sheet_excludes_raw_material() {
    let packet = packet();
    for sheet in &packet.sheets {
        assert!(
            sheet.raw_material_excluded,
            "{} does not exclude raw material",
            sheet.sheet_id
        );
    }
}

#[test]
fn every_sheet_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_sheets_gate_consistent());
    for sheet in &packet.sheets {
        assert_eq!(
            sheet.consent_status,
            sheet.computed_status(),
            "{}",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.presentation,
            sheet.effective_presentation(),
            "{}",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.downgrade_reasons,
            sheet.computed_downgrade_reasons(),
            "{}",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.blocked_before_send,
            sheet.effective_presentation().warns_before_send(),
            "{}",
            sheet.sheet_id
        );
    }
}

#[test]
fn review_ready_sheets_are_whole() {
    let packet = packet();
    let ready = packet.review_ready_sheets().count();
    assert!(
        ready >= 1,
        "fixture needs at least one review-ready sheet to prove the gate is not a blanket flag"
    );
    for sheet in packet.review_ready_sheets() {
        assert_eq!(sheet.computed_status(), ConsentStatus::ReviewReady);
        assert_eq!(sheet.schema_freshness, SchemaFreshness::Current);
        assert!(sheet.downgrade_reasons.is_empty());
        assert!(sheet.caveats.is_empty());
        assert!(sheet.blockers.is_empty());
        assert!(!sheet.blocked_before_send);
        assert_eq!(sheet.policy_locked_total(), 0);
        assert!(!sheet.has_redaction_override());
        assert!(sheet.local_save_first_class);
    }
}

#[test]
fn narrowed_and_blocked_sheets_carry_caveats() {
    let packet = packet();
    for sheet in &packet.sheets {
        if sheet.effective_presentation().requires_attention() {
            assert!(!sheet.caveats.is_empty(), "{}", sheet.sheet_id);
        }
        if sheet.computed_status().requires_blockers() {
            assert!(!sheet.blockers.is_empty(), "{}", sheet.sheet_id);
        }
    }
}

#[test]
fn local_save_only_flow_is_clean_and_first_class() {
    // A local-save-only export is the safest path and presents transparently, not as a degraded fallback.
    let packet = packet();
    let sheet = packet.sheet("local-only-review").expect("local-only sheet");
    assert_eq!(
        sheet.selected_destination_class,
        ConsentDestinationClass::LocalOnlyReview
    );
    assert_eq!(sheet.presentation, ConsentPresentation::ReviewReady);
    assert!(sheet.send_destinations().next().is_none());
    assert!(sheet.local_save_is_first_class());
    assert_eq!(sheet.retention_class, RetentionClass::NotRetainedLocalOnly);
}

#[test]
fn policy_locked_export_shows_locked_counts_not_hidden() {
    let packet = packet();
    let sheet = packet
        .sheet("managed-policy-locked")
        .expect("policy-locked sheet");
    assert_eq!(sheet.consent_status, ConsentStatus::PolicyNarrowed);
    assert_eq!(sheet.presentation, ConsentPresentation::NarrowedReview);
    assert!(sheet.policy_locked_total() > 0);
    assert!(sheet
        .downgrade_reasons
        .contains(&ConsentDowngradeReason::DestinationPolicyLocked));
    assert!(!sheet.blockers.is_empty());
    // Locked content is excluded, not silently included.
    let code = sheet
        .class_row(ConsentDataClass::CodeAdjacent)
        .expect("code row");
    assert_eq!(code.included_count, 0);
    assert!(code.policy_locked_count > 0);
}

#[test]
fn redaction_override_is_surfaced() {
    let packet = packet();
    let sheet = packet
        .sheet("redaction-override-upload")
        .expect("redaction-override sheet");
    assert_eq!(sheet.consent_status, ConsentStatus::RedactionAdjusted);
    assert_eq!(sheet.presentation, ConsentPresentation::NarrowedReview);
    assert!(sheet.has_redaction_override());
    assert!(sheet
        .downgrade_reasons
        .contains(&ConsentDowngradeReason::RedactionOverrideApplied));
}

#[test]
fn stale_schema_warns_before_send() {
    let packet = packet();
    let sheet = packet.sheet("stale-schema-vendor").expect("stale sheet");
    assert_eq!(sheet.schema_freshness, SchemaFreshness::Stale);
    assert_ne!(
        sheet.schema_version_label,
        sheet.current_schema_version_label
    );
    assert_eq!(sheet.presentation, ConsentPresentation::NarrowedReview);
    assert!(sheet
        .downgrade_reasons
        .contains(&ConsentDowngradeReason::StaleSchemaWarning));
}

#[test]
fn send_blocked_sheet_refuses_unsafe_content() {
    let packet = packet();
    let sheet = packet
        .sheet("send-blocked-retained-local")
        .expect("send-blocked sheet");
    assert_eq!(sheet.consent_status, ConsentStatus::SendBlocked);
    assert_eq!(sheet.presentation, ConsentPresentation::SendBlocked);
    assert!(sheet.blocked_before_send);
    assert!(sheet.send_unsafe());
    assert!(sheet
        .downgrade_reasons
        .contains(&ConsentDowngradeReason::ExportBlockedUnsafeContent));
    // Local save remains primary so the user is steered to the safe path.
    let local = sheet
        .local_save_destinations()
        .next()
        .expect("local-save path");
    assert_eq!(local.prominence, PathProminence::Primary);
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in ConsentConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn presentations_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ConsentPresentation> =
        packet.sheets.iter().map(|s| s.presentation).collect();
    for decision in ConsentPresentation::ALL {
        assert!(
            present.contains(&decision),
            "no sheet exercises {}",
            decision.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ConsentDowngradeReason> = packet
        .sheets
        .iter()
        .flat_map(|s| s.downgrade_reasons.iter().copied())
        .collect();
    for reason in ConsentDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no sheet exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn consent_statuses_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ConsentStatus> = packet.sheets.iter().map(|s| s.consent_status).collect();
    for status in ConsentStatus::ALL {
        assert!(
            present.contains(&status),
            "no sheet exercises status {}",
            status.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_sheets_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.sheets.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_sheets_gate_consistent,
        packet.all_sheets_gate_consistent()
    );
    assert!(projection.all_local_save_first_class);
    assert_eq!(
        projection.review_ready_count,
        packet.review_ready_sheets().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_sheets().count());
    assert_eq!(
        projection.send_blocked_count,
        packet.send_blocked_sheets().count()
    );
    for (sheet, row) in packet.sheets.iter().zip(projection.rows.iter()) {
        assert_eq!(row.presentation, sheet.presentation.as_str());
        assert_eq!(row.review_ready, sheet.is_review_ready());
        assert_eq!(row.included_total, sheet.included_total());
        assert_eq!(row.policy_locked_total, sheet.policy_locked_total());
        assert_eq!(row.local_save_first_class, sheet.local_save_first_class);
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("support:m5:consent", "2026-06-16T13:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, packet.packet_id);
    assert!(export.raw_material_excluded);
}

#[test]
fn validate_flags_overstated_presentation() {
    let mut packet = packet();
    if let Some(sheet) = packet
        .sheets
        .iter_mut()
        .find(|s| s.effective_presentation() != ConsentPresentation::ReviewReady)
    {
        sheet.presentation = ConsentPresentation::ReviewReady;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportBundleConsentViolation::OverstatedPresentation { .. }
        )));
    }
}

#[test]
fn validate_flags_local_save_demoted_below_send() {
    // Demoting the local-save path beneath a send path is exactly the upload-first UI the gate forbids.
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.send_destinations().next().is_some())
        .expect("a sheet that offers a send path");
    for dest in &mut sheet.destinations {
        if dest.is_local_save() {
            dest.prominence = PathProminence::Secondary;
        } else if dest.leaves_machine {
            dest.prominence = PathProminence::Primary;
        }
    }
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5SupportBundleConsentViolation::LocalSaveNotFirstClass { .. }
            | M5SupportBundleConsentViolation::LocalSaveAttestationMismatch { .. }
    )));
}

#[test]
fn validate_flags_missing_local_save_destination() {
    let mut packet = packet();
    if let Some(sheet) = packet.sheets.first_mut() {
        sheet.destinations.retain(|d| !d.is_local_save());
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportBundleConsentViolation::NoLocalSaveDestination { .. }
                | M5SupportBundleConsentViolation::LocalSaveNotFirstClass { .. }
        )));
    }
}

#[test]
fn validate_flags_secret_bearing_included_by_default() {
    let mut packet = packet();
    if let Some(sheet) = packet.sheets.first_mut() {
        if let Some(row) = sheet
            .class_rows
            .iter_mut()
            .find(|r| r.data_class == ConsentDataClass::HighRisk)
        {
            row.default_inclusion = DefaultInclusion::IncludedByDefault;
            assert!(packet.validate().iter().any(|v| matches!(
                v,
                M5SupportBundleConsentViolation::SecretBearingIncludedByDefault { .. }
            )));
        }
    }
}

#[test]
fn validate_flags_secret_bearing_exportable_toggle() {
    let mut packet = packet();
    if let Some(sheet) = packet.sheets.first_mut() {
        if let Some(row) = sheet
            .class_rows
            .iter_mut()
            .find(|r| r.data_class == ConsentDataClass::HighRisk)
        {
            row.redaction_toggle.allowed_modes = vec![RedactionMode::RedactedSummary];
            row.redaction_toggle.current_mode = RedactionMode::RedactedSummary;
            row.redaction_toggle.default_mode = RedactionMode::RedactedSummary;
            assert!(packet.validate().iter().any(|v| matches!(
                v,
                M5SupportBundleConsentViolation::SecretBearingExportableToggle { .. }
            )));
        }
    }
}

#[test]
fn validate_flags_consent_status_misclassification() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.computed_status() == ConsentStatus::ReviewReady)
        .expect("a review-ready sheet");
    sheet.consent_status = ConsentStatus::PolicyNarrowed;
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5SupportBundleConsentViolation::ConsentStatusMismatch { .. }
    )));
}

#[test]
fn validate_flags_missing_data_class() {
    let mut packet = packet();
    if let Some(sheet) = packet.sheets.first_mut() {
        sheet
            .class_rows
            .retain(|r| r.data_class != ConsentDataClass::HighRisk);
        assert!(packet
            .validate()
            .iter()
            .any(|v| matches!(v, M5SupportBundleConsentViolation::MissingDataClass { .. })));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != ConsentConsumerSurface::FormalSupportHandoff);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5SupportBundleConsentViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_drops_local_save() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.local_save_first_class = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportBundleConsentViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_sheets = packet.summary.total_sheets.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5SupportBundleConsentViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ConsentDataClass::HighRisk.as_str(), "high_risk");
    assert_eq!(
        ConsentDestinationClass::LocalOnlyReview.as_str(),
        "local_only_review"
    );
    assert_eq!(PathProminence::CoEqual.as_str(), "co_equal");
    assert_eq!(
        RetentionClass::NotRetainedLocalOnly.as_str(),
        "not_retained_local_only"
    );
    assert_eq!(SchemaFreshness::Stale.as_str(), "stale");
    assert_eq!(ConsentStatus::SendBlocked.as_str(), "send_blocked");
    assert_eq!(
        ConsentPresentation::NarrowedReview.as_str(),
        "narrowed_review"
    );
    assert_eq!(
        ConsentDowngradeReason::StaleSchemaWarning.as_str(),
        "stale_schema_warning"
    );
    assert_eq!(
        RedactionMode::RetainedLocalOnly.as_str(),
        "retained_local_only"
    );
    assert_eq!(DefaultInclusion::NonExportable.as_str(), "non_exportable");
    assert_eq!(
        ConsentConsumerSurface::FormalSupportHandoff.as_str(),
        "formal_support_handoff"
    );
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        ConsentStatus::ReviewReady.presentation_ceiling(),
        ConsentPresentation::ReviewReady
    );
    assert_eq!(
        ConsentStatus::PolicyNarrowed.presentation_ceiling(),
        ConsentPresentation::NarrowedReview
    );
    assert_eq!(
        ConsentStatus::SendBlocked.presentation_ceiling(),
        ConsentPresentation::SendBlocked
    );
    assert_eq!(
        SchemaFreshness::Current.presentation_ceiling(),
        ConsentPresentation::ReviewReady
    );
    assert_eq!(
        SchemaFreshness::Stale.presentation_ceiling(),
        ConsentPresentation::NarrowedReview
    );
}

#[test]
fn redaction_mode_exportability_is_correct() {
    assert!(RedactionMode::NotRequiredMetadata.is_exportable_off_machine());
    assert!(RedactionMode::RedactedSummary.is_exportable_off_machine());
    assert!(RedactionMode::SanitizedSnapshot.is_exportable_off_machine());
    assert!(!RedactionMode::RetainedLocalOnly.is_exportable_off_machine());
    assert!(!RedactionMode::Prohibited.is_exportable_off_machine());
    assert!(!RedactionMode::PolicyLocked.is_exportable_off_machine());
    assert!(!RedactionMode::OmittedPendingOptIn.is_exportable_off_machine());
}
