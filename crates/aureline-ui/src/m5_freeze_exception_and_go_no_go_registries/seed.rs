//! Canonical seed builders for the M5 freeze-exception and go-no-go registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean freeze-exception and go-no-go entries are built
//! so the one typed freeze-exception object resolving per regression asset type, rings never advancing without a
//! visible known-limits and go-no-go posture, partner / public support language never running ahead of ring
//! proof, the canonical / accessible / audit resolution forms, and the complete transition-identity /
//! active-stop-condition-ledger / go-no-go-target / protected-metric-regression / packet-freshness /
//! crash-data-loss-or-trust / last-ring-transition-revision go-no-go record are proven across the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-ring assumption, widen-without-stop, incomplete object, hidden go-no-go, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_PACKET_ID: &str =
    "m5-freeze-exception-and-go-no-go-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn descriptor(input: M5FreezeExceptionEntryResolutionInput) -> M5ResolvedFreezeExceptionEntry {
    resolve_freeze_exception_entry(input).expect("seed freeze-exception entry resolves")
}

fn evidence(input: M5GoNoGoEntryResolutionInput) -> M5ResolvedGoNoGoEntry {
    resolve_go_no_go_entry(input).expect("seed go-no-go entry resolves")
}

fn all_forms() -> Vec<M5FreezeExceptionResolutionForm> {
    M5FreezeExceptionResolutionForm::ALL.to_vec()
}

// -- Clean freeze-exception entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_descriptor_base(
    entry_id: &str,
    exception_binding_id: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    freeze_exception_change_class: M5FreezeExceptionChangeClass,
    surface_context: M5FreezeExceptionSurfaceContext,
    exception_scope_reference: &str,
    rollback_or_narrowing_reference: &str,
    docs_support_migration_reference: &str,
    owner_capture_reference: &str,
    risk_capture_reference: &str,
    change_budget_reference: &str,
    expiry_reference: &str,
) -> M5FreezeExceptionEntryResolutionInput {
    M5FreezeExceptionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        exception_binding_id: exception_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        freeze_exception_change_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        exception_scope_reference: exception_scope_reference.to_owned(),
        rollback_or_narrowing_reference: rollback_or_narrowing_reference.to_owned(),
        docs_support_migration_reference: docs_support_migration_reference.to_owned(),
        owner_capture_reference: owner_capture_reference.to_owned(),
        risk_capture_reference: risk_capture_reference.to_owned(),
        change_budget_reference: change_budget_reference.to_owned(),
        expiry_reference: expiry_reference.to_owned(),
        bound_to_registry: true,
        freeze_exception_documented_before_widening: true,
        requires_documented_exception: false,
        attributable_asset_or_approved_exception: true,
        proof_fresh: true,
    }
}

fn descriptor_phase_allowed_change_clean() -> M5ResolvedFreezeExceptionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:shiproom:dogfood-core-team-canary",
        "incident.lane.core-team-canary",
        "freeze.exception.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5FreezeExceptionChangeClass::PhaseAllowedChange,
        M5FreezeExceptionSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn descriptor_exception_required_change_clean() -> M5ResolvedFreezeExceptionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:migration-alpha",
        "incident.lane.migration-alpha",
        "freeze.exception.exception_required_change",
        M5LaunchControlRole::ReadinessEvent,
        M5FreezeExceptionChangeClass::ExceptionRequiredChange,
        M5FreezeExceptionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn descriptor_api_or_contract_change_clean() -> M5ResolvedFreezeExceptionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:program-governance:extension-author",
        "incident.lane.extension-author",
        "freeze.exception.api_or_contract_change",
        M5LaunchControlRole::RehearsalCurrency,
        M5FreezeExceptionChangeClass::ApiOrContractChange,
        M5FreezeExceptionSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn descriptor_scope_widening_change_clean() -> M5ResolvedFreezeExceptionEntry {
    // A design-partner preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:design-partner-preview",
        "incident.lane.design-partner-preview",
        "freeze.exception.scope_widening_change",
        M5LaunchControlRole::CohortMembership,
        M5FreezeExceptionChangeClass::ScopeWideningChange,
        M5FreezeExceptionSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-archetypes",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.requires_documented_exception = true;
    base.attributable_asset_or_approved_exception = true;
    descriptor(base)
}

fn descriptor_migration_or_data_change_clean() -> M5ResolvedFreezeExceptionEntry {
    // A public preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:support:public-preview",
        "incident.lane.public-preview",
        "freeze.exception.migration_or_data_change",
        M5LaunchControlRole::ReadinessEvent,
        M5FreezeExceptionChangeClass::MigrationOrDataChange,
        M5FreezeExceptionSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-archetypes",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.requires_documented_exception = true;
    base.attributable_asset_or_approved_exception = true;
    descriptor(base)
}

fn descriptor_dependency_or_toolchain_change_clean() -> M5ResolvedFreezeExceptionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:certified-archetype",
        "incident.lane.certified-archetype",
        "freeze.exception.dependency_or_toolchain_change",
        M5LaunchControlRole::GoNoGoAuthority,
        M5FreezeExceptionChangeClass::DependencyOrToolchainChange,
        M5FreezeExceptionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-archetype-archetypes",
        "bundle.ids.certified-0007",
        "install.topology.certified-archetype-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-archetype",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded freeze-exception entries ---------------------------------------------------------

/// Degraded descriptor entry: the resolved descriptor object is incomplete — the bundle IDs are unstated.
fn descriptor_object_incomplete() -> M5ResolvedFreezeExceptionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:shiproom:incomplete",
        "incident.lane.core-team-canary",
        "freeze.exception.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5FreezeExceptionChangeClass::PhaseAllowedChange,
        M5FreezeExceptionSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.rollback_or_narrowing_reference = "   ".to_owned();
    descriptor(base)
}

/// Degraded descriptor entry: the cohort's rollback and diagnostics posture is not preserved before widening —
/// a cohort widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn descriptor_widen_fold() -> M5ResolvedFreezeExceptionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:release-center:widen-fold",
        "incident.lane.migration-alpha",
        "freeze.exception.exception_required_change",
        M5LaunchControlRole::ReadinessEvent,
        M5FreezeExceptionChangeClass::ExceptionRequiredChange,
        M5FreezeExceptionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.freeze_exception_documented_before_widening = false;
    descriptor(base)
}

/// Degraded descriptor entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn descriptor_unbound() -> M5ResolvedFreezeExceptionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:unbound",
        "incident.lane.certified-archetype",
        "freeze.exception.dependency_or_toolchain_change",
        M5LaunchControlRole::GoNoGoAuthority,
        M5FreezeExceptionChangeClass::DependencyOrToolchainChange,
        M5FreezeExceptionSurfaceContext::ExecutiveSteeringSurface,
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
fn freeze_exception_token_unstated() -> M5ResolvedFreezeExceptionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:program-governance:token-unstated",
        "incident.lane.extension-author",
        "  ",
        M5LaunchControlRole::RehearsalCurrency,
        M5FreezeExceptionChangeClass::ApiOrContractChange,
        M5FreezeExceptionSurfaceContext::ProgramGovernanceSurface,
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

// -- Clean go-no-go entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_evidence_base(
    entry_id: &str,
    go_no_go_ref: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    go_no_go_decision: M5GoNoGoDecisionKind,
    surface_context: M5FreezeExceptionSurfaceContext,
    resolved_decision_identity: &str,
    evidence_snapshot_ledger: &str,
    orr_signoff_reference: &str,
    on_call_roster_state: &str,
    go_no_go_freshness_state: &str,
    widening_stage_reference: &str,
    last_go_no_go_revision: &str,
) -> M5GoNoGoEntryResolutionInput {
    M5GoNoGoEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        go_no_go_ref: go_no_go_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        go_no_go_decision,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_decision_identity: resolved_decision_identity.to_owned(),
        evidence_snapshot_ledger: evidence_snapshot_ledger.to_owned(),
        orr_signoff_reference: orr_signoff_reference.to_owned(),
        on_call_roster_state: on_call_roster_state.to_owned(),
        go_no_go_freshness_state: go_no_go_freshness_state.to_owned(),
        widening_stage_reference: widening_stage_reference.to_owned(),
        last_go_no_go_revision: last_go_no_go_revision.to_owned(),
        keeps_evidence_snapshot_visible: true,
        go_no_go_lineage_is_truthful: true,
        override_without_evidence_requested: false,
        blocked_until_evidence_linked: false,
        lineage_gap_present: false,
        lineage_gap_flagged: false,
        proof_fresh: true,
    }
}

fn evidence_dogfood_ring_clean() -> M5ResolvedGoNoGoEntry {
    // A dogfood-ring evidence packet carries partner / public support language bound to cohort proof.
    let mut base = clean_evidence_base(
        "evidence:shiproom:dogfood-ring",
        "incident.lane.core-team-canary",
        "go.no.go.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5GoNoGoDecisionKind::GoDecision,
        M5FreezeExceptionSurfaceContext::ShiproomSurface,
        "transition-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.override_without_evidence_requested = true;
    base.blocked_until_evidence_linked = true;
    evidence(base)
}

fn evidence_rehearsal_currency_clean() -> M5ResolvedGoNoGoEntry {
    evidence(clean_evidence_base(
        "evidence:program-governance:rehearsal-currency",
        "incident.lane.extension-author",
        "go.no.go.api_or_contract_change",
        M5LaunchControlRole::RehearsalCurrency,
        M5GoNoGoDecisionKind::NoGoDecision,
        M5FreezeExceptionSurfaceContext::ProgramGovernanceSurface,
        "transition-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn evidence_go_no_go_signoff_clean() -> M5ResolvedGoNoGoEntry {
    evidence(clean_evidence_base(
        "evidence:release-center:go-no-go-signoff",
        "incident.lane.certified-archetype",
        "go.no.go.dependency_or_toolchain_change",
        M5LaunchControlRole::GoNoGoAuthority,
        M5GoNoGoDecisionKind::ConditionalGoDecision,
        M5FreezeExceptionSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded go-no-go entries ----------------------------------------------------

/// Degraded evidence entry: the evidence would run partner / public support language ahead of cohort proof — a
/// support-language reference present but not bound to cohort proof reads as trustworthy when the cohort proof
/// does not yet back it.
fn evidence_support_ahead() -> M5ResolvedGoNoGoEntry {
    let mut base = clean_evidence_base(
        "evidence:shiproom:support-ahead",
        "incident.lane.public-preview",
        "go.no.go.migration_or_data_change",
        M5LaunchControlRole::ReadinessEvent,
        M5GoNoGoDecisionKind::GoDecision,
        M5FreezeExceptionSurfaceContext::ShiproomSurface,
        "transition-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.override_without_evidence_requested = true;
    base.blocked_until_evidence_linked = false;
    evidence(base)
}

/// Degraded evidence entry: the canonical / accessible / audit resolution-form coverage of the evidence is
/// incomplete.
fn evidence_form_incomplete() -> M5ResolvedGoNoGoEntry {
    let mut base = clean_evidence_base(
        "evidence:release-center:form-incomplete",
        "incident.lane.certified-archetype",
        "go.no.go.dependency_or_toolchain_change",
        M5LaunchControlRole::GoNoGoAuthority,
        M5GoNoGoDecisionKind::ConditionalGoDecision,
        M5FreezeExceptionSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5FreezeExceptionResolutionForm::CanonicalObject];
    evidence(base)
}

/// Degraded evidence entry: the evidence scope is unclassified.
fn go_no_go_decision_unclassified() -> M5ResolvedGoNoGoEntry {
    evidence(clean_evidence_base(
        "evidence:executive-steering:scope-unclassified",
        "incident.lane.design-partner-preview",
        "go.no.go.scope_widening_change",
        M5LaunchControlRole::CohortMembership,
        M5GoNoGoDecisionKind::DecisionUnclassified,
        M5FreezeExceptionSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5FreezeExceptionGoNoGoRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    freeze_exception_entries: Vec<M5ResolvedFreezeExceptionEntry>,
    go_no_go_entries: Vec<M5ResolvedGoNoGoEntry>,
) -> M5FreezeExceptionGoNoGoRegistriesRow {
    M5FreezeExceptionGoNoGoRegistriesRow {
        consumer_surface,
        qualification: M5LaunchControlQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5LaunchControlWideningStage::ALL.to_vec(),
        required_labels: M5LaunchControlRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5LaunchControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5FreezeExceptionAnatomyPart::ALL.to_vec(),
        export_fields: M5FreezeExceptionExportField::ALL.to_vec(),
        downgrade_triggers,
        freeze_exception_entries,
        go_no_go_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_REF,
            M5_FREEZE_EXCEPTION_DOMAIN_SCHEMA_REF,
            M5_GO_NO_GO_DOMAIN_SCHEMA_REF,
        ]),
        widens_scope_without_a_documented_freeze_exception: false,
        lets_a_freeze_exception_become_undocumented_scope_widening: false,
        hides_the_change_budget_or_owner_risk_on_the_freeze_exception: false,
        collapses_distinct_go_no_go_decision_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5FreezeExceptionGoNoGoRegistriesRow> {
    use M5LaunchControlConsumerSurface as C;
    use M5LaunchControlDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the canary regression asset type's freeze-exception rule to one typed object — regression asset type, minimum entry evidence, soak-window expectation, widening-allow rationale, known-limits packet, issue-template ref, claim-narrowing action, and go-no-go reference — from the shared registry and proves the crash / data-loss / trust go-no-go record for the canary ring; a progression object missing its soak-window expectation and a go-no-go record that advances the ring while a stop condition is active degrade honestly instead of reading as a clean pass",
            "evidence:m5-launch-control-shiproom:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![
                descriptor_phase_allowed_change_clean(),
                descriptor_object_incomplete(),
            ],
            vec![evidence_dogfood_ring_clean(), evidence_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the broad-internal-dogfood regression asset type's freeze-exception rule and the stale-readiness-packet go-no-go record while keeping the go-no-go visible; a ring advancing without a visible go-no-go reference and known-limits posture and a resolution-form gap on a go-no-go record are caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-launch-control-release-center:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::ImpliedGreenWhileGoNoGoOrOrrWasStale,
                D::ProofStale,
            ],
            vec![descriptor_exception_required_change_clean(), descriptor_widen_fold()],
            vec![evidence_go_no_go_signoff_clean(), evidence_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the design-partner-preview regression asset type's freeze-exception rule while keeping its partner support language matched to ring proof and reports the certified-stable go-no-go record; a progression rule that is a hand-copied per-entry assumption and a go-no-go record on an unclassified go-no-go condition degrade honestly",
            "evidence:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ReadinessStateUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_scope_widening_change_clean(),
                descriptor_unbound(),
            ],
            vec![go_no_go_decision_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the extension-author regression asset type's freeze-exception rule and the repeated-protected-metric-regression go-no-go record bound to the registry; an unstated registry token on a progression rule is caught before it can drift",
            "evidence:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CohortMembershipUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_api_or_contract_change_clean(),
                freeze_exception_token_unstated(),
            ],
            vec![evidence_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved freeze-exception and go-no-go truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied ring table; the certified-stable progression rule and the stale-readiness-packet go-no-go record stay inspectable off-renderer",
            "evidence:m5-launch-control-diagnostics:001",
            vec![
                D::CohortMembershipUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![descriptor_dependency_or_toolchain_change_clean()],
            vec![evidence_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved freeze-exception and go-no-go truth, so a hand-copied constant, an unstated registry token, a widen-without-stop attempt, or support language running ahead of proof is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![descriptor_migration_or_data_change_clean()],
            vec![evidence_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5FreezeExceptionGoNoGoRegistriesGovernanceReview {
    M5FreezeExceptionGoNoGoRegistriesGovernanceReview {
        freeze_exception_registry_names_token_role_and_type: true,
        type_resolves_to_typed_freeze_exception_from_shared_registry: true,
        build_row_and_cohort_lineage_published: true,
        scope_cannot_widen_without_documented_freeze_exception: true,
        go_no_go_keeps_evidence_visible_and_blocks_stale_green: true,
        approved_exception_matched_to_scope_for_widening: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        exception_or_go_no_go_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5FreezeExceptionGoNoGoRegistriesConsumerProjection {
    M5FreezeExceptionGoNoGoRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5FreezeExceptionGoNoGoRegistriesProofFreshness {
    M5FreezeExceptionGoNoGoRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5FreezeExceptionGoNoGoRegistriesReleasePosture {
    M5FreezeExceptionGoNoGoRegistriesReleasePosture {
        proof_packet_ref: M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_ARTIFACT_REF.to_owned(),
        go_no_go_control_audit_ref: M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_SCHEMA_REF,
        M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_FREEZE_EXCEPTION_DOMAIN_SCHEMA_REF,
        M5_GO_NO_GO_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 freeze-exception and go-no-go registries packet.
pub fn seeded_m5_freeze_exception_and_go_no_go_registries(
) -> M5FreezeExceptionGoNoGoRegistriesPacket {
    M5FreezeExceptionGoNoGoRegistriesPacket::new(
        M5FreezeExceptionGoNoGoRegistriesPacketInput {
            packet_id: M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 freeze-exception and go-no-go registries with one typed freeze-exception object resolving per asset type (automated test, fixture repository, recovery drill, protected-corpus case, schema/policy guard, monitoring regression check), severe incidents never closing without a linked regression asset and preserved build/row/cohort lineage, an approved exception never becoming an untracked close, canonical / accessible / audit resolution-form coverage, and the complete resolved-incident-identity / linked-freeze-exception-ledger / exact-build-and-row / cohort-ring-lineage / close-lineage-freshness / workaround-lineage / last-go-no-go-revision go-no-go record across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5FreezeExceptionGoNoGoRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending freeze-exception parity on every archetype; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_freeze_exception_and_go_no_go_registries_freeze_exception_beta_narrowed(
) -> M5FreezeExceptionGoNoGoRegistriesPacket {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.packet_id =
        "m5-freeze-exception-and-go-no-go-registries:freeze-exception-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5LaunchControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending go-no-go parity on every
/// archetype; every row stays visible and every example stays honest.
pub fn seeded_m5_freeze_exception_and_go_no_go_registries_go_no_go_preview_narrowed(
) -> M5FreezeExceptionGoNoGoRegistriesPacket {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.packet_id =
        "m5-freeze-exception-and-go-no-go-registries:go-no-go-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5LaunchControlQualificationClass::Preview;
    packet
}
