//! Canonical seed builders for the M5 line-refresh_policy and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-refresh_policy and line-downgrade-packet entries
//! are built so the one typed line-refresh_policy object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_STABLE_LINE_REFRESH_POLICY_CLAIM_DOWNGRADE_REGISTRIES_PACKET_ID: &str =
    "m5-stable-line-refresh-policy-and-claim-downgrade-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn refresh_policy(input: M5RefreshPolicyEntryResolutionInput) -> M5ResolvedRefreshPolicyEntry {
    resolve_refresh_policy_entry(input).expect("seed line-refresh_policy entry resolves")
}

fn downgrade(input: M5ClaimDowngradeEntryResolutionInput) -> M5ResolvedClaimDowngradeEntry {
    resolve_claim_downgrade_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5StableLineRefreshResolutionForm> {
    M5StableLineRefreshResolutionForm::ALL.to_vec()
}

// -- Clean line-refresh_policy entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_refresh_policy_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    refresh_surface: M5RefreshSurfaceKind,
    surface_context: M5StableLineRefreshSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5RefreshPolicyEntryResolutionInput {
    M5RefreshPolicyEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        line_binding_id: line_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        refresh_surface,
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

fn refresh_policy_certified_archetype_report_clean() -> M5ResolvedRefreshPolicyEntry {
    refresh_policy(clean_refresh_policy_base(
        "refresh_policy:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.refresh_policy.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5RefreshSurfaceKind::CertifiedArchetypeReport,
        M5StableLineRefreshSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn refresh_policy_compatibility_packet_clean() -> M5ResolvedRefreshPolicyEntry {
    refresh_policy(clean_refresh_policy_base(
        "refresh_policy:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.refresh_policy.compatibility_packet",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5RefreshSurfaceKind::CompatibilityPacket,
        M5StableLineRefreshSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn refresh_policy_known_limits_doc_clean() -> M5ResolvedRefreshPolicyEntry {
    refresh_policy(clean_refresh_policy_base(
        "refresh_policy:program-governance:extension-author",
        "launch.line.extension-author",
        "line.refresh_policy.known_limits_doc",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5RefreshSurfaceKind::KnownLimitsDoc,
        M5StableLineRefreshSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn refresh_policy_release_help_about_surface_clean() -> M5ResolvedRefreshPolicyEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_refresh_policy_base(
        "refresh_policy:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.refresh_policy.release_help_about_surface",
        M5StableLineProtectionRole::SupportWindow,
        M5RefreshSurfaceKind::ReleaseHelpAboutSurface,
        M5StableLineRefreshSurfaceContext::ExecutiveSteeringSurface,
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
    refresh_policy(base)
}

fn refresh_policy_public_proof_surface_clean() -> M5ResolvedRefreshPolicyEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_refresh_policy_base(
        "refresh_policy:support:public-preview",
        "launch.line.public-preview",
        "line.refresh_policy.public_proof_surface",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5RefreshSurfaceKind::PublicProofSurface,
        M5StableLineRefreshSurfaceContext::SupportOrExportForm,
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
    refresh_policy(base)
}

fn refresh_policy_support_export_packet_clean() -> M5ResolvedRefreshPolicyEntry {
    refresh_policy(clean_refresh_policy_base(
        "refresh_policy:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.refresh_policy.support_export_packet",
        M5StableLineProtectionRole::BackportDecision,
        M5RefreshSurfaceKind::SupportExportPacket,
        M5StableLineRefreshSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-refresh_policy entries ---------------------------------------------------------

/// Degraded refresh_policy entry: the resolved refresh_policy object is incomplete — the bundle IDs are unstated.
fn refresh_policy_object_incomplete() -> M5ResolvedRefreshPolicyEntry {
    let mut base = clean_refresh_policy_base(
        "refresh_policy:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.refresh_policy.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5RefreshSurfaceKind::CertifiedArchetypeReport,
        M5StableLineRefreshSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    refresh_policy(base)
}

/// Degraded refresh_policy entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn refresh_policy_widen_fold() -> M5ResolvedRefreshPolicyEntry {
    let mut base = clean_refresh_policy_base(
        "refresh_policy:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.refresh_policy.compatibility_packet",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5RefreshSurfaceKind::CompatibilityPacket,
        M5StableLineRefreshSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    refresh_policy(base)
}

/// Degraded refresh_policy entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn refresh_policy_unbound() -> M5ResolvedRefreshPolicyEntry {
    let mut base = clean_refresh_policy_base(
        "refresh_policy:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.refresh_policy.support_export_packet",
        M5StableLineProtectionRole::BackportDecision,
        M5RefreshSurfaceKind::SupportExportPacket,
        M5StableLineRefreshSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    refresh_policy(base)
}

/// Degraded refresh_policy entry: the canonical registry token name is unstated.
fn refresh_policy_token_unstated() -> M5ResolvedRefreshPolicyEntry {
    let mut base = clean_refresh_policy_base(
        "refresh_policy:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5RefreshSurfaceKind::KnownLimitsDoc,
        M5StableLineRefreshSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    refresh_policy(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    downgrade_ref: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    downgrade_scope: M5ClaimDowngradeScope,
    surface_context: M5StableLineRefreshSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ClaimDowngradeEntryResolutionInput {
    M5ClaimDowngradeEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        downgrade_ref: downgrade_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        downgrade_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_line_identity: resolved_line_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_claim_downgrade_visible: true,
        downgrade_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedClaimDowngradeEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5ClaimDowngradeScope::RetestPendingDowngrade,
        M5StableLineRefreshSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedClaimDowngradeEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.known_limits_doc",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5ClaimDowngradeScope::EvidenceStaleDowngrade,
        M5StableLineRefreshSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedClaimDowngradeEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.support_export_packet",
        M5StableLineProtectionRole::BackportDecision,
        M5ClaimDowngradeScope::NarrowedSupportDowngrade,
        M5StableLineRefreshSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedClaimDowngradeEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.public_proof_surface",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5ClaimDowngradeScope::RetestPendingDowngrade,
        M5StableLineRefreshSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedClaimDowngradeEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.support_export_packet",
        M5StableLineProtectionRole::BackportDecision,
        M5ClaimDowngradeScope::NarrowedSupportDowngrade,
        M5StableLineRefreshSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5StableLineRefreshResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn downgrade_scope_unclassified() -> M5ResolvedClaimDowngradeEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.release_help_about_surface",
        M5StableLineProtectionRole::SupportWindow,
        M5ClaimDowngradeScope::ScopeUnclassified,
        M5StableLineRefreshSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5StableLineRefreshPolicyClaimDowngradeRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5StableLineProtectionDowngradeTrigger>,
    refresh_policy_entries: Vec<M5ResolvedRefreshPolicyEntry>,
    claim_downgrade_entries: Vec<M5ResolvedClaimDowngradeEntry>,
) -> M5StableLineRefreshPolicyClaimDowngradeRegistriesRow {
    M5StableLineRefreshPolicyClaimDowngradeRegistriesRow {
        consumer_surface,
        qualification: M5StableLineProtectionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5StableLineProtectionWideningStage::ALL.to_vec(),
        required_labels: M5StableLineProtectionRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5StableLineProtectionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5StableLineRefreshAnatomyPart::ALL.to_vec(),
        export_fields: M5StableLineRefreshExportField::ALL.to_vec(),
        downgrade_triggers,
        refresh_policy_entries,
        claim_downgrade_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STABLE_LINE_REFRESH_POLICY_CLAIM_DOWNGRADE_REGISTRIES_SCHEMA_REF,
            M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF,
            M5_CLAIM_DOWNGRADE_PACKET_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_claim_downgrade_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5StableLineRefreshPolicyClaimDowngradeRegistriesRow> {
    use M5StableLineProtectionConsumerSurface as C;
    use M5StableLineProtectionDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the active stable line's certified-archetype refresh schedule to one typed object — the refresh surface, its scheduled rows, cadence window, last-run and next-run identity, next-run owner, last-success state, and freshness posture — from the shared registry and proves the Retest-pending claim-downgrade packet for that surface; a refresh-policy object missing its schedule fields and a claim downgrade that keeps support language ahead of current proof degrade honestly instead of reading as silently stale",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![
                refresh_policy_certified_archetype_report_clean(),
                refresh_policy_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the compatibility-packet refresh schedule and the narrowed-support claim-downgrade packet while keeping the active downgrade reason visible; a line widening its claim without a current last-success run and a resolution-form gap on a claim downgrade are caught before a screenshot can reintroduce a silently-stale reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::ImpliedGreenWhileRefreshOrLedgerWasStale,
                D::ProofStale,
            ],
            vec![refresh_policy_compatibility_packet_clean(), refresh_policy_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the release / help / About refresh schedule while keeping its public support language matched to current refresh proof and reports the claim-downgrade packet; a refresh policy that is a hand-copied per-entry assumption and a claim downgrade on an unclassified downgrade scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RefreshStateUnstated,
                D::ProofStale,
            ],
            vec![
                refresh_policy_release_help_about_surface_clean(),
                refresh_policy_unbound(),
            ],
            vec![downgrade_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the known-limits-doc refresh schedule and the Evidence-stale claim-downgrade packet bound to the registry; an unstated registry token on a refresh policy is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportWindowUnstated,
                D::ProofStale,
            ],
            vec![
                refresh_policy_known_limits_doc_clean(),
                refresh_policy_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved refresh-policy and claim-downgrade packet truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the support-export-packet refresh schedule and the narrowed-support downgrade stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::SupportWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![refresh_policy_support_export_packet_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved refresh-policy and claim-downgrade packet truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-evidence attempt, or support language running ahead of current refresh proof is visible in evidence — current or explicitly downgraded — rather than hidden behind a screenshot",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![refresh_policy_public_proof_surface_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5StableLineRefreshPolicyClaimDowngradeRegistriesGovernanceReview {
    M5StableLineRefreshPolicyClaimDowngradeRegistriesGovernanceReview {
        refresh_policy_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_refresh_policy_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        claim_downgrade_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        refresh_policy_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5StableLineRefreshPolicyClaimDowngradeRegistriesConsumerProjection {
    M5StableLineRefreshPolicyClaimDowngradeRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5StableLineRefreshPolicyClaimDowngradeRegistriesProofFreshness {
    M5StableLineRefreshPolicyClaimDowngradeRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StableLineRefreshPolicyClaimDowngradeRegistriesReleasePosture {
    M5StableLineRefreshPolicyClaimDowngradeRegistriesReleasePosture {
        proof_packet_ref: M5_STABLE_LINE_REFRESH_POLICY_CLAIM_DOWNGRADE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_STABLE_LINE_REFRESH_POLICY_CLAIM_DOWNGRADE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STABLE_LINE_REFRESH_POLICY_CLAIM_DOWNGRADE_REGISTRIES_SCHEMA_REF,
        M5_STABLE_LINE_REFRESH_POLICY_CLAIM_DOWNGRADE_REGISTRIES_DOC_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF,
        M5_CLAIM_DOWNGRADE_PACKET_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 line-refresh_policy and line-downgrade-packet registries packet.
pub fn seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries(
) -> M5StableLineRefreshPolicyClaimDowngradeRegistriesPacket {
    M5StableLineRefreshPolicyClaimDowngradeRegistriesPacket::new(
        M5StableLineRefreshPolicyClaimDowngradeRegistriesPacketInput {
            packet_id: M5_STABLE_LINE_REFRESH_POLICY_CLAIM_DOWNGRADE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 stable-line refresh-policy scheduler and claim-downgrade packet registries with one typed scheduled-refresh object resolving per active stable line — the certified-archetype report, compatibility packet, known-limits doc, release / help / About surface, public-proof surface, and support-export packet, each carrying an exact last-run and next-run identity, next-run owner, last-success state, and freshness SLO — support language never running ahead of current refresh proof, canonical / accessible / audit resolution-form coverage, and a machine-readable claim-downgrade packet (Retest-pending, Evidence-stale, or narrower support language) that narrows the affected claim automatically and names the active downgrade reason across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5StableLineRefreshPolicyClaimDowngradeRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending line-refresh_policy parity on every journey; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_refresh_policy_beta_narrowed(
) -> M5StableLineRefreshPolicyClaimDowngradeRegistriesPacket {
    let mut packet = seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries();
    packet.packet_id =
        "m5-stable-line-refresh-policy-and-claim-downgrade-registries:refresh-policy-beta:0001"
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
pub fn seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_claim_downgrade_preview_narrowed(
) -> M5StableLineRefreshPolicyClaimDowngradeRegistriesPacket {
    let mut packet = seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries();
    packet.packet_id =
        "m5-stable-line-refresh-policy-and-claim-downgrade-registries:claim-downgrade-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5StableLineProtectionQualificationClass::Preview;
    packet
}
