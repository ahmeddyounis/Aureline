use super::*;

fn packet() -> M5CrashIntakeAndRecovery {
    current_m5_crash_intake_and_recovery().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_CRASH_INTAKE_RECOVERY_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_CRASH_INTAKE_RECOVERY_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_screens() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_screen_offers_the_core_recovery_actions() {
    // The guardrail: recovery is never collapsed into one generic "try again" — each screen offers the
    // distinct, named core actions.
    let packet = packet();
    for screen in &packet.screens {
        for class in RecoveryActionClass::CORE {
            assert!(
                screen.action(class).is_some(),
                "{} missing core action {}",
                screen.screen_id,
                class.as_str()
            );
        }
    }
}

#[test]
fn restore_is_the_only_rerun_action() {
    // Users must be able to tell which action reruns or discards state without guessing.
    let packet = packet();
    for screen in &packet.screens {
        for action in &screen.recovery_actions {
            assert!(
                !action.discards_state,
                "{} action {} discards state",
                screen.screen_id,
                action.action_class.as_str()
            );
            let expected_rerun = action.action_class == RecoveryActionClass::Restore;
            assert_eq!(
                action.reruns_session,
                expected_rerun,
                "{} action {} rerun flag is wrong",
                screen.screen_id,
                action.action_class.as_str()
            );
            assert!(action.is_class_consistent());
        }
    }
}

#[test]
fn session_reentry_actions_require_explicit_no_silent_rerun() {
    let packet = packet();
    for screen in &packet.screens {
        for action in &screen.recovery_actions {
            if action.action_class.is_session_reentry() {
                assert!(action.no_silent_rerun, "{}", screen.screen_id);
                assert!(
                    action.requires_explicit_confirmation,
                    "{}",
                    screen.screen_id
                );
            }
        }
    }
}

#[test]
fn disable_actions_target_a_present_recent_change() {
    let packet = packet();
    for screen in &packet.screens {
        for action in &screen.recovery_actions {
            if action.action_class.is_disable() {
                let change_ref = action
                    .targets_change_ref
                    .as_deref()
                    .expect("disable action targets a change");
                let matched = screen.recent_changes.iter().any(|c| {
                    c.change_id == change_ref
                        && c.change_kind.disable_action_class() == action.action_class
                });
                assert!(matched, "{} {}", screen.screen_id, change_ref);
            }
        }
    }
}

#[test]
fn both_disable_action_classes_are_exercised() {
    let packet = packet();
    let mut classes: BTreeSet<RecoveryActionClass> = BTreeSet::new();
    for screen in &packet.screens {
        for action in &screen.recovery_actions {
            classes.insert(action.action_class);
        }
    }
    for class in RecoveryActionClass::ALL {
        assert!(
            classes.contains(&class),
            "no screen exercises action {}",
            class.as_str()
        );
    }
}

#[test]
fn every_screen_shows_a_visible_copyable_exact_build_id() {
    let packet = packet();
    for screen in &packet.screens {
        assert!(
            !screen.exact_build_id.trim().is_empty(),
            "{}",
            screen.screen_id
        );
        assert!(screen.build_id_copyable, "{}", screen.screen_id);
        assert!(
            !screen.crash_envelope_id.trim().is_empty(),
            "{}",
            screen.screen_id
        );
    }
}

#[test]
fn every_screen_carries_one_step_explainability() {
    let packet = packet();
    for screen in &packet.screens {
        assert!(
            screen.has_one_step_explainability(),
            "{} lacks one-step explainability",
            screen.screen_id
        );
    }
}

#[test]
fn local_save_is_first_class_on_every_screen() {
    let packet = packet();
    assert!(packet.all_local_save_first_class());
    for screen in &packet.screens {
        assert!(
            screen.local_save_modes().next().is_some(),
            "{} offers no enabled local-save mode",
            screen.screen_id
        );
        assert!(
            screen.local_save_is_first_class(),
            "{} buries the local-save mode beneath a send mode",
            screen.screen_id
        );
        assert!(screen.local_save_first_class);
    }
    assert_eq!(
        packet.summary.local_save_first_class_screens,
        packet.screens.len()
    );
}

#[test]
fn every_screen_offers_all_three_intake_modes() {
    // Local save, team share, and formal support are offered from the same surface.
    let packet = packet();
    for screen in &packet.screens {
        let present: BTreeSet<IntakeMode> = screen.intake_modes.iter().map(|m| m.mode).collect();
        for mode in IntakeMode::ALL {
            assert!(
                present.contains(&mode),
                "{} missing intake mode {}",
                screen.screen_id,
                mode.as_str()
            );
        }
    }
}

#[test]
fn no_screen_offers_a_destructive_action() {
    let packet = packet();
    for screen in &packet.screens {
        assert!(!screen.destructive_action_offered, "{}", screen.screen_id);
        assert!(screen.silent_restart_suppressed, "{}", screen.screen_id);
        assert!(screen.raw_material_excluded, "{}", screen.screen_id);
    }
}

#[test]
fn every_screen_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_screens_gate_consistent());
    for screen in &packet.screens {
        assert_eq!(
            screen.intake_status,
            screen.computed_status(),
            "{}",
            screen.screen_id
        );
        assert_eq!(
            screen.presentation,
            screen.effective_presentation(),
            "{}",
            screen.screen_id
        );
        assert_eq!(
            screen.downgrade_reasons,
            screen.computed_downgrade_reasons(),
            "{}",
            screen.screen_id
        );
        assert_eq!(
            screen.blocked_before_send,
            screen.effective_presentation().warns_before_send(),
            "{}",
            screen.screen_id
        );
        assert_eq!(
            screen.claims_exact_build,
            screen.build_is_exact(),
            "{}",
            screen.screen_id
        );
        assert_eq!(
            screen.claims_resolved_symbolication,
            screen.symbolication_is_resolved(),
            "{}",
            screen.screen_id
        );
    }
}

#[test]
fn exact_ready_screen_is_whole_and_not_overclaimed() {
    let packet = packet();
    let exact = packet.exact_ready_screens().count();
    assert!(
        exact >= 1,
        "fixture needs at least one exact-ready screen to prove the gate is not a blanket flag"
    );
    for screen in packet.exact_ready_screens() {
        assert!(screen.build_is_exact());
        assert!(screen.symbolication_is_resolved());
        assert!(!screen.restore_provenance.is_downgraded());
        assert!(!screen.install_advisory_state.narrows());
        assert!(!screen.send_unsafe());
        assert!(screen.downgrade_reasons.is_empty());
        assert!(screen.caveats.is_empty());
        assert!(!screen.blocked_before_send);
        assert!(screen.claims_exact_build);
        assert!(screen.claims_resolved_symbolication);
    }
}

#[test]
fn stale_symbol_map_screen_never_implies_exact_or_resolved() {
    // The out-of-scope guardrail: do not imply exact-build or resolved symbolication for approximate /
    // unresolved data.
    let packet = packet();
    let screen = packet.screen("stale-symbol-map").expect("stale screen");
    assert_eq!(
        screen.build_identity_fidelity,
        BuildIdentityFidelity::ApproximateBuild
    );
    assert_eq!(
        screen.symbolication_fidelity,
        SymbolicationFidelity::StaleSymbolMap
    );
    assert!(!screen.claims_exact_build);
    assert!(!screen.claims_resolved_symbolication);
    assert_eq!(screen.presentation, RecoveryPresentation::Narrowed);
    assert!(screen
        .downgrade_reasons
        .contains(&CrashIntakeDowngradeReason::ApproximateBuildIdentity));
    assert!(screen
        .downgrade_reasons
        .contains(&CrashIntakeDowngradeReason::StaleOrPartialSymbolication));
}

#[test]
fn quarantined_extension_screen_offers_a_bounded_disable() {
    let packet = packet();
    let screen = packet
        .screen("quarantined-extension")
        .expect("quarantine screen");
    assert_eq!(
        screen.install_advisory_state,
        InstallAdvisoryState::ExtensionQuarantineActive
    );
    assert_eq!(screen.intake_status, CrashIntakeStatus::AdvisoryNarrowed);
    assert!(screen
        .downgrade_reasons
        .contains(&CrashIntakeDowngradeReason::ExtensionQuarantineActive));
    let disable = screen
        .action(RecoveryActionClass::DisableRecentlyChangedExtension)
        .expect("disable-extension action");
    assert_eq!(disable.blast_radius, BlastRadiusClass::SingleSuspectToggle);
    assert!(disable.targets_change_ref.is_some());
    assert!(!disable.reruns_session);
}

#[test]
fn restore_downgrade_screen_never_implies_exact_restore() {
    let packet = packet();
    let screen = packet
        .screen("restore-downgrade")
        .expect("downgrade screen");
    assert_eq!(
        screen.restore_provenance,
        RestoreProvenanceClass::RestoreDowngraded
    );
    assert_eq!(screen.intake_status, CrashIntakeStatus::FidelityNarrowed);
    assert_eq!(screen.presentation, RecoveryPresentation::Narrowed);
    assert!(screen
        .downgrade_reasons
        .contains(&CrashIntakeDowngradeReason::RestoreProvenanceDowngraded));
}

#[test]
fn send_blocked_screen_refuses_unsafe_intake_and_keeps_local_save_primary() {
    let packet = packet();
    let screen = packet
        .screen("send-blocked-unsafe-intake")
        .expect("send-blocked screen");
    assert_eq!(screen.intake_status, CrashIntakeStatus::SendBlocked);
    assert_eq!(screen.presentation, RecoveryPresentation::SendBlocked);
    assert!(screen.blocked_before_send);
    assert!(screen.send_unsafe());
    assert!(!screen.blockers.is_empty());
    assert!(screen
        .downgrade_reasons
        .contains(&CrashIntakeDowngradeReason::IntakeSendBlockedUnsafeContent));
    let local = screen.local_save_modes().next().expect("local-save mode");
    assert_eq!(local.prominence, PathProminence::Primary);
}

#[test]
fn narrowed_and_blocked_screens_carry_caveats() {
    let packet = packet();
    for screen in &packet.screens {
        if screen.effective_presentation().requires_attention() {
            assert!(!screen.caveats.is_empty(), "{}", screen.screen_id);
        }
        if screen.computed_status().requires_blockers() {
            assert!(!screen.blockers.is_empty(), "{}", screen.screen_id);
        }
    }
}

#[test]
fn presentations_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RecoveryPresentation> =
        packet.screens.iter().map(|s| s.presentation).collect();
    for decision in RecoveryPresentation::ALL {
        assert!(
            present.contains(&decision),
            "no screen exercises {}",
            decision.as_str()
        );
    }
}

#[test]
fn intake_statuses_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CrashIntakeStatus> =
        packet.screens.iter().map(|s| s.intake_status).collect();
    for status in CrashIntakeStatus::ALL {
        assert!(
            present.contains(&status),
            "no screen exercises status {}",
            status.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CrashIntakeDowngradeReason> = packet
        .screens
        .iter()
        .flat_map(|s| s.downgrade_reasons.iter().copied())
        .collect();
    for reason in CrashIntakeDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no screen exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_screens_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.screens.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert!(projection.all_screens_gate_consistent);
    assert!(projection.all_local_save_first_class);
    assert_eq!(
        projection.exact_ready_count,
        packet.exact_ready_screens().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_screens().count());
    assert_eq!(
        projection.send_blocked_count,
        packet.send_blocked_screens().count()
    );
    for (screen, row) in packet.screens.iter().zip(projection.rows.iter()) {
        assert_eq!(row.presentation, screen.presentation.as_str());
        assert_eq!(row.exact_ready, screen.is_exact_ready());
        assert_eq!(row.exact_build_id, screen.exact_build_id);
        assert_eq!(row.claims_exact_build, screen.claims_exact_build);
        assert_eq!(row.recovery_actions.len(), screen.recovery_actions.len());
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("support:m5:crash-recovery", "2026-06-17T12:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, packet.packet_id);
    assert!(export.raw_material_excluded);
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in RecoveryConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn validate_flags_overstated_presentation() {
    let mut packet = packet();
    if let Some(screen) = packet
        .screens
        .iter_mut()
        .find(|s| s.effective_presentation() != RecoveryPresentation::ExactReady)
    {
        screen.presentation = RecoveryPresentation::ExactReady;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5CrashIntakeAndRecoveryViolation::OverstatedPresentation { .. }
        )));
    }
}

#[test]
fn validate_flags_overclaimed_exact_build() {
    // Forcing an exact-build claim on an approximate screen is exactly the overclaim the gate forbids.
    let mut packet = packet();
    let screen = packet
        .screens
        .iter_mut()
        .find(|s| !s.build_is_exact())
        .expect("an approximate / unresolved screen");
    screen.build_identity_fidelity = BuildIdentityFidelity::ExactBuild;
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5CrashIntakeAndRecoveryViolation::OverclaimedFidelity { .. }
            | M5CrashIntakeAndRecoveryViolation::ExactBuildClaimMismatch { .. }
            | M5CrashIntakeAndRecoveryViolation::OverstatedPresentation { .. }
            | M5CrashIntakeAndRecoveryViolation::DowngradeReasonsMismatch { .. }
            | M5CrashIntakeAndRecoveryViolation::IntakeStatusMismatch { .. }
    )));
}

#[test]
fn validate_flags_collapsed_recovery_actions() {
    // Dropping a core action is the "collapse into a generic affordance" the guardrail forbids.
    let mut packet = packet();
    if let Some(screen) = packet.screens.first_mut() {
        screen
            .recovery_actions
            .retain(|a| a.action_class != RecoveryActionClass::SafeMode);
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5CrashIntakeAndRecoveryViolation::MissingCoreAction { .. }
        )));
    }
}

#[test]
fn validate_flags_destructive_action() {
    let mut packet = packet();
    if let Some(screen) = packet.screens.first_mut() {
        screen.destructive_action_offered = true;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5CrashIntakeAndRecoveryViolation::DestructiveActionOffered { .. }
        )));
    }
}

#[test]
fn validate_flags_local_save_demoted_below_send() {
    let mut packet = packet();
    let screen = packet
        .screens
        .iter_mut()
        .find(|s| s.send_modes().next().is_some())
        .expect("a screen with a send mode");
    for mode in &mut screen.intake_modes {
        if mode.is_local_save() {
            mode.prominence = PathProminence::Secondary;
        } else if mode.leaves_machine {
            mode.prominence = PathProminence::Primary;
        }
    }
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5CrashIntakeAndRecoveryViolation::LocalSaveNotFirstClass { .. }
            | M5CrashIntakeAndRecoveryViolation::LocalSaveAttestationMismatch { .. }
    )));
}

#[test]
fn validate_flags_disable_action_without_target() {
    let mut packet = packet();
    let screen = packet
        .screens
        .iter_mut()
        .find(|s| {
            s.recovery_actions
                .iter()
                .any(|a| a.action_class.is_disable())
        })
        .expect("a screen with a disable action");
    if let Some(action) = screen
        .recovery_actions
        .iter_mut()
        .find(|a| a.action_class.is_disable())
    {
        action.targets_change_ref = None;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5CrashIntakeAndRecoveryViolation::DisableActionTargetMissing { .. }
        )));
    }
}

#[test]
fn validate_flags_build_id_not_copyable() {
    let mut packet = packet();
    if let Some(screen) = packet.screens.first_mut() {
        screen.build_id_copyable = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5CrashIntakeAndRecoveryViolation::BuildIdNotCopyable { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != RecoveryConsumerSurface::IssueReportPacket);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5CrashIntakeAndRecoveryViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_drops_exact_build_lineage() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.preserves_exact_build_lineage = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5CrashIntakeAndRecoveryViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_screens = packet.summary.total_screens.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5CrashIntakeAndRecoveryViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(RecoveryActionClass::Restore.as_str(), "restore");
    assert_eq!(
        RecoveryActionClass::DisableRecentlyChangedExtension.as_str(),
        "disable_recently_changed_extension"
    );
    assert_eq!(
        RecoveryActionEffect::RerunsRestore.as_str(),
        "reruns_restore"
    );
    assert_eq!(
        BlastRadiusClass::FullSessionReplay.as_str(),
        "full_session_replay"
    );
    assert_eq!(BuildIdentityFidelity::ExactBuild.as_str(), "exact_build");
    assert_eq!(
        SymbolicationFidelity::StaleSymbolMap.as_str(),
        "stale_symbol_map"
    );
    assert_eq!(
        RestoreProvenanceClass::RestoreDowngraded.as_str(),
        "restore_downgraded"
    );
    assert_eq!(
        InstallAdvisoryState::ExtensionQuarantineActive.as_str(),
        "extension_quarantine_active"
    );
    assert_eq!(
        RedactionExportPosture::LocalOnlyRetained.as_str(),
        "local_only_retained"
    );
    assert_eq!(
        IntakeMode::FormalSupportHandoff.as_str(),
        "formal_support_handoff"
    );
    assert_eq!(CrashIntakeStatus::SendBlocked.as_str(), "send_blocked");
    assert_eq!(RecoveryPresentation::Narrowed.as_str(), "narrowed");
    assert_eq!(
        CrashIntakeDowngradeReason::IntakeSendBlockedUnsafeContent.as_str(),
        "intake_send_blocked_unsafe_content"
    );
    assert_eq!(
        RecoveryConsumerSurface::IssueReportPacket.as_str(),
        "issue_report_packet"
    );
}

#[test]
fn ceilings_and_effects_hold() {
    assert_eq!(
        BuildIdentityFidelity::ExactBuild.presentation_ceiling(),
        RecoveryPresentation::ExactReady
    );
    assert_eq!(
        BuildIdentityFidelity::ApproximateBuild.presentation_ceiling(),
        RecoveryPresentation::Narrowed
    );
    assert_eq!(
        SymbolicationFidelity::Resolved.presentation_ceiling(),
        RecoveryPresentation::ExactReady
    );
    assert_eq!(
        SymbolicationFidelity::Unresolved.presentation_ceiling(),
        RecoveryPresentation::Narrowed
    );
    assert!(RecoveryActionEffect::RerunsRestore.reruns_session());
    assert!(!RecoveryActionEffect::RestartsInSafeProfile.reruns_session());
    assert!(!RecoveryActionEffect::OpensWithoutReplay.reruns_session());
    for effect in RecoveryActionEffect::ALL {
        assert!(!effect.discards_state());
    }
    assert!(RedactionExportPosture::MetadataSafeDefault.is_export_safe_off_machine());
    assert!(RedactionExportPosture::RedactedSummary.is_export_safe_off_machine());
    assert!(!RedactionExportPosture::LocalOnlyRetained.is_export_safe_off_machine());
    assert!(!RedactionExportPosture::BlockedUnsafeContent.is_export_safe_off_machine());
}
