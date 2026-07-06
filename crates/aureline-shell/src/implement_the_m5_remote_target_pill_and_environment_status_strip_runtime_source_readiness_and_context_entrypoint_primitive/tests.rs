use super::*;

fn ready_local(title: &str) -> M5RunContextResolutionInput {
    M5RunContextResolutionInput {
        context_title: title.to_owned(),
        host_boundary: M5HostBoundaryClass::LocalHost,
        connection_state: None,
        runtime_kind_repr: "node".to_owned(),
        resolved_runtime_repr: "node 20.11.0".to_owned(),
        runtime_source: M5RuntimeSourceClass::ProjectPinned,
        scope: M5ResolvedScope::ProjectScope,
        effective_value_provenance: M5EffectiveValueProvenance::Resolved,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_local_resolved_is_ready_and_local_inline() {
    let resolved = resolve_run_context(&ready_local("run-local")).expect("resolves");
    assert_eq!(resolved.target_posture, M5RemoteTargetPosture::LocalInline);
    assert!(!resolved.is_remote);
    assert!(!resolved.target_is_degraded);
    assert_eq!(resolved.readiness, M5EnvironmentReadiness::Ready);
    assert!(resolved.is_ready);
    assert!(!resolved.is_blocked);
    assert!(resolved.exposes_why_context_entrypoint);
}

#[test]
fn resolver_remote_connection_states_map_to_postures() {
    for (conn, posture, degraded) in [
        (
            M5RemoteConnectionState::Connected,
            M5RemoteTargetPosture::ConnectedHealthy,
            false,
        ),
        (
            M5RemoteConnectionState::Connecting,
            M5RemoteTargetPosture::Establishing,
            false,
        ),
        (
            M5RemoteConnectionState::Reconnecting,
            M5RemoteTargetPosture::Reconnecting,
            true,
        ),
        (
            M5RemoteConnectionState::Disconnected,
            M5RemoteTargetPosture::Disconnected,
            true,
        ),
        (
            M5RemoteConnectionState::OfflineCached,
            M5RemoteTargetPosture::OfflineCached,
            true,
        ),
    ] {
        let input = M5RunContextResolutionInput {
            host_boundary: M5HostBoundaryClass::RemoteSshHost,
            connection_state: Some(conn),
            ..ready_local("remote")
        };
        let resolved = resolve_run_context(&input).expect("resolves");
        assert!(resolved.is_remote);
        assert_eq!(resolved.target_posture, posture);
        assert_eq!(resolved.target_is_degraded, degraded);
    }
}

#[test]
fn resolver_provenance_drives_readiness() {
    // Cached and narrowed values are degraded, never ready.
    let cached = M5RunContextResolutionInput {
        effective_value_provenance: M5EffectiveValueProvenance::CachedOffline,
        ..ready_local("cached")
    };
    assert_eq!(
        resolve_run_context(&cached).unwrap().readiness,
        M5EnvironmentReadiness::DegradedCached
    );

    let narrowed = M5RunContextResolutionInput {
        effective_value_provenance: M5EffectiveValueProvenance::NarrowedApproximate,
        ..ready_local("narrowed")
    };
    let narrowed = resolve_run_context(&narrowed).unwrap();
    assert_eq!(narrowed.readiness, M5EnvironmentReadiness::DegradedNarrowed);
    assert!(!narrowed.is_ready);

    // Policy-blocked and unresolved values block before work starts.
    let blocked = M5RunContextResolutionInput {
        effective_value_provenance: M5EffectiveValueProvenance::PolicyBlocked,
        ..ready_local("blocked")
    };
    let blocked = resolve_run_context(&blocked).unwrap();
    assert_eq!(blocked.readiness, M5EnvironmentReadiness::BlockedByPolicy);
    assert!(blocked.is_blocked);

    let unresolved = M5RunContextResolutionInput {
        effective_value_provenance: M5EffectiveValueProvenance::Unresolved,
        ..ready_local("unresolved")
    };
    let unresolved = resolve_run_context(&unresolved).unwrap();
    assert_eq!(
        unresolved.readiness,
        M5EnvironmentReadiness::BlockedUnresolved
    );
    assert!(unresolved.is_blocked);
}

#[test]
fn resolver_unreachable_remote_is_degraded_even_when_value_resolved() {
    let input = M5RunContextResolutionInput {
        host_boundary: M5HostBoundaryClass::RemoteSshHost,
        connection_state: Some(M5RemoteConnectionState::Reconnecting),
        effective_value_provenance: M5EffectiveValueProvenance::Resolved,
        ..ready_local("remote")
    };
    let resolved = resolve_run_context(&input).expect("resolves");
    assert_eq!(
        resolved.readiness,
        M5EnvironmentReadiness::DegradedUnreachableTarget
    );
    assert!(!resolved.is_ready);

    // A disconnected-but-resolved remote is also degraded-unreachable, but a
    // policy-blocked value on the same connection is still blocked (provenance wins).
    let blocked = M5RunContextResolutionInput {
        host_boundary: M5HostBoundaryClass::RemoteSshHost,
        connection_state: Some(M5RemoteConnectionState::Disconnected),
        effective_value_provenance: M5EffectiveValueProvenance::PolicyBlocked,
        ..ready_local("remote")
    };
    assert_eq!(
        resolve_run_context(&blocked).unwrap().readiness,
        M5EnvironmentReadiness::BlockedByPolicy
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_title = M5RunContextResolutionInput {
        context_title: "  ".to_owned(),
        ..ready_local("x")
    };
    assert_eq!(
        resolve_run_context(&empty_title),
        Err(M5RunContextResolutionError::EmptyContextTitle)
    );

    let empty_kind = M5RunContextResolutionInput {
        runtime_kind_repr: "".to_owned(),
        ..ready_local("x")
    };
    assert_eq!(
        resolve_run_context(&empty_kind),
        Err(M5RunContextResolutionError::EmptyRuntimeKind)
    );

    let empty_runtime = M5RunContextResolutionInput {
        resolved_runtime_repr: " ".to_owned(),
        ..ready_local("x")
    };
    assert_eq!(
        resolve_run_context(&empty_runtime),
        Err(M5RunContextResolutionError::EmptyResolvedRuntime)
    );

    let remote_no_conn = M5RunContextResolutionInput {
        host_boundary: M5HostBoundaryClass::RemoteSshHost,
        connection_state: None,
        ..ready_local("x")
    };
    assert_eq!(
        resolve_run_context(&remote_no_conn),
        Err(M5RunContextResolutionError::RemoteHostMissingConnectionState)
    );

    let local_with_conn = M5RunContextResolutionInput {
        connection_state: Some(M5RemoteConnectionState::Connected),
        ..ready_local("x")
    };
    assert_eq!(
        resolve_run_context(&local_with_conn),
        Err(M5RunContextResolutionError::LocalHostWithConnectionState)
    );

    let forbidden = M5RunContextResolutionInput {
        resolved_runtime_repr: "node from https://example.test".to_owned(),
        ..ready_local("x")
    };
    assert_eq!(
        resolve_run_context(&forbidden),
        Err(M5RunContextResolutionError::ForbiddenContextMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_remote_target_environment_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_run_surface() {
    let packet = seeded_m5_remote_target_environment_primitive_packet();
    let present: std::collections::BTreeSet<_> =
        packet.surface_rows.iter().map(|r| r.run_surface).collect();
    for surface in M5RunCapableSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing run surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.surface_rows.len(), M5RunCapableSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_remote_target_environment_primitive_packet();
    for row in &packet.surface_rows {
        for part in M5RemoteTargetPillPart::MANDATORY {
            assert!(row.pill_parts.contains(&part));
        }
        for part in M5EnvironmentStripPart::MANDATORY {
            assert!(row.strip_parts.contains(&part));
        }
        for field in M5RunContextExportField::MANDATORY {
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
    let packet = seeded_m5_remote_target_environment_primitive_packet();
    let cases: Vec<&M5RunContextResolutionCase> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5RemoteTargetPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.target_posture == posture),
            "no worked resolution exercises target posture {}",
            posture.as_str()
        );
    }
    for readiness in M5EnvironmentReadiness::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.readiness == readiness),
            "no worked resolution exercises readiness {}",
            readiness.as_str()
        );
    }
    for provenance in M5EffectiveValueProvenance::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.effective_value_provenance == provenance),
            "no worked resolution exercises provenance {}",
            provenance.as_str()
        );
    }
    for scope in M5ResolvedScope::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.scope == scope),
            "no worked resolution exercises scope {}",
            scope.as_str()
        );
    }
    for host in M5HostBoundaryClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.host_boundary == host),
            "no worked resolution exercises host boundary {}",
            host.as_str()
        );
    }
    for conn in M5RemoteConnectionState::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.connection_state == Some(conn)),
            "no worked resolution exercises connection state {}",
            conn.as_str()
        );
    }
    for source in M5RuntimeSourceClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.runtime_source == source),
            "no worked resolution exercises runtime source {}",
            source.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_remote_target_environment_primitive_packet();
    for row in &packet.surface_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.run_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_run_surface_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet
        .surface_rows
        .retain(|row| row.run_surface != M5RunCapableSurface::RequestRunner);
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.vocabulary_set.readiness_states.pop();
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_pill_part_missing_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.surface_rows[0]
        .pill_parts
        .retain(|p| *p != M5RemoteTargetPillPart::HostOrEnvironmentClass);
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::MandatoryPillPartMissing));
}

#[test]
fn mandatory_strip_part_missing_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.surface_rows[0]
        .strip_parts
        .retain(|p| *p != M5EnvironmentStripPart::WhyThisContextEntrypoint);
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::MandatoryStripPartMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5RunContextExportField::Readiness);
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.surface_rows[0].example_resolutions[0]
        .resolved
        .is_ready = false;
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.surface_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn policy_blocked_readiness_unproven_fails_when_no_blocked_example_present() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    // Drop every policy-blocked example across the matrix and confirm the packet-level
    // lint fires; replace with a ready example so rows still carry one.
    for row in &mut packet.surface_rows {
        row.example_resolutions.retain(|c| {
            c.resolved.effective_value_provenance != M5EffectiveValueProvenance::PolicyBlocked
        });
        if row.example_resolutions.is_empty() {
            row.example_resolutions
                .push(M5RunContextResolutionCase::resolved(
                    M5RunContextResolutionInput {
                        context_title: "placeholder".to_owned(),
                        host_boundary: M5HostBoundaryClass::LocalHost,
                        connection_state: None,
                        runtime_kind_repr: "node".to_owned(),
                        resolved_runtime_repr: "node 20.11.0".to_owned(),
                        runtime_source: M5RuntimeSourceClass::ProjectPinned,
                        scope: M5ResolvedScope::ProjectScope,
                        effective_value_provenance: M5EffectiveValueProvenance::Resolved,
                    },
                ));
        }
    }
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::PolicyBlockedReadinessUnproven));
}

#[test]
fn surface_invariant_violation_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.surface_rows[0].conflates_ready_and_degraded_or_blocked = true;
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::SurfaceInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.surface_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet
        .governance_review
        .cached_narrowed_or_blocked_never_shown_as_ready = false;
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet
        .consumer_projection
        .readiness_resolver_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RemoteTargetEnvironmentPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_run_surface() {
    let summary = seeded_m5_remote_target_environment_primitive_packet().render_markdown_summary();
    for surface in M5RunCapableSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing run surface {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_remote_target_environment_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5RunCapableSurface::ALL.len());
    assert!(lines[0].starts_with("run_surface,qualification,owner,"));
    for surface in M5RunCapableSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing run surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_remote_target_environment_primitive_export()
        .expect("checked M5 remote-target / environment primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_remote_target_environment_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed(),
        seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.surface_rows.len(), M5RunCapableSurface::ALL.len());
    }

    let incident = seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed();
    let row = incident
        .surface_rows
        .iter()
        .find(|r| r.run_surface == M5RunCapableSurface::IncidentSurface)
        .expect("incident surface row present");
    assert_eq!(row.qualification, M5RuntimeBoundaryQualificationClass::Beta);

    let pipeline = seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed();
    let row = pipeline
        .surface_rows
        .iter()
        .find(|r| r.run_surface == M5RunCapableSurface::PipelineRun)
        .expect("pipeline run row present");
    assert_eq!(
        row.qualification,
        M5RuntimeBoundaryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let incident: M5RemoteTargetEnvironmentPrimitivePacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-remote-target-environment-primitive/incident_surface_beta_narrowed.json"
        )
    ))
    .expect("incident fixture parses");
    assert!(incident.validate().is_empty());
    assert_eq!(
        incident,
        seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed()
    );

    let pipeline: M5RemoteTargetEnvironmentPrimitivePacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-remote-target-environment-primitive/pipeline_run_preview_narrowed.json"
        )
    ))
    .expect("pipeline fixture parses");
    assert!(pipeline.validate().is_empty());
    assert_eq!(
        pipeline,
        seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_remote_target_environment_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
