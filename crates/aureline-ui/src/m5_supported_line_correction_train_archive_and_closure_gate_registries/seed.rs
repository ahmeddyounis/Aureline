//! Canonical seed builders for the M5 correction-train-archive and closure-gate registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean correction-train-archive and closure-gate entries
//! are built so the one typed correction-train archive resolving per line, lines never widening a correction
//! claim without preserving their exact-build provenance, a claim never running ahead of archived provenance,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / affected-archive-record /
//! archived-versus-active-build / gate-scope / active-reason closure-gate object are proven across the
//! shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-line assumption, widen-without-provenance, incomplete object, hidden gap, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SUPPORTED_LINE_CORRECTION_TRAIN_ARCHIVE_CLOSURE_GATE_REGISTRIES_PACKET_ID: &str =
    "m5-supported-line-correction-train-archive-and-closure-gate-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn correction_train_archive(
    input: M5CorrectionTrainArchiveEntryResolutionInput,
) -> M5ResolvedCorrectionTrainArchiveEntry {
    resolve_correction_train_archive_entry(input)
        .expect("seed line-correction_train_archive entry resolves")
}

fn downgrade(input: M5ClosureGateEntryResolutionInput) -> M5ResolvedClosureGateEntry {
    resolve_closure_gate_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5SupportedLineCorrectionTrainArchiveResolutionForm> {
    M5SupportedLineCorrectionTrainArchiveResolutionForm::ALL.to_vec()
}

// -- Clean line-correction_train_archive entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_correction_train_archive_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    report_section: M5CorrectionTrainArchiveKind,
    surface_context: M5SupportedLineCorrectionTrainArchiveSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5CorrectionTrainArchiveEntryResolutionInput {
    M5CorrectionTrainArchiveEntryResolutionInput {
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

fn correction_train_archive_hotfix_packet_archive_clean() -> M5ResolvedCorrectionTrainArchiveEntry {
    correction_train_archive(clean_correction_train_archive_base(
        "correction_train_archive:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.correction_train_archive.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5CorrectionTrainArchiveKind::HotfixPacketArchive,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn correction_train_archive_backport_packet_archive_clean() -> M5ResolvedCorrectionTrainArchiveEntry
{
    correction_train_archive(clean_correction_train_archive_base(
        "correction_train_archive:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.correction_train_archive.backport_packet_archive",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5CorrectionTrainArchiveKind::BackportPacketArchive,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn correction_train_archive_rollback_outcome_record_clean() -> M5ResolvedCorrectionTrainArchiveEntry
{
    correction_train_archive(clean_correction_train_archive_base(
        "correction_train_archive:program-governance:extension-author",
        "launch.line.extension-author",
        "line.correction_train_archive.rollback_outcome_record",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5CorrectionTrainArchiveKind::RollbackOutcomeRecord,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn correction_train_archive_advisory_publication_clean() -> M5ResolvedCorrectionTrainArchiveEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_correction_train_archive_base(
        "correction_train_archive:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.correction_train_archive.advisory_publication",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5CorrectionTrainArchiveKind::AdvisoryPublication,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ExecutiveSteeringSurface,
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
    correction_train_archive(base)
}

fn correction_train_archive_public_communication_bundle_clean(
) -> M5ResolvedCorrectionTrainArchiveEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_correction_train_archive_base(
        "correction_train_archive:support:public-preview",
        "launch.line.public-preview",
        "line.correction_train_archive.public_communication_bundle",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5CorrectionTrainArchiveKind::PublicCommunicationBundle,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::SupportOrExportForm,
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
    correction_train_archive(base)
}

fn correction_train_archive_revocation_record_clean() -> M5ResolvedCorrectionTrainArchiveEntry {
    correction_train_archive(clean_correction_train_archive_base(
        "correction_train_archive:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.correction_train_archive.revocation_record",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5CorrectionTrainArchiveKind::RevocationRecord,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-correction_train_archive entries ---------------------------------------------------------

/// Degraded correction_train_archive entry: the resolved correction_train_archive object is incomplete — the bundle IDs are unstated.
fn correction_train_archive_object_incomplete() -> M5ResolvedCorrectionTrainArchiveEntry {
    let mut base = clean_correction_train_archive_base(
        "correction_train_archive:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.correction_train_archive.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5CorrectionTrainArchiveKind::HotfixPacketArchive,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    correction_train_archive(base)
}

/// Degraded correction_train_archive entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn correction_train_archive_widen_fold() -> M5ResolvedCorrectionTrainArchiveEntry {
    let mut base = clean_correction_train_archive_base(
        "correction_train_archive:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.correction_train_archive.backport_packet_archive",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5CorrectionTrainArchiveKind::BackportPacketArchive,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    correction_train_archive(base)
}

/// Degraded correction_train_archive entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn correction_train_archive_unbound() -> M5ResolvedCorrectionTrainArchiveEntry {
    let mut base = clean_correction_train_archive_base(
        "correction_train_archive:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.correction_train_archive.revocation_record",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5CorrectionTrainArchiveKind::RevocationRecord,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    correction_train_archive(base)
}

/// Degraded correction_train_archive entry: the canonical registry token name is unstated.
fn correction_train_archive_token_unstated() -> M5ResolvedCorrectionTrainArchiveEntry {
    let mut base = clean_correction_train_archive_base(
        "correction_train_archive:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5CorrectionTrainArchiveKind::RollbackOutcomeRecord,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    correction_train_archive(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    comparison_scope: M5ClosureGateScope,
    surface_context: M5SupportedLineCorrectionTrainArchiveSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ClosureGateEntryResolutionInput {
    M5ClosureGateEntryResolutionInput {
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
        keeps_closure_gate_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedClosureGateEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5ClosureGateScope::MissingArchiveCoverage,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedClosureGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.rollback_outcome_record",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5ClosureGateScope::BrokenExactBuildJoin,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedClosureGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.revocation_record",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5ClosureGateScope::UntraceableCorrectionLine,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedClosureGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.public_communication_bundle",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5ClosureGateScope::MissingArchiveCoverage,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedClosureGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.revocation_record",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5ClosureGateScope::UntraceableCorrectionLine,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5SupportedLineCorrectionTrainArchiveResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_closure_gate_scope_unclassified() -> M5ResolvedClosureGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.advisory_publication",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5ClosureGateScope::ClosureGateScopeUnclassified,
        M5SupportedLineCorrectionTrainArchiveSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    correction_train_archive_entries: Vec<M5ResolvedCorrectionTrainArchiveEntry>,
    closure_gate_entries: Vec<M5ResolvedClosureGateEntry>,
) -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesRow {
    M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesRow {
        consumer_surface,
        qualification: M5SupportedLineTransparencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5SupportedLineTransparencyWideningStage::ALL.to_vec(),
        required_labels: M5SupportedLineTransparencyRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5SupportedLineTransparencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SupportedLineCorrectionTrainArchiveAnatomyPart::ALL.to_vec(),
        export_fields: M5SupportedLineCorrectionTrainArchiveExportField::ALL.to_vec(),
        downgrade_triggers,
        correction_train_archive_entries,
        closure_gate_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUPPORTED_LINE_CORRECTION_TRAIN_ARCHIVE_CLOSURE_GATE_REGISTRIES_SCHEMA_REF,
            M5_CORRECTION_TRAIN_ARCHIVE_DOMAIN_SCHEMA_REF,
            M5_CLOSURE_GATE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_closure_gate_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesRow> {
    use M5SupportedLineTransparencyConsumerSurface as C;
    use M5SupportedLineTransparencyDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the line's archived hotfix packet to one typed correction-train archive — the corrective action class, its rollback outcome, the public communication state, and the owning roster — from the shared registry and proves the missing-archive-coverage closure-gate event for that line; a correction-train archive missing its exact-build provenance and a gate event that keeps a correction claim ahead of archived provenance degrade honestly instead of leaving a shipped correction to read as still green",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![
                correction_train_archive_hotfix_packet_archive_clean(),
                correction_train_archive_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the backport-packet-archive record and the untraceable-correction-line closure-gate event while keeping the active gate reason visible; a line widening its claim on stale archive evidence and a resolution-form gap on a gate event are caught before a screenshot can reintroduce a still-green reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::ImpliedGreenWhileProofOrArchiveWasStale,
                D::ProofStale,
            ],
            vec![correction_train_archive_backport_packet_archive_clean(), correction_train_archive_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the advisory-publication record (public-facing) while keeping its published communication matched to archived exact-build provenance and reports the closure-gate outcome; a correction-train archive that is a hand-copied per-entry assumption and a gate event on an unclassified gate scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ExportClassUnstated,
                D::ProofStale,
            ],
            vec![
                correction_train_archive_advisory_publication_clean(),
                correction_train_archive_unbound(),
            ],
            vec![comparison_closure_gate_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the rollback-outcome-record and the broken-exact-build-join closure-gate event bound to the registry; an unstated registry token on a correction-train archive is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::FreshnessWindowUnstated,
                D::ProofStale,
            ],
            vec![
                correction_train_archive_rollback_outcome_record_clean(),
                correction_train_archive_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved correction-archive and closure-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the revocation-record archive and the untraceable-correction-line gate event stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::FreshnessWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![correction_train_archive_revocation_record_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved correction-archive and closure-gate truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-archive attempt, or a correction claim running ahead of archived provenance is visible in evidence — missing archive coverage, a broken exact-build join, or an untraceable correction line — rather than hidden behind a shiproom note or private materials",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![correction_train_archive_public_communication_bundle_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesGovernanceReview
{
    M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesGovernanceReview {
        correction_train_archive_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_correction_train_archive_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        closure_gate_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        correction_train_archive_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection(
) -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesConsumerProjection {
    M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesProofFreshness {
    M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesReleasePosture {
    M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesReleasePosture {
        proof_packet_ref:
            M5_SUPPORTED_LINE_CORRECTION_TRAIN_ARCHIVE_CLOSURE_GATE_REGISTRIES_ARTIFACT_REF
                .to_owned(),
        line_audit_ref:
            M5_SUPPORTED_LINE_CORRECTION_TRAIN_ARCHIVE_CLOSURE_GATE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORTED_LINE_CORRECTION_TRAIN_ARCHIVE_CLOSURE_GATE_REGISTRIES_SCHEMA_REF,
        M5_SUPPORTED_LINE_CORRECTION_TRAIN_ARCHIVE_CLOSURE_GATE_REGISTRIES_DOC_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_CORRECTION_TRAIN_ARCHIVE_DOMAIN_SCHEMA_REF,
        M5_CLOSURE_GATE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 supported-line correction-train-archive and closure-gate registries packet.
pub fn seeded_m5_supported_line_correction_train_archive_and_closure_gate_registries(
) -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesPacket {
    M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesPacket::new(
        M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesPacketInput {
            packet_id: M5_SUPPORTED_LINE_CORRECTION_TRAIN_ARCHIVE_CLOSURE_GATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 correction-train-archive and closure-gate registries archiving one correction-train archive per shipped correction packet on each active stable or LTS-candidate line — one row per corrective action class: a hotfix packet, a backport packet, a rollback outcome, an advisory publication, a public-communication bundle, and a revocation record, tracked against exact build / release-line identity — each bound to one supported-line identity with its bug-ID / defect-ledger / release-artifact-graph joins and the public-claim or support-window state the correction affected, public-safe advisory and public-communication history separated from internal-only hotfix / backport / rollback / revocation incident payloads, exact-build provenance preserved so a correction claim never runs ahead of it, canonical / accessible / audit resolution-form coverage, and a machine-readable closure-gate event (missing-archive-coverage, broken-exact-build-join, or untraceable-correction-line) that turns missing archive coverage, a broken exact-build join, or a correction that can no longer be traced back to its archived evidence into a typed event on the active line, blocking correction-line closure until fixed and naming the active gate reason across release / help, docs, support, and procurement surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending correction-train-archive parity on every action class;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_correction_train_archive_and_closure_gate_registries_correction_train_archive_beta_narrowed(
) -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesPacket {
    let mut packet =
        seeded_m5_supported_line_correction_train_archive_and_closure_gate_registries();
    packet.packet_id =
        "m5-supported-line-correction-train-archive-and-closure-gate-registries:correction-train-archive-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SupportedLineTransparencyConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending closure-gate parity on every
/// gate scope; every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_correction_train_archive_and_closure_gate_registries_closure_gate_preview_narrowed(
) -> M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesPacket {
    let mut packet =
        seeded_m5_supported_line_correction_train_archive_and_closure_gate_registries();
    packet.packet_id =
        "m5-supported-line-correction-train-archive-and-closure-gate-registries:closure-gate-preview:0001"
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
