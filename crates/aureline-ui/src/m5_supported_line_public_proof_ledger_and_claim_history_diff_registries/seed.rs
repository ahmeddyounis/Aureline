//! Canonical seed builders for the M5 line-public_proof_ledger and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-public_proof_ledger and line-downgrade-packet entries
//! are built so the one typed line-public_proof_ledger object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SUPPORTED_LINE_PUBLIC_PROOF_LEDGER_CLAIM_HISTORY_DIFF_REGISTRIES_PACKET_ID: &str =
    "m5-supported-line-public-proof-ledger-and-claim-history-diff-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn public_proof_ledger(
    input: M5PublicProofLedgerEntryResolutionInput,
) -> M5ResolvedPublicProofLedgerEntry {
    resolve_public_proof_ledger_entry(input).expect("seed line-public_proof_ledger entry resolves")
}

fn downgrade(input: M5ClaimHistoryDiffEntryResolutionInput) -> M5ResolvedClaimHistoryDiffEntry {
    resolve_claim_history_diff_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5SupportedLinePublicProofLedgerResolutionForm> {
    M5SupportedLinePublicProofLedgerResolutionForm::ALL.to_vec()
}

// -- Clean line-public_proof_ledger entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_public_proof_ledger_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    report_section: M5PublicProofLedgerKind,
    surface_context: M5SupportedLinePublicProofLedgerSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5PublicProofLedgerEntryResolutionInput {
    M5PublicProofLedgerEntryResolutionInput {
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

fn public_proof_ledger_compatibility_report_proof_clean() -> M5ResolvedPublicProofLedgerEntry {
    public_proof_ledger(clean_public_proof_ledger_base(
        "public_proof_ledger:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.public_proof_ledger.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5PublicProofLedgerKind::CompatibilityReportProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn public_proof_ledger_benchmark_packet_proof_clean() -> M5ResolvedPublicProofLedgerEntry {
    public_proof_ledger(clean_public_proof_ledger_base(
        "public_proof_ledger:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.public_proof_ledger.benchmark_packet_proof",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5PublicProofLedgerKind::BenchmarkPacketProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn public_proof_ledger_support_window_statement_proof_clean() -> M5ResolvedPublicProofLedgerEntry {
    public_proof_ledger(clean_public_proof_ledger_base(
        "public_proof_ledger:program-governance:extension-author",
        "launch.line.extension-author",
        "line.public_proof_ledger.support_window_statement_proof",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5PublicProofLedgerKind::SupportWindowStatementProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn public_proof_ledger_known_limits_set_proof_clean() -> M5ResolvedPublicProofLedgerEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_public_proof_ledger_base(
        "public_proof_ledger:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.public_proof_ledger.known_limits_set_proof",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5PublicProofLedgerKind::KnownLimitsSetProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ExecutiveSteeringSurface,
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
    public_proof_ledger(base)
}

fn public_proof_ledger_deprecation_report_proof_clean() -> M5ResolvedPublicProofLedgerEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_public_proof_ledger_base(
        "public_proof_ledger:support:public-preview",
        "launch.line.public-preview",
        "line.public_proof_ledger.deprecation_report_proof",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5PublicProofLedgerKind::DeprecationReportProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::SupportOrExportForm,
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
    public_proof_ledger(base)
}

fn public_proof_ledger_successor_report_proof_clean() -> M5ResolvedPublicProofLedgerEntry {
    public_proof_ledger(clean_public_proof_ledger_base(
        "public_proof_ledger:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.public_proof_ledger.successor_report_proof",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5PublicProofLedgerKind::SuccessorReportProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-public_proof_ledger entries ---------------------------------------------------------

/// Degraded public_proof_ledger entry: the resolved public_proof_ledger object is incomplete — the bundle IDs are unstated.
fn public_proof_ledger_object_incomplete() -> M5ResolvedPublicProofLedgerEntry {
    let mut base = clean_public_proof_ledger_base(
        "public_proof_ledger:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.public_proof_ledger.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5PublicProofLedgerKind::CompatibilityReportProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    public_proof_ledger(base)
}

/// Degraded public_proof_ledger entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn public_proof_ledger_widen_fold() -> M5ResolvedPublicProofLedgerEntry {
    let mut base = clean_public_proof_ledger_base(
        "public_proof_ledger:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.public_proof_ledger.benchmark_packet_proof",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5PublicProofLedgerKind::BenchmarkPacketProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    public_proof_ledger(base)
}

/// Degraded public_proof_ledger entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn public_proof_ledger_unbound() -> M5ResolvedPublicProofLedgerEntry {
    let mut base = clean_public_proof_ledger_base(
        "public_proof_ledger:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.public_proof_ledger.successor_report_proof",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5PublicProofLedgerKind::SuccessorReportProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    public_proof_ledger(base)
}

/// Degraded public_proof_ledger entry: the canonical registry token name is unstated.
fn public_proof_ledger_token_unstated() -> M5ResolvedPublicProofLedgerEntry {
    let mut base = clean_public_proof_ledger_base(
        "public_proof_ledger:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5PublicProofLedgerKind::SupportWindowStatementProof,
        M5SupportedLinePublicProofLedgerSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    public_proof_ledger(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    comparison_scope: M5ClaimHistoryDiffScope,
    surface_context: M5SupportedLinePublicProofLedgerSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ClaimHistoryDiffEntryResolutionInput {
    M5ClaimHistoryDiffEntryResolutionInput {
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
        keeps_claim_history_diff_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedClaimHistoryDiffEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5ClaimHistoryDiffScope::FreshnessChange,
        M5SupportedLinePublicProofLedgerSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedClaimHistoryDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.support_window_statement_proof",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5ClaimHistoryDiffScope::ScopeNarrowing,
        M5SupportedLinePublicProofLedgerSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedClaimHistoryDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.successor_report_proof",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5ClaimHistoryDiffScope::ReleaseLineReassociation,
        M5SupportedLinePublicProofLedgerSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedClaimHistoryDiffEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.deprecation_report_proof",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5ClaimHistoryDiffScope::FreshnessChange,
        M5SupportedLinePublicProofLedgerSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedClaimHistoryDiffEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.successor_report_proof",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5ClaimHistoryDiffScope::ReleaseLineReassociation,
        M5SupportedLinePublicProofLedgerSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5SupportedLinePublicProofLedgerResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_diff_scope_unclassified() -> M5ResolvedClaimHistoryDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.known_limits_set_proof",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5ClaimHistoryDiffScope::DiffScopeUnclassified,
        M5SupportedLinePublicProofLedgerSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    public_proof_ledger_entries: Vec<M5ResolvedPublicProofLedgerEntry>,
    claim_history_diff_entries: Vec<M5ResolvedClaimHistoryDiffEntry>,
) -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesRow {
    M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesRow {
        consumer_surface,
        qualification: M5SupportedLineTransparencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5SupportedLineTransparencyWideningStage::ALL.to_vec(),
        required_labels: M5SupportedLineTransparencyRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5SupportedLineTransparencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SupportedLinePublicProofLedgerAnatomyPart::ALL.to_vec(),
        export_fields: M5SupportedLinePublicProofLedgerExportField::ALL.to_vec(),
        downgrade_triggers,
        public_proof_ledger_entries,
        claim_history_diff_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUPPORTED_LINE_PUBLIC_PROOF_LEDGER_CLAIM_HISTORY_DIFF_REGISTRIES_SCHEMA_REF,
            M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
            M5_CLAIM_HISTORY_DIFF_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_claim_history_diff_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesRow> {
    use M5SupportedLineTransparencyConsumerSurface as C;
    use M5SupportedLineTransparencyDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the line's compatibility-report proof source to one typed public-proof-ledger object — the affected line rows, joined proof source, linked evidence-packet refs, freshness state, rollback target, and owning roster — from the shared registry and proves the freshness-change diff for that line; a public-proof-ledger object missing its linked evidence and a diff that keeps support language ahead of current proof degrade honestly instead of leaving a claim to read as still current",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![
                public_proof_ledger_compatibility_report_proof_clean(),
                public_proof_ledger_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the benchmark / evidence-packet proof source and the release-line-reassociation diff while keeping the active diff reason visible; a line widening its claim on stale proof and a resolution-form gap on a diff are caught before a screenshot can reintroduce a still-current reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::ImpliedGreenWhileProofOrArchiveWasStale,
                D::ProofStale,
            ],
            vec![public_proof_ledger_benchmark_packet_proof_clean(), public_proof_ledger_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the known-limits-set proof source while keeping its compatibility / known-issues claim matched to current public proof and reports the claim-history-diff outcome; a public-proof-ledger entry that is a hand-copied per-entry assumption and a diff on an unclassified diff scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ExportClassUnstated,
                D::ProofStale,
            ],
            vec![
                public_proof_ledger_known_limits_set_proof_clean(),
                public_proof_ledger_unbound(),
            ],
            vec![comparison_diff_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the support-window-statement proof source and the scope-narrowing diff bound to the registry; an unstated registry token on a public-proof-ledger entry is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::FreshnessWindowUnstated,
                D::ProofStale,
            ],
            vec![
                public_proof_ledger_support_window_statement_proof_clean(),
                public_proof_ledger_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved public-proof-ledger and claim-history-diff truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the successor-report proof source and the release-line-reassociation diff stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::FreshnessWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![public_proof_ledger_successor_report_proof_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved public-proof-ledger and claim-history-diff truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-proof attempt, or support language running ahead of current proof is visible in evidence — a freshness change, a scope narrowing, or a release-line reassociation — rather than hidden behind a screenshot",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![public_proof_ledger_deprecation_report_proof_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesGovernanceReview
{
    M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesGovernanceReview {
        public_proof_ledger_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_public_proof_ledger_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        claim_history_diff_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        public_proof_ledger_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection(
) -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesConsumerProjection {
    M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesProofFreshness {
    M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesReleasePosture {
    M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesReleasePosture {
        proof_packet_ref:
            M5_SUPPORTED_LINE_PUBLIC_PROOF_LEDGER_CLAIM_HISTORY_DIFF_REGISTRIES_ARTIFACT_REF
                .to_owned(),
        line_audit_ref:
            M5_SUPPORTED_LINE_PUBLIC_PROOF_LEDGER_CLAIM_HISTORY_DIFF_REGISTRIES_REPORT_REF
                .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORTED_LINE_PUBLIC_PROOF_LEDGER_CLAIM_HISTORY_DIFF_REGISTRIES_SCHEMA_REF,
        M5_SUPPORTED_LINE_PUBLIC_PROOF_LEDGER_CLAIM_HISTORY_DIFF_REGISTRIES_DOC_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
        M5_CLAIM_HISTORY_DIFF_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch public-proof-ledger and claim-history-diff registries packet.
pub fn seeded_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries(
) -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesPacket {
    M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesPacket::new(
        M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesPacketInput {
            packet_id: M5_SUPPORTED_LINE_PUBLIC_PROOF_LEDGER_CLAIM_HISTORY_DIFF_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 public-proof-ledger and claim-history-diff registries publishing one line-by-line public-proof ledger per active supported line — one section per joined proof source: a compatibility report, a benchmark / evidence packet, a support-window statement, a known-limits set, a deprecation report, and a successor report — each bound to one supported-line identity with its freshness state, last-versus-current diff, and the exact evidence-packet refs currently backing its public claims, with rollback posture preserved so onboarding / migration / support language never runs ahead of current public proof, canonical / accessible / audit resolution-form coverage, and a machine-readable claim-history diff (freshness-change, scope-narrowing, or release-line-reassociation) that turns a stale or mismatched proof source into a typed diff event showing current-versus-previous claim-state history, naming the active diff reason across release / help, About, docs, support, and public-proof surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries_public_proof_ledger_beta_narrowed(
) -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesPacket {
    let mut packet =
        seeded_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries();
    packet.packet_id =
        "m5-supported-line-public-proof-ledger-and-claim-history-diff-registries:public-proof-ledger-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SupportedLineTransparencyConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending claim-history-diff parity on every
/// diff scope; every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries_claim_history_diff_preview_narrowed(
) -> M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesPacket {
    let mut packet =
        seeded_m5_supported_line_public_proof_ledger_and_claim_history_diff_registries();
    packet.packet_id =
        "m5-supported-line-public-proof-ledger-and-claim-history-diff-registries:claim-history-diff-preview:0001"
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
