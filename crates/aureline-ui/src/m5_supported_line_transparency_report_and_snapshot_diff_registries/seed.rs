//! Canonical seed builders for the M5 transparency-report and snapshot-diff registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean transparency-report and snapshot-diff entries
//! are built so the one typed transparency-report object resolving per line, lines never widening without
//! preserving their health posture, partner / public support language never running ahead of current public proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / affected-report-section /
//! previous-versus-current-snapshot / diff-scope / active-reason snapshot-diff object are proven across the
//! shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-line assumption, widen-without-posture, incomplete object, hidden diff, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_SNAPSHOT_DIFF_REGISTRIES_PACKET_ID: &str =
    "m5-supported-line-transparency-report-and-snapshot-diff-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn transparency_report(
    input: M5TransparencyReportEntryResolutionInput,
) -> M5ResolvedTransparencyReportEntry {
    resolve_transparency_report_entry(input).expect("seed line-transparency_report entry resolves")
}

fn downgrade(input: M5SnapshotDiffEntryResolutionInput) -> M5ResolvedSnapshotDiffEntry {
    resolve_snapshot_diff_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5SupportedLineTransparencyReportResolutionForm> {
    M5SupportedLineTransparencyReportResolutionForm::ALL.to_vec()
}

// -- Clean line-transparency_report entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_transparency_report_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    report_section: M5TransparencyReportKind,
    surface_context: M5SupportedLineTransparencyReportSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5TransparencyReportEntryResolutionInput {
    M5TransparencyReportEntryResolutionInput {
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

fn transparency_report_critical_upstream_status_section_clean() -> M5ResolvedTransparencyReportEntry
{
    transparency_report(clean_transparency_report_base(
        "transparency_report:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.transparency_report.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5TransparencyReportKind::CriticalUpstreamStatusSection,
        M5SupportedLineTransparencyReportSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn transparency_report_backup_maintainer_coverage_section_clean(
) -> M5ResolvedTransparencyReportEntry {
    transparency_report(clean_transparency_report_base(
        "transparency_report:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.transparency_report.backup_maintainer_coverage_section",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5TransparencyReportKind::BackupMaintainerCoverageSection,
        M5SupportedLineTransparencyReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn transparency_report_signer_quorum_health_section_clean() -> M5ResolvedTransparencyReportEntry {
    transparency_report(clean_transparency_report_base(
        "transparency_report:program-governance:extension-author",
        "launch.line.extension-author",
        "line.transparency_report.signer_quorum_health_section",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5TransparencyReportKind::SignerQuorumHealthSection,
        M5SupportedLineTransparencyReportSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn transparency_report_emergency_authority_coverage_section_clean(
) -> M5ResolvedTransparencyReportEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_transparency_report_base(
        "transparency_report:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.transparency_report.emergency_authority_coverage_section",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5TransparencyReportKind::EmergencyAuthorityCoverageSection,
        M5SupportedLineTransparencyReportSurfaceContext::ExecutiveSteeringSurface,
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
    transparency_report(base)
}

fn transparency_report_sustainment_sponsor_posture_section_clean(
) -> M5ResolvedTransparencyReportEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_transparency_report_base(
        "transparency_report:support:public-preview",
        "launch.line.public-preview",
        "line.transparency_report.sustainment_sponsor_posture_section",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5TransparencyReportKind::SustainmentSponsorPostureSection,
        M5SupportedLineTransparencyReportSurfaceContext::SupportOrExportForm,
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
    transparency_report(base)
}

fn transparency_report_red_risk_dependency_section_clean() -> M5ResolvedTransparencyReportEntry {
    transparency_report(clean_transparency_report_base(
        "transparency_report:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.transparency_report.red_risk_dependency_section",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5TransparencyReportKind::RedRiskDependencySection,
        M5SupportedLineTransparencyReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-transparency_report entries ---------------------------------------------------------

/// Degraded transparency_report entry: the resolved transparency_report object is incomplete — the bundle IDs are unstated.
fn transparency_report_object_incomplete() -> M5ResolvedTransparencyReportEntry {
    let mut base = clean_transparency_report_base(
        "transparency_report:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.transparency_report.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5TransparencyReportKind::CriticalUpstreamStatusSection,
        M5SupportedLineTransparencyReportSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    transparency_report(base)
}

/// Degraded transparency_report entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn transparency_report_widen_fold() -> M5ResolvedTransparencyReportEntry {
    let mut base = clean_transparency_report_base(
        "transparency_report:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.transparency_report.backup_maintainer_coverage_section",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5TransparencyReportKind::BackupMaintainerCoverageSection,
        M5SupportedLineTransparencyReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    transparency_report(base)
}

/// Degraded transparency_report entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn transparency_report_unbound() -> M5ResolvedTransparencyReportEntry {
    let mut base = clean_transparency_report_base(
        "transparency_report:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.transparency_report.red_risk_dependency_section",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5TransparencyReportKind::RedRiskDependencySection,
        M5SupportedLineTransparencyReportSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    transparency_report(base)
}

/// Degraded transparency_report entry: the canonical registry token name is unstated.
fn transparency_report_token_unstated() -> M5ResolvedTransparencyReportEntry {
    let mut base = clean_transparency_report_base(
        "transparency_report:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5TransparencyReportKind::SignerQuorumHealthSection,
        M5SupportedLineTransparencyReportSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    transparency_report(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    comparison_scope: M5SnapshotDiffScope,
    surface_context: M5SupportedLineTransparencyReportSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5SnapshotDiffEntryResolutionInput {
    M5SnapshotDiffEntryResolutionInput {
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
        keeps_snapshot_diff_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedSnapshotDiffEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5SnapshotDiffScope::HealthStatusChange,
        M5SupportedLineTransparencyReportSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedSnapshotDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.signer_quorum_health_section",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5SnapshotDiffScope::CoverageNarrowing,
        M5SupportedLineTransparencyReportSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedSnapshotDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.red_risk_dependency_section",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5SnapshotDiffScope::RedRiskDrift,
        M5SupportedLineTransparencyReportSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedSnapshotDiffEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.sustainment_sponsor_posture_section",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5SnapshotDiffScope::HealthStatusChange,
        M5SupportedLineTransparencyReportSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedSnapshotDiffEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.red_risk_dependency_section",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5SnapshotDiffScope::RedRiskDrift,
        M5SupportedLineTransparencyReportSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5SupportedLineTransparencyReportResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_diff_scope_unclassified() -> M5ResolvedSnapshotDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.emergency_authority_coverage_section",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5SnapshotDiffScope::DiffScopeUnclassified,
        M5SupportedLineTransparencyReportSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5SupportedLineTransparencyReportSnapshotDiffRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    transparency_report_entries: Vec<M5ResolvedTransparencyReportEntry>,
    snapshot_diff_entries: Vec<M5ResolvedSnapshotDiffEntry>,
) -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesRow {
    M5SupportedLineTransparencyReportSnapshotDiffRegistriesRow {
        consumer_surface,
        qualification: M5SupportedLineTransparencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5SupportedLineTransparencyWideningStage::ALL.to_vec(),
        required_labels: M5SupportedLineTransparencyRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5SupportedLineTransparencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SupportedLineTransparencyReportAnatomyPart::ALL.to_vec(),
        export_fields: M5SupportedLineTransparencyReportExportField::ALL.to_vec(),
        downgrade_triggers,
        transparency_report_entries,
        snapshot_diff_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_SNAPSHOT_DIFF_REGISTRIES_SCHEMA_REF,
            M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
            M5_SNAPSHOT_DIFF_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_snapshot_diff_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5SupportedLineTransparencyReportSnapshotDiffRegistriesRow> {
    use M5SupportedLineTransparencyConsumerSurface as C;
    use M5SupportedLineTransparencyDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the line's critical-upstream-status section to one typed transparency-report object — the affected line rows, upstream-health state, linked critical-upstream register refs, and owning roster — from the shared registry and proves the health-status-change diff for that line; a transparency-report object missing its linked upstream register and a diff that keeps support language ahead of current proof degrade honestly instead of leaving a claim to read as still green",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![
                transparency_report_critical_upstream_status_section_clean(),
                transparency_report_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the backup-maintainer-coverage section and the red-risk-drift diff while keeping the active diff reason visible; a line widening its health claim on stale proof and a resolution-form gap on a diff are caught before a screenshot can reintroduce a still-green reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::ImpliedGreenWhileProofOrArchiveWasStale,
                D::ProofStale,
            ],
            vec![transparency_report_backup_maintainer_coverage_section_clean(), transparency_report_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the emergency-authority-coverage section (public-facing) while keeping its published health claim matched to current public proof and reports the snapshot-diff outcome; a transparency-report entry that is a hand-copied per-entry assumption and a diff on an unclassified diff scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ExportClassUnstated,
                D::ProofStale,
            ],
            vec![
                transparency_report_emergency_authority_coverage_section_clean(),
                transparency_report_unbound(),
            ],
            vec![comparison_diff_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the signer-quorum-health section and the coverage-narrowing diff bound to the registry; an unstated registry token on a transparency-report entry is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::FreshnessWindowUnstated,
                D::ProofStale,
            ],
            vec![
                transparency_report_signer_quorum_health_section_clean(),
                transparency_report_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved transparency-report and snapshot-diff truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the red-risk-dependency section and the red-risk-drift diff stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::FreshnessWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![transparency_report_red_risk_dependency_section_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved transparency-report and snapshot-diff truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-health attempt, or support language running ahead of current proof is visible in evidence — a health-status change, a coverage narrowing, or a red-risk drift — rather than hidden behind a screenshot",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![transparency_report_sustainment_sponsor_posture_section_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesGovernanceReview {
    M5SupportedLineTransparencyReportSnapshotDiffRegistriesGovernanceReview {
        transparency_report_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_transparency_report_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        snapshot_diff_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        transparency_report_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesConsumerProjection
{
    M5SupportedLineTransparencyReportSnapshotDiffRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesProofFreshness {
    M5SupportedLineTransparencyReportSnapshotDiffRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesReleasePosture {
    M5SupportedLineTransparencyReportSnapshotDiffRegistriesReleasePosture {
        proof_packet_ref:
            M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_SNAPSHOT_DIFF_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_SNAPSHOT_DIFF_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_SNAPSHOT_DIFF_REGISTRIES_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_SNAPSHOT_DIFF_REGISTRIES_DOC_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
        M5_SNAPSHOT_DIFF_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch transparency-report and snapshot-diff registries packet.
pub fn seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries(
) -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacket {
    M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacket::new(
        M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacketInput {
            packet_id: M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_SNAPSHOT_DIFF_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 transparency-report and snapshot-diff registries publishing one export-safe transparency / upstream-health report per active supported line — one section per upstream-health dimension: critical-upstream status, backup maintainer coverage, signer-quorum health, emergency-authority coverage, sustainment / sponsor posture, and unresolved red-risk dependencies — each bound to one supported-line identity with public-safe health separated from internal-only incident / security detail, posture preserved so support language never runs ahead of current public proof, canonical / accessible / audit resolution-form coverage, and a machine-readable report snapshot diff (health-status-change, coverage-narrowing, or red-risk-drift) that turns a shift against the prior published snapshot into a typed diff event showing trend and drift, naming the active diff reason across release / help, About, docs, support, and procurement surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5SupportedLineTransparencyReportSnapshotDiffRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending transparency-report parity on every health section;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_transparency_report_beta_narrowed(
) -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacket {
    let mut packet = seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries();
    packet.packet_id =
        "m5-supported-line-transparency-report-and-snapshot-diff-registries:transparency-report-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SupportedLineTransparencyConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending snapshot-diff parity on every
/// diff scope; every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_snapshot_diff_preview_narrowed(
) -> M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacket {
    let mut packet = seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries();
    packet.packet_id =
        "m5-supported-line-transparency-report-and-snapshot-diff-registries:snapshot-diff-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5SupportedLineTransparencyConsumerSurface::ReleaseCenter
        })
        .expect("release-center row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Preview;
    packet
}
