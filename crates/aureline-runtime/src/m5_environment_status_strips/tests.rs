use super::*;

fn packet() -> M5EnvironmentStatusStrips {
    current_m5_environment_status_strips().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_ENVIRONMENT_STATUS_STRIP_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_strips() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_run_surface_has_exactly_one_strip() {
    let packet = packet();
    assert_eq!(packet.strips.len(), RunSurface::ALL.len());
    for surface in RunSurface::ALL {
        assert!(
            packet.strip_for(surface).is_some(),
            "missing strip for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_strip_carries_one_step_explainability() {
    // Every run-capable surface can answer where the run happens and why in one step, including the
    // CLI/headless equivalent — even when the environment is blocked.
    let packet = packet();
    for strip in &packet.strips {
        assert!(
            strip.has_one_step_explainability(),
            "{} lacks one-step explainability",
            strip.strip_id
        );
        assert!(
            !strip.explain_entrypoint_ref.trim().is_empty(),
            "{} has no explain entrypoint",
            strip.strip_id
        );
        assert!(
            !strip.cli_object_ref.trim().is_empty(),
            "{} has no CLI-equivalent object",
            strip.strip_id
        );
    }
}

#[test]
fn every_strip_shows_a_facet_and_carries_refs() {
    let packet = packet();
    for strip in &packet.strips {
        assert!(
            !strip.shown_facets.is_empty(),
            "{} shows no facet",
            strip.strip_id
        );
        assert!(strip.has_required_evidence(), "{}", strip.strip_id);
    }
}

#[test]
fn every_strip_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_strips_gate_consistent());
    for strip in &packet.strips {
        assert_eq!(
            strip.presentation,
            strip.effective_presentation(),
            "{}",
            strip.strip_id
        );
        assert_eq!(
            strip.downgrade_reasons,
            strip.computed_downgrade_reasons(),
            "{}",
            strip.strip_id
        );
        assert_eq!(
            strip.resolution_path,
            strip.computed_resolution_path(),
            "{}",
            strip.strip_id
        );
        assert_eq!(
            strip.blocked_before_run,
            strip.effective_presentation().warns_before_run(),
            "{}",
            strip.strip_id
        );
    }
}

#[test]
fn resolved_strips_are_whole() {
    let packet = packet();
    let resolved = packet.resolved_strips().count();
    assert!(
        resolved >= 2,
        "fixture needs at least two cleanly resolved strips to prove the gate is not a blanket flag"
    );
    for strip in packet.resolved_strips() {
        assert!(strip.all_facets_current());
        assert_eq!(strip.status, ContextStatusClass::Resolved);
        assert!(strip.downgrade_reasons.is_empty());
        assert!(strip.caveats.is_empty());
        assert!(strip.unmet_or_stale_fields.is_empty());
        assert!(!strip.resolution_path.is_offered());
        assert!(!strip.blocked_before_run);
    }
}

#[test]
fn flagged_and_blocked_strips_name_resolution_and_caveats() {
    let packet = packet();
    for strip in &packet.strips {
        if strip.effective_presentation().requires_attention() {
            assert!(strip.resolution_path.is_offered(), "{}", strip.strip_id);
            assert!(!strip.caveats.is_empty(), "{}", strip.strip_id);
            assert!(
                !strip.unmet_or_stale_fields.is_empty(),
                "{}",
                strip.strip_id
            );
        }
    }
}

#[test]
fn blocked_strips_warn_before_run() {
    // A blocked environment becomes visible before the downstream run failure, not after.
    let packet = packet();
    for strip in packet.blocked_strips() {
        assert_eq!(strip.presentation, StripPresentation::Blocked);
        assert!(
            strip.blocked_before_run,
            "{} is blocked but does not warn before run",
            strip.strip_id
        );
    }
    for strip in &packet.strips {
        if strip.effective_presentation() != StripPresentation::Blocked {
            assert!(
                !strip.blocked_before_run,
                "{} warns before run without being blocked",
                strip.strip_id
            );
        }
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in StripConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_strips_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.strips.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_strips_gate_consistent,
        packet.all_strips_gate_consistent()
    );
    assert_eq!(projection.resolved_count, packet.resolved_strips().count());
    assert_eq!(projection.flagged_count, packet.flagged_strips().count());
    assert_eq!(projection.blocked_count, packet.blocked_strips().count());
    for (strip, row) in packet.strips.iter().zip(projection.rows.iter()) {
        assert_eq!(row.presentation, strip.presentation.as_str());
        assert_eq!(row.resolved, strip.is_resolved());
        assert_eq!(row.explain_entrypoint_ref, strip.explain_entrypoint_ref);
        assert_eq!(row.cli_object_ref, strip.cli_object_ref);
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("support:m5:execution-context", "2026-06-16T13:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, packet.packet_id);
    assert!(export.raw_private_material_excluded);
}

#[test]
fn presentations_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<StripPresentation> =
        packet.strips.iter().map(|s| s.presentation).collect();
    for decision in StripPresentation::ALL {
        assert!(
            present.contains(&decision),
            "no strip exercises {}",
            decision.as_str()
        );
    }
}

#[test]
fn status_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ContextStatusClass> = packet.strips.iter().map(|s| s.status).collect();
    for status in ContextStatusClass::ALL {
        assert!(
            present.contains(&status),
            "no strip exercises status {}",
            status.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<StripDowngradeReason> = packet
        .strips
        .iter()
        .flat_map(|s| s.downgrade_reasons.iter().copied())
        .collect();
    for reason in StripDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no strip exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn resolution_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ContextResolutionPath> =
        packet.strips.iter().map(|s| s.resolution_path).collect();
    for path in ContextResolutionPath::ALL {
        assert!(
            present.contains(&path),
            "no strip exercises {}",
            path.as_str()
        );
    }
}

#[test]
fn freshness_and_facets_are_exhaustive() {
    let packet = packet();
    let freshness: BTreeSet<ContextFreshness> = packet
        .strips
        .iter()
        .flat_map(|s| s.shown_facets.iter().map(|f| f.freshness))
        .collect();
    for state in ContextFreshness::ALL {
        assert!(
            freshness.contains(&state),
            "no facet exercises {}",
            state.as_str()
        );
    }
    let facets: BTreeSet<ContextFacet> = packet
        .strips
        .iter()
        .flat_map(|s| s.shown_facets.iter().map(|f| f.facet))
        .collect();
    for facet in ContextFacet::ALL {
        assert!(facets.contains(&facet), "no strip shows {}", facet.as_str());
    }
}

#[test]
fn stale_target_flags_strip() {
    let packet = packet();
    let strip = packet.strip_for(RunSurface::Debug).expect("debug strip");
    assert_eq!(strip.status, ContextStatusClass::Stale);
    assert_eq!(strip.presentation, StripPresentation::Flagged);
    assert_eq!(strip.resolution_path, ContextResolutionPath::RefreshTarget);
    assert!(strip
        .downgrade_reasons
        .contains(&StripDowngradeReason::StaleContext));
    assert!(!strip.blocked_before_run);
}

#[test]
fn blocked_environment_blocks_strip_before_run() {
    let packet = packet();
    let strip = packet
        .strip_for(RunSurface::Database)
        .expect("database strip");
    assert_eq!(strip.status, ContextStatusClass::Blocked);
    assert_eq!(strip.presentation, StripPresentation::Blocked);
    assert_eq!(
        strip.resolution_path,
        ContextResolutionPath::UnblockEnvironment
    );
    assert!(strip
        .downgrade_reasons
        .contains(&StripDowngradeReason::BlockedEnvironment));
    assert!(strip.blocked_before_run);
}

#[test]
fn remote_drift_flags_strip_and_reconnects() {
    let packet = packet();
    let strip = packet
        .strip_for(RunSurface::Request)
        .expect("request strip");
    assert_eq!(strip.status, ContextStatusClass::RemoteDrift);
    assert_eq!(strip.presentation, StripPresentation::Flagged);
    assert_eq!(
        strip.resolution_path,
        ContextResolutionPath::ReconnectRemote
    );
    assert!(strip
        .downgrade_reasons
        .contains(&StripDowngradeReason::RemoteDrift));
}

#[test]
fn conflicting_context_flags_strip_and_resolves_conflict() {
    let packet = packet();
    let strip = packet
        .strip_for(RunSurface::Preview)
        .expect("preview strip");
    assert_eq!(strip.status, ContextStatusClass::Conflicting);
    assert_eq!(strip.presentation, StripPresentation::Flagged);
    assert_eq!(
        strip.resolution_path,
        ContextResolutionPath::ResolveConflict
    );
    assert_eq!(
        strip.downgrade_reasons,
        vec![StripDowngradeReason::ConflictingContext]
    );
}

#[test]
fn validate_flags_overstated_presentation() {
    let mut packet = packet();
    if let Some(strip) = packet
        .strips
        .iter_mut()
        .find(|s| s.effective_presentation() != StripPresentation::Resolved)
    {
        strip.presentation = StripPresentation::Resolved;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EnvironmentStatusStripViolation::OverstatedPresentation { .. }
        )));
    }
}

#[test]
fn validate_flags_generic_chip_without_facets() {
    let mut packet = packet();
    if let Some(strip) = packet.strips.first_mut() {
        strip.shown_facets.clear();
        assert!(packet
            .validate()
            .iter()
            .any(|v| matches!(v, M5EnvironmentStatusStripViolation::NoShownFacets { .. })));
    }
}

#[test]
fn validate_flags_missing_one_step_explainability() {
    let mut packet = packet();
    if let Some(strip) = packet.strips.first_mut() {
        strip.cli_object_ref = "  ".to_owned();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EnvironmentStatusStripViolation::MissingOneStepExplainability { .. }
        )));
    }
}

#[test]
fn validate_flags_blocked_strip_that_does_not_warn() {
    let mut packet = packet();
    if let Some(strip) = packet
        .strips
        .iter_mut()
        .find(|s| s.effective_presentation() == StripPresentation::Blocked)
    {
        strip.blocked_before_run = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EnvironmentStatusStripViolation::BlockedBeforeRunMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != StripConsumerSurface::SupportCenter);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5EnvironmentStatusStripViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_stops_preserving_object_ids() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.preserves_object_ids = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EnvironmentStatusStripViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_surface() {
    let mut packet = packet();
    packet.strips.retain(|s| s.surface != RunSurface::Run);
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5EnvironmentStatusStripViolation::MissingSurface { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_strips = packet.summary.total_strips.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5EnvironmentStatusStripViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(RunSurface::Incident.as_str(), "incident");
    assert_eq!(ContextFacet::RemoteTarget.as_str(), "remote_target");
    assert_eq!(ContextFreshness::Unknown.as_str(), "unknown");
    assert_eq!(ContextStatusClass::RemoteDrift.as_str(), "remote_drift");
    assert_eq!(StripPresentation::Blocked.as_str(), "blocked");
    assert_eq!(
        StripDowngradeReason::ConflictingContext.as_str(),
        "conflicting_context"
    );
    assert_eq!(ContextResolutionPath::NoneNeeded.as_str(), "none");
    assert_eq!(
        StripConsumerSurface::IssueReportPacket.as_str(),
        "issue_report_packet"
    );
}

#[test]
fn freshness_ceilings_hold_for_each_state() {
    assert_eq!(
        ContextFreshness::Fresh.presentation_ceiling(),
        StripPresentation::Resolved
    );
    assert_eq!(
        ContextFreshness::Stale.presentation_ceiling(),
        StripPresentation::Flagged
    );
    assert_eq!(
        ContextFreshness::Unknown.presentation_ceiling(),
        StripPresentation::Flagged
    );
}

#[test]
fn status_ceilings_hold_for_each_class() {
    assert_eq!(
        ContextStatusClass::Resolved.presentation_ceiling(),
        StripPresentation::Resolved
    );
    assert_eq!(
        ContextStatusClass::Stale.presentation_ceiling(),
        StripPresentation::Flagged
    );
    assert_eq!(
        ContextStatusClass::Blocked.presentation_ceiling(),
        StripPresentation::Blocked
    );
}
