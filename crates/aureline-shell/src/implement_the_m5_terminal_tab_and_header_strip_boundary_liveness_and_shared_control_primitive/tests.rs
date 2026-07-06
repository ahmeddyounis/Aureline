use super::*;

fn live_local(title: &str) -> M5TerminalTabResolutionInput {
    M5TerminalTabResolutionInput {
        session_title: title.to_owned(),
        host_boundary: M5HostBoundaryClass::LocalHost,
        shell_integration: M5ShellIntegrationQuality::FullyIntegrated,
        liveness: M5TerminalSessionLiveness::LiveAttached,
        connection_state: None,
        cwd_repr: Some("workspace/app".to_owned()),
        last_known_cwd_repr: None,
        collaboration_role: None,
        follow_state: None,
        reauthorization_required: false,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_live_local_is_write_capable_with_live_cwd() {
    let resolved = resolve_terminal_tab(&live_local("app-server")).expect("resolves");
    assert_eq!(
        resolved.input_posture,
        M5TerminalInputPosture::WriteCapableLive
    );
    assert!(resolved.is_write_capable);
    assert!(!resolved.is_restored_transcript);
    assert!(resolved.boundary_is_local);
    assert_eq!(resolved.cwd_display, M5CwdDisplayState::LiveCwdReported);
    assert_eq!(resolved.cwd_repr.as_deref(), Some("workspace/app"));
    assert_eq!(
        resolved.shared_control_posture,
        M5SharedControlPosture::SoloSession
    );
}

#[test]
fn resolver_restored_transcript_is_read_only_never_write_capable() {
    let input = M5TerminalTabResolutionInput {
        session_title: "api-server".to_owned(),
        host_boundary: M5HostBoundaryClass::RemoteSshHost,
        shell_integration: M5ShellIntegrationQuality::CwdReportingOnly,
        liveness: M5TerminalSessionLiveness::RestoredFromTranscript,
        connection_state: Some(M5RemoteConnectionState::OfflineCached),
        cwd_repr: None,
        last_known_cwd_repr: Some("workspace/api".to_owned()),
        collaboration_role: None,
        follow_state: None,
        reauthorization_required: false,
    };
    let resolved = resolve_terminal_tab(&input).expect("resolves");
    assert!(resolved.is_restored_transcript);
    assert!(!resolved.is_write_capable);
    assert_eq!(
        resolved.input_posture,
        M5TerminalInputPosture::ReadOnlyRestored
    );
    // The last-known cwd is shown, not a live one.
    assert_eq!(resolved.cwd_display, M5CwdDisplayState::LastKnownCwdShown);
    assert_eq!(resolved.cwd_repr.as_deref(), Some("workspace/api"));
}

#[test]
fn resolver_observer_is_inspect_only() {
    let input = M5TerminalTabResolutionInput {
        session_title: "repl-managed".to_owned(),
        host_boundary: M5HostBoundaryClass::ManagedWorkspaceHost,
        shell_integration: M5ShellIntegrationQuality::CommandMarksOnly,
        liveness: M5TerminalSessionLiveness::LiveAttached,
        connection_state: Some(M5RemoteConnectionState::Connected),
        cwd_repr: None,
        last_known_cwd_repr: None,
        collaboration_role: Some(M5CollaborationRole::Observer),
        follow_state: Some(M5FollowState::FollowingPresenter),
        reauthorization_required: false,
    };
    let resolved = resolve_terminal_tab(&input).expect("resolves");
    assert_eq!(
        resolved.input_posture,
        M5TerminalInputPosture::InspectOnlyObserver
    );
    assert!(!resolved.is_write_capable);
    assert_eq!(
        resolved.shared_control_posture,
        M5SharedControlPosture::SharedObserverOnly
    );
    // command-marks-only shell does not report cwd at all.
    assert_eq!(
        resolved.cwd_display,
        M5CwdDisplayState::CwdNotReportedByShell
    );
    assert!(resolved.cwd_repr.is_none());
}

#[test]
fn resolver_reauthorization_blocks_input_and_is_explicit() {
    let input = M5TerminalTabResolutionInput {
        session_title: "wasm-preview".to_owned(),
        host_boundary: M5HostBoundaryClass::WasmSandboxHost,
        shell_integration: M5ShellIntegrationQuality::BasicPtyNoIntegration,
        liveness: M5TerminalSessionLiveness::LiveAttached,
        connection_state: Some(M5RemoteConnectionState::Connected),
        cwd_repr: None,
        last_known_cwd_repr: None,
        collaboration_role: Some(M5CollaborationRole::SessionHost),
        follow_state: None,
        reauthorization_required: true,
    };
    let resolved = resolve_terminal_tab(&input).expect("resolves");
    assert_eq!(
        resolved.input_posture,
        M5TerminalInputPosture::ReauthorizationBlocked
    );
    assert!(!resolved.is_write_capable);
    assert!(resolved.requires_reauthorization);
    assert_eq!(
        resolved.shared_control_posture,
        M5SharedControlPosture::ReauthorizationRequired
    );
}

#[test]
fn resolver_closed_and_reconnecting_and_following() {
    let closed = M5TerminalTabResolutionInput {
        session_title: "vite-preview".to_owned(),
        host_boundary: M5HostBoundaryClass::VirtualMachineHost,
        shell_integration: M5ShellIntegrationQuality::FullyIntegrated,
        liveness: M5TerminalSessionLiveness::ClosedExited,
        connection_state: Some(M5RemoteConnectionState::Disconnected),
        cwd_repr: None,
        last_known_cwd_repr: Some("preview/build".to_owned()),
        collaboration_role: None,
        follow_state: None,
        reauthorization_required: false,
    };
    let resolved = resolve_terminal_tab(&closed).expect("resolves");
    assert_eq!(
        resolved.input_posture,
        M5TerminalInputPosture::ClosedNoInput
    );
    assert_eq!(resolved.cwd_display, M5CwdDisplayState::LastKnownCwdShown);

    let following = M5TerminalTabResolutionInput {
        session_title: "repl-local".to_owned(),
        host_boundary: M5HostBoundaryClass::LocalHost,
        shell_integration: M5ShellIntegrationQuality::FullyIntegrated,
        liveness: M5TerminalSessionLiveness::LiveAttached,
        connection_state: None,
        cwd_repr: None,
        last_known_cwd_repr: None,
        collaboration_role: Some(M5CollaborationRole::Collaborator),
        follow_state: Some(M5FollowState::FollowingPresenter),
        reauthorization_required: false,
    };
    let resolved = resolve_terminal_tab(&following).expect("resolves");
    assert_eq!(
        resolved.input_posture,
        M5TerminalInputPosture::WriteCapableLive
    );
    assert_eq!(
        resolved.shared_control_posture,
        M5SharedControlPosture::SharedFollowingPresenter
    );
    // Live but no cwd and no last-known: unavailable, never invented.
    assert_eq!(resolved.cwd_display, M5CwdDisplayState::CwdUnavailable);
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_title = M5TerminalTabResolutionInput {
        session_title: "  ".to_owned(),
        ..live_local("x")
    };
    assert_eq!(
        resolve_terminal_tab(&empty_title),
        Err(M5TerminalTabResolutionError::EmptySessionTitle)
    );

    let remote_no_conn = M5TerminalTabResolutionInput {
        host_boundary: M5HostBoundaryClass::RemoteSshHost,
        connection_state: None,
        ..live_local("x")
    };
    assert_eq!(
        resolve_terminal_tab(&remote_no_conn),
        Err(M5TerminalTabResolutionError::RemoteHostMissingConnectionState)
    );

    let local_with_conn = M5TerminalTabResolutionInput {
        connection_state: Some(M5RemoteConnectionState::Connected),
        ..live_local("x")
    };
    assert_eq!(
        resolve_terminal_tab(&local_with_conn),
        Err(M5TerminalTabResolutionError::LocalHostWithConnectionState)
    );

    let follow_no_role = M5TerminalTabResolutionInput {
        follow_state: Some(M5FollowState::FollowingPresenter),
        collaboration_role: None,
        ..live_local("x")
    };
    assert_eq!(
        resolve_terminal_tab(&follow_no_role),
        Err(M5TerminalTabResolutionError::FollowStateWithoutRole)
    );

    let reauth_solo = M5TerminalTabResolutionInput {
        reauthorization_required: true,
        collaboration_role: None,
        ..live_local("x")
    };
    assert_eq!(
        resolve_terminal_tab(&reauth_solo),
        Err(M5TerminalTabResolutionError::ReauthorizationWithoutSharedSession)
    );

    let forbidden = M5TerminalTabResolutionInput {
        cwd_repr: Some("https://example.test".to_owned()),
        ..live_local("x")
    };
    assert_eq!(
        resolve_terminal_tab(&forbidden),
        Err(M5TerminalTabResolutionError::ForbiddenSessionMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_terminal_tab_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_TERMINAL_TAB_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_console_surface() {
    let packet = seeded_m5_terminal_tab_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .console_rows
        .iter()
        .map(|r| r.console_surface)
        .collect();
    for surface in M5TerminalConsoleSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing console surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.console_rows.len(),
        M5TerminalConsoleSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_terminal_tab_primitive_packet();
    for row in &packet.console_rows {
        for part in M5TerminalTabAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5TerminalTabExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_terminal_tab_primitive_packet();
    let cases: Vec<&M5TerminalTabResolutionCase> = packet
        .console_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5TerminalInputPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.input_posture == posture),
            "no worked resolution exercises input posture {}",
            posture.as_str()
        );
    }
    for state in M5CwdDisplayState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.cwd_display == state),
            "no worked resolution exercises cwd display state {}",
            state.as_str()
        );
    }
    for posture in M5SharedControlPosture::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.shared_control_posture == posture),
            "no worked resolution exercises shared-control posture {}",
            posture.as_str()
        );
    }
    for host in M5HostBoundaryClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.host_boundary == host),
            "no worked resolution exercises host boundary {}",
            host.as_str()
        );
    }
    for liveness in M5TerminalSessionLiveness::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.liveness == liveness),
            "no worked resolution exercises liveness {}",
            liveness.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_terminal_tab_primitive_packet();
    for row in &packet.console_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.console_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_console_surface_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet
        .console_rows
        .retain(|row| row.console_surface != M5TerminalConsoleSurface::RequestConsole);
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::RequiredConsoleMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.vocabulary_set.input_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.console_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TerminalTabAnatomyPart::HostBoundaryBadge);
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.console_rows[0]
        .export_fields
        .retain(|f| *f != M5TerminalTabExportField::InputPosture);
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.console_rows[0].example_resolutions[0]
        .resolved
        .is_write_capable = false;
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.console_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn restored_write_confusion_unproven_fails_when_no_restored_example_present() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    // Drop every restored-transcript example across the matrix and confirm the
    // packet-level lint fires. Replace with a live example so rows still carry one.
    for row in &mut packet.console_rows {
        row.example_resolutions
            .retain(|c| !c.resolved.is_restored_transcript);
        if row.example_resolutions.is_empty() {
            row.example_resolutions
                .push(M5TerminalTabResolutionCase::resolved(
                    M5TerminalTabResolutionInput {
                        session_title: "placeholder".to_owned(),
                        host_boundary: M5HostBoundaryClass::LocalHost,
                        shell_integration: M5ShellIntegrationQuality::FullyIntegrated,
                        liveness: M5TerminalSessionLiveness::LiveAttached,
                        connection_state: None,
                        cwd_repr: Some("workspace/x".to_owned()),
                        last_known_cwd_repr: None,
                        collaboration_role: None,
                        follow_state: None,
                        reauthorization_required: false,
                    },
                ));
        }
    }
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::RestoredWriteConfusionUnproven));
}

#[test]
fn console_invariant_violation_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.console_rows[0].conflates_live_and_restored_session = true;
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::ConsoleInvariantViolated));
}

#[test]
fn stable_console_missing_proof_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.console_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::StableConsoleMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet
        .governance_review
        .restored_transcript_never_write_capable = false;
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet
        .consumer_projection
        .shared_control_reads_single_collaboration_source = false;
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TerminalTabPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_console_surface() {
    let summary = seeded_m5_terminal_tab_primitive_packet().render_markdown_summary();
    for surface in M5TerminalConsoleSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing console {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_console() {
    let csv = seeded_m5_terminal_tab_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5TerminalConsoleSurface::ALL.len());
    assert!(lines[0].starts_with("console_surface,qualification,owner,"));
    for surface in M5TerminalConsoleSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing console {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_terminal_tab_primitive_export()
        .expect("checked M5 terminal-tab primitive export validates");
    assert_eq!(from_disk.packet_id, M5_TERMINAL_TAB_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_terminal_tab_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consoles_visible() {
    for packet in [
        seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed(),
        seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.console_rows.len(),
            M5TerminalConsoleSurface::ALL.len()
        );
    }

    let incident = seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed();
    let row = incident
        .console_rows
        .iter()
        .find(|r| r.console_surface == M5TerminalConsoleSurface::IncidentShell)
        .expect("incident row present");
    assert_eq!(row.qualification, M5RuntimeBoundaryQualificationClass::Beta);

    let preview = seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed();
    let row = preview
        .console_rows
        .iter()
        .find(|r| r.console_surface == M5TerminalConsoleSurface::PreviewDevServer)
        .expect("preview row present");
    assert_eq!(
        row.qualification,
        M5RuntimeBoundaryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let incident: M5TerminalTabPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-terminal-tab-primitive/incident_shell_beta_narrowed.json"
    )))
    .expect("incident fixture parses");
    assert!(incident.validate().is_empty());
    assert_eq!(
        incident,
        seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed()
    );

    let preview: M5TerminalTabPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-terminal-tab-primitive/preview_dev_server_preview_narrowed.json"
    )))
    .expect("preview fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_terminal_tab_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
