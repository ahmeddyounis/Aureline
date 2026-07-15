//! Canonical seed builders for the M5 line-bundle_refresh_audit and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-bundle_refresh_audit and line-downgrade-packet entries
//! are built so the one typed line-bundle_refresh_audit object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_STABLE_LINE_BUNDLE_REFRESH_AUDIT_SHIPPING_LINE_DRIFT_REPORT_REGISTRIES_PACKET_ID:
    &str =
    "m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn bundle_refresh_audit(
    input: M5BundleRefreshAuditEntryResolutionInput,
) -> M5ResolvedBundleRefreshAuditEntry {
    resolve_bundle_refresh_audit_entry(input)
        .expect("seed line-bundle_refresh_audit entry resolves")
}

fn downgrade(
    input: M5ShippingLineDriftReportEntryResolutionInput,
) -> M5ResolvedShippingLineDriftReportEntry {
    resolve_shipping_line_drift_report_entry(input)
        .expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5StableLineBundleAuditResolutionForm> {
    M5StableLineBundleAuditResolutionForm::ALL.to_vec()
}

// -- Clean line-bundle_refresh_audit entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_bundle_refresh_audit_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    audited_bundle: M5BundleRefreshAuditKind,
    surface_context: M5StableLineBundleAuditSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5BundleRefreshAuditEntryResolutionInput {
    M5BundleRefreshAuditEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        line_binding_id: line_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        audited_bundle,
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

fn bundle_refresh_audit_launch_bundle_freshness_audit_clean() -> M5ResolvedBundleRefreshAuditEntry {
    bundle_refresh_audit(clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.bundle_refresh_audit.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5BundleRefreshAuditKind::LaunchBundleFreshnessAudit,
        M5StableLineBundleAuditSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn bundle_refresh_audit_launch_bundle_reversibility_audit_clean(
) -> M5ResolvedBundleRefreshAuditEntry {
    bundle_refresh_audit(clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.bundle_refresh_audit.launch_bundle_reversibility_audit",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5BundleRefreshAuditKind::LaunchBundleReversibilityAudit,
        M5StableLineBundleAuditSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn bundle_refresh_audit_missing_artifact_audit_clean() -> M5ResolvedBundleRefreshAuditEntry {
    bundle_refresh_audit(clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:program-governance:extension-author",
        "launch.line.extension-author",
        "line.bundle_refresh_audit.missing_artifact_audit",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5BundleRefreshAuditKind::MissingArtifactAudit,
        M5StableLineBundleAuditSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn bundle_refresh_audit_imported_user_handoff_bundle_audit_clean(
) -> M5ResolvedBundleRefreshAuditEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.bundle_refresh_audit.imported_user_handoff_bundle_audit",
        M5StableLineProtectionRole::SupportWindow,
        M5BundleRefreshAuditKind::ImportedUserHandoffBundleAudit,
        M5StableLineBundleAuditSurfaceContext::ExecutiveSteeringSurface,
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
    bundle_refresh_audit(base)
}

fn bundle_refresh_audit_org_approved_bundle_audit_clean() -> M5ResolvedBundleRefreshAuditEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:support:public-preview",
        "launch.line.public-preview",
        "line.bundle_refresh_audit.org_approved_bundle_audit",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5BundleRefreshAuditKind::OrgApprovedBundleAudit,
        M5StableLineBundleAuditSurfaceContext::SupportOrExportForm,
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
    bundle_refresh_audit(base)
}

fn bundle_refresh_audit_unsupported_drift_audit_clean() -> M5ResolvedBundleRefreshAuditEntry {
    bundle_refresh_audit(clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.bundle_refresh_audit.unsupported_drift_audit",
        M5StableLineProtectionRole::BackportDecision,
        M5BundleRefreshAuditKind::UnsupportedDriftAudit,
        M5StableLineBundleAuditSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-bundle_refresh_audit entries ---------------------------------------------------------

/// Degraded bundle_refresh_audit entry: the resolved bundle_refresh_audit object is incomplete — the bundle IDs are unstated.
fn bundle_refresh_audit_object_incomplete() -> M5ResolvedBundleRefreshAuditEntry {
    let mut base = clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.bundle_refresh_audit.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5BundleRefreshAuditKind::LaunchBundleFreshnessAudit,
        M5StableLineBundleAuditSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    bundle_refresh_audit(base)
}

/// Degraded bundle_refresh_audit entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn bundle_refresh_audit_widen_fold() -> M5ResolvedBundleRefreshAuditEntry {
    let mut base = clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.bundle_refresh_audit.launch_bundle_reversibility_audit",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5BundleRefreshAuditKind::LaunchBundleReversibilityAudit,
        M5StableLineBundleAuditSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    bundle_refresh_audit(base)
}

/// Degraded bundle_refresh_audit entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn bundle_refresh_audit_unbound() -> M5ResolvedBundleRefreshAuditEntry {
    let mut base = clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.bundle_refresh_audit.unsupported_drift_audit",
        M5StableLineProtectionRole::BackportDecision,
        M5BundleRefreshAuditKind::UnsupportedDriftAudit,
        M5StableLineBundleAuditSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    bundle_refresh_audit(base)
}

/// Degraded bundle_refresh_audit entry: the canonical registry token name is unstated.
fn bundle_refresh_audit_token_unstated() -> M5ResolvedBundleRefreshAuditEntry {
    let mut base = clean_bundle_refresh_audit_base(
        "bundle_refresh_audit:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5BundleRefreshAuditKind::MissingArtifactAudit,
        M5StableLineBundleAuditSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    bundle_refresh_audit(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    drift_ref: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    drift_scope: M5ShippingLineDriftScope,
    surface_context: M5StableLineBundleAuditSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ShippingLineDriftReportEntryResolutionInput {
    M5ShippingLineDriftReportEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        drift_ref: drift_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        drift_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_line_identity: resolved_line_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_shipping_line_drift_report_visible: true,
        drift_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedShippingLineDriftReportEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5ShippingLineDriftScope::StaleBundleDrift,
        M5StableLineBundleAuditSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedShippingLineDriftReportEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.missing_artifact_audit",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5ShippingLineDriftScope::NonReversibleBundleDrift,
        M5StableLineBundleAuditSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedShippingLineDriftReportEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.unsupported_drift_audit",
        M5StableLineProtectionRole::BackportDecision,
        M5ShippingLineDriftScope::UnsupportedBundleDrift,
        M5StableLineBundleAuditSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedShippingLineDriftReportEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.org_approved_bundle_audit",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5ShippingLineDriftScope::StaleBundleDrift,
        M5StableLineBundleAuditSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedShippingLineDriftReportEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.unsupported_drift_audit",
        M5StableLineProtectionRole::BackportDecision,
        M5ShippingLineDriftScope::UnsupportedBundleDrift,
        M5StableLineBundleAuditSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5StableLineBundleAuditResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn drift_scope_unclassified() -> M5ResolvedShippingLineDriftReportEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.imported_user_handoff_bundle_audit",
        M5StableLineProtectionRole::SupportWindow,
        M5ShippingLineDriftScope::ScopeUnclassified,
        M5StableLineBundleAuditSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5StableLineProtectionDowngradeTrigger>,
    bundle_refresh_audit_entries: Vec<M5ResolvedBundleRefreshAuditEntry>,
    shipping_line_drift_report_entries: Vec<M5ResolvedShippingLineDriftReportEntry>,
) -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesRow {
    M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesRow {
        consumer_surface,
        qualification: M5StableLineProtectionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5StableLineProtectionWideningStage::ALL.to_vec(),
        required_labels: M5StableLineProtectionRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5StableLineProtectionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5StableLineBundleAuditAnatomyPart::ALL.to_vec(),
        export_fields: M5StableLineBundleAuditExportField::ALL.to_vec(),
        downgrade_triggers,
        bundle_refresh_audit_entries,
        shipping_line_drift_report_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STABLE_LINE_BUNDLE_REFRESH_AUDIT_SHIPPING_LINE_DRIFT_REPORT_REGISTRIES_SCHEMA_REF,
            M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
            M5_SHIPPING_LINE_DRIFT_REPORT_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_shipping_line_drift_report_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesRow> {
    use M5StableLineProtectionConsumerSurface as C;
    use M5StableLineProtectionDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the active shipping line's launch-bundle freshness audit to one typed bundle-currentness object — the audited bundle, its affected rows, freshness / reversibility state, rollback target, and required refresh / narrow decision — from the shared registry and proves the stale-bundle drift report for that bundle; a bundle-refresh-audit object missing its fields and a drift report that keeps support language ahead of a refreshed bundle degrade honestly instead of leaving a stale launch bundle to read as silently supportable",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![
                bundle_refresh_audit_launch_bundle_freshness_audit_clean(),
                bundle_refresh_audit_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the launch-bundle reversibility audit and the unsupported-bundle drift report while keeping the active drift reason visible; a line widening its claim while a bundle is stale or non-reversible and a resolution-form gap on a drift report are caught before a screenshot can reintroduce a silently-supportable reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::ImpliedGreenWhileRefreshOrLedgerWasStale,
                D::ProofStale,
            ],
            vec![bundle_refresh_audit_launch_bundle_reversibility_audit_clean(), bundle_refresh_audit_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the imported-user-handoff-bundle audit while keeping its onboarding / migration claim matched to a current, reversible bundle and reports the shipping-line-drift-report outcome; a bundle audit that is a hand-copied per-entry assumption and a drift report on an unclassified drift scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RefreshStateUnstated,
                D::ProofStale,
            ],
            vec![
                bundle_refresh_audit_imported_user_handoff_bundle_audit_clean(),
                bundle_refresh_audit_unbound(),
            ],
            vec![drift_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the missing-artifact audit and the non-reversible-bundle drift report bound to the registry; an unstated registry token on a bundle audit is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportWindowUnstated,
                D::ProofStale,
            ],
            vec![
                bundle_refresh_audit_missing_artifact_audit_clean(),
                bundle_refresh_audit_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved bundle-refresh-audit and shipping-line-drift-report truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the unsupported-drift audit and the stale-bundle drift report stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::SupportWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![bundle_refresh_audit_unsupported_drift_audit_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved bundle-refresh-audit and shipping-line-drift-report truth, so a hand-copied constant, an unstated registry token, a widen-over-stale-bundle attempt, or support language running ahead of a refreshed bundle is visible in evidence — stale, non-reversible, or unsupported drift — rather than hidden behind a screenshot",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![bundle_refresh_audit_org_approved_bundle_audit_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review(
) -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesGovernanceReview {
    M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesGovernanceReview {
        bundle_refresh_audit_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_bundle_refresh_audit_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        shipping_line_drift_report_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        bundle_refresh_audit_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection(
) -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesConsumerProjection {
    M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesProofFreshness
{
    M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesReleasePosture
{
    M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesReleasePosture {
        proof_packet_ref:
            M5_STABLE_LINE_BUNDLE_REFRESH_AUDIT_SHIPPING_LINE_DRIFT_REPORT_REGISTRIES_ARTIFACT_REF
                .to_owned(),
        line_audit_ref:
            M5_STABLE_LINE_BUNDLE_REFRESH_AUDIT_SHIPPING_LINE_DRIFT_REPORT_REGISTRIES_REPORT_REF
                .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STABLE_LINE_BUNDLE_REFRESH_AUDIT_SHIPPING_LINE_DRIFT_REPORT_REGISTRIES_SCHEMA_REF,
        M5_STABLE_LINE_BUNDLE_REFRESH_AUDIT_SHIPPING_LINE_DRIFT_REPORT_REGISTRIES_DOC_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
        M5_SHIPPING_LINE_DRIFT_REPORT_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 line-bundle_refresh_audit and line-downgrade-packet registries packet.
pub fn seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries(
) -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacket {
    M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacket::new(
        M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacketInput {
            packet_id: M5_STABLE_LINE_BUNDLE_REFRESH_AUDIT_SHIPPING_LINE_DRIFT_REPORT_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 stable-line bundle-refresh-audit and shipping-line-drift-report registries with one typed bundle-currentness audit recording, for each claimed bundle on the active shipping line — a launch-bundle freshness audit, a launch-bundle reversibility audit, a missing-artifact audit, an imported-user-handoff-bundle audit, an org-approved-bundle audit, or an unsupported-drift audit — its exact affected rows, freshness / reversibility state, missing-artifact posture, rollback target, and required refresh / narrow decision — onboarding / migration / support language never running ahead of a current, reversible bundle, canonical / accessible / audit resolution-form coverage, and a machine-readable shipping-line-drift-report (stale-bundle, non-reversible-bundle, or unsupported-bundle drift) that narrows the affected claim automatically when a claimed bundle drifts and names the active drift reason across start-center, migration / help, release / support, admin / public-proof, shiproom, executive-steering, and program-governance surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending line-bundle_refresh_audit parity on every journey; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_bundle_refresh_audit_beta_narrowed(
) -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacket {
    let mut packet =
        seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries();
    packet.packet_id =
        "m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries:bundle-refresh-audit-beta:0001"
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
pub fn seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_shipping_line_drift_report_preview_narrowed(
) -> M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacket {
    let mut packet =
        seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries();
    packet.packet_id =
        "m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries:shipping-line-drift-report-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5StableLineProtectionQualificationClass::Preview;
    packet
}
