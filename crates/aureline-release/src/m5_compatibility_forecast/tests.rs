//! Inline tests for the compatibility-forecast lane.

use super::*;

fn packet() -> CompatibilityForecastSheet {
    seeded_m5_compatibility_forecast_sheet()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COMPATIBILITY_FORECAST_PACKET_ID);
    assert_eq!(packet.record_kind, M5_COMPATIBILITY_FORECAST_RECORD_KIND);
    assert_eq!(packet.subjects.len(), CompatibilitySubject::ALL.len());
    assert_eq!(packet.consumers.len(), ForecastConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_subject_is_forecast_exactly_once() {
    let packet = packet();
    for subject in CompatibilitySubject::ALL {
        let matches: Vec<&SubjectForecast> = packet
            .subjects
            .iter()
            .filter(|s| s.subject == subject)
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "subject `{}` not forecast once",
            subject.as_str()
        );
        let forecast = matches[0];
        assert_eq!(
            forecast.primary_artifact_class,
            subject.primary_artifact_class()
        );
        assert_eq!(forecast.owner_role, subject.owner_role());
        // Every claimed line is forecast.
        assert_eq!(forecast.line_forecasts.len(), CompatibilityLine::ALL.len());
    }
}

#[test]
fn forecast_identifies_affected_subjects_before_widening() {
    // Acceptance criterion: compatibility forecasts identify affected archetypes / schemas / extensions
    // / helpers before restart or rollout widening.
    let packet = packet();
    // The narrowed subjects are surfaced with their per-line drift and a non-clear readiness.
    let sdk = packet
        .subject(CompatibilitySubject::ExtensionSdkRange)
        .unwrap();
    assert_eq!(sdk.readiness, ForecastReadiness::ReviewBeforeWidening);
    assert!(sdk
        .line_forecasts
        .iter()
        .any(|l| l.line == CompatibilityLine::Stable
            && l.drift_class == DriftClass::MigrationRequired));
    assert!(sdk.requires_migration_task);
    // The clear subjects are clear to widen.
    let archetype = packet
        .subject(CompatibilitySubject::CertifiedArchetype)
        .unwrap();
    assert_eq!(archetype.readiness, ForecastReadiness::ClearToWiden);
    assert!(!archetype.requires_migration_task);
}

#[test]
fn every_narrowed_subject_has_an_actionable_migration_task() {
    // Acceptance criterion: a drifting subject routes to a concrete, actionable migration task.
    let packet = packet();
    for subject in &packet.subjects {
        if subject.requires_migration_task {
            let tasks = packet.tasks_for(subject.subject);
            assert!(
                !tasks.is_empty(),
                "narrowed subject `{}` has no migration task",
                subject.subject.as_str()
            );
            for task in tasks {
                // Each task discloses the actionable fields the spec requires.
                assert_ne!(task.task_class, MigrationTaskClass::NoActionRequired);
                assert!(!task.owner_role.is_empty());
                assert!(!task.affected_artifact_classes.is_empty());
                assert_ne!(task.due_before, DueBoundary::NotRequired);
                assert_ne!(task.rollback_guidance, RollbackGuidance::NotApplicable);
                assert!(!task.available_actions.is_empty());
            }
        }
    }
    assert!(packet.summary.total_tasks >= 1);
}

#[test]
fn compatible_is_distinguished_from_breaking() {
    // Guardrail: a compatible-within-window forecast must not read like a breaking drift.
    let packet = packet();
    let remote = packet
        .subject(CompatibilitySubject::RemoteAgentSkew)
        .unwrap();
    // A scheduled deprecation narrows but never holds.
    assert_eq!(remote.worst_gate, DescriptorGate::Narrowed);
    assert!(!remote.requires_pre_rollout_resolution);

    let hold = seeded_m5_compatibility_forecast_sheet_hold();
    let archetype = hold
        .subject(CompatibilitySubject::CertifiedArchetype)
        .unwrap();
    assert!(archetype
        .line_forecasts
        .iter()
        .any(|l| l.drift_class == DriftClass::BreakingDrift));
    assert_eq!(archetype.worst_gate, DescriptorGate::Blocked);
    assert!(archetype.requires_pre_rollout_resolution);
}

#[test]
fn confirmed_migration_narrows_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a real migration forecast surfaces before widening and narrows the right
    // consumers without forcing a resolution.
    let packet = seeded_m5_compatibility_forecast_sheet_review();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let subject = CompatibilitySubject::PublicSchemaReader;
    assert_eq!(
        packet.subject(subject).unwrap().worst_gate,
        DescriptorGate::Narrowed
    );
    for c in &packet.consumers {
        if c.read_subjects.contains(&subject) {
            assert!(
                c.is_review(),
                "consumer `{}` reads migration but did not review",
                c.consumer.as_str()
            );
            assert!(c.gaps.iter().any(
                |g| g.subject == subject && g.gap_kind == ForecastGapKind::ReviewBeforeWidening
            ));
        } else {
            assert!(
                c.is_clear(),
                "consumer `{}` should stay clear",
                c.consumer.as_str()
            );
        }
    }
    assert!(!packet.requires_pre_rollout_resolution());
    assert!(packet
        .release_gate
        .affected_subjects
        .contains(&subject.as_str().to_owned()));
}

#[test]
fn confirmed_breaking_drift_holds_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a breaking drift must surface before widening and force resolution from the
    // consumers that read it.
    let packet = seeded_m5_compatibility_forecast_sheet_hold();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let subject = CompatibilitySubject::CertifiedArchetype;
    assert_eq!(
        packet.subject(subject).unwrap().worst_gate,
        DescriptorGate::Blocked
    );
    for c in &packet.consumers {
        if c.read_subjects.contains(&subject) {
            assert!(
                c.is_hold(),
                "consumer `{}` reads breaking drift but did not hold",
                c.consumer.as_str()
            );
            assert!(c.requires_pre_rollout_resolution);
            assert!(c
                .gaps
                .iter()
                .any(|g| g.subject == subject
                    && g.gap_kind == ForecastGapKind::ResolveBeforeWidening));
        }
    }
    assert!(packet.requires_pre_rollout_resolution());
    assert!(packet.summary.hold_consumers >= 1);
}

#[test]
fn out_of_window_forecast_is_never_a_hard_failure() {
    // Guardrail: do not overstate coverage for subjects outside Aureline's claimed window; label them,
    // never raise them as a hard failure.
    let packet = seeded_m5_compatibility_forecast_sheet_out_of_window();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let subject = CompatibilitySubject::ExtensionSdkRange;
    let forecast = packet.subject(subject).unwrap();
    assert!(!forecast.within_claimed_window);
    assert!(forecast.out_of_window_message_id.is_some());
    // The underlying drift is breaking, but out-of-window caps the worst gate at narrowed.
    assert!(forecast
        .line_forecasts
        .iter()
        .any(|l| l.drift_class == DriftClass::BreakingDrift));
    assert_eq!(forecast.worst_gate, DescriptorGate::Narrowed);
    assert!(!forecast.requires_pre_rollout_resolution);
    assert!(!packet.requires_pre_rollout_resolution());
    for c in &packet.consumers {
        if c.read_subjects.contains(&subject) {
            assert!(c.is_review());
            assert!(c.gaps.iter().any(
                |g| g.subject == subject && g.gap_kind == ForecastGapKind::OutsideClaimedWindow
            ));
        }
    }
    assert!(packet.coverage.has_partial_coverage);
    assert!(packet.coverage.outside_window_lines >= 1);
}

#[test]
fn tampering_an_out_of_window_line_to_blocked_is_rejected() {
    // The guardrail is enforced in validation, not just in the builder.
    let mut packet = seeded_m5_compatibility_forecast_sheet_out_of_window();
    let s_idx = packet
        .subjects
        .iter()
        .position(|s| !s.within_claimed_window)
        .expect("an out-of-window subject exists");
    let l_idx = packet.subjects[s_idx]
        .line_forecasts
        .iter()
        .position(|l| l.confidence.caps_below_blocked())
        .expect("a capped line exists");
    packet.subjects[s_idx].line_forecasts[l_idx].gate = DescriptorGate::Blocked;
    let violations = packet.validate();
    assert!(
        violations.contains(&CompatibilityForecastViolation::SpeculativeHardFailure)
            || violations.contains(&CompatibilityForecastViolation::SubjectDerivationDrift),
        "{violations:?}"
    );
}

#[test]
fn waiver_requires_recorded_rationale_where_required() {
    // Acceptance criterion: migration tasks are suppressible only with recorded rationale where
    // required.
    let packet = packet();
    let task = packet
        .migration_tasks
        .iter()
        .find(|t| t.is_waived())
        .expect("a waived task exists in the canonical packet");
    assert!(task.requires_recorded_rationale);
    let rationale = task.waiver.as_ref().unwrap().rationale.as_ref().unwrap();
    assert!(!rationale.trim().is_empty());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn waiver_without_rationale_is_rejected() {
    let mut packet = packet();
    let idx = packet
        .migration_tasks
        .iter()
        .position(|t| t.is_waived() && t.requires_recorded_rationale)
        .expect("a rationale-gated waived task exists");
    if let Some(waiver) = packet.migration_tasks[idx].waiver.as_mut() {
        waiver.rationale = None;
    }
    assert!(packet
        .validate()
        .contains(&CompatibilityForecastViolation::WaiverRationaleMissing));
}

#[test]
fn waiving_a_non_skippable_task_is_rejected() {
    let mut packet = seeded_m5_compatibility_forecast_sheet_hold();
    let idx = packet
        .migration_tasks
        .iter()
        .position(|t| t.skip_policy == SkipPolicy::NotSkippable)
        .expect("a non-skippable task exists in the hold drill");
    packet.migration_tasks[idx].waiver = Some(MigrationWaiver {
        waived: true,
        rationale: Some("attempted suppression".to_owned()),
        waived_by_role: Some("release_owner".to_owned()),
        waiver_message_id: format!(
            "{}task.{}.{}.waiver",
            M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
            packet.migration_tasks[idx].subject.as_str(),
            packet.migration_tasks[idx].task_class.as_str(),
        ),
    });
    assert!(packet
        .validate()
        .contains(&CompatibilityForecastViolation::IllegalWaiver));
}

#[test]
fn narrowed_subject_without_a_task_is_rejected() {
    // Mixed-version / drift conditions must route to a concrete task, not remain implicit.
    let mut packet = seeded_m5_compatibility_forecast_sheet_review();
    packet.migration_tasks.clear();
    assert!(packet
        .validate()
        .contains(&CompatibilityForecastViolation::MissingMigrationTask));
}

#[test]
fn consumers_read_one_sheet() {
    // Acceptance criterion: every consumer reads one sheet and derives its disclosed scope from it.
    let packet = packet();
    assert_eq!(
        packet.consumer_tokens,
        tokens(&ForecastConsumer::ALL, |c| c.as_str())
    );
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.consumers_read_one_sheet);
    for c in &packet.consumers {
        let mut expected: Vec<ArtifactClass> = Vec::new();
        for &subject in &c.read_subjects {
            expected.extend(
                packet
                    .subject(subject)
                    .unwrap()
                    .affected_artifact_classes
                    .iter()
                    .copied(),
            );
        }
        expected.sort_by_key(|x| artifact_rank(*x));
        expected.dedup();
        assert_eq!(c.disclosed_artifact_classes, expected);
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(ForecastChannel::DesktopUi);
    let cli = packet.render_for_channel(ForecastChannel::CliHeadless);
    let export = packet.render_for_channel(ForecastChannel::OfflineExport);
    assert_eq!(desktop, cli);
    assert_eq!(cli, export);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = ForecastVocabulary::canonical();
    assert_eq!(vocab.subjects.len(), CompatibilitySubject::ALL.len());
    assert_eq!(vocab.lines.len(), CompatibilityLine::ALL.len());
    for needle in [
        "certified_archetype",
        "extension_sdk_range",
        "extension_manifest_range",
        "remote_agent_skew",
        "public_export_reader",
        "public_schema_reader",
    ] {
        assert!(
            vocab.subjects.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in ["stable", "beta", "preview", "lts"] {
        assert!(vocab.lines.contains(&needle.to_owned()), "missing {needle}");
    }
    for needle in ["no_drift", "migration_required", "breaking_drift"] {
        assert!(
            vocab.drift_classes.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in ["pin", "postpone", "side_by_side", "validator", "repair"] {
        assert!(
            vocab.actions.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in ["qualified", "outside_claimed_window", "unknown"] {
        assert!(
            vocab.confidence_levels.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
}

#[test]
fn packet_round_trips() {
    for packet in [
        seeded_m5_compatibility_forecast_sheet(),
        seeded_m5_compatibility_forecast_sheet_review(),
        seeded_m5_compatibility_forecast_sheet_hold(),
        seeded_m5_compatibility_forecast_sheet_out_of_window(),
    ] {
        let json = packet.export_safe_json();
        let parsed: CompatibilityForecastSheet =
            serde_json::from_str(&json).expect("packet deserializes");
        assert_eq!(parsed, packet);
        assert!(parsed.validate().is_empty(), "{:?}", parsed.validate());
    }
}

#[test]
fn task_csv_enumerates_every_task() {
    let csv = packet().render_task_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("task_id,subject,task_class,"));
    assert!(header.contains("rollback_guidance"));
    assert!(header.contains("requires_recorded_rationale"));
    let rows = csv.lines().count() - 1;
    assert_eq!(rows, packet().migration_tasks.len());
}

#[test]
fn markdown_summary_names_subjects_tasks_and_consumers() {
    let md = seeded_m5_compatibility_forecast_sheet_review().render_markdown_summary();
    assert!(md.contains("compatibility forecast"));
    assert!(md.contains("Compatibility forecasts"));
    assert!(md.contains("Migration tasks"));
    assert!(md.contains("public_schema_reader"));
    assert!(md.contains("gap:"));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_compatibility_forecast_sheet_review();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_review())
        .expect("a review consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].readiness = ForecastReadiness::ClearToWiden;
    assert!(packet
        .validate()
        .contains(&CompatibilityForecastViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_subject_derivation_is_rejected() {
    let mut packet = packet();
    packet.subjects[0].line_forecasts[0].drift_class = DriftClass::BreakingDrift;
    let violations = packet.validate();
    assert!(
        violations.contains(&CompatibilityForecastViolation::SubjectDerivationDrift)
            || violations.contains(&CompatibilityForecastViolation::SummaryDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_subject_is_rejected() {
    let mut packet = packet();
    packet
        .subjects
        .retain(|s| s.subject != CompatibilitySubject::ExtensionManifestRange);
    assert!(packet
        .validate()
        .contains(&CompatibilityForecastViolation::SubjectCoverageDrift));
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_compatibility_forecast_sheet(),
        seeded_m5_compatibility_forecast_sheet_review(),
        seeded_m5_compatibility_forecast_sheet_hold(),
        seeded_m5_compatibility_forecast_sheet_out_of_window(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
