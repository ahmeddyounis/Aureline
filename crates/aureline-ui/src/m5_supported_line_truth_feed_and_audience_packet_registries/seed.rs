//! Canonical seed builders for the M5 truth-feed and audience-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean truth-feed and audience-packet entries
//! are built so the one typed truth feed resolving per line, lines never widening a claim without preserving
//! their exact-build provenance, a claim never running ahead of current proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / bundled-truth-feed /
//! public-safe-versus-internal / packet-scope / active-note audience-packet object are proven across the
//! shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-line assumption, widen-without-provenance, incomplete object, internal-only leak, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_PACKET_ID: &str =
    "m5-supported-line-truth-feed-and-audience-packet-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn truth_feed(input: M5TruthFeedEntryResolutionInput) -> M5ResolvedTruthFeedEntry {
    resolve_truth_feed_entry(input).expect("seed line-truth_feed entry resolves")
}

fn downgrade(input: M5AudiencePacketEntryResolutionInput) -> M5ResolvedAudiencePacketEntry {
    resolve_audience_packet_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5SupportedLineTruthFeedResolutionForm> {
    M5SupportedLineTruthFeedResolutionForm::ALL.to_vec()
}

// -- Clean line-truth_feed entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_truth_feed_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    report_section: M5TruthFeedKind,
    surface_context: M5SupportedLineTruthFeedSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5TruthFeedEntryResolutionInput {
    M5TruthFeedEntryResolutionInput {
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

fn truth_feed_public_proof_summary_clean() -> M5ResolvedTruthFeedEntry {
    truth_feed(clean_truth_feed_base(
        "truth_feed:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.truth_feed.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5TruthFeedKind::PublicProofSummary,
        M5SupportedLineTruthFeedSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn truth_feed_migration_scoreboard_summary_clean() -> M5ResolvedTruthFeedEntry {
    truth_feed(clean_truth_feed_base(
        "truth_feed:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.truth_feed.migration_scoreboard_summary",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5TruthFeedKind::MigrationScoreboardSummary,
        M5SupportedLineTruthFeedSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn truth_feed_transparency_snapshot_clean() -> M5ResolvedTruthFeedEntry {
    truth_feed(clean_truth_feed_base(
        "truth_feed:program-governance:extension-author",
        "launch.line.extension-author",
        "line.truth_feed.transparency_snapshot",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5TruthFeedKind::TransparencySnapshot,
        M5SupportedLineTruthFeedSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn truth_feed_correction_history_summary_clean() -> M5ResolvedTruthFeedEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_truth_feed_base(
        "truth_feed:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.truth_feed.correction_history_summary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5TruthFeedKind::CorrectionHistorySummary,
        M5SupportedLineTruthFeedSurfaceContext::ExecutiveSteeringSurface,
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
    truth_feed(base)
}

fn truth_feed_claim_history_summary_clean() -> M5ResolvedTruthFeedEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_truth_feed_base(
        "truth_feed:support:public-preview",
        "launch.line.public-preview",
        "line.truth_feed.claim_history_summary",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5TruthFeedKind::ClaimHistorySummary,
        M5SupportedLineTruthFeedSurfaceContext::SupportOrExportForm,
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
    truth_feed(base)
}

fn truth_feed_release_evidence_link_clean() -> M5ResolvedTruthFeedEntry {
    truth_feed(clean_truth_feed_base(
        "truth_feed:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.truth_feed.release_evidence_link",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5TruthFeedKind::ReleaseEvidenceLink,
        M5SupportedLineTruthFeedSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-truth_feed entries ---------------------------------------------------------

/// Degraded truth_feed entry: the resolved truth_feed object is incomplete — the bundle IDs are unstated.
fn truth_feed_object_incomplete() -> M5ResolvedTruthFeedEntry {
    let mut base = clean_truth_feed_base(
        "truth_feed:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.truth_feed.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5TruthFeedKind::PublicProofSummary,
        M5SupportedLineTruthFeedSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    truth_feed(base)
}

/// Degraded truth_feed entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn truth_feed_widen_fold() -> M5ResolvedTruthFeedEntry {
    let mut base = clean_truth_feed_base(
        "truth_feed:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.truth_feed.migration_scoreboard_summary",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5TruthFeedKind::MigrationScoreboardSummary,
        M5SupportedLineTruthFeedSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    truth_feed(base)
}

/// Degraded truth_feed entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn truth_feed_unbound() -> M5ResolvedTruthFeedEntry {
    let mut base = clean_truth_feed_base(
        "truth_feed:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.truth_feed.release_evidence_link",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5TruthFeedKind::ReleaseEvidenceLink,
        M5SupportedLineTruthFeedSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    truth_feed(base)
}

/// Degraded truth_feed entry: the canonical registry token name is unstated.
fn truth_feed_token_unstated() -> M5ResolvedTruthFeedEntry {
    let mut base = clean_truth_feed_base(
        "truth_feed:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5TruthFeedKind::TransparencySnapshot,
        M5SupportedLineTruthFeedSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    truth_feed(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    comparison_scope: M5AudiencePacketScope,
    surface_context: M5SupportedLineTruthFeedSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5AudiencePacketEntryResolutionInput {
    M5AudiencePacketEntryResolutionInput {
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
        keeps_audience_packet_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedAudiencePacketEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5AudiencePacketScope::SupportBundle,
        M5SupportedLineTruthFeedSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedAudiencePacketEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.transparency_snapshot",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5AudiencePacketScope::ProcurementBundle,
        M5SupportedLineTruthFeedSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedAudiencePacketEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.release_evidence_link",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5AudiencePacketScope::PartnerReviewBundle,
        M5SupportedLineTruthFeedSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedAudiencePacketEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.claim_history_summary",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5AudiencePacketScope::SupportBundle,
        M5SupportedLineTruthFeedSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedAudiencePacketEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.release_evidence_link",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5AudiencePacketScope::PartnerReviewBundle,
        M5SupportedLineTruthFeedSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5SupportedLineTruthFeedResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_audience_packet_scope_unclassified() -> M5ResolvedAudiencePacketEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.correction_history_summary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5AudiencePacketScope::AudiencePacketScopeUnclassified,
        M5SupportedLineTruthFeedSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    truth_feed_entries: Vec<M5ResolvedTruthFeedEntry>,
    audience_packet_entries: Vec<M5ResolvedAudiencePacketEntry>,
) -> M5SupportedLineTruthFeedAudiencePacketRegistriesRow {
    M5SupportedLineTruthFeedAudiencePacketRegistriesRow {
        consumer_surface,
        qualification: M5SupportedLineTransparencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5SupportedLineTransparencyWideningStage::ALL.to_vec(),
        required_labels: M5SupportedLineTransparencyRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5SupportedLineTransparencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SupportedLineTruthFeedAnatomyPart::ALL.to_vec(),
        export_fields: M5SupportedLineTruthFeedExportField::ALL.to_vec(),
        downgrade_triggers,
        truth_feed_entries,
        audience_packet_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_REF,
            M5_TRUTH_FEED_DOMAIN_SCHEMA_REF,
            M5_AUDIENCE_PACKET_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_audience_packet_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5SupportedLineTruthFeedAudiencePacketRegistriesRow> {
    use M5SupportedLineTransparencyConsumerSurface as C;
    use M5SupportedLineTransparencyDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the line's public-proof summary to one typed truth feed — the feed section, its current claim, its evidence freshness, and the owning roster — from the shared registry and projects the support-bundle audience packet for that line; a truth feed missing its exact-build provenance and a packet variant that keeps a claim ahead of current proof degrade honestly instead of leaving a stale line to read as still green",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![
                truth_feed_public_proof_summary_clean(),
                truth_feed_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the migration-scoreboard-summary feed section and the partner-review-bundle audience packet while keeping the active packet note visible; a line widening its claim on stale proof and a resolution-form gap on a packet variant are caught before a screenshot can reintroduce a still-green reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::ImpliedGreenWhileProofOrArchiveWasStale,
                D::ProofStale,
            ],
            vec![truth_feed_migration_scoreboard_summary_clean(), truth_feed_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the correction-history-summary feed section (public-facing) while keeping its published summary matched to current exact-build proof and reports the audience-packet outcome; a truth feed that is a hand-copied per-entry assumption and a packet variant on an unclassified packet scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ExportClassUnstated,
                D::ProofStale,
            ],
            vec![
                truth_feed_correction_history_summary_clean(),
                truth_feed_unbound(),
            ],
            vec![comparison_audience_packet_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the transparency-snapshot feed section and the procurement-bundle audience packet bound to the registry; an unstated registry token on a truth feed is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::FreshnessWindowUnstated,
                D::ProofStale,
            ],
            vec![
                truth_feed_transparency_snapshot_clean(),
                truth_feed_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved truth-feed and audience-packet truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the release-evidence-link feed section and the partner-review-bundle packet variant stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::FreshnessWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![truth_feed_release_evidence_link_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved truth-feed and audience-packet truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-proof attempt, or a claim running ahead of current proof is visible in evidence — the support, procurement, and partner-review packet variants projected from one canonical feed — rather than hidden behind a shiproom note or private materials",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![truth_feed_claim_history_summary_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5SupportedLineTruthFeedAudiencePacketRegistriesGovernanceReview {
    M5SupportedLineTruthFeedAudiencePacketRegistriesGovernanceReview {
        truth_feed_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_truth_feed_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        audience_packet_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        truth_feed_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerProjection {
    M5SupportedLineTruthFeedAudiencePacketRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SupportedLineTruthFeedAudiencePacketRegistriesProofFreshness {
    M5SupportedLineTruthFeedAudiencePacketRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportedLineTruthFeedAudiencePacketRegistriesReleasePosture {
    M5SupportedLineTruthFeedAudiencePacketRegistriesReleasePosture {
        proof_packet_ref: M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_DOC_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_TRUTH_FEED_DOMAIN_SCHEMA_REF,
        M5_AUDIENCE_PACKET_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 supported-line truth-feed and audience-packet registries packet.
pub fn seeded_m5_supported_line_truth_feed_and_audience_packet_registries(
) -> M5SupportedLineTruthFeedAudiencePacketRegistriesPacket {
    M5SupportedLineTruthFeedAudiencePacketRegistriesPacket::new(
        M5SupportedLineTruthFeedAudiencePacketRegistriesPacketInput {
            packet_id: M5_SUPPORTED_LINE_TRUTH_FEED_AUDIENCE_PACKET_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 supported-line truth-feed and audience-packet registries bundling one truth feed per active stable or LTS-candidate line — one row per feed section: a public-proof summary, a migration-scoreboard summary, a transparency snapshot, a correction-history summary, a claim-history summary, and a release-evidence link, tracked against exact build / release-line identity — each bound to one supported-line identity with its stable ID and freshness date and its links out to compatibility reports, known limits, migration guides, and release evidence, public-safe correction-history and claim-history summaries separated from internal-only incident / security payloads, exact-build provenance preserved so a claim never runs ahead of it, canonical / accessible / audit resolution-form coverage, and export-safe audience-packet variants (support-bundle, procurement-bundle, or partner-review-bundle) that project one canonical feed for a named audience, excluding internal-only detail by default while still naming the current claim, evidence freshness, migration posture, and correction history across release / help, docs, support, procurement, and partner surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5SupportedLineTruthFeedAudiencePacketRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending truth-feed parity on every feed section;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_truth_feed_and_audience_packet_registries_truth_feed_beta_narrowed(
) -> M5SupportedLineTruthFeedAudiencePacketRegistriesPacket {
    let mut packet = seeded_m5_supported_line_truth_feed_and_audience_packet_registries();
    packet.packet_id =
        "m5-supported-line-truth-feed-and-audience-packet-registries:truth-feed-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SupportedLineTransparencyConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending audience-packet parity on every
/// packet scope; every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_truth_feed_and_audience_packet_registries_audience_packet_preview_narrowed(
) -> M5SupportedLineTruthFeedAudiencePacketRegistriesPacket {
    let mut packet = seeded_m5_supported_line_truth_feed_and_audience_packet_registries();
    packet.packet_id =
        "m5-supported-line-truth-feed-and-audience-packet-registries:audience-packet-preview:0001"
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
