//! Canonical seed builders for the M5 line-protection_plan and line-correction-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-protection_plan and line-correction-packet entries
//! are built so the one typed line-protection_plan object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-correction object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line correction, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_PACKET_ID: &str =
    "m5-stable-line-protection-plan-and-correction-queue-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn protection_plan(input: M5ProtectionPlanEntryResolutionInput) -> M5ResolvedProtectionPlanEntry {
    resolve_protection_plan_entry(input).expect("seed line-protection_plan entry resolves")
}

fn correction(input: M5CorrectionQueueEntryResolutionInput) -> M5ResolvedCorrectionQueueEntry {
    resolve_correction_queue_entry(input).expect("seed line-correction-packet entry resolves")
}

fn all_forms() -> Vec<M5StableLinePlanResolutionForm> {
    M5StableLinePlanResolutionForm::ALL.to_vec()
}

// -- Clean line-protection_plan entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_protection_plan_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    protected_journey: M5ProtectedJourneyKind,
    surface_context: M5StableLinePlanSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5ProtectionPlanEntryResolutionInput {
    M5ProtectionPlanEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        line_binding_id: line_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        protected_journey,
        surface_context,
        resolution_form_coverage: all_forms(),
        exact_repo_journey_rows: exact_repo_journey_rows.to_owned(),
        bundle_ids: bundle_ids.to_owned(),
        install_topology: install_topology.to_owned(),
        toolchain_envelope: toolchain_envelope.to_owned(),
        known_limits: known_limits.to_owned(),
        rollback_target: rollback_target.to_owned(),
        diagnostics_posture: diagnostics_posture.to_owned(),
        bound_to_registry: true,
        rollback_and_diagnostics_bounded: true,
        is_public_facing_line: false,
        support_language_matches_line_proof: true,
        proof_fresh: true,
    }
}

fn protection_plan_crash_recovery_journey_clean() -> M5ResolvedProtectionPlanEntry {
    protection_plan(clean_protection_plan_base(
        "protection_plan:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.protection_plan.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5ProtectedJourneyKind::CrashRecoveryJourney,
        M5StableLinePlanSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn protection_plan_migration_import_journey_clean() -> M5ResolvedProtectionPlanEntry {
    protection_plan(clean_protection_plan_base(
        "protection_plan:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.protection_plan.migration_import_journey",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5ProtectedJourneyKind::MigrationImportJourney,
        M5StableLinePlanSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn protection_plan_support_export_journey_clean() -> M5ResolvedProtectionPlanEntry {
    protection_plan(clean_protection_plan_base(
        "protection_plan:program-governance:extension-author",
        "launch.line.extension-author",
        "line.protection_plan.support_export_journey",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5ProtectedJourneyKind::SupportExportJourney,
        M5StableLinePlanSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn protection_plan_rollback_update_journey_clean() -> M5ResolvedProtectionPlanEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_protection_plan_base(
        "protection_plan:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.protection_plan.rollback_update_journey",
        M5StableLineProtectionRole::SupportWindow,
        M5ProtectedJourneyKind::RollbackUpdateJourney,
        M5StableLinePlanSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-journeys",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.is_public_facing_line = true;
    base.support_language_matches_line_proof = true;
    protection_plan(base)
}

fn protection_plan_launch_bearing_flow_journey_clean() -> M5ResolvedProtectionPlanEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_protection_plan_base(
        "protection_plan:support:public-preview",
        "launch.line.public-preview",
        "line.protection_plan.launch_bearing_flow_journey",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5ProtectedJourneyKind::LaunchBearingFlowJourney,
        M5StableLinePlanSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-journeys",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.is_public_facing_line = true;
    base.support_language_matches_line_proof = true;
    protection_plan(base)
}

fn protection_plan_named_protected_journey_clean() -> M5ResolvedProtectionPlanEntry {
    protection_plan(clean_protection_plan_base(
        "protection_plan:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.protection_plan.named_protected_journey",
        M5StableLineProtectionRole::BackportDecision,
        M5ProtectedJourneyKind::NamedProtectedJourney,
        M5StableLinePlanSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-protection_plan entries ---------------------------------------------------------

/// Degraded protection_plan entry: the resolved protection_plan object is incomplete — the bundle IDs are unstated.
fn protection_plan_object_incomplete() -> M5ResolvedProtectionPlanEntry {
    let mut base = clean_protection_plan_base(
        "protection_plan:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.protection_plan.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5ProtectedJourneyKind::CrashRecoveryJourney,
        M5StableLinePlanSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    protection_plan(base)
}

/// Degraded protection_plan entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn protection_plan_widen_fold() -> M5ResolvedProtectionPlanEntry {
    let mut base = clean_protection_plan_base(
        "protection_plan:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.protection_plan.migration_import_journey",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5ProtectedJourneyKind::MigrationImportJourney,
        M5StableLinePlanSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    protection_plan(base)
}

/// Degraded protection_plan entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn protection_plan_unbound() -> M5ResolvedProtectionPlanEntry {
    let mut base = clean_protection_plan_base(
        "protection_plan:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.protection_plan.named_protected_journey",
        M5StableLineProtectionRole::BackportDecision,
        M5ProtectedJourneyKind::NamedProtectedJourney,
        M5StableLinePlanSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    protection_plan(base)
}

/// Degraded protection_plan entry: the canonical registry token name is unstated.
fn protection_plan_token_unstated() -> M5ResolvedProtectionPlanEntry {
    let mut base = clean_protection_plan_base(
        "protection_plan:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5ProtectedJourneyKind::SupportExportJourney,
        M5StableLinePlanSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    protection_plan(base)
}

// -- Clean line-correction-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_correction_base(
    entry_id: &str,
    correction_ref: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    correction_scope: M5CorrectionQueueScope,
    surface_context: M5StableLinePlanSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5CorrectionQueueEntryResolutionInput {
    M5CorrectionQueueEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        correction_ref: correction_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        correction_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_line_identity: resolved_line_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_correction_queue_visible: true,
        correction_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn correction_dogfood_ring_clean() -> M5ResolvedCorrectionQueueEntry {
    // A dogfood-ring correction packet carries partner / public support language bound to line proof.
    let mut base = clean_correction_base(
        "correction:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.correction.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5CorrectionQueueScope::QueuedRegressionScope,
        M5StableLinePlanSurfaceContext::ShiproomSurface,
        "line-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = true;
    correction(base)
}

fn correction_rehearsal_currency_clean() -> M5ResolvedCorrectionQueueEntry {
    correction(clean_correction_base(
        "correction:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.correction.support_export_journey",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5CorrectionQueueScope::BackportDecisionScope,
        M5StableLinePlanSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn correction_go_no_go_signoff_clean() -> M5ResolvedCorrectionQueueEntry {
    correction(clean_correction_base(
        "correction:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.correction.named_protected_journey",
        M5StableLineProtectionRole::BackportDecision,
        M5CorrectionQueueScope::CorrectionReportScope,
        M5StableLinePlanSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded line-correction-packet entries ----------------------------------------------------

/// Degraded correction entry: the correction would run partner / public support language ahead of line proof — a
/// support-language reference present but not bound to line proof reads as trustworthy when the line proof
/// does not yet back it.
fn correction_support_ahead() -> M5ResolvedCorrectionQueueEntry {
    let mut base = clean_correction_base(
        "correction:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.correction.launch_bearing_flow_journey",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5CorrectionQueueScope::QueuedRegressionScope,
        M5StableLinePlanSurfaceContext::ShiproomSurface,
        "line-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = false;
    correction(base)
}

/// Degraded correction entry: the canonical / accessible / audit resolution-form coverage of the correction is
/// incomplete.
fn correction_form_incomplete() -> M5ResolvedCorrectionQueueEntry {
    let mut base = clean_correction_base(
        "correction:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.correction.named_protected_journey",
        M5StableLineProtectionRole::BackportDecision,
        M5CorrectionQueueScope::CorrectionReportScope,
        M5StableLinePlanSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5StableLinePlanResolutionForm::CanonicalObject];
    correction(base)
}

/// Degraded correction entry: the correction scope is unclassified.
fn correction_scope_unclassified() -> M5ResolvedCorrectionQueueEntry {
    correction(clean_correction_base(
        "correction:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.correction.rollback_update_journey",
        M5StableLineProtectionRole::SupportWindow,
        M5CorrectionQueueScope::ScopeUnclassified,
        M5StableLinePlanSurfaceContext::ExecutiveSteeringSurface,
        "line-id.design-partner-preview-0007",
        "known-limits.ledger.design-partner",
        "rollback.target.ref.design-partner",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.design-partner-reviewed",
        "support.language.design-partner-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5StableLineProtectionDowngradeTrigger>,
    protection_plan_entries: Vec<M5ResolvedProtectionPlanEntry>,
    correction_queue_entries: Vec<M5ResolvedCorrectionQueueEntry>,
) -> M5StableLineProtectionPlanCorrectionQueueRegistriesRow {
    M5StableLineProtectionPlanCorrectionQueueRegistriesRow {
        consumer_surface,
        qualification: M5StableLineProtectionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5StableLineProtectionWideningStage::ALL.to_vec(),
        required_labels: M5StableLineProtectionRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5StableLineProtectionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5StableLinePlanAnatomyPart::ALL.to_vec(),
        export_fields: M5StableLinePlanExportField::ALL.to_vec(),
        downgrade_triggers,
        protection_plan_entries,
        correction_queue_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_REF,
            M5_STABLE_LINE_PROTECTION_PLAN_DOMAIN_SCHEMA_REF,
            M5_CORRECTION_LANE_QUEUE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_correction: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_correction_queue_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5StableLineProtectionPlanCorrectionQueueRegistriesRow> {
    use M5StableLineProtectionConsumerSurface as C;
    use M5StableLineProtectionDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the fresh stable line's first-30-day protection plan to one typed object — the protected journey, its queued-regression rows, issue IDs, release line, correction packet, delayed-breadth ledger, rollback target, and diagnostics posture — from the shared registry and proves the queued-regression correction-lane queue for that line; a protection-plan object missing its queued-regression issue IDs and a correction queue that lets breadth work outrank an open regression degrade honestly instead of reading as a clean pass",
            "correction:m5-launch-control-shiproom:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![
                protection_plan_crash_recovery_journey_clean(),
                protection_plan_object_incomplete(),
            ],
            vec![correction_dogfood_ring_clean(), correction_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the migration-import protection plan and the correction-report correction-lane queue while keeping the queue visible; a line resuming breadth work without a preserved rollback target and diagnostics posture and a resolution-form gap on a correction queue are caught before a screenshot can reintroduce a false-truth reading",
            "correction:m5-launch-control-release-center:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::ImpliedGreenWhileRefreshOrLedgerWasStale,
                D::ProofStale,
            ],
            vec![protection_plan_migration_import_journey_clean(), protection_plan_widen_fold()],
            vec![correction_go_no_go_signoff_clean(), correction_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the rollback-update protection plan while keeping its delayed breadth work bound to a recorded override and reports the correction-lane queue; a protection plan that is a hand-copied per-entry assumption and a correction queue on an unclassified correction scope degrade honestly",
            "correction:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RefreshStateUnstated,
                D::ProofStale,
            ],
            vec![
                protection_plan_rollback_update_journey_clean(),
                protection_plan_unbound(),
            ],
            vec![correction_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the support-export protection plan and the backport-decision correction-lane queue bound to the registry; an unstated registry token on a protection plan is caught before it can drift",
            "correction:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportWindowUnstated,
                D::ProofStale,
            ],
            vec![
                protection_plan_support_export_journey_clean(),
                protection_plan_token_unstated(),
            ],
            vec![correction_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved protection-plan and correction-lane queue truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the named-protected-journey protection plan and the correction-report queue stay inspectable off-renderer",
            "correction:m5-launch-control-diagnostics:001",
            vec![
                D::SupportWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![protection_plan_named_protected_journey_clean()],
            vec![correction_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved protection-plan and correction-lane queue truth, so a hand-copied constant, an unstated registry token, a breadth-over-regression attempt, or breadth work running ahead of an open regression is visible in evidence rather than hidden behind a screenshot",
            "correction:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![protection_plan_launch_bearing_flow_journey_clean()],
            vec![correction_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5StableLineProtectionPlanCorrectionQueueRegistriesGovernanceReview {
    M5StableLineProtectionPlanCorrectionQueueRegistriesGovernanceReview {
        protection_plan_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_protection_plan_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        correction_queue_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        protection_plan_or_correction_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerProjection {
    M5StableLineProtectionPlanCorrectionQueueRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5StableLineProtectionPlanCorrectionQueueRegistriesProofFreshness {
    M5StableLineProtectionPlanCorrectionQueueRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StableLineProtectionPlanCorrectionQueueRegistriesReleasePosture {
    M5StableLineProtectionPlanCorrectionQueueRegistriesReleasePosture {
        proof_packet_ref: M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_DOC_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_STABLE_LINE_PROTECTION_PLAN_DOMAIN_SCHEMA_REF,
        M5_CORRECTION_LANE_QUEUE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 line-protection_plan and line-correction-packet registries packet.
pub fn seeded_m5_stable_line_protection_plan_and_correction_queue_registries(
) -> M5StableLineProtectionPlanCorrectionQueueRegistriesPacket {
    M5StableLineProtectionPlanCorrectionQueueRegistriesPacket::new(
        M5StableLineProtectionPlanCorrectionQueueRegistriesPacketInput {
            packet_id: M5_STABLE_LINE_PROTECTION_PLAN_CORRECTION_QUEUE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 stable-line protection-plan and correction-lane queue registries with one typed first-30-day protection-plan object resolving per supported line, breadth work never silently outranking a crash / rollback / support-export / migration regression, delayed breadth work always bound to a recorded override or claim-narrowing action, canonical / accessible / audit resolution-form coverage, and the complete line-identity / queued-regression-ledger / rollback-target / correction-packet / backport-decision / delayed-breadth / last-correction-revision queue object across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5StableLineProtectionPlanCorrectionQueueRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the shiproom row is held at Beta pending line-protection_plan parity on every journey; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_protection_plan_and_correction_queue_registries_protection_plan_beta_narrowed(
) -> M5StableLineProtectionPlanCorrectionQueueRegistriesPacket {
    let mut packet = seeded_m5_stable_line_protection_plan_and_correction_queue_registries();
    packet.packet_id =
        "m5-stable-line-protection-plan-and-correction-queue-registries:protection-plan-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5StableLineProtectionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending line-correction parity on every
/// journey; every row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_protection_plan_and_correction_queue_registries_correction_queue_preview_narrowed(
) -> M5StableLineProtectionPlanCorrectionQueueRegistriesPacket {
    let mut packet = seeded_m5_stable_line_protection_plan_and_correction_queue_registries();
    packet.packet_id =
        "m5-stable-line-protection-plan-and-correction-queue-registries:correction-queue-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5StableLineProtectionQualificationClass::Preview;
    packet
}
