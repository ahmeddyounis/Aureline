//! Canonical seed builders for the M5 regression-asset and incident-close registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean regression-asset and incident-close entries are built
//! so the one typed regression-asset object resolving per regression asset type, rings never advancing without a
//! visible known-limits and incident-close posture, partner / public support language never running ahead of ring
//! proof, the canonical / accessible / audit resolution forms, and the complete transition-identity /
//! active-stop-condition-ledger / incident-close-target / protected-metric-regression / packet-freshness /
//! crash-data-loss-or-trust / last-ring-transition-revision incident-close record are proven across the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-ring assumption, widen-without-stop, incomplete object, hidden incident-close, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_PACKET_ID: &str =
    "m5-regression-asset-and-incident-close-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn descriptor(input: M5RegressionAssetEntryResolutionInput) -> M5ResolvedRegressionAssetEntry {
    resolve_regression_asset_entry(input).expect("seed regression-asset entry resolves")
}

fn evidence(input: M5IncidentCloseEntryResolutionInput) -> M5ResolvedIncidentCloseEntry {
    resolve_incident_close_entry(input).expect("seed incident-close entry resolves")
}

fn all_forms() -> Vec<M5RegressionAssetResolutionForm> {
    M5RegressionAssetResolutionForm::ALL.to_vec()
}

// -- Clean regression-asset entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_descriptor_base(
    entry_id: &str,
    asset_binding_id: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    regression_asset_type: M5RegressionAssetTypeKind,
    surface_context: M5RegressionAssetSurfaceContext,
    exact_build_reference: &str,
    affected_row_reference: &str,
    cohort_ring_reference: &str,
    workaround_lineage: &str,
    regression_asset_reference: &str,
    approved_exception_reference: &str,
    close_blocker_reference: &str,
) -> M5RegressionAssetEntryResolutionInput {
    M5RegressionAssetEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        asset_binding_id: asset_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        regression_asset_type,
        surface_context,
        resolution_form_coverage: all_forms(),
        exact_build_reference: exact_build_reference.to_owned(),
        affected_row_reference: affected_row_reference.to_owned(),
        cohort_ring_reference: cohort_ring_reference.to_owned(),
        workaround_lineage: workaround_lineage.to_owned(),
        regression_asset_reference: regression_asset_reference.to_owned(),
        approved_exception_reference: approved_exception_reference.to_owned(),
        close_blocker_reference: close_blocker_reference.to_owned(),
        bound_to_registry: true,
        regression_asset_linked_before_closure: true,
        is_severe_incident: false,
        attributable_asset_or_approved_exception: true,
        proof_fresh: true,
    }
}

fn descriptor_automated_test_clean() -> M5ResolvedRegressionAssetEntry {
    descriptor(clean_descriptor_base(
        "descriptor:shiproom:dogfood-core-team-canary",
        "incident.lane.core-team-canary",
        "regression.asset.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5RegressionAssetTypeKind::AutomatedTest,
        M5RegressionAssetSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn descriptor_fixture_repository_clean() -> M5ResolvedRegressionAssetEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:migration-alpha",
        "incident.lane.migration-alpha",
        "regression.asset.fixture_repository",
        M5LaunchControlRole::ReadinessEvent,
        M5RegressionAssetTypeKind::FixtureRepository,
        M5RegressionAssetSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn descriptor_recovery_drill_clean() -> M5ResolvedRegressionAssetEntry {
    descriptor(clean_descriptor_base(
        "descriptor:program-governance:extension-author",
        "incident.lane.extension-author",
        "regression.asset.recovery_drill",
        M5LaunchControlRole::RehearsalCurrency,
        M5RegressionAssetTypeKind::RecoveryDrill,
        M5RegressionAssetSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn descriptor_protected_corpus_case_clean() -> M5ResolvedRegressionAssetEntry {
    // A design-partner preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:design-partner-preview",
        "incident.lane.design-partner-preview",
        "regression.asset.protected_corpus_case",
        M5LaunchControlRole::CohortMembership,
        M5RegressionAssetTypeKind::ProtectedCorpusCase,
        M5RegressionAssetSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-archetypes",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.is_severe_incident = true;
    base.attributable_asset_or_approved_exception = true;
    descriptor(base)
}

fn descriptor_schema_policy_guard_clean() -> M5ResolvedRegressionAssetEntry {
    // A public preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:support:public-preview",
        "incident.lane.public-preview",
        "regression.asset.schema_policy_guard",
        M5LaunchControlRole::ReadinessEvent,
        M5RegressionAssetTypeKind::SchemaPolicyGuard,
        M5RegressionAssetSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-archetypes",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.is_severe_incident = true;
    base.attributable_asset_or_approved_exception = true;
    descriptor(base)
}

fn descriptor_monitoring_regression_check_clean() -> M5ResolvedRegressionAssetEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:certified-archetype",
        "incident.lane.certified-archetype",
        "regression.asset.monitoring_regression_check",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RegressionAssetTypeKind::MonitoringRegressionCheck,
        M5RegressionAssetSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-archetype-archetypes",
        "bundle.ids.certified-0007",
        "install.topology.certified-archetype-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-archetype",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded regression-asset entries ---------------------------------------------------------

/// Degraded descriptor entry: the resolved descriptor object is incomplete — the bundle IDs are unstated.
fn descriptor_object_incomplete() -> M5ResolvedRegressionAssetEntry {
    let mut base = clean_descriptor_base(
        "descriptor:shiproom:incomplete",
        "incident.lane.core-team-canary",
        "regression.asset.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5RegressionAssetTypeKind::AutomatedTest,
        M5RegressionAssetSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.affected_row_reference = "   ".to_owned();
    descriptor(base)
}

/// Degraded descriptor entry: the cohort's rollback and diagnostics posture is not preserved before widening —
/// a cohort widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn descriptor_widen_fold() -> M5ResolvedRegressionAssetEntry {
    let mut base = clean_descriptor_base(
        "descriptor:release-center:widen-fold",
        "incident.lane.migration-alpha",
        "regression.asset.fixture_repository",
        M5LaunchControlRole::ReadinessEvent,
        M5RegressionAssetTypeKind::FixtureRepository,
        M5RegressionAssetSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.regression_asset_linked_before_closure = false;
    descriptor(base)
}

/// Degraded descriptor entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn descriptor_unbound() -> M5ResolvedRegressionAssetEntry {
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:unbound",
        "incident.lane.certified-archetype",
        "regression.asset.monitoring_regression_check",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RegressionAssetTypeKind::MonitoringRegressionCheck,
        M5RegressionAssetSurfaceContext::ExecutiveSteeringSurface,
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
fn regression_asset_token_unstated() -> M5ResolvedRegressionAssetEntry {
    let mut base = clean_descriptor_base(
        "descriptor:program-governance:token-unstated",
        "incident.lane.extension-author",
        "  ",
        M5LaunchControlRole::RehearsalCurrency,
        M5RegressionAssetTypeKind::RecoveryDrill,
        M5RegressionAssetSurfaceContext::ProgramGovernanceSurface,
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

// -- Clean incident-close entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_evidence_base(
    entry_id: &str,
    incident_close_ref: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    incident_severity: M5IncidentSeverityKind,
    surface_context: M5RegressionAssetSurfaceContext,
    resolved_incident_identity: &str,
    linked_regression_asset_ledger: &str,
    exact_build_and_row_reference: &str,
    cohort_ring_lineage_state: &str,
    close_lineage_freshness_state: &str,
    workaround_lineage_reference: &str,
    last_incident_close_revision: &str,
) -> M5IncidentCloseEntryResolutionInput {
    M5IncidentCloseEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        incident_close_ref: incident_close_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        incident_severity,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_incident_identity: resolved_incident_identity.to_owned(),
        linked_regression_asset_ledger: linked_regression_asset_ledger.to_owned(),
        exact_build_and_row_reference: exact_build_and_row_reference.to_owned(),
        cohort_ring_lineage_state: cohort_ring_lineage_state.to_owned(),
        close_lineage_freshness_state: close_lineage_freshness_state.to_owned(),
        workaround_lineage_reference: workaround_lineage_reference.to_owned(),
        last_incident_close_revision: last_incident_close_revision.to_owned(),
        keeps_incident_lineage_visible: true,
        close_lineage_is_truthful: true,
        close_without_asset_requested: false,
        close_blocked_until_asset_linked: false,
        lineage_gap_present: false,
        lineage_gap_flagged: false,
        proof_fresh: true,
    }
}

fn evidence_dogfood_ring_clean() -> M5ResolvedIncidentCloseEntry {
    // A dogfood-ring evidence packet carries partner / public support language bound to cohort proof.
    let mut base = clean_evidence_base(
        "evidence:shiproom:dogfood-ring",
        "incident.lane.core-team-canary",
        "incident.close.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5IncidentSeverityKind::SevOneIncident,
        M5RegressionAssetSurfaceContext::ShiproomSurface,
        "transition-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.close_without_asset_requested = true;
    base.close_blocked_until_asset_linked = true;
    evidence(base)
}

fn evidence_rehearsal_currency_clean() -> M5ResolvedIncidentCloseEntry {
    evidence(clean_evidence_base(
        "evidence:program-governance:rehearsal-currency",
        "incident.lane.extension-author",
        "incident.close.recovery_drill",
        M5LaunchControlRole::RehearsalCurrency,
        M5IncidentSeverityKind::SevTwoIncident,
        M5RegressionAssetSurfaceContext::ProgramGovernanceSurface,
        "transition-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn evidence_go_no_go_signoff_clean() -> M5ResolvedIncidentCloseEntry {
    evidence(clean_evidence_base(
        "evidence:release-center:go-no-go-signoff",
        "incident.lane.certified-archetype",
        "incident.close.monitoring_regression_check",
        M5LaunchControlRole::GoNoGoAuthority,
        M5IncidentSeverityKind::LaunchBearingFailure,
        M5RegressionAssetSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded incident-close entries ----------------------------------------------------

/// Degraded evidence entry: the evidence would run partner / public support language ahead of cohort proof — a
/// support-language reference present but not bound to cohort proof reads as trustworthy when the cohort proof
/// does not yet back it.
fn evidence_support_ahead() -> M5ResolvedIncidentCloseEntry {
    let mut base = clean_evidence_base(
        "evidence:shiproom:support-ahead",
        "incident.lane.public-preview",
        "incident.close.schema_policy_guard",
        M5LaunchControlRole::ReadinessEvent,
        M5IncidentSeverityKind::SevOneIncident,
        M5RegressionAssetSurfaceContext::ShiproomSurface,
        "transition-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.close_without_asset_requested = true;
    base.close_blocked_until_asset_linked = false;
    evidence(base)
}

/// Degraded evidence entry: the canonical / accessible / audit resolution-form coverage of the evidence is
/// incomplete.
fn evidence_form_incomplete() -> M5ResolvedIncidentCloseEntry {
    let mut base = clean_evidence_base(
        "evidence:release-center:form-incomplete",
        "incident.lane.certified-archetype",
        "incident.close.monitoring_regression_check",
        M5LaunchControlRole::GoNoGoAuthority,
        M5IncidentSeverityKind::LaunchBearingFailure,
        M5RegressionAssetSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5RegressionAssetResolutionForm::CanonicalObject];
    evidence(base)
}

/// Degraded evidence entry: the evidence scope is unclassified.
fn incident_severity_unclassified() -> M5ResolvedIncidentCloseEntry {
    evidence(clean_evidence_base(
        "evidence:executive-steering:scope-unclassified",
        "incident.lane.design-partner-preview",
        "incident.close.protected_corpus_case",
        M5LaunchControlRole::CohortMembership,
        M5IncidentSeverityKind::SeverityUnclassified,
        M5RegressionAssetSurfaceContext::ExecutiveSteeringSurface,
        "transition-id.design-partner-preview-0007",
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
    consumer_surface: M5RegressionAssetIncidentCloseRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    regression_asset_entries: Vec<M5ResolvedRegressionAssetEntry>,
    incident_close_entries: Vec<M5ResolvedIncidentCloseEntry>,
) -> M5RegressionAssetIncidentCloseRegistriesRow {
    M5RegressionAssetIncidentCloseRegistriesRow {
        consumer_surface,
        qualification: M5LaunchControlQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5LaunchControlWideningStage::ALL.to_vec(),
        required_labels: M5LaunchControlRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5LaunchControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RegressionAssetAnatomyPart::ALL.to_vec(),
        export_fields: M5RegressionAssetExportField::ALL.to_vec(),
        downgrade_triggers,
        regression_asset_entries,
        incident_close_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_REF,
            M5_REGRESSION_ASSET_DOMAIN_SCHEMA_REF,
            M5_INCIDENT_CLOSE_DOMAIN_SCHEMA_REF,
        ]),
        closes_a_severe_incident_without_a_linked_regression_asset: false,
        lets_an_approved_exception_become_an_untracked_close: false,
        hides_the_build_row_or_cohort_lineage_on_the_regression_asset: false,
        collapses_distinct_incident_severity_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5RegressionAssetIncidentCloseRegistriesRow> {
    use M5LaunchControlConsumerSurface as C;
    use M5LaunchControlDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the canary regression asset type's regression-asset rule to one typed object — regression asset type, minimum entry evidence, soak-window expectation, widening-allow rationale, known-limits packet, issue-template ref, claim-narrowing action, and incident-close reference — from the shared registry and proves the crash / data-loss / trust incident-close record for the canary ring; a progression object missing its soak-window expectation and a incident-close record that advances the ring while a stop condition is active degrade honestly instead of reading as a clean pass",
            "evidence:m5-launch-control-shiproom:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![
                descriptor_automated_test_clean(),
                descriptor_object_incomplete(),
            ],
            vec![evidence_dogfood_ring_clean(), evidence_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the broad-internal-dogfood regression asset type's regression-asset rule and the stale-readiness-packet incident-close record while keeping the incident-close visible; a ring advancing without a visible incident-close reference and known-limits posture and a resolution-form gap on a incident-close record are caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-launch-control-release-center:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::ImpliedGreenWhileGoNoGoOrOrrWasStale,
                D::ProofStale,
            ],
            vec![descriptor_fixture_repository_clean(), descriptor_widen_fold()],
            vec![evidence_go_no_go_signoff_clean(), evidence_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the design-partner-preview regression asset type's regression-asset rule while keeping its partner support language matched to ring proof and reports the certified-stable incident-close record; a progression rule that is a hand-copied per-entry assumption and a incident-close record on an unclassified incident-close condition degrade honestly",
            "evidence:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ReadinessStateUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_protected_corpus_case_clean(),
                descriptor_unbound(),
            ],
            vec![incident_severity_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the extension-author regression asset type's regression-asset rule and the repeated-protected-metric-regression incident-close record bound to the registry; an unstated registry token on a progression rule is caught before it can drift",
            "evidence:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CohortMembershipUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_recovery_drill_clean(),
                regression_asset_token_unstated(),
            ],
            vec![evidence_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved regression-asset and incident-close truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied ring table; the certified-stable progression rule and the stale-readiness-packet incident-close record stay inspectable off-renderer",
            "evidence:m5-launch-control-diagnostics:001",
            vec![
                D::CohortMembershipUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![descriptor_monitoring_regression_check_clean()],
            vec![evidence_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved regression-asset and incident-close truth, so a hand-copied constant, an unstated registry token, a widen-without-stop attempt, or support language running ahead of proof is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![descriptor_schema_policy_guard_clean()],
            vec![evidence_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5RegressionAssetIncidentCloseRegistriesGovernanceReview {
    M5RegressionAssetIncidentCloseRegistriesGovernanceReview {
        regression_asset_registry_names_token_role_and_type: true,
        type_resolves_to_typed_regression_asset_from_shared_registry: true,
        build_row_and_cohort_lineage_published: true,
        severe_incidents_cannot_close_without_regression_asset_and_lineage: true,
        incident_close_keeps_lineage_visible_and_blocks_assetless_close: true,
        approved_exception_matched_to_asset_proof_for_severe_incidents: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        asset_or_close_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RegressionAssetIncidentCloseRegistriesConsumerProjection {
    M5RegressionAssetIncidentCloseRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5RegressionAssetIncidentCloseRegistriesProofFreshness {
    M5RegressionAssetIncidentCloseRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RegressionAssetIncidentCloseRegistriesReleasePosture {
    M5RegressionAssetIncidentCloseRegistriesReleasePosture {
        proof_packet_ref: M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_ARTIFACT_REF.to_owned(),
        incident_control_audit_ref: M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_SCHEMA_REF,
        M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_REGRESSION_ASSET_DOMAIN_SCHEMA_REF,
        M5_INCIDENT_CLOSE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 regression-asset and incident-close registries packet.
pub fn seeded_m5_regression_asset_and_incident_close_registries(
) -> M5RegressionAssetIncidentCloseRegistriesPacket {
    M5RegressionAssetIncidentCloseRegistriesPacket::new(
        M5RegressionAssetIncidentCloseRegistriesPacketInput {
            packet_id: M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 regression-asset and incident-close registries with one typed regression-asset object resolving per asset type (automated test, fixture repository, recovery drill, protected-corpus case, schema/policy guard, monitoring regression check), severe incidents never closing without a linked regression asset and preserved build/row/cohort lineage, an approved exception never becoming an untracked close, canonical / accessible / audit resolution-form coverage, and the complete resolved-incident-identity / linked-regression-asset-ledger / exact-build-and-row / cohort-ring-lineage / close-lineage-freshness / workaround-lineage / last-incident-close-revision incident-close record across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5RegressionAssetIncidentCloseRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending regression-asset parity on every archetype; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_regression_asset_and_incident_close_registries_regression_asset_beta_narrowed(
) -> M5RegressionAssetIncidentCloseRegistriesPacket {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.packet_id =
        "m5-regression-asset-and-incident-close-registries:regression-asset-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5LaunchControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending incident-close parity on every
/// archetype; every row stays visible and every example stays honest.
pub fn seeded_m5_regression_asset_and_incident_close_registries_incident_close_preview_narrowed(
) -> M5RegressionAssetIncidentCloseRegistriesPacket {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.packet_id =
        "m5-regression-asset-and-incident-close-registries:incident-close-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5LaunchControlQualificationClass::Preview;
    packet
}
