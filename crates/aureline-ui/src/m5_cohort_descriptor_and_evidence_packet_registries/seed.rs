//! Canonical seed builders for the M5 cohort-descriptor and cohort-evidence-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean cohort-descriptor and cohort-evidence-packet entries
//! are built so the one typed cohort-descriptor object resolving per cohort, cohorts never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of cohort proof,
//! the canonical / accessible / audit resolution forms, and the complete cohort-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! cohort-evidence object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-cohort assumption,
//! widen-without-rollback, incomplete object, hidden cohort evidence, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_PACKET_ID: &str =
    "m5-cohort-descriptor-and-evidence-packet-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn descriptor(input: M5CohortDescriptorEntryResolutionInput) -> M5ResolvedCohortDescriptorEntry {
    resolve_cohort_descriptor_entry(input).expect("seed cohort-descriptor entry resolves")
}

fn evidence(
    input: M5CohortEvidencePacketEntryResolutionInput,
) -> M5ResolvedCohortEvidencePacketEntry {
    resolve_cohort_evidence_packet_entry(input).expect("seed cohort-evidence-packet entry resolves")
}

fn all_forms() -> Vec<M5CohortResolutionForm> {
    M5CohortResolutionForm::ALL.to_vec()
}

// -- Clean cohort-descriptor entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_descriptor_base(
    entry_id: &str,
    cohort_binding_id: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    cohort_archetype: M5CohortArchetypeKind,
    surface_context: M5CohortSurfaceContext,
    exact_repo_archetype_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5CohortDescriptorEntryResolutionInput {
    M5CohortDescriptorEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        cohort_binding_id: cohort_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        cohort_archetype,
        surface_context,
        resolution_form_coverage: all_forms(),
        exact_repo_archetype_rows: exact_repo_archetype_rows.to_owned(),
        bundle_ids: bundle_ids.to_owned(),
        install_topology: install_topology.to_owned(),
        toolchain_envelope: toolchain_envelope.to_owned(),
        known_limits: known_limits.to_owned(),
        rollback_target: rollback_target.to_owned(),
        diagnostics_posture: diagnostics_posture.to_owned(),
        bound_to_registry: true,
        rollback_and_diagnostics_bounded: true,
        is_public_facing_cohort: false,
        support_language_matches_cohort_proof: true,
        proof_fresh: true,
    }
}

fn descriptor_dogfood_core_team_canary_clean() -> M5ResolvedCohortDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:shiproom:dogfood-core-team-canary",
        "launch.cohort.core-team-canary",
        "cohort.descriptor.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5CohortArchetypeKind::DogfoodCoreTeamCanary,
        M5CohortSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn descriptor_migration_alpha_clean() -> M5ResolvedCohortDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:migration-alpha",
        "launch.cohort.migration-alpha",
        "cohort.descriptor.migration_alpha",
        M5LaunchControlRole::ReadinessEvent,
        M5CohortArchetypeKind::MigrationAlpha,
        M5CohortSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn descriptor_extension_author_clean() -> M5ResolvedCohortDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:program-governance:extension-author",
        "launch.cohort.extension-author",
        "cohort.descriptor.extension_author",
        M5LaunchControlRole::RehearsalCurrency,
        M5CohortArchetypeKind::ExtensionAuthor,
        M5CohortSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn descriptor_design_partner_preview_clean() -> M5ResolvedCohortDescriptorEntry {
    // A design-partner preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:design-partner-preview",
        "launch.cohort.design-partner-preview",
        "cohort.descriptor.design_partner_preview",
        M5LaunchControlRole::CohortMembership,
        M5CohortArchetypeKind::DesignPartnerPreview,
        M5CohortSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-archetypes",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.is_public_facing_cohort = true;
    base.support_language_matches_cohort_proof = true;
    descriptor(base)
}

fn descriptor_public_preview_clean() -> M5ResolvedCohortDescriptorEntry {
    // A public preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:support:public-preview",
        "launch.cohort.public-preview",
        "cohort.descriptor.public_preview",
        M5LaunchControlRole::ReadinessEvent,
        M5CohortArchetypeKind::PublicPreview,
        M5CohortSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-archetypes",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.is_public_facing_cohort = true;
    base.support_language_matches_cohort_proof = true;
    descriptor(base)
}

fn descriptor_certified_archetype_clean() -> M5ResolvedCohortDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:certified-archetype",
        "launch.cohort.certified-archetype",
        "cohort.descriptor.certified_archetype",
        M5LaunchControlRole::GoNoGoAuthority,
        M5CohortArchetypeKind::CertifiedArchetype,
        M5CohortSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-archetype-archetypes",
        "bundle.ids.certified-0007",
        "install.topology.certified-archetype-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-archetype",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded cohort-descriptor entries ---------------------------------------------------------

/// Degraded descriptor entry: the resolved descriptor object is incomplete — the bundle IDs are unstated.
fn descriptor_object_incomplete() -> M5ResolvedCohortDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:shiproom:incomplete",
        "launch.cohort.core-team-canary",
        "cohort.descriptor.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5CohortArchetypeKind::DogfoodCoreTeamCanary,
        M5CohortSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    descriptor(base)
}

/// Degraded descriptor entry: the cohort's rollback and diagnostics posture is not preserved before widening —
/// a cohort widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn descriptor_widen_fold() -> M5ResolvedCohortDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:release-center:widen-fold",
        "launch.cohort.migration-alpha",
        "cohort.descriptor.migration_alpha",
        M5LaunchControlRole::ReadinessEvent,
        M5CohortArchetypeKind::MigrationAlpha,
        M5CohortSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    descriptor(base)
}

/// Degraded descriptor entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn descriptor_unbound() -> M5ResolvedCohortDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:unbound",
        "launch.cohort.certified-archetype",
        "cohort.descriptor.certified_archetype",
        M5LaunchControlRole::GoNoGoAuthority,
        M5CohortArchetypeKind::CertifiedArchetype,
        M5CohortSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-archetype-archetypes",
        "bundle.ids.certified-0007",
        "install.topology.certified-archetype-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-archetype",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    descriptor(base)
}

/// Degraded descriptor entry: the canonical registry token name is unstated.
fn descriptor_token_unstated() -> M5ResolvedCohortDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:program-governance:token-unstated",
        "launch.cohort.extension-author",
        "  ",
        M5LaunchControlRole::RehearsalCurrency,
        M5CohortArchetypeKind::ExtensionAuthor,
        M5CohortSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    descriptor(base)
}

// -- Clean cohort-evidence-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_evidence_base(
    entry_id: &str,
    evidence_ref: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    evidence_scope: M5CohortEvidenceScope,
    surface_context: M5CohortSurfaceContext,
    resolved_cohort_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5CohortEvidencePacketEntryResolutionInput {
    M5CohortEvidencePacketEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        evidence_ref: evidence_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        evidence_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_cohort_identity: resolved_cohort_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_cohort_evidence_visible: true,
        evidence_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn evidence_dogfood_ring_clean() -> M5ResolvedCohortEvidencePacketEntry {
    // A dogfood-ring evidence packet carries partner / public support language bound to cohort proof.
    let mut base = clean_evidence_base(
        "evidence:shiproom:dogfood-ring",
        "launch.cohort.core-team-canary",
        "cohort.evidence.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5CohortEvidenceScope::DogfoodRingEvidence,
        M5CohortSurfaceContext::ShiproomSurface,
        "cohort-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = true;
    evidence(base)
}

fn evidence_rehearsal_currency_clean() -> M5ResolvedCohortEvidencePacketEntry {
    evidence(clean_evidence_base(
        "evidence:program-governance:rehearsal-currency",
        "launch.cohort.extension-author",
        "cohort.evidence.extension_author",
        M5LaunchControlRole::RehearsalCurrency,
        M5CohortEvidenceScope::RehearsalCurrencyEvidence,
        M5CohortSurfaceContext::ProgramGovernanceSurface,
        "cohort-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn evidence_go_no_go_signoff_clean() -> M5ResolvedCohortEvidencePacketEntry {
    evidence(clean_evidence_base(
        "evidence:release-center:go-no-go-signoff",
        "launch.cohort.certified-archetype",
        "cohort.evidence.certified_archetype",
        M5LaunchControlRole::GoNoGoAuthority,
        M5CohortEvidenceScope::GoNoGoSignoffEvidence,
        M5CohortSurfaceContext::ReleaseCenterSurface,
        "cohort-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded cohort-evidence-packet entries ----------------------------------------------------

/// Degraded evidence entry: the evidence would run partner / public support language ahead of cohort proof — a
/// support-language reference present but not bound to cohort proof reads as trustworthy when the cohort proof
/// does not yet back it.
fn evidence_support_ahead() -> M5ResolvedCohortEvidencePacketEntry {
    let mut base = clean_evidence_base(
        "evidence:shiproom:support-ahead",
        "launch.cohort.public-preview",
        "cohort.evidence.public_preview",
        M5LaunchControlRole::ReadinessEvent,
        M5CohortEvidenceScope::DogfoodRingEvidence,
        M5CohortSurfaceContext::ShiproomSurface,
        "cohort-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = false;
    evidence(base)
}

/// Degraded evidence entry: the canonical / accessible / audit resolution-form coverage of the evidence is
/// incomplete.
fn evidence_form_incomplete() -> M5ResolvedCohortEvidencePacketEntry {
    let mut base = clean_evidence_base(
        "evidence:release-center:form-incomplete",
        "launch.cohort.certified-archetype",
        "cohort.evidence.certified_archetype",
        M5LaunchControlRole::GoNoGoAuthority,
        M5CohortEvidenceScope::GoNoGoSignoffEvidence,
        M5CohortSurfaceContext::ReleaseCenterSurface,
        "cohort-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5CohortResolutionForm::CanonicalObject];
    evidence(base)
}

/// Degraded evidence entry: the evidence scope is unclassified.
fn evidence_scope_unclassified() -> M5ResolvedCohortEvidencePacketEntry {
    evidence(clean_evidence_base(
        "evidence:executive-steering:scope-unclassified",
        "launch.cohort.design-partner-preview",
        "cohort.evidence.design_partner_preview",
        M5LaunchControlRole::CohortMembership,
        M5CohortEvidenceScope::ScopeUnclassified,
        M5CohortSurfaceContext::ExecutiveSteeringSurface,
        "cohort-id.design-partner-preview-0007",
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
    consumer_surface: M5CohortDescriptorEvidencePacketRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    cohort_descriptor_entries: Vec<M5ResolvedCohortDescriptorEntry>,
    cohort_evidence_packet_entries: Vec<M5ResolvedCohortEvidencePacketEntry>,
) -> M5CohortDescriptorEvidencePacketRegistriesRow {
    M5CohortDescriptorEvidencePacketRegistriesRow {
        consumer_surface,
        qualification: M5LaunchControlQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5LaunchControlWideningStage::ALL.to_vec(),
        required_labels: M5LaunchControlRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5LaunchControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5CohortAnatomyPart::ALL.to_vec(),
        export_fields: M5CohortExportField::ALL.to_vec(),
        downgrade_triggers,
        cohort_descriptor_entries,
        cohort_evidence_packet_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_REF,
            M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_COHORT_EVIDENCE_PACKET_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_cohort_without_current_rollback_and_diagnostics_evidence: false,
        runs_partner_or_public_support_language_ahead_of_cohort_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_cohort_evidence_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5CohortDescriptorEvidencePacketRegistriesRow> {
    use M5LaunchControlConsumerSurface as C;
    use M5LaunchControlDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the core-team canary cohort's descriptor to one typed object — cohort archetype, exact repo / archetype rows, bundle IDs, install topology, toolchain envelope, known limits, rollback target, and diagnostics posture — from the shared registry and proves the dogfood-ring cohort-evidence packet for the canary cohort; a descriptor object missing its bundle IDs and an evidence packet that runs partner / public support language ahead of cohort proof degrade honestly instead of reading as a clean pass",
            "evidence:m5-launch-control-shiproom:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![
                descriptor_dogfood_core_team_canary_clean(),
                descriptor_object_incomplete(),
            ],
            vec![evidence_dogfood_ring_clean(), evidence_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the migration alpha cohort's descriptor and the go/no-go-signoff cohort-evidence packet while keeping the cohort evidence visible; a cohort widening without a preserved rollback target and diagnostics posture and a resolution-form gap on an evidence packet are caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-launch-control-release-center:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::ImpliedGreenWhileGoNoGoOrOrrWasStale,
                D::ProofStale,
            ],
            vec![descriptor_migration_alpha_clean(), descriptor_widen_fold()],
            vec![evidence_go_no_go_signoff_clean(), evidence_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the design-partner preview cohort's descriptor while keeping its partner support language matched to cohort proof and reports the certified-archetype cohort evidence; a descriptor that is a hand-copied per-entry assumption and an evidence packet on an unclassified evidence scope degrade honestly",
            "evidence:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ReadinessStateUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_design_partner_preview_clean(),
                descriptor_unbound(),
            ],
            vec![evidence_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the extension-author cohort's descriptor and the rehearsal-currency cohort-evidence packet bound to the registry; an unstated registry token on a descriptor is caught before it can drift",
            "evidence:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CohortMembershipUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_extension_author_clean(),
                descriptor_token_unstated(),
            ],
            vec![evidence_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved cohort-descriptor and cohort-evidence-packet truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied cohort table; the certified-archetype descriptor and the go/no-go-signoff evidence stay inspectable off-renderer",
            "evidence:m5-launch-control-diagnostics:001",
            vec![
                D::CohortMembershipUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![descriptor_certified_archetype_clean()],
            vec![evidence_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved cohort-descriptor and cohort-evidence-packet truth, so a hand-copied constant, an unstated registry token, a widen-without-rollback attempt, or support language running ahead of proof is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![descriptor_public_preview_clean()],
            vec![evidence_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5CohortDescriptorEvidencePacketRegistriesGovernanceReview {
    M5CohortDescriptorEvidencePacketRegistriesGovernanceReview {
        descriptor_registry_names_token_role_and_archetype: true,
        cohort_resolves_to_typed_descriptor_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        cohorts_cannot_widen_without_rollback_and_diagnostics: true,
        cohort_evidence_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_cohort_proof_for_public_cohorts: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        descriptor_or_evidence_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5CohortDescriptorEvidencePacketRegistriesConsumerProjection {
    M5CohortDescriptorEvidencePacketRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5CohortDescriptorEvidencePacketRegistriesProofFreshness {
    M5CohortDescriptorEvidencePacketRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CohortDescriptorEvidencePacketRegistriesReleasePosture {
    M5CohortDescriptorEvidencePacketRegistriesReleasePosture {
        proof_packet_ref: M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_ARTIFACT_REF.to_owned(),
        cohort_audit_ref: M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_SCHEMA_REF,
        M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_COHORT_EVIDENCE_PACKET_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 cohort-descriptor and cohort-evidence-packet registries packet.
pub fn seeded_m5_cohort_descriptor_and_evidence_packet_registries(
) -> M5CohortDescriptorEvidencePacketRegistriesPacket {
    M5CohortDescriptorEvidencePacketRegistriesPacket::new(
        M5CohortDescriptorEvidencePacketRegistriesPacketInput {
            packet_id: M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 cohort-descriptor and cohort-evidence-packet registries with one typed cohort-descriptor object resolving per cohort, cohorts never widening without preserving rollback and diagnostics, partner / public support language never running ahead of cohort proof, canonical / accessible / audit resolution-form coverage, and the complete cohort-identity / known-limits-ledger / rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision cohort-evidence object across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5CohortDescriptorEvidencePacketRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending cohort-descriptor parity on every archetype; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_descriptor_beta_narrowed(
) -> M5CohortDescriptorEvidencePacketRegistriesPacket {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.packet_id =
        "m5-cohort-descriptor-and-evidence-packet-registries:cohort-descriptor-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5LaunchControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending cohort-evidence parity on every
/// archetype; every row stays visible and every example stays honest.
pub fn seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_evidence_preview_narrowed(
) -> M5CohortDescriptorEvidencePacketRegistriesPacket {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.packet_id =
        "m5-cohort-descriptor-and-evidence-packet-registries:cohort-evidence-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5LaunchControlQualificationClass::Preview;
    packet
}
