//! Canonical seed builders for the M5 flaky-state-badge / retry-history-row primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical flaky-retry-components primitive packet.
pub const M5_FLAKY_RETRY_COMPONENTS_PACKET_ID: &str = "m5-flaky-retry-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked flaky-state-badge resolution case from a full flaky state.
#[allow(clippy::too_many_arguments)]
fn flaky_case(
    classification: M5FlakyClassification,
    confidence_class: M5FlakyConfidenceClass,
    classifier_source: M5FlakyClassifierSource,
    provenance_class: M5TestIntelligenceProvenanceClass,
    mute_state: M5FlakyMuteState,
    retry_window_size: u32,
    observed_failures: u32,
    last_outcome: M5RetryAttemptOutcome,
    badge_identity_ref: &str,
    test_identity_ref: &str,
) -> M5FlakyBadgeResolutionCase {
    M5FlakyBadgeResolutionCase::resolved(M5FlakyBadgeResolutionInput {
        classification,
        confidence_class,
        classifier_source,
        provenance_class,
        mute_state,
        retry_window_size,
        observed_failures,
        last_outcome,
        badge_identity_ref: badge_identity_ref.to_owned(),
        test_identity_ref: test_identity_ref.to_owned(),
    })
}

/// Builds a worked retry-history-row resolution case from a full attempt history.
#[allow(clippy::too_many_arguments)]
fn retry_case(
    last_outcome: M5RetryAttemptOutcome,
    recent_outcomes: &[M5RetryAttemptOutcome],
    scope_class: M5RetryScopeClass,
    attempt_origin: M5RetryAttemptOrigin,
    confidence_class: M5FlakyConfidenceClass,
    provenance_class: M5TestIntelligenceProvenanceClass,
    has_env_delta: bool,
    has_build_delta: bool,
    has_runtime_delta: bool,
    test_identity_ref: &str,
    attempt_log_ref: &str,
) -> M5RetryRowResolutionCase {
    M5RetryRowResolutionCase::resolved(M5RetryRowResolutionInput {
        last_outcome,
        recent_outcomes: recent_outcomes.to_vec(),
        scope_class,
        attempt_origin,
        confidence_class,
        provenance_class,
        has_env_delta,
        has_build_delta,
        has_runtime_delta,
        test_identity_ref: test_identity_ref.to_owned(),
        attempt_log_ref: attempt_log_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full flaky/retry anatomy, classification,
/// confidence, classifier-source, mute-state, provenance, posture, outcome, scope, origin,
/// action, export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5FlakyRetryComponentConsumerSurface,
    qualification: M5TestIntelligenceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    flaky_examples: Vec<M5FlakyBadgeResolutionCase>,
    retry_examples: Vec<M5RetryRowResolutionCase>,
) -> M5FlakyRetryComponentConsumerRow {
    M5FlakyRetryComponentConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestIntelligenceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestIntelligenceDeploymentLine::ALL.to_vec(),
        flaky_anatomy_parts: M5FlakyBadgeAnatomyPart::ALL.to_vec(),
        retry_anatomy_parts: M5RetryRowAnatomyPart::ALL.to_vec(),
        flaky_classifications: M5FlakyClassification::ALL.to_vec(),
        flaky_confidence_classes: M5FlakyConfidenceClass::ALL.to_vec(),
        classifier_sources: M5FlakyClassifierSource::ALL.to_vec(),
        mute_states: M5FlakyMuteState::ALL.to_vec(),
        provenance_classes: M5TestIntelligenceProvenanceClass::ALL.to_vec(),
        flaky_postures: M5FlakyBadgePosture::ALL.to_vec(),
        retry_attempt_outcomes: M5RetryAttemptOutcome::ALL.to_vec(),
        retry_scope_classes: M5RetryScopeClass::ALL.to_vec(),
        retry_attempt_origins: M5RetryAttemptOrigin::ALL.to_vec(),
        retry_postures: M5RetryRowPosture::ALL.to_vec(),
        flaky_actions: M5FlakyBadgeAction::ALL.to_vec(),
        retry_actions: M5RetryRowAction::ALL.to_vec(),
        flaky_export_fields: M5FlakyBadgeExportField::ALL.to_vec(),
        retry_export_fields: M5RetryRowExportField::ALL.to_vec(),
        accessibility_routes: M5TestIntelligenceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestIntelligenceConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestIntelligenceDowngradeTrigger::ProvenanceClassUnstated,
            M5TestIntelligenceDowngradeTrigger::FreshnessClassUndisclosed,
            M5TestIntelligenceDowngradeTrigger::FlakyConfidenceOverstated,
            M5TestIntelligenceDowngradeTrigger::RetryScopeWidened,
            M5TestIntelligenceDowngradeTrigger::AlternateStateLabelInvented,
            M5TestIntelligenceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_FLAKY_RETRY_COMPONENTS_FLAKY_SCHEMA_REF,
            M5_FLAKY_RETRY_COMPONENTS_RETRY_SCHEMA_REF,
            M5_FLAKY_RETRY_COMPONENTS_FLAKY_VERDICT_REF,
            M5_FLAKY_RETRY_COMPONENTS_TEST_ATTEMPT_REF,
        ]),
        flaky_examples,
        retry_examples,
        labels_intermittent_as_confirmed_flaky: false,
        hides_retry_window_or_classifier_source: false,
        drops_env_build_runtime_delta_context: false,
        invents_alternate_flaky_or_retry_state_label: false,
    }
}

fn rows() -> Vec<M5FlakyRetryComponentConsumerRow> {
    use M5FlakyClassification as Flaky;
    use M5FlakyClassifierSource as Source;
    use M5FlakyConfidenceClass as Conf;
    use M5FlakyMuteState as Mute;
    use M5RetryAttemptOrigin as Origin;
    use M5RetryAttemptOutcome as Outcome;
    use M5RetryScopeClass as Scope;
    use M5TestIntelligenceProvenanceClass as Prov;

    vec![
        // 1. Flaky dashboard — a reproduced-flaky verdict backed by a large evidence window that
        //    reads as a confirmed flake, and a suspected-flaky verdict from a single occurrence
        //    that stays suspected rather than masquerading as reproduced; a divergent
        //    passed-on-retry row whose environment delta explains why the same test passed on
        //    the second attempt.
        base_row(
            M5FlakyRetryComponentConsumerSurface::FlakyDashboardPanel,
            M5TestIntelligenceQualificationClass::Stable,
            "Flaky dashboard panel owner",
            "The flaky dashboard renders the shared flaky-state badge so a reproduced-flaky verdict measured over eight attempts with five observed failures reads as a confirmed flake, while a single-occurrence suspicion stays a suspected-flaky badge rather than borrowing the authority of a reproduced verdict; it renders the shared retry-history row so a divergent pass-on-retry names its ordered outcomes and the environment delta that explains why the same test passed on the second attempt",
            "evidence:m5-flaky-dashboard-panel:001",
            vec![
                flaky_case(
                    Flaky::ReproducedFlaky,
                    Conf::HighConfidence,
                    Source::StatisticalModel,
                    Prov::ReproducedFlaky,
                    Mute::NotMuted,
                    8,
                    5,
                    Outcome::PassedOnRetry,
                    "flaky-badge:dashboard::reproduced-checkout",
                    "test:dashboard::checkout-flow",
                ),
                flaky_case(
                    Flaky::SuspectedFlaky,
                    Conf::SingleOccurrence,
                    Source::LocalHeuristic,
                    Prov::SuspectedFlaky,
                    Mute::NotMuted,
                    1,
                    1,
                    Outcome::FailedAllRetries,
                    "flaky-badge:dashboard::suspected-payment",
                    "test:dashboard::payment-retry",
                ),
            ],
            vec![retry_case(
                Outcome::PassedOnRetry,
                &[Outcome::FailedAllRetries, Outcome::PassedOnRetry],
                Scope::SameSelection,
                Origin::LocalAttempt,
                Conf::ModerateConfidence,
                Prov::VerifiedCurrentRun,
                true,
                false,
                false,
                "test:dashboard::checkout-flow",
                "attempt-log:dashboard::checkout-flow-2",
            )],
        ),
        // 2. Editor / test-tree badge — a stable badge with a high-confidence statistical model
        //    and a clean first-try pass rerun on a remote attempt, proving the same grammar in
        //    the editor gutter.
        base_row(
            M5FlakyRetryComponentConsumerSurface::EditorTestTreeBadge,
            M5TestIntelligenceQualificationClass::Stable,
            "Editor / test-tree badge owner",
            "The editor / test-tree flaky badge renders the shared flaky-state badge so a stable test reads as a stable badge with its high-confidence statistical classifier source shown, and it renders the shared retry-history row so a clean first-try pass rerun on a remote attempt keeps its stable test identity and a path back to the raw logs",
            "evidence:m5-flaky-editor-badge:001",
            vec![flaky_case(
                Flaky::Stable,
                Conf::HighConfidence,
                Source::StatisticalModel,
                Prov::VerifiedCurrentRun,
                Mute::NotMuted,
                10,
                0,
                Outcome::PassedFirstTry,
                "flaky-badge:editor::stable-parser",
                "test:editor::parser-unit",
            )],
            vec![retry_case(
                Outcome::PassedFirstTry,
                &[Outcome::PassedFirstTry],
                Scope::FailedOnlyRerun,
                Origin::RemoteAttempt,
                Conf::HighConfidence,
                Prov::VerifiedCurrentRun,
                false,
                true,
                false,
                "test:editor::parser-unit",
                "attempt-log:editor::parser-unit-remote",
            )],
        ),
        // 3. Retry history panel — a stable-again badge, plus a failed-all-retries row with a
        //    widened rerun scope (kept disclosed) on a notebook attempt whose runtime delta is
        //    shown, and an errored row.
        base_row(
            M5FlakyRetryComponentConsumerSurface::RetryHistoryPanel,
            M5TestIntelligenceQualificationClass::Stable,
            "Retry history panel owner",
            "The retry-history panel renders the shared flaky-state badge so a previously flaky test that is stable again reads as a stable-again badge, and it renders the shared retry-history row so a failed-all-retries row on a notebook attempt discloses its widened rerun scope and its runtime delta rather than presenting the rerun as the same selection, and an errored row keeps its errored meaning",
            "evidence:m5-flaky-retry-history-panel:001",
            vec![flaky_case(
                Flaky::StableAgain,
                Conf::ModerateConfidence,
                Source::StatisticalModel,
                Prov::StableAgain,
                Mute::NotMuted,
                6,
                1,
                Outcome::PassedFirstTry,
                "flaky-badge:retry::stable-again-index",
                "test:retry::index-rebuild",
            )],
            vec![
                retry_case(
                    Outcome::FailedAllRetries,
                    &[Outcome::FailedAllRetries, Outcome::FailedAllRetries],
                    Scope::WidenedSelection,
                    Origin::NotebookAttempt,
                    Conf::LowConfidence,
                    Prov::VerifiedCurrentRun,
                    false,
                    false,
                    true,
                    "test:retry::index-rebuild",
                    "attempt-log:retry::index-rebuild-widened",
                ),
                retry_case(
                    Outcome::ErroredAttempt,
                    &[Outcome::ErroredAttempt],
                    Scope::SingleTestRerun,
                    Origin::NotebookAttempt,
                    Conf::LowConfidence,
                    Prov::VerifiedCurrentRun,
                    true,
                    false,
                    false,
                    "test:retry::notebook-cell",
                    "attempt-log:retry::notebook-cell-error",
                ),
            ],
        ),
        // 4. Headless / CLI flaky-retry — a manually-muted / quarantined badge whose mute status
        //    stays disclosed, and a skipped row imported from a CI attempt.
        base_row(
            M5FlakyRetryComponentConsumerSurface::HeadlessCliFlakyRetry,
            M5TestIntelligenceQualificationClass::Stable,
            "Headless CLI flaky-retry owner",
            "The headless / CLI flaky-retry surface renders the shared flaky-state badge so a manually-quarantined verdict reads as a manually-muted badge that keeps its quarantine status disclosed rather than silently suppressing a failure, and it renders the shared retry-history row so a skipped attempt imported from CI reads as a skipped row that names its imported origin — proving the same grammar works without a desktop surface",
            "evidence:m5-flaky-headless-cli:001",
            vec![flaky_case(
                Flaky::ManuallyMuted,
                Conf::PolicyOverridden,
                Source::ManualOverride,
                Prov::ManuallyMuted,
                Mute::QuarantineActive,
                4,
                3,
                Outcome::SkippedAttempt,
                "flaky-badge:headless::quarantined-network",
                "test:headless::network-timeout",
            )],
            vec![retry_case(
                Outcome::SkippedAttempt,
                &[Outcome::SkippedAttempt],
                Scope::ImportedAttempt,
                Origin::ImportedCiAttempt,
                Conf::InsufficientData,
                Prov::ImportedCiArtifact,
                false,
                true,
                false,
                "test:headless::network-timeout",
                "attempt-log:headless::network-timeout-ci",
            )],
        ),
        // 5. Flaky-retry export — an unknown-flaky badge whose insufficient data keeps it
        //    unknown, and an aborted row a reviewer reads elsewhere with the same vocabulary.
        base_row(
            M5FlakyRetryComponentConsumerSurface::FlakyRetryExport,
            M5TestIntelligenceQualificationClass::Stable,
            "Flaky-retry export owner",
            "The flaky-retry export renders the shared flaky-state badge so a verdict with insufficient data reads as an unknown-flaky badge rather than a settled one, and it renders the shared retry-history row so an aborted attempt reads with the same aborted vocabulary a reviewer sees in the dashboard and the editor, with a path back to the raw logs",
            "evidence:m5-flaky-retry-export:001",
            vec![flaky_case(
                Flaky::UnknownFlaky,
                Conf::InsufficientData,
                Source::UnknownClassifier,
                Prov::Unknown,
                Mute::NotMuted,
                2,
                1,
                Outcome::AbortedAttempt,
                "flaky-badge:export::unknown-migration",
                "test:export::migration-smoke",
            )],
            vec![retry_case(
                Outcome::AbortedAttempt,
                &[Outcome::AbortedAttempt],
                Scope::UnknownScope,
                Origin::LocalAttempt,
                Conf::InsufficientData,
                Prov::Unknown,
                true,
                false,
                false,
                "test:export::migration-smoke",
                "attempt-log:export::migration-smoke-aborted",
            )],
        ),
    ]
}

fn governance_review() -> M5FlakyRetryComponentGovernanceReview {
    M5FlakyRetryComponentGovernanceReview {
        badge_shows_classification_and_confidence: true,
        badge_shows_retry_window: true,
        badge_shows_classifier_source_and_last_outcome: true,
        badge_shows_mute_or_quarantine_status: true,
        intermittent_never_confirmed_without_evidence_window: true,
        retry_row_shows_ordered_outcomes: true,
        retry_row_shows_env_build_runtime_deltas: true,
        retry_row_shows_classifier_confidence: true,
        retry_row_offers_rerun_and_open_logs: true,
        retry_row_preserves_stable_test_identity: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_component_declares_accessibility_route: true,
        support_export_reconstructs_flaky_retry_truth: true,
        later_components_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5FlakyRetryComponentConsumerProjection {
    M5FlakyRetryComponentConsumerProjection {
        flaky_and_retry_surfaces_consume_shared_vocabulary: true,
        flaky_posture_reads_single_source: true,
        retry_posture_reads_single_source: true,
        ci_and_support_read_same_flaky_retry_vocabulary: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5FlakyRetryComponentProofFreshness {
    M5FlakyRetryComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5FlakyRetryComponentReleasePosture {
    M5FlakyRetryComponentReleasePosture {
        release_packet_ref: M5_FLAKY_RETRY_COMPONENTS_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_FLAKY_RETRY_COMPONENTS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_FLAKY_RETRY_COMPONENTS_FLAKY_SCHEMA_REF,
        M5_FLAKY_RETRY_COMPONENTS_RETRY_SCHEMA_REF,
        M5_FLAKY_RETRY_COMPONENTS_DOC_REF,
        M5_FLAKY_RETRY_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_FLAKY_RETRY_COMPONENTS_FLAKY_VERDICT_REF,
        M5_FLAKY_RETRY_COMPONENTS_TEST_ATTEMPT_REF,
    ])
}

/// Builds the canonical M5 flaky-retry-components packet.
pub fn seeded_m5_flaky_retry_components_packet() -> M5FlakyRetryComponentsPacket {
    M5FlakyRetryComponentsPacket::new(M5FlakyRetryComponentsPacketInput {
        packet_id: M5_FLAKY_RETRY_COMPONENTS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 flaky-state-badge / retry-history-row primitive: controlled flaky classification, classifier confidence, classifier source, retry-window visibility, last outcome, mute/quarantine status, distinct stable/suspected/reproduced/stable-again/muted/unknown flaky postures, controlled passed-first-try/passed-on-retry/failed-all-retries/errored/skipped/aborted retry postures, ordered attempt outcomes, environment/build/runtime deltas, local/remote/notebook/imported-CI attempt origins, a required evidence window before a reproduced verdict, and bounded reveal/open-retry-history/rerun/mute-or-quarantine and reveal/rerun/open-logs/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5FlakyRetryComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the flaky-dashboard consumer is narrowed to Preview pending
/// reproduced-versus-suspected evidence-window parity proof across every deployment line; every
/// consumer stays visible.
pub fn seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed(
) -> M5FlakyRetryComponentsPacket {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.packet_id = "m5-flaky-retry-primitive:flaky-dashboard-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5FlakyRetryComponentConsumerSurface::FlakyDashboardPanel
        })
        .expect("flaky-dashboard-panel row present");
    row.qualification = M5TestIntelligenceQualificationClass::Preview;
    packet
}

/// Narrowed variant: the editor / test-tree badge consumer is held at Beta because a slice of
/// editor surfaces do not yet render the retry-window cue on every profile; every consumer stays
/// visible.
pub fn seeded_m5_flaky_retry_components_editor_badge_beta_narrowed() -> M5FlakyRetryComponentsPacket
{
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.packet_id = "m5-flaky-retry-primitive:editor-badge-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5FlakyRetryComponentConsumerSurface::EditorTestTreeBadge
        })
        .expect("editor-test-tree-badge row present");
    row.qualification = M5TestIntelligenceQualificationClass::Beta;
    packet
}
