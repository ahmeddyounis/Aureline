//! Canonical seed builders for the M5 line-correction_report and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-correction_report and line-downgrade-packet entries
//! are built so the one typed line-correction_report object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_STABLE_LINE_CORRECTION_REPORT_TRAIN_COMPARISON_REGISTRIES_PACKET_ID: &str =
    "m5-stable-line-correction-report-and-train-comparison-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn correction_report(
    input: M5CorrectionReportEntryResolutionInput,
) -> M5ResolvedCorrectionReportEntry {
    resolve_correction_report_entry(input).expect("seed line-correction_report entry resolves")
}

fn downgrade(input: M5TrainComparisonEntryResolutionInput) -> M5ResolvedTrainComparisonEntry {
    resolve_train_comparison_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5StableLineCorrectionReportResolutionForm> {
    M5StableLineCorrectionReportResolutionForm::ALL.to_vec()
}

// -- Clean line-correction_report entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_correction_report_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    report_section: M5CorrectionReportKind,
    surface_context: M5StableLineCorrectionReportSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5CorrectionReportEntryResolutionInput {
    M5CorrectionReportEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        line_binding_id: line_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        report_section,
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

fn correction_report_crash_support_signal_section_clean() -> M5ResolvedCorrectionReportEntry {
    correction_report(clean_correction_report_base(
        "correction_report:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.correction_report.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5CorrectionReportKind::CrashSupportSignalSection,
        M5StableLineCorrectionReportSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn correction_report_bundle_drift_section_clean() -> M5ResolvedCorrectionReportEntry {
    correction_report(clean_correction_report_base(
        "correction_report:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.correction_report.bundle_drift_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5CorrectionReportKind::BundleDriftSection,
        M5StableLineCorrectionReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn correction_report_adoption_blocker_section_clean() -> M5ResolvedCorrectionReportEntry {
    correction_report(clean_correction_report_base(
        "correction_report:program-governance:extension-author",
        "launch.line.extension-author",
        "line.correction_report.adoption_blocker_section",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5CorrectionReportKind::AdoptionBlockerSection,
        M5StableLineCorrectionReportSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn correction_report_compatibility_freshness_section_clean() -> M5ResolvedCorrectionReportEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_correction_report_base(
        "correction_report:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.correction_report.compatibility_freshness_section",
        M5StableLineProtectionRole::SupportWindow,
        M5CorrectionReportKind::CompatibilityFreshnessSection,
        M5StableLineCorrectionReportSurfaceContext::ExecutiveSteeringSurface,
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
    correction_report(base)
}

fn correction_report_public_truth_delta_section_clean() -> M5ResolvedCorrectionReportEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_correction_report_base(
        "correction_report:support:public-preview",
        "launch.line.public-preview",
        "line.correction_report.public_truth_delta_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5CorrectionReportKind::PublicTruthDeltaSection,
        M5StableLineCorrectionReportSurfaceContext::SupportOrExportForm,
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
    correction_report(base)
}

fn correction_report_backport_exception_section_clean() -> M5ResolvedCorrectionReportEntry {
    correction_report(clean_correction_report_base(
        "correction_report:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.correction_report.backport_exception_section",
        M5StableLineProtectionRole::BackportDecision,
        M5CorrectionReportKind::BackportExceptionSection,
        M5StableLineCorrectionReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-correction_report entries ---------------------------------------------------------

/// Degraded correction_report entry: the resolved correction_report object is incomplete — the bundle IDs are unstated.
fn correction_report_object_incomplete() -> M5ResolvedCorrectionReportEntry {
    let mut base = clean_correction_report_base(
        "correction_report:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.correction_report.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5CorrectionReportKind::CrashSupportSignalSection,
        M5StableLineCorrectionReportSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    correction_report(base)
}

/// Degraded correction_report entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn correction_report_widen_fold() -> M5ResolvedCorrectionReportEntry {
    let mut base = clean_correction_report_base(
        "correction_report:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.correction_report.bundle_drift_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5CorrectionReportKind::BundleDriftSection,
        M5StableLineCorrectionReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    correction_report(base)
}

/// Degraded correction_report entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn correction_report_unbound() -> M5ResolvedCorrectionReportEntry {
    let mut base = clean_correction_report_base(
        "correction_report:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.correction_report.backport_exception_section",
        M5StableLineProtectionRole::BackportDecision,
        M5CorrectionReportKind::BackportExceptionSection,
        M5StableLineCorrectionReportSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    correction_report(base)
}

/// Degraded correction_report entry: the canonical registry token name is unstated.
fn correction_report_token_unstated() -> M5ResolvedCorrectionReportEntry {
    let mut base = clean_correction_report_base(
        "correction_report:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5CorrectionReportKind::AdoptionBlockerSection,
        M5StableLineCorrectionReportSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    correction_report(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    comparison_scope: M5TrainComparisonScope,
    surface_context: M5StableLineCorrectionReportSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5TrainComparisonEntryResolutionInput {
    M5TrainComparisonEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        comparison_ref: comparison_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        comparison_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_line_identity: resolved_line_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_train_comparison_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedTrainComparisonEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5TrainComparisonScope::CorrectedIssue,
        M5StableLineCorrectionReportSurfaceContext::ShiproomSurface,
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
    downgrade(base)
}

fn downgrade_rehearsal_currency_clean() -> M5ResolvedTrainComparisonEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.adoption_blocker_section",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5TrainComparisonScope::RemainingNarrowedClaim,
        M5StableLineCorrectionReportSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedTrainComparisonEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.backport_exception_section",
        M5StableLineProtectionRole::BackportDecision,
        M5TrainComparisonScope::OpenExceptionClosure,
        M5StableLineCorrectionReportSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded line-downgrade-packet entries ----------------------------------------------------

/// Degraded downgrade entry: the downgrade would run partner / public support language ahead of line proof — a
/// support-language reference present but not bound to line proof reads as trustworthy when the line proof
/// does not yet back it.
fn downgrade_support_ahead() -> M5ResolvedTrainComparisonEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.public_truth_delta_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5TrainComparisonScope::CorrectedIssue,
        M5StableLineCorrectionReportSurfaceContext::ShiproomSurface,
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
    downgrade(base)
}

/// Degraded downgrade entry: the canonical / accessible / audit resolution-form coverage of the downgrade is
/// incomplete.
fn downgrade_form_incomplete() -> M5ResolvedTrainComparisonEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.backport_exception_section",
        M5StableLineProtectionRole::BackportDecision,
        M5TrainComparisonScope::OpenExceptionClosure,
        M5StableLineCorrectionReportSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5StableLineCorrectionReportResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_scope_unclassified() -> M5ResolvedTrainComparisonEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.compatibility_freshness_section",
        M5StableLineProtectionRole::SupportWindow,
        M5TrainComparisonScope::ScopeUnclassified,
        M5StableLineCorrectionReportSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5StableLineCorrectionReportTrainComparisonRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5StableLineProtectionDowngradeTrigger>,
    correction_report_entries: Vec<M5ResolvedCorrectionReportEntry>,
    train_comparison_entries: Vec<M5ResolvedTrainComparisonEntry>,
) -> M5StableLineCorrectionReportTrainComparisonRegistriesRow {
    M5StableLineCorrectionReportTrainComparisonRegistriesRow {
        consumer_surface,
        qualification: M5StableLineProtectionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5StableLineProtectionWideningStage::ALL.to_vec(),
        required_labels: M5StableLineProtectionRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5StableLineProtectionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5StableLineCorrectionReportAnatomyPart::ALL.to_vec(),
        export_fields: M5StableLineCorrectionReportExportField::ALL.to_vec(),
        downgrade_triggers,
        correction_report_entries,
        train_comparison_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STABLE_LINE_CORRECTION_REPORT_TRAIN_COMPARISON_REGISTRIES_SCHEMA_REF,
            M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
            M5_TRAIN_COMPARISON_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_train_comparison_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5StableLineCorrectionReportTrainComparisonRegistriesRow> {
    use M5StableLineProtectionConsumerSurface as C;
    use M5StableLineProtectionDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the train's crash / support-signal section to one typed correction-report object — the affected line rows, report section, linked correction packets / defect-ledger entries / claim rows, rollback target, and owning roster — from the shared registry and proves the corrected-issue comparison for that train; a correction-report object missing its linked evidence and a comparison that keeps support language ahead of the linked correction degrade honestly instead of leaving a field signal to read as silently resolved",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![
                correction_report_crash_support_signal_section_clean(),
                correction_report_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the bundle-drift section and the open-exception-closure comparison while keeping the active comparison reason visible; a train widening its claim while its correction is unresolved and a resolution-form gap on a comparison are caught before a screenshot can reintroduce a silently-resolved reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::ImpliedGreenWhileRefreshOrLedgerWasStale,
                D::ProofStale,
            ],
            vec![correction_report_bundle_drift_section_clean(), correction_report_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the compatibility-freshness-delta section while keeping its compatibility / known-issues claim matched to the linked correction evidence and reports the train-comparison outcome; a correction-report entry that is a hand-copied per-entry assumption and a comparison on an unclassified comparison scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RefreshStateUnstated,
                D::ProofStale,
            ],
            vec![
                correction_report_compatibility_freshness_section_clean(),
                correction_report_unbound(),
            ],
            vec![comparison_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the top adoption-blocker section and the remaining-narrowed-claim comparison bound to the registry; an unstated registry token on a correction-report entry is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportWindowUnstated,
                D::ProofStale,
            ],
            vec![
                correction_report_adoption_blocker_section_clean(),
                correction_report_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved correction-report and train-comparison truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the backport-exception section and the open-exception-closure comparison stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::SupportWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![correction_report_backport_exception_section_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved correction-report and train-comparison truth, so a hand-copied constant, an unstated registry token, a widen-over-unresolved-train attempt, or support language running ahead of the linked correction is visible in evidence — a corrected issue, a remaining narrowed claim, or an open exception still needing closure — rather than hidden behind a screenshot",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![correction_report_public_truth_delta_section_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5StableLineCorrectionReportTrainComparisonRegistriesGovernanceReview {
    M5StableLineCorrectionReportTrainComparisonRegistriesGovernanceReview {
        correction_report_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_correction_report_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        train_comparison_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        correction_report_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5StableLineCorrectionReportTrainComparisonRegistriesConsumerProjection
{
    M5StableLineCorrectionReportTrainComparisonRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5StableLineCorrectionReportTrainComparisonRegistriesProofFreshness {
    M5StableLineCorrectionReportTrainComparisonRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StableLineCorrectionReportTrainComparisonRegistriesReleasePosture {
    M5StableLineCorrectionReportTrainComparisonRegistriesReleasePosture {
        proof_packet_ref: M5_STABLE_LINE_CORRECTION_REPORT_TRAIN_COMPARISON_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_STABLE_LINE_CORRECTION_REPORT_TRAIN_COMPARISON_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STABLE_LINE_CORRECTION_REPORT_TRAIN_COMPARISON_REGISTRIES_SCHEMA_REF,
        M5_STABLE_LINE_CORRECTION_REPORT_TRAIN_COMPARISON_REGISTRIES_DOC_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
        M5_TRAIN_COMPARISON_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch correction-report and train-comparison registries packet.
pub fn seeded_m5_stable_line_correction_report_and_train_comparison_registries(
) -> M5StableLineCorrectionReportTrainComparisonRegistriesPacket {
    M5StableLineCorrectionReportTrainComparisonRegistriesPacket::new(
        M5StableLineCorrectionReportTrainComparisonRegistriesPacketInput {
            packet_id: M5_STABLE_LINE_CORRECTION_REPORT_TRAIN_COMPARISON_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 post-launch correction-report and train-comparison registries publishing one typed correction report per release train — one section per operating signal: top adoption blockers, crash / support signals, compatibility-report freshness deltas, bundle drift, public-truth deltas, and backport exceptions or deferrals — each linked to its correction packets, supported-line defect-ledger entries, and current claim rows, with rollback posture preserved so onboarding / migration / support language never runs ahead of the linked correction evidence, canonical / accessible / audit resolution-form coverage, and a machine-readable train-comparison (corrected-issue, remaining-narrowed-claim, or open-exception-closure) that lets operators compare trains to see which supported-line issues were corrected, which narrowed claims remain, and which exceptions still need explicit closure, naming the active comparison reason across release / help, support, shiproom, executive-steering, program-governance, and public-proof surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5StableLineCorrectionReportTrainComparisonRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending line-correction_report parity on every journey; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_correction_report_and_train_comparison_registries_correction_report_beta_narrowed(
) -> M5StableLineCorrectionReportTrainComparisonRegistriesPacket {
    let mut packet = seeded_m5_stable_line_correction_report_and_train_comparison_registries();
    packet.packet_id =
        "m5-stable-line-correction-report-and-train-comparison-registries:correction-report-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5StableLineProtectionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending line-downgrade parity on every
/// journey; every row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_correction_report_and_train_comparison_registries_train_comparison_preview_narrowed(
) -> M5StableLineCorrectionReportTrainComparisonRegistriesPacket {
    let mut packet = seeded_m5_stable_line_correction_report_and_train_comparison_registries();
    packet.packet_id =
        "m5-stable-line-correction-report-and-train-comparison-registries:train-comparison-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5StableLineProtectionQualificationClass::Preview;
    packet
}
