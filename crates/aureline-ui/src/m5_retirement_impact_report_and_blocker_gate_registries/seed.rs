//! Canonical seed builders for the M5 line-retirement_impact_report and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-retirement_impact_report and line-downgrade-packet entries
//! are built so the one typed line-retirement_impact_report object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_RETIREMENT_IMPACT_REPORT_BLOCKER_GATE_REGISTRIES_PACKET_ID: &str =
    "m5-retirement-impact-report-and-blocker-gate-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn retirement_impact_report(
    input: M5RetirementImpactReportEntryResolutionInput,
) -> M5ResolvedRetirementImpactReportEntry {
    resolve_retirement_impact_report_entry(input)
        .expect("seed line-retirement_impact_report entry resolves")
}

fn downgrade(input: M5ImpactBlockerGateEntryResolutionInput) -> M5ResolvedImpactBlockerGateEntry {
    resolve_impact_blocker_gate_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5RetirementImpactReportResolutionForm> {
    M5RetirementImpactReportResolutionForm::ALL.to_vec()
}

// -- Clean line-retirement_impact_report entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_retirement_impact_report_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    report_section: M5RetirementImpactReportKind,
    surface_context: M5RetirementImpactReportSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5RetirementImpactReportEntryResolutionInput {
    M5RetirementImpactReportEntryResolutionInput {
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

fn retirement_impact_report_blocking_clean() -> M5ResolvedRetirementImpactReportEntry {
    retirement_impact_report(clean_retirement_impact_report_base(
        "retirement_impact_report:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.retirement_impact_report.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementImpactReportKind::Blocking,
        M5RetirementImpactReportSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn retirement_impact_report_migration_required_clean() -> M5ResolvedRetirementImpactReportEntry {
    retirement_impact_report(clean_retirement_impact_report_base(
        "retirement_impact_report:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.retirement_impact_report.migration_required",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementImpactReportKind::MigrationRequired,
        M5RetirementImpactReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn retirement_impact_report_historical_only_clean() -> M5ResolvedRetirementImpactReportEntry {
    retirement_impact_report(clean_retirement_impact_report_base(
        "retirement_impact_report:program-governance:extension-author",
        "launch.line.extension-author",
        "line.retirement_impact_report.historical_only",
        M5RetiredStateRole::DisablePath,
        M5RetirementImpactReportKind::HistoricalOnly,
        M5RetirementImpactReportSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn retirement_impact_report_mirror_only_clean() -> M5ResolvedRetirementImpactReportEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_impact_report_base(
        "retirement_impact_report:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.retirement_impact_report.mirror_only",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementImpactReportKind::MirrorOnly,
        M5RetirementImpactReportSurfaceContext::ExecutiveSteeringSurface,
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
    retirement_impact_report(base)
}

fn retirement_impact_report_tenant_gated_clean() -> M5ResolvedRetirementImpactReportEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_impact_report_base(
        "retirement_impact_report:support:public-preview",
        "launch.line.public-preview",
        "line.retirement_impact_report.tenant_gated",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementImpactReportKind::TenantGated,
        M5RetirementImpactReportSurfaceContext::SupportOrExportForm,
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
    retirement_impact_report(base)
}

fn retirement_impact_report_informational_clean() -> M5ResolvedRetirementImpactReportEntry {
    retirement_impact_report(clean_retirement_impact_report_base(
        "retirement_impact_report:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.retirement_impact_report.informational",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementImpactReportKind::Informational,
        M5RetirementImpactReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-retirement_impact_report entries ---------------------------------------------------------

/// Degraded retirement_impact_report entry: the resolved retirement_impact_report object is incomplete — the bundle IDs are unstated.
fn retirement_impact_report_object_incomplete() -> M5ResolvedRetirementImpactReportEntry {
    let mut base = clean_retirement_impact_report_base(
        "retirement_impact_report:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.retirement_impact_report.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementImpactReportKind::Blocking,
        M5RetirementImpactReportSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    retirement_impact_report(base)
}

/// Degraded retirement_impact_report entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn retirement_impact_report_widen_fold() -> M5ResolvedRetirementImpactReportEntry {
    let mut base = clean_retirement_impact_report_base(
        "retirement_impact_report:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.retirement_impact_report.migration_required",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementImpactReportKind::MigrationRequired,
        M5RetirementImpactReportSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    retirement_impact_report(base)
}

/// Degraded retirement_impact_report entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn retirement_impact_report_unbound() -> M5ResolvedRetirementImpactReportEntry {
    let mut base = clean_retirement_impact_report_base(
        "retirement_impact_report:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.retirement_impact_report.informational",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementImpactReportKind::Informational,
        M5RetirementImpactReportSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    retirement_impact_report(base)
}

/// Degraded retirement_impact_report entry: the canonical registry token name is unstated.
fn retirement_impact_report_token_unstated() -> M5ResolvedRetirementImpactReportEntry {
    let mut base = clean_retirement_impact_report_base(
        "retirement_impact_report:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5RetiredStateRole::DisablePath,
        M5RetirementImpactReportKind::HistoricalOnly,
        M5RetirementImpactReportSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    retirement_impact_report(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    comparison_scope: M5ImpactBlockerGateScope,
    surface_context: M5RetirementImpactReportSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ImpactBlockerGateEntryResolutionInput {
    M5ImpactBlockerGateEntryResolutionInput {
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
        keeps_impact_blocker_gate_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedImpactBlockerGateEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5ImpactBlockerGateScope::BundleStillPointsAtCandidate,
        M5RetirementImpactReportSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedImpactBlockerGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.historical_only",
        M5RetiredStateRole::DisablePath,
        M5ImpactBlockerGateScope::TenantStillPointsAtCandidate,
        M5RetirementImpactReportSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedImpactBlockerGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.informational",
        M5RetiredStateRole::SupportNoteClosure,
        M5ImpactBlockerGateScope::SchemaOrPublicArtifactStillPointsAtCandidate,
        M5RetirementImpactReportSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedImpactBlockerGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.tenant_gated",
        M5RetiredStateRole::SuccessorRouting,
        M5ImpactBlockerGateScope::BundleStillPointsAtCandidate,
        M5RetirementImpactReportSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedImpactBlockerGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.informational",
        M5RetiredStateRole::SupportNoteClosure,
        M5ImpactBlockerGateScope::SchemaOrPublicArtifactStillPointsAtCandidate,
        M5RetirementImpactReportSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5RetirementImpactReportResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_blocker_scope_unclassified() -> M5ResolvedImpactBlockerGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.mirror_only",
        M5RetiredStateRole::LastSupportedPin,
        M5ImpactBlockerGateScope::BlockerScopeUnclassified,
        M5RetirementImpactReportSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5RetirementImpactReportBlockerGateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RetiredStateDowngradeTrigger>,
    retirement_impact_report_entries: Vec<M5ResolvedRetirementImpactReportEntry>,
    impact_blocker_gate_entries: Vec<M5ResolvedImpactBlockerGateEntry>,
) -> M5RetirementImpactReportBlockerGateRegistriesRow {
    M5RetirementImpactReportBlockerGateRegistriesRow {
        consumer_surface,
        qualification: M5RetiredStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        removal_horizon_stages: M5RetiredStateRemovalHorizonStage::ALL.to_vec(),
        required_labels: M5RetiredStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5RetiredStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RetirementImpactReportAnatomyPart::ALL.to_vec(),
        export_fields: M5RetirementImpactReportExportField::ALL.to_vec(),
        downgrade_triggers,
        retirement_impact_report_entries,
        impact_blocker_gate_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RETIREMENT_IMPACT_REPORT_BLOCKER_GATE_REGISTRIES_SCHEMA_REF,
            M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF,
            M5_IMPACT_BLOCKER_GATE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_impact_blocker_gate_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5RetirementImpactReportBlockerGateRegistriesRow> {
    use M5RetiredStateConsumerSurface as C;
    use M5RetiredStateDowngradeTrigger as D;

    vec![
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves a retirement candidate to one typed retirement-impact-report object — the classified dependency finding, its owning team, the exact-build joins, and the successor path or manual fallback — from the shared registry and proves the bundle-still-points-at-candidate blocker for that candidate; an impact report missing its exact-build joins and a blocker that keeps support language ahead of the closed support note degrade honestly instead of leaving a retiring surface to read as safe to close",
            "retirement:m5-release-center:001",
            vec![
                D::SuccessorPathUnnamed,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_impact_report_blocking_clean(),
                retirement_impact_report_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::HelpDocs,
            "Help/docs owner",
            "Help / docs resolves the migration-required finding and the schema-or-public-artifact-still-points-at-candidate blocker while keeping the active gate reason visible; a retiring surface widening its claim without a preserved rollback / export route and a resolution-form gap on a blocker are caught before a screenshot can reintroduce a safe-to-close reading",
            "retirement:m5-help-docs:001",
            vec![
                D::SuccessorPathUnnamed,
                D::ArchivalNoteMissing,
                D::RetirementManifestStale,
            ],
            vec![retirement_impact_report_migration_required_clean(), retirement_impact_report_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::Support,
            "Support owner",
            "Support resolves the mirror-only finding while keeping its public-facing successor / fallback claim matched to the closed support note and reports the impact-blocker-gate outcome; an impact-report entry that is a hand-copied per-entry assumption and a blocker on an unclassified gate scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DisablePathUnnamed,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_impact_report_mirror_only_clean(),
                retirement_impact_report_unbound(),
            ],
            vec![comparison_blocker_scope_unclassified()],
        ),
        base_row(
            C::MarketplaceRegistry,
            "Marketplace/registry owner",
            "The marketplace / registry resolves the historical-only finding and the tenant-still-points-at-candidate blocker bound to the registry so a retiring surface can no longer be selected in a new install or by a new tenant while a dependent remains; an unstated registry token on an impact-report entry is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CutoffDateUnstated,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_impact_report_historical_only_clean(),
                retirement_impact_report_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::InstallUpdate,
            "Install/update owner",
            "Install / update renders the same resolved retirement-impact-report and impact-blocker-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the informational finding and the schema-or-public-artifact-still-points-at-candidate blocker stay inspectable off-renderer so no new install can still select a retiring surface with an open blocker",
            "retirement:m5-install-update:001",
            vec![
                D::CutoffDateUnstated,
                D::RegistryReferenceUnstated,
                D::RetirementManifestStale,
            ],
            vec![retirement_impact_report_informational_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::PartnerProcurement,
            "Partner/procurement owner",
            "The partner / procurement feed carries the same resolved retirement-impact-report and impact-blocker-gate truth, so a hand-copied constant, an unstated registry token, an impact report widening its claim without a preserved rollback / export route, or support language running ahead of the closed support note is visible in evidence — a bundle, a tenant, or a schema / public artifact still pointing at the candidate — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![retirement_impact_report_tenant_gated_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5RetirementImpactReportBlockerGateRegistriesGovernanceReview {
    M5RetirementImpactReportBlockerGateRegistriesGovernanceReview {
        retirement_impact_report_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_retirement_impact_report_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        impact_blocker_gate_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        retirement_impact_report_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RetirementImpactReportBlockerGateRegistriesConsumerProjection {
    M5RetirementImpactReportBlockerGateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5RetirementImpactReportBlockerGateRegistriesProofFreshness {
    M5RetirementImpactReportBlockerGateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RetirementImpactReportBlockerGateRegistriesReleasePosture {
    M5RetirementImpactReportBlockerGateRegistriesReleasePosture {
        proof_packet_ref: M5_RETIREMENT_IMPACT_REPORT_BLOCKER_GATE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_RETIREMENT_IMPACT_REPORT_BLOCKER_GATE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RETIREMENT_IMPACT_REPORT_BLOCKER_GATE_REGISTRIES_SCHEMA_REF,
        M5_RETIREMENT_IMPACT_REPORT_BLOCKER_GATE_REGISTRIES_DOC_REF,
        M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
        M5_RETIRED_STATE_MATRIX_DOC_REF,
        M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF,
        M5_IMPACT_BLOCKER_GATE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 retirement-impact-report and impact-blocker-gate registries packet.
pub fn seeded_m5_retirement_impact_report_and_blocker_gate_registries(
) -> M5RetirementImpactReportBlockerGateRegistriesPacket {
    M5RetirementImpactReportBlockerGateRegistriesPacket::new(
        M5RetirementImpactReportBlockerGateRegistriesPacketInput {
            packet_id: M5_RETIREMENT_IMPACT_REPORT_BLOCKER_GATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 retirement-impact-report and impact-blocker-gate registries emitting one export-safe retirement impact report per retirement candidate — one classified dependency finding per detected dependency (a workflow bundle, migration pack, command / deep link, CLI alias, SDK contract row, saved artifact, profile, recipe, marketplace entry, mirror, or managed / new-tenant offering), typed as blocking, migration-required, historical-only, mirror-only, tenant-gated, or informational with an owning team and joined to the current compatibility / public-proof state and the successor path or manual fallback, so a retiring surface never closes its support window over a dangling dependency and no new install or new tenant can still select it, with canonical / accessible / audit resolution-form coverage, and a machine-readable impact blocker gate (bundle-still-points-at-candidate, tenant-still-points-at-candidate, or schema-or-public-artifact-still-points-at-candidate) that blocks closure while a live dependency still points at the candidate across review-packet, support-export, and public-proof surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5RetirementImpactReportBlockerGateRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending public-proof-ledger parity on every proof source;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_impact_report_and_blocker_gate_registries_retirement_impact_report_beta_narrowed(
) -> M5RetirementImpactReportBlockerGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_impact_report_and_blocker_gate_registries();
    packet.packet_id =
        "m5-retirement-impact-report-and-blocker-gate-registries:public-proof-ledger-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::ReleaseCenter)
        .expect("shiproom row present");
    row.qualification = M5RetiredStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending claim-history-diff parity on every
/// diff scope; every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_impact_report_and_blocker_gate_registries_impact_blocker_gate_preview_narrowed(
) -> M5RetirementImpactReportBlockerGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_impact_report_and_blocker_gate_registries();
    packet.packet_id =
        "m5-retirement-impact-report-and-blocker-gate-registries:claim-history-diff-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::HelpDocs)
        .expect("release-center row present");
    row.qualification = M5RetiredStateQualificationClass::Preview;
    packet
}
