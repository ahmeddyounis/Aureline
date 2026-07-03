//! Canonical seed builders for the compatibility-forecast sheet.
//!
//! These builders are the single producer of the checked-in forecast-sheet packet, the published
//! inventory, the release-grade migration-assistant parity proof (and its Markdown report), the
//! machine-readable per-task CSV export, and the per-state drill fixtures. The headless emitter and the
//! inline tests both call them so the in-code packet, the artifacts, and the fixtures never drift.
//!
//! The canonical packet forecasts a realistic update: most subjects are clear to widen, three drift
//! within the claimed window (so they narrow and each carries a concrete migration task — one of them
//! waived with a recorded rationale), and none break. The drills start from an all-clear base and
//! perturb exactly one subject so the derivation can be read in isolation:
//!
//! - the **review** drill makes one subject a *confirmed* migration, so the consumers that read it
//!   recommend review before widening;
//! - the **hold** drill makes one subject a *confirmed* breaking drift, so those consumers hold for
//!   resolution; and
//! - the **out-of-window** drill makes one subject a breaking drift forecast *outside Aureline's
//!   claimed window*, so the guardrail caps it at review rather than a hard failure.

use super::*;

/// Stable packet id for the canonical compatibility-forecast sheet.
pub const M5_COMPATIBILITY_FORECAST_PACKET_ID: &str = "m5-compatibility-forecast:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// The subject the review drill turns into a confirmed migration. Read by every consumer except the
/// admin console, so the drill narrows exactly the consumers that read it.
const REVIEW_DRILL_SUBJECT: CompatibilitySubject = CompatibilitySubject::PublicSchemaReader;

/// The subject the hold drill turns into a confirmed breaking drift. Read by every consumer, so the
/// drill holds all of them.
const HOLD_DRILL_SUBJECT: CompatibilitySubject = CompatibilitySubject::CertifiedArchetype;

/// The subject the out-of-window drill turns into a breaking drift forecast outside the claimed
/// window, so the guardrail caps it at review rather than a hold.
const OUT_OF_WINDOW_DRILL_SUBJECT: CompatibilitySubject = CompatibilitySubject::ExtensionSdkRange;

fn both_profiles() -> Vec<DeploymentProfile> {
    vec![DeploymentProfile::Managed, DeploymentProfile::SelfHosted]
}

fn evidence(subject: CompatibilitySubject) -> Vec<String> {
    vec![format!(
        "artifacts/release/m5-migration-assistant-proof/{}.evidence",
        subject.as_str()
    )]
}

/// Builds one line forecast.
fn lf(
    subject: CompatibilitySubject,
    line: CompatibilityLine,
    drift: DriftClass,
    confidence: ForecastConfidence,
) -> LineForecast {
    LineForecast::new(
        subject,
        LineForecastInput {
            line,
            drift_class: drift,
            confidence,
            supported_from: None,
            supported_to: None,
        },
    )
}

/// All four lines forecast clear (no drift, qualified) for a subject.
fn clear_lines(subject: CompatibilitySubject) -> Vec<LineForecast> {
    CompatibilityLine::ALL
        .iter()
        .map(|&line| {
            lf(
                subject,
                line,
                DriftClass::NoDrift,
                ForecastConfidence::Qualified,
            )
        })
        .collect()
}

/// A clear-to-widen subject forecast: no drift on any line, within the claimed window.
fn clear_subject(subject: CompatibilitySubject) -> SubjectForecast {
    SubjectForecast::new(SubjectForecastInput {
        subject,
        subject_id: format!("{}:current", subject.as_str()),
        within_claimed_window: true,
        line_forecasts: clear_lines(subject),
        affected_artifact_classes: vec![subject.primary_artifact_class()],
        affected_profiles: both_profiles(),
        evidence_refs: evidence(subject),
    })
}

/// The six all-clear subject forecasts the drills perturb.
fn all_clear_subjects() -> Vec<SubjectForecast> {
    CompatibilitySubject::ALL
        .iter()
        .map(|&s| clear_subject(s))
        .collect()
}

/// The canonical, realistic subject forecasts: three subjects drift within the claimed window.
fn canonical_subjects() -> Vec<SubjectForecast> {
    let mut subjects = all_clear_subjects();
    // Extension SDK range: a migration is required on stable/beta, scheduled deprecation on LTS.
    set_subject(
        &mut subjects,
        SubjectForecast::new(SubjectForecastInput {
            subject: CompatibilitySubject::ExtensionSdkRange,
            subject_id: "extension-sdk:3.x".to_owned(),
            within_claimed_window: true,
            line_forecasts: vec![
                lf(
                    CompatibilitySubject::ExtensionSdkRange,
                    CompatibilityLine::Stable,
                    DriftClass::MigrationRequired,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    CompatibilitySubject::ExtensionSdkRange,
                    CompatibilityLine::Beta,
                    DriftClass::MigrationRequired,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    CompatibilitySubject::ExtensionSdkRange,
                    CompatibilityLine::Preview,
                    DriftClass::CompatibleWithinWindow,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    CompatibilitySubject::ExtensionSdkRange,
                    CompatibilityLine::Lts,
                    DriftClass::DeprecationScheduled,
                    ForecastConfidence::Qualified,
                ),
            ],
            affected_artifact_classes: vec![ArtifactClass::ExtensionPacks],
            affected_profiles: both_profiles(),
            evidence_refs: evidence(CompatibilitySubject::ExtensionSdkRange),
        }),
    );
    // Remote-agent skew: a scheduled deprecation warns of a coming helper upgrade.
    set_subject(
        &mut subjects,
        SubjectForecast::new(SubjectForecastInput {
            subject: CompatibilitySubject::RemoteAgentSkew,
            subject_id: "remote-helper:1.8".to_owned(),
            within_claimed_window: true,
            line_forecasts: vec![
                lf(
                    CompatibilitySubject::RemoteAgentSkew,
                    CompatibilityLine::Stable,
                    DriftClass::DeprecationScheduled,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    CompatibilitySubject::RemoteAgentSkew,
                    CompatibilityLine::Beta,
                    DriftClass::DeprecationScheduled,
                    ForecastConfidence::Likely,
                ),
                lf(
                    CompatibilitySubject::RemoteAgentSkew,
                    CompatibilityLine::Preview,
                    DriftClass::CompatibleWithinWindow,
                    ForecastConfidence::Likely,
                ),
                lf(
                    CompatibilitySubject::RemoteAgentSkew,
                    CompatibilityLine::Lts,
                    DriftClass::NoDrift,
                    ForecastConfidence::Qualified,
                ),
            ],
            affected_artifact_classes: vec![ArtifactClass::CoreRuntime],
            affected_profiles: vec![DeploymentProfile::Managed],
            evidence_refs: evidence(CompatibilitySubject::RemoteAgentSkew),
        }),
    );
    // Public schema reader: a scheduled deprecation the user can defer with a recorded rationale.
    set_subject(
        &mut subjects,
        SubjectForecast::new(SubjectForecastInput {
            subject: CompatibilitySubject::PublicSchemaReader,
            subject_id: "schema-reader:v12".to_owned(),
            within_claimed_window: true,
            line_forecasts: vec![
                lf(
                    CompatibilitySubject::PublicSchemaReader,
                    CompatibilityLine::Stable,
                    DriftClass::DeprecationScheduled,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    CompatibilitySubject::PublicSchemaReader,
                    CompatibilityLine::Beta,
                    DriftClass::DeprecationScheduled,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    CompatibilitySubject::PublicSchemaReader,
                    CompatibilityLine::Preview,
                    DriftClass::MigrationRequired,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    CompatibilitySubject::PublicSchemaReader,
                    CompatibilityLine::Lts,
                    DriftClass::NoDrift,
                    ForecastConfidence::Qualified,
                ),
            ],
            affected_artifact_classes: vec![ArtifactClass::SchemaContracts],
            affected_profiles: both_profiles(),
            evidence_refs: evidence(CompatibilitySubject::PublicSchemaReader),
        }),
    );
    subjects
}

/// The canonical migration tasks: one per narrowed subject, exercising auto-fix, due boundaries, skip
/// policies, rollback guidance, available actions, and a recorded waiver.
fn canonical_tasks() -> Vec<MigrationTaskRow> {
    vec![
        MigrationTaskRow::new(MigrationTaskRowInput {
            task_id: "sdk-range-bump".to_owned(),
            subject: CompatibilitySubject::ExtensionSdkRange,
            task_class: MigrationTaskClass::ExtensionSdkRangeBump,
            addresses_drift: DriftClass::MigrationRequired,
            confidence: ForecastConfidence::Qualified,
            affected_artifact_classes: vec![ArtifactClass::ExtensionPacks],
            affected_profiles: both_profiles(),
            affected_lines: vec![CompatibilityLine::Stable, CompatibilityLine::Beta],
            auto_fix: AutoFixAvailability::AssistedFix,
            due_before: DueBoundary::BeforeRolloutWidening,
            skip_policy: SkipPolicy::SkippableWithRationale,
            rollback_guidance: RollbackGuidance::PinCurrentVersion,
            available_actions: vec![
                MigrationAction::Pin,
                MigrationAction::Validator,
                MigrationAction::Repair,
            ],
            waiver: None,
            evidence_refs: evidence(CompatibilitySubject::ExtensionSdkRange),
        }),
        MigrationTaskRow::new(MigrationTaskRowInput {
            task_id: "remote-helper-upgrade".to_owned(),
            subject: CompatibilitySubject::RemoteAgentSkew,
            task_class: MigrationTaskClass::RemoteHelperUpgrade,
            addresses_drift: DriftClass::DeprecationScheduled,
            confidence: ForecastConfidence::Qualified,
            affected_artifact_classes: vec![ArtifactClass::CoreRuntime],
            affected_profiles: vec![DeploymentProfile::Managed],
            affected_lines: vec![CompatibilityLine::Stable, CompatibilityLine::Beta],
            auto_fix: AutoFixAvailability::AdminRequired,
            due_before: DueBoundary::BeforeNextStableLine,
            skip_policy: SkipPolicy::OptionalRecommended,
            rollback_guidance: RollbackGuidance::SideBySideFallback,
            available_actions: vec![
                MigrationAction::Postpone,
                MigrationAction::SideBySide,
                MigrationAction::Validator,
            ],
            waiver: None,
            evidence_refs: evidence(CompatibilitySubject::RemoteAgentSkew),
        }),
        MigrationTaskRow::new(MigrationTaskRowInput {
            task_id: "schema-reader-migration".to_owned(),
            subject: CompatibilitySubject::PublicSchemaReader,
            task_class: MigrationTaskClass::SchemaReaderMigration,
            addresses_drift: DriftClass::MigrationRequired,
            confidence: ForecastConfidence::Qualified,
            affected_artifact_classes: vec![ArtifactClass::SchemaContracts],
            affected_profiles: both_profiles(),
            affected_lines: vec![CompatibilityLine::Preview],
            auto_fix: AutoFixAvailability::ManualOnly,
            due_before: DueBoundary::BeforeEndOfSupport,
            skip_policy: SkipPolicy::SkippableWithRationale,
            rollback_guidance: RollbackGuidance::RollbackSupported,
            available_actions: vec![MigrationAction::Validator, MigrationAction::Repair],
            // The deferral is recorded with a rationale, as the policy requires.
            waiver: Some(MigrationWaiver {
                waived: true,
                rationale: Some(
                    "Reader pinned to v12 until the next maintenance window.".to_owned(),
                ),
                waived_by_role: Some("release_owner".to_owned()),
                waiver_message_id: format!(
                    "{}task.{}.{}.waiver",
                    M5_COMPATIBILITY_FORECAST_MESSAGE_ID_PREFIX,
                    CompatibilitySubject::PublicSchemaReader.as_str(),
                    MigrationTaskClass::SchemaReaderMigration.as_str(),
                ),
            }),
            evidence_refs: evidence(CompatibilitySubject::PublicSchemaReader),
        }),
    ]
}

/// Replaces the forecast for a subject in place.
fn set_subject(subjects: &mut [SubjectForecast], replacement: SubjectForecast) {
    for forecast in subjects.iter_mut() {
        if forecast.subject == replacement.subject {
            *forecast = replacement.clone();
        }
    }
}

/// The claimed consumer rows. Update center, migration assistant, release center, and support export
/// read every subject; the admin console reads only the operationally governed subjects (archetype,
/// extension SDK / manifest, remote skew) — so a drill on a public reader narrows everyone but admin.
fn consumer_rows() -> Vec<ForecastConsumerRow> {
    vec![
        ForecastConsumerRow::new(ForecastConsumer::UpdateCenter, &CompatibilitySubject::ALL),
        ForecastConsumerRow::new(
            ForecastConsumer::MigrationAssistant,
            &CompatibilitySubject::ALL,
        ),
        ForecastConsumerRow::new(ForecastConsumer::ReleaseCenter, &CompatibilitySubject::ALL),
        ForecastConsumerRow::new(
            ForecastConsumer::AdminConsole,
            &[
                CompatibilitySubject::CertifiedArchetype,
                CompatibilitySubject::ExtensionSdkRange,
                CompatibilitySubject::ExtensionManifestRange,
                CompatibilitySubject::RemoteAgentSkew,
            ],
        ),
        ForecastConsumerRow::new(ForecastConsumer::SupportExport, &CompatibilitySubject::ALL),
    ]
}

/// The canonical forecast target.
fn canonical_target() -> ForecastTarget {
    ForecastTarget {
        channel: ChannelScope::Stable,
        lines: CompatibilityLine::ALL.to_vec(),
        profiles: both_profiles(),
        current_version: "1.8.0".to_owned(),
        target_version: "1.9.0".to_owned(),
        forecast_basis: ForecastBasis::ReleaseAndLocalScan,
    }
}

/// Assembles a packet from the given subjects and tasks.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    subjects: Vec<SubjectForecast>,
    migration_tasks: Vec<MigrationTaskRow>,
) -> CompatibilityForecastSheet {
    CompatibilityForecastSheet::new(CompatibilityForecastSheetInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        target: canonical_target(),
        subjects,
        migration_tasks,
        consumers: consumer_rows(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, realistic compatibility-forecast sheet: most subjects clear, three drift within the
/// claimed window with concrete migration tasks, none break.
pub fn seeded_m5_compatibility_forecast_sheet() -> CompatibilityForecastSheet {
    assemble_packet(
        M5_COMPATIBILITY_FORECAST_PACKET_ID,
        "Aureline M5 compatibility forecast",
        canonical_subjects(),
        canonical_tasks(),
    )
}

/// Drill: one subject is a confirmed migration on stable, so the consumers that read it recommend
/// review before widening while the rest stay clear and no resolution is forced.
pub fn seeded_m5_compatibility_forecast_sheet_review() -> CompatibilityForecastSheet {
    let mut subjects = all_clear_subjects();
    set_subject(
        &mut subjects,
        SubjectForecast::new(SubjectForecastInput {
            subject: REVIEW_DRILL_SUBJECT,
            subject_id: "schema-reader:v12".to_owned(),
            within_claimed_window: true,
            line_forecasts: vec![
                lf(
                    REVIEW_DRILL_SUBJECT,
                    CompatibilityLine::Stable,
                    DriftClass::MigrationRequired,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    REVIEW_DRILL_SUBJECT,
                    CompatibilityLine::Beta,
                    DriftClass::MigrationRequired,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    REVIEW_DRILL_SUBJECT,
                    CompatibilityLine::Preview,
                    DriftClass::CompatibleWithinWindow,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    REVIEW_DRILL_SUBJECT,
                    CompatibilityLine::Lts,
                    DriftClass::NoDrift,
                    ForecastConfidence::Qualified,
                ),
            ],
            affected_artifact_classes: vec![ArtifactClass::SchemaContracts],
            affected_profiles: both_profiles(),
            evidence_refs: evidence(REVIEW_DRILL_SUBJECT),
        }),
    );
    let task = MigrationTaskRow::new(MigrationTaskRowInput {
        task_id: "schema-reader-migration".to_owned(),
        subject: REVIEW_DRILL_SUBJECT,
        task_class: MigrationTaskClass::SchemaReaderMigration,
        addresses_drift: DriftClass::MigrationRequired,
        confidence: ForecastConfidence::Qualified,
        affected_artifact_classes: vec![ArtifactClass::SchemaContracts],
        affected_profiles: both_profiles(),
        affected_lines: vec![CompatibilityLine::Stable, CompatibilityLine::Beta],
        auto_fix: AutoFixAvailability::AssistedFix,
        due_before: DueBoundary::BeforeRolloutWidening,
        skip_policy: SkipPolicy::SkippableWithRationale,
        rollback_guidance: RollbackGuidance::RollbackSupported,
        available_actions: vec![MigrationAction::Validator, MigrationAction::Repair],
        waiver: None,
        evidence_refs: evidence(REVIEW_DRILL_SUBJECT),
    });
    assemble_packet(
        "m5-compatibility-forecast:drill-review:0001",
        "Aureline M5 compatibility forecast — review drill",
        subjects,
        vec![task],
    )
}

/// Drill: one subject is a confirmed breaking drift, so the consumers that read it hold for resolution
/// before rollout widening.
pub fn seeded_m5_compatibility_forecast_sheet_hold() -> CompatibilityForecastSheet {
    let mut subjects = all_clear_subjects();
    set_subject(
        &mut subjects,
        SubjectForecast::new(SubjectForecastInput {
            subject: HOLD_DRILL_SUBJECT,
            subject_id: "archetype:web-app".to_owned(),
            within_claimed_window: true,
            line_forecasts: vec![
                lf(
                    HOLD_DRILL_SUBJECT,
                    CompatibilityLine::Stable,
                    DriftClass::BreakingDrift,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    HOLD_DRILL_SUBJECT,
                    CompatibilityLine::Beta,
                    DriftClass::MigrationRequired,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    HOLD_DRILL_SUBJECT,
                    CompatibilityLine::Preview,
                    DriftClass::CompatibleWithinWindow,
                    ForecastConfidence::Qualified,
                ),
                lf(
                    HOLD_DRILL_SUBJECT,
                    CompatibilityLine::Lts,
                    DriftClass::NoDrift,
                    ForecastConfidence::Qualified,
                ),
            ],
            affected_artifact_classes: vec![ArtifactClass::WorkspaceState],
            affected_profiles: both_profiles(),
            evidence_refs: evidence(HOLD_DRILL_SUBJECT),
        }),
    );
    let task = MigrationTaskRow::new(MigrationTaskRowInput {
        task_id: "archetype-revalidation".to_owned(),
        subject: HOLD_DRILL_SUBJECT,
        task_class: MigrationTaskClass::ArchetypeRevalidation,
        addresses_drift: DriftClass::BreakingDrift,
        confidence: ForecastConfidence::Qualified,
        affected_artifact_classes: vec![ArtifactClass::WorkspaceState],
        affected_profiles: both_profiles(),
        affected_lines: vec![CompatibilityLine::Stable],
        auto_fix: AutoFixAvailability::ManualOnly,
        due_before: DueBoundary::BeforeRestart,
        skip_policy: SkipPolicy::NotSkippable,
        rollback_guidance: RollbackGuidance::PinCurrentVersion,
        available_actions: vec![
            MigrationAction::Pin,
            MigrationAction::Validator,
            MigrationAction::Repair,
        ],
        waiver: None,
        evidence_refs: evidence(HOLD_DRILL_SUBJECT),
    });
    assemble_packet(
        "m5-compatibility-forecast:drill-hold:0001",
        "Aureline M5 compatibility forecast — hold drill",
        subjects,
        vec![task],
    )
}

/// Drill: one subject is a breaking drift forecast *outside* Aureline's claimed window. The guardrail
/// caps every line at review-recommended rather than a hard failure, so the consumers that read it
/// review and no resolution is forced.
pub fn seeded_m5_compatibility_forecast_sheet_out_of_window() -> CompatibilityForecastSheet {
    let mut subjects = all_clear_subjects();
    set_subject(
        &mut subjects,
        SubjectForecast::new(SubjectForecastInput {
            subject: OUT_OF_WINDOW_DRILL_SUBJECT,
            subject_id: "extension-sdk:third-party".to_owned(),
            within_claimed_window: false,
            line_forecasts: vec![
                lf(
                    OUT_OF_WINDOW_DRILL_SUBJECT,
                    CompatibilityLine::Stable,
                    DriftClass::BreakingDrift,
                    ForecastConfidence::OutsideClaimedWindow,
                ),
                lf(
                    OUT_OF_WINDOW_DRILL_SUBJECT,
                    CompatibilityLine::Beta,
                    DriftClass::BreakingDrift,
                    ForecastConfidence::OutsideClaimedWindow,
                ),
                lf(
                    OUT_OF_WINDOW_DRILL_SUBJECT,
                    CompatibilityLine::Preview,
                    DriftClass::MigrationRequired,
                    ForecastConfidence::OutsideClaimedWindow,
                ),
                lf(
                    OUT_OF_WINDOW_DRILL_SUBJECT,
                    CompatibilityLine::Lts,
                    DriftClass::NoDrift,
                    ForecastConfidence::NotApplicable,
                ),
            ],
            affected_artifact_classes: vec![ArtifactClass::ExtensionPacks],
            affected_profiles: both_profiles(),
            evidence_refs: evidence(OUT_OF_WINDOW_DRILL_SUBJECT),
        }),
    );
    let task = MigrationTaskRow::new(MigrationTaskRowInput {
        task_id: "sdk-range-bump".to_owned(),
        subject: OUT_OF_WINDOW_DRILL_SUBJECT,
        task_class: MigrationTaskClass::ExtensionSdkRangeBump,
        addresses_drift: DriftClass::BreakingDrift,
        confidence: ForecastConfidence::OutsideClaimedWindow,
        affected_artifact_classes: vec![ArtifactClass::ExtensionPacks],
        affected_profiles: both_profiles(),
        affected_lines: vec![CompatibilityLine::Stable, CompatibilityLine::Beta],
        auto_fix: AutoFixAvailability::ManualOnly,
        due_before: DueBoundary::BeforeApply,
        skip_policy: SkipPolicy::OptionalRecommended,
        rollback_guidance: RollbackGuidance::SideBySideFallback,
        available_actions: vec![
            MigrationAction::Pin,
            MigrationAction::SideBySide,
            MigrationAction::Validator,
        ],
        waiver: None,
        evidence_refs: evidence(OUT_OF_WINDOW_DRILL_SUBJECT),
    });
    assemble_packet(
        "m5-compatibility-forecast:drill-out-of-window:0001",
        "Aureline M5 compatibility forecast — out-of-window drill",
        subjects,
        vec![task],
    )
}
