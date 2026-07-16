//! Canonical seed builders for the M5 line-last_supported_snapshot and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-last_supported_snapshot and line-downgrade-packet entries
//! are built so the one typed line-last_supported_snapshot object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_LAST_SUPPORTED_SNAPSHOT_ARCHIVE_EXPORT_GATE_REGISTRIES_PACKET_ID: &str =
    "m5-last-supported-snapshot-and-archive-export-gate-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn last_supported_snapshot(
    input: M5LastSupportedSnapshotEntryResolutionInput,
) -> M5ResolvedLastSupportedSnapshotEntry {
    resolve_last_supported_snapshot_entry(input)
        .expect("seed line-last_supported_snapshot entry resolves")
}

fn downgrade(input: M5ArchiveExportGateEntryResolutionInput) -> M5ResolvedArchiveExportGateEntry {
    resolve_archive_export_gate_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5LastSupportedSnapshotResolutionForm> {
    M5LastSupportedSnapshotResolutionForm::ALL.to_vec()
}

// -- Clean line-last_supported_snapshot entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_last_supported_snapshot_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    report_section: M5LastSupportedSnapshotKind,
    surface_context: M5LastSupportedSnapshotSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5LastSupportedSnapshotEntryResolutionInput {
    M5LastSupportedSnapshotEntryResolutionInput {
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

fn last_supported_snapshot_docs_help_truth_clean() -> M5ResolvedLastSupportedSnapshotEntry {
    last_supported_snapshot(clean_last_supported_snapshot_base(
        "last_supported_snapshot:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.last_supported_snapshot.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5LastSupportedSnapshotKind::DocsHelpTruth,
        M5LastSupportedSnapshotSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn last_supported_snapshot_schema_contract_set_clean() -> M5ResolvedLastSupportedSnapshotEntry {
    last_supported_snapshot(clean_last_supported_snapshot_base(
        "last_supported_snapshot:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.last_supported_snapshot.schema_contract_set",
        M5RetiredStateRole::SuccessorRouting,
        M5LastSupportedSnapshotKind::SchemaContractSet,
        M5LastSupportedSnapshotSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn last_supported_snapshot_known_limits_snapshot_clean() -> M5ResolvedLastSupportedSnapshotEntry {
    last_supported_snapshot(clean_last_supported_snapshot_base(
        "last_supported_snapshot:program-governance:extension-author",
        "launch.line.extension-author",
        "line.last_supported_snapshot.known_limits_snapshot",
        M5RetiredStateRole::DisablePath,
        M5LastSupportedSnapshotKind::KnownLimitsSnapshot,
        M5LastSupportedSnapshotSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn last_supported_snapshot_compatibility_report_reference_clean(
) -> M5ResolvedLastSupportedSnapshotEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_last_supported_snapshot_base(
        "last_supported_snapshot:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.last_supported_snapshot.compatibility_report_reference",
        M5RetiredStateRole::LastSupportedPin,
        M5LastSupportedSnapshotKind::CompatibilityReportReference,
        M5LastSupportedSnapshotSurfaceContext::ExecutiveSteeringSurface,
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
    last_supported_snapshot(base)
}

fn last_supported_snapshot_support_article_links_clean() -> M5ResolvedLastSupportedSnapshotEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_last_supported_snapshot_base(
        "last_supported_snapshot:support:public-preview",
        "launch.line.public-preview",
        "line.last_supported_snapshot.support_article_links",
        M5RetiredStateRole::SuccessorRouting,
        M5LastSupportedSnapshotKind::SupportArticleLinks,
        M5LastSupportedSnapshotSurfaceContext::SupportOrExportForm,
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
    last_supported_snapshot(base)
}

fn last_supported_snapshot_provenance_sbom_reference_clean() -> M5ResolvedLastSupportedSnapshotEntry
{
    last_supported_snapshot(clean_last_supported_snapshot_base(
        "last_supported_snapshot:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.last_supported_snapshot.provenance_sbom_reference",
        M5RetiredStateRole::SupportNoteClosure,
        M5LastSupportedSnapshotKind::ProvenanceSbomReference,
        M5LastSupportedSnapshotSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-last_supported_snapshot entries ---------------------------------------------------------

/// Degraded last_supported_snapshot entry: the resolved last_supported_snapshot object is incomplete — the bundle IDs are unstated.
fn last_supported_snapshot_object_incomplete() -> M5ResolvedLastSupportedSnapshotEntry {
    let mut base = clean_last_supported_snapshot_base(
        "last_supported_snapshot:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.last_supported_snapshot.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5LastSupportedSnapshotKind::DocsHelpTruth,
        M5LastSupportedSnapshotSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    last_supported_snapshot(base)
}

/// Degraded last_supported_snapshot entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn last_supported_snapshot_widen_fold() -> M5ResolvedLastSupportedSnapshotEntry {
    let mut base = clean_last_supported_snapshot_base(
        "last_supported_snapshot:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.last_supported_snapshot.schema_contract_set",
        M5RetiredStateRole::SuccessorRouting,
        M5LastSupportedSnapshotKind::SchemaContractSet,
        M5LastSupportedSnapshotSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    last_supported_snapshot(base)
}

/// Degraded last_supported_snapshot entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn last_supported_snapshot_unbound() -> M5ResolvedLastSupportedSnapshotEntry {
    let mut base = clean_last_supported_snapshot_base(
        "last_supported_snapshot:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.last_supported_snapshot.provenance_sbom_reference",
        M5RetiredStateRole::SupportNoteClosure,
        M5LastSupportedSnapshotKind::ProvenanceSbomReference,
        M5LastSupportedSnapshotSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    last_supported_snapshot(base)
}

/// Degraded last_supported_snapshot entry: the canonical registry token name is unstated.
fn last_supported_snapshot_token_unstated() -> M5ResolvedLastSupportedSnapshotEntry {
    let mut base = clean_last_supported_snapshot_base(
        "last_supported_snapshot:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5RetiredStateRole::DisablePath,
        M5LastSupportedSnapshotKind::KnownLimitsSnapshot,
        M5LastSupportedSnapshotSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    last_supported_snapshot(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    comparison_scope: M5ArchiveExportGateScope,
    surface_context: M5LastSupportedSnapshotSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ArchiveExportGateEntryResolutionInput {
    M5ArchiveExportGateEntryResolutionInput {
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
        keeps_archive_export_gate_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedArchiveExportGateEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5ArchiveExportGateScope::LiveDependencyPresent,
        M5LastSupportedSnapshotSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedArchiveExportGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.known_limits_snapshot",
        M5RetiredStateRole::DisablePath,
        M5ArchiveExportGateScope::InternalOnlyOrSecretLeak,
        M5LastSupportedSnapshotSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedArchiveExportGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.provenance_sbom_reference",
        M5RetiredStateRole::SupportNoteClosure,
        M5ArchiveExportGateScope::UnboundManifestOrReviewPacket,
        M5LastSupportedSnapshotSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedArchiveExportGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.support_article_links",
        M5RetiredStateRole::SuccessorRouting,
        M5ArchiveExportGateScope::LiveDependencyPresent,
        M5LastSupportedSnapshotSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedArchiveExportGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.provenance_sbom_reference",
        M5RetiredStateRole::SupportNoteClosure,
        M5ArchiveExportGateScope::UnboundManifestOrReviewPacket,
        M5LastSupportedSnapshotSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5LastSupportedSnapshotResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_archive_export_gate_scope_unclassified() -> M5ResolvedArchiveExportGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.compatibility_report_reference",
        M5RetiredStateRole::LastSupportedPin,
        M5ArchiveExportGateScope::ArchiveExportGateScopeUnclassified,
        M5LastSupportedSnapshotSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5LastSupportedSnapshotArchiveExportGateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RetiredStateDowngradeTrigger>,
    last_supported_snapshot_entries: Vec<M5ResolvedLastSupportedSnapshotEntry>,
    archive_export_gate_entries: Vec<M5ResolvedArchiveExportGateEntry>,
) -> M5LastSupportedSnapshotArchiveExportGateRegistriesRow {
    M5LastSupportedSnapshotArchiveExportGateRegistriesRow {
        consumer_surface,
        qualification: M5RetiredStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        removal_horizon_stages: M5RetiredStateRemovalHorizonStage::ALL.to_vec(),
        required_labels: M5RetiredStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5RetiredStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5LastSupportedSnapshotAnatomyPart::ALL.to_vec(),
        export_fields: M5LastSupportedSnapshotExportField::ALL.to_vec(),
        downgrade_triggers,
        last_supported_snapshot_entries,
        archive_export_gate_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_LAST_SUPPORTED_SNAPSHOT_ARCHIVE_EXPORT_GATE_REGISTRIES_SCHEMA_REF,
            M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF,
            M5_ARCHIVE_EXPORT_GATE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_archive_export_gate_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5LastSupportedSnapshotArchiveExportGateRegistriesRow> {
    use M5RetiredStateConsumerSurface as C;
    use M5RetiredStateDowngradeTrigger as D;

    vec![
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves a retiring object to one typed last-supported snapshot — its docs / help truth, schema / contract set, known-limits snapshot, and compatibility report from the shared registry, joined to its exact build — and proves the live-dependency-present archive-export gate for that bundle; a snapshot missing its docs / help truth and an archive-export gate that would hand off a bundle carrying a live vendor dependency degrade honestly instead of shipping an untrustworthy historical reference",
            "retirement:m5-release-center:001",
            vec![
                D::SuccessorPathUnnamed,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![
                last_supported_snapshot_docs_help_truth_clean(),
                last_supported_snapshot_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::HelpDocs,
            "Help/docs owner",
            "Help / docs resolves the schema / contract-set snapshot field and the unbound-manifest-or-review-packet archive-export gate while keeping the docs / help truth and compatibility report visible; an archive bundle not bound back to the retirement manifest and a resolution-form gap on an archive-export gate are caught before a help / docs card can point at a non-reproducible historical reference",
            "retirement:m5-help-docs:001",
            vec![
                D::SuccessorPathUnnamed,
                D::ArchivalNoteMissing,
                D::RetirementManifestStale,
            ],
            vec![last_supported_snapshot_schema_contract_set_clean(), last_supported_snapshot_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::Support,
            "Support owner",
            "Support resolves the compatibility-report snapshot field while keeping its public-facing compatibility / support claim matched to the archived successor and reports the archive-export-gate outcome; a snapshot that is a hand-copied per-entry assumption and an archive-export gate on an unclassified scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DisablePathUnnamed,
                D::RetirementManifestStale,
            ],
            vec![
                last_supported_snapshot_compatibility_report_reference_clean(),
                last_supported_snapshot_unbound(),
            ],
            vec![comparison_archive_export_gate_scope_unclassified()],
        ),
        base_row(
            C::MarketplaceRegistry,
            "Marketplace/registry owner",
            "The marketplace / registry surface resolves the known-limits-snapshot field and the internal-only-or-secret-leak archive-export gate bound to the registry so a retirement archive bundle can never be handed off carrying a leaked secret or internal-only detail while staying inspectable by its docs / help truth and known-limits snapshot; an unstated registry token on a snapshot is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CutoffDateUnstated,
                D::RetirementManifestStale,
            ],
            vec![
                last_supported_snapshot_known_limits_snapshot_clean(),
                last_supported_snapshot_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::InstallUpdate,
            "Install/update owner",
            "Install / update surfaces render the same resolved last-supported-snapshot and archive-export-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the provenance / SBOM snapshot field and the archive-export gate stay inspectable off-renderer so an offline consumer can open the historical reference without live vendor dependencies",
            "retirement:m5-install-update:001",
            vec![
                D::CutoffDateUnstated,
                D::RegistryReferenceUnstated,
                D::RetirementManifestStale,
            ],
            vec![last_supported_snapshot_provenance_sbom_reference_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::PartnerProcurement,
            "Partner/procurement owner",
            "The partner / procurement and CLI / headless inspect feed carries the same resolved last-supported-snapshot and archive-export-gate truth, so a hand-copied constant, an unstated registry token, an archive bundle carrying a live dependency, leaking a secret / internal-only detail, or unbound from its retirement manifest and review packet is visible in evidence — an archive bundle blocked from handoff until it is export-safe and mirror-aware — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![last_supported_snapshot_support_article_links_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5LastSupportedSnapshotArchiveExportGateRegistriesGovernanceReview {
    M5LastSupportedSnapshotArchiveExportGateRegistriesGovernanceReview {
        last_supported_snapshot_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_last_supported_snapshot_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        archive_export_gate_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        last_supported_snapshot_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5LastSupportedSnapshotArchiveExportGateRegistriesConsumerProjection {
    M5LastSupportedSnapshotArchiveExportGateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5LastSupportedSnapshotArchiveExportGateRegistriesProofFreshness {
    M5LastSupportedSnapshotArchiveExportGateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LastSupportedSnapshotArchiveExportGateRegistriesReleasePosture {
    M5LastSupportedSnapshotArchiveExportGateRegistriesReleasePosture {
        proof_packet_ref: M5_LAST_SUPPORTED_SNAPSHOT_ARCHIVE_EXPORT_GATE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_LAST_SUPPORTED_SNAPSHOT_ARCHIVE_EXPORT_GATE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LAST_SUPPORTED_SNAPSHOT_ARCHIVE_EXPORT_GATE_REGISTRIES_SCHEMA_REF,
        M5_LAST_SUPPORTED_SNAPSHOT_ARCHIVE_EXPORT_GATE_REGISTRIES_DOC_REF,
        M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
        M5_RETIRED_STATE_MATRIX_DOC_REF,
        M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF,
        M5_ARCHIVE_EXPORT_GATE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 last-supported-snapshot and archive-export-gate registries packet.
pub fn seeded_m5_last_supported_snapshot_and_archive_export_gate_registries(
) -> M5LastSupportedSnapshotArchiveExportGateRegistriesPacket {
    M5LastSupportedSnapshotArchiveExportGateRegistriesPacket::new(
        M5LastSupportedSnapshotArchiveExportGateRegistriesPacketInput {
            packet_id: M5_LAST_SUPPORTED_SNAPSHOT_ARCHIVE_EXPORT_GATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 last-supported-snapshot and archive-export-gate registries shipping last-supported snapshot and retirement archive bundles for a retiring M5 line or stable-facing surface across the release-center, help / docs, support, marketplace / registry, install / update, and partner / procurement surfaces so migration, audit, procurement, and support can inspect what was retired without keeping the retired surface live — one export-safe last-supported snapshot per retiring object (its docs / help truth, schema / contract set, known-limits snapshot, compatibility report, provenance / SBOM reference, and support-article links captured for the final supported build or line state and joined to its exact build) with canonical / accessible / audit resolution-form coverage, and a machine-readable archive-export gate (live-dependency-present, internal-only-or-secret-leak, or unbound-manifest-or-review-packet) that blocks an archive bundle from being handed off while it carries a live vendor dependency, would leak a secret or internal-only detail, or is not bound back to the retirement manifest and review packet, so self-hosted, offline, and procurement / support consumers open one export-safe historical reference that names the final supported version / channel and the successor path without contradiction"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5LastSupportedSnapshotArchiveExportGateRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the release-center row is held at Beta pending last-supported-snapshot parity on every
/// snapshot field; every row stays visible and every example stays honest.
pub fn seeded_m5_last_supported_snapshot_and_archive_export_gate_registries_last_supported_snapshot_beta_narrowed(
) -> M5LastSupportedSnapshotArchiveExportGateRegistriesPacket {
    let mut packet = seeded_m5_last_supported_snapshot_and_archive_export_gate_registries();
    packet.packet_id =
        "m5-last-supported-snapshot-and-archive-export-gate-registries:last-supported-snapshot-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5RetiredStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the help/docs row is narrowed to Preview pending archive-export-gate parity on every
/// archive-export scope; every row stays visible and every example stays honest.
pub fn seeded_m5_last_supported_snapshot_and_archive_export_gate_registries_archive_export_gate_preview_narrowed(
) -> M5LastSupportedSnapshotArchiveExportGateRegistriesPacket {
    let mut packet = seeded_m5_last_supported_snapshot_and_archive_export_gate_registries();
    packet.packet_id =
        "m5-last-supported-snapshot-and-archive-export-gate-registries:archive-export-gate-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::HelpDocs)
        .expect("help/docs row present");
    row.qualification = M5RetiredStateQualificationClass::Preview;
    packet
}
