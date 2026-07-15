//! Canonical seed builders for the M5 ring-progression and rollback-stop registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean ring-progression and rollback-stop entries are built
//! so the one typed ring-progression object resolving per widening transition, rings never advancing without a
//! visible known-limits and rollback-stop posture, partner / public support language never running ahead of ring
//! proof, the canonical / accessible / audit resolution forms, and the complete transition-identity /
//! active-stop-condition-ledger / rollback-stop-target / protected-metric-regression / packet-freshness /
//! crash-data-loss-or-trust / last-ring-transition-revision rollback-stop record are proven across the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-ring assumption, widen-without-stop, incomplete object, hidden rollback-stop, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_PACKET_ID: &str =
    "m5-ring-progression-and-rollback-stop-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn descriptor(input: M5RingProgressionEntryResolutionInput) -> M5ResolvedRingProgressionEntry {
    resolve_ring_progression_entry(input).expect("seed cohort-descriptor entry resolves")
}

fn evidence(input: M5RollbackStopEntryResolutionInput) -> M5ResolvedRollbackStopEntry {
    resolve_rollback_stop_entry(input).expect("seed cohort-evidence-packet entry resolves")
}

fn all_forms() -> Vec<M5RingResolutionForm> {
    M5RingResolutionForm::ALL.to_vec()
}

// -- Clean cohort-descriptor entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_descriptor_base(
    entry_id: &str,
    transition_binding_id: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    ring_widening_transition: M5RingWideningTransitionKind,
    surface_context: M5RingSurfaceContext,
    entry_evidence_minimum: &str,
    soak_window_expectation: &str,
    widening_allow_rationale: &str,
    issue_template_ref: &str,
    known_limits: &str,
    claim_narrowing_action: &str,
    rollback_stop_reference: &str,
) -> M5RingProgressionEntryResolutionInput {
    M5RingProgressionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        transition_binding_id: transition_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        ring_widening_transition,
        surface_context,
        resolution_form_coverage: all_forms(),
        entry_evidence_minimum: entry_evidence_minimum.to_owned(),
        soak_window_expectation: soak_window_expectation.to_owned(),
        widening_allow_rationale: widening_allow_rationale.to_owned(),
        issue_template_ref: issue_template_ref.to_owned(),
        known_limits: known_limits.to_owned(),
        claim_narrowing_action: claim_narrowing_action.to_owned(),
        rollback_stop_reference: rollback_stop_reference.to_owned(),
        bound_to_registry: true,
        stop_and_rollback_visible_before_widening: true,
        is_public_facing_ring: false,
        support_language_matches_ring_proof: true,
        proof_fresh: true,
    }
}

fn descriptor_canary_widening_clean() -> M5ResolvedRingProgressionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:shiproom:dogfood-core-team-canary",
        "launch.ring.core-team-canary",
        "ring.progression.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5RingWideningTransitionKind::CanaryWidening,
        M5RingSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn descriptor_broad_internal_dogfood_widening_clean() -> M5ResolvedRingProgressionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:migration-alpha",
        "launch.ring.migration-alpha",
        "ring.progression.broad_internal_dogfood_widening",
        M5LaunchControlRole::ReadinessEvent,
        M5RingWideningTransitionKind::BroadInternalDogfoodWidening,
        M5RingSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn descriptor_extension_author_widening_clean() -> M5ResolvedRingProgressionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:program-governance:extension-author",
        "launch.ring.extension-author",
        "ring.progression.extension_author_widening",
        M5LaunchControlRole::RehearsalCurrency,
        M5RingWideningTransitionKind::ExtensionAuthorWidening,
        M5RingSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn descriptor_design_partner_preview_widening_clean() -> M5ResolvedRingProgressionEntry {
    // A design-partner preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:design-partner-preview",
        "launch.ring.design-partner-preview",
        "ring.progression.design_partner_preview_widening",
        M5LaunchControlRole::CohortMembership,
        M5RingWideningTransitionKind::DesignPartnerPreviewWidening,
        M5RingSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-archetypes",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.is_public_facing_ring = true;
    base.support_language_matches_ring_proof = true;
    descriptor(base)
}

fn descriptor_public_preview_widening_clean() -> M5ResolvedRingProgressionEntry {
    // A public preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:support:public-preview",
        "launch.ring.public-preview",
        "ring.progression.public_preview_widening",
        M5LaunchControlRole::ReadinessEvent,
        M5RingWideningTransitionKind::PublicPreviewWidening,
        M5RingSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-archetypes",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.is_public_facing_ring = true;
    base.support_language_matches_ring_proof = true;
    descriptor(base)
}

fn descriptor_certified_stable_widening_clean() -> M5ResolvedRingProgressionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:certified-archetype",
        "launch.ring.certified-archetype",
        "ring.progression.certified_stable_widening",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RingWideningTransitionKind::CertifiedStableWidening,
        M5RingSurfaceContext::ReleaseCenterSurface,
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
fn descriptor_object_incomplete() -> M5ResolvedRingProgressionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:shiproom:incomplete",
        "launch.ring.core-team-canary",
        "ring.progression.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5RingWideningTransitionKind::CanaryWidening,
        M5RingSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.soak_window_expectation = "   ".to_owned();
    descriptor(base)
}

/// Degraded descriptor entry: the cohort's rollback and diagnostics posture is not preserved before widening —
/// a cohort widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn descriptor_widen_fold() -> M5ResolvedRingProgressionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:release-center:widen-fold",
        "launch.ring.migration-alpha",
        "ring.progression.broad_internal_dogfood_widening",
        M5LaunchControlRole::ReadinessEvent,
        M5RingWideningTransitionKind::BroadInternalDogfoodWidening,
        M5RingSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.stop_and_rollback_visible_before_widening = false;
    descriptor(base)
}

/// Degraded descriptor entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn descriptor_unbound() -> M5ResolvedRingProgressionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:unbound",
        "launch.ring.certified-archetype",
        "ring.progression.certified_stable_widening",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RingWideningTransitionKind::CertifiedStableWidening,
        M5RingSurfaceContext::ExecutiveSteeringSurface,
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
fn ring_token_unstated() -> M5ResolvedRingProgressionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:program-governance:token-unstated",
        "launch.ring.extension-author",
        "  ",
        M5LaunchControlRole::RehearsalCurrency,
        M5RingWideningTransitionKind::ExtensionAuthorWidening,
        M5RingSurfaceContext::ProgramGovernanceSurface,
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
    stop_condition_ref: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    rollback_stop_condition: M5RollbackStopConditionKind,
    surface_context: M5RingSurfaceContext,
    resolved_transition_identity: &str,
    active_stop_condition_ledger: &str,
    rollback_stop_target_reference: &str,
    protected_metric_regression_state: &str,
    packet_freshness_state: &str,
    crash_data_loss_or_trust_reference: &str,
    last_ring_transition_revision: &str,
) -> M5RollbackStopEntryResolutionInput {
    M5RollbackStopEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        stop_condition_ref: stop_condition_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        rollback_stop_condition,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_transition_identity: resolved_transition_identity.to_owned(),
        active_stop_condition_ledger: active_stop_condition_ledger.to_owned(),
        rollback_stop_target_reference: rollback_stop_target_reference.to_owned(),
        protected_metric_regression_state: protected_metric_regression_state.to_owned(),
        packet_freshness_state: packet_freshness_state.to_owned(),
        crash_data_loss_or_trust_reference: crash_data_loss_or_trust_reference.to_owned(),
        last_ring_transition_revision: last_ring_transition_revision.to_owned(),
        keeps_rollback_stop_visible: true,
        stop_state_is_truthful: true,
        stop_condition_active: false,
        ring_progression_halted_when_stop_active: false,
        protected_metric_regression_present: false,
        protected_metric_regression_flagged: false,
        proof_fresh: true,
    }
}

fn evidence_dogfood_ring_clean() -> M5ResolvedRollbackStopEntry {
    // A dogfood-ring evidence packet carries partner / public support language bound to cohort proof.
    let mut base = clean_evidence_base(
        "evidence:shiproom:dogfood-ring",
        "launch.ring.core-team-canary",
        "rollback.stop.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5RollbackStopConditionKind::CrashDataLossOrTrustDefect,
        M5RingSurfaceContext::ShiproomSurface,
        "transition-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.stop_condition_active = true;
    base.ring_progression_halted_when_stop_active = true;
    evidence(base)
}

fn evidence_rehearsal_currency_clean() -> M5ResolvedRollbackStopEntry {
    evidence(clean_evidence_base(
        "evidence:program-governance:rehearsal-currency",
        "launch.ring.extension-author",
        "rollback.stop.extension_author_widening",
        M5LaunchControlRole::RehearsalCurrency,
        M5RollbackStopConditionKind::RepeatedProtectedMetricRegression,
        M5RingSurfaceContext::ProgramGovernanceSurface,
        "transition-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn evidence_go_no_go_signoff_clean() -> M5ResolvedRollbackStopEntry {
    evidence(clean_evidence_base(
        "evidence:release-center:go-no-go-signoff",
        "launch.ring.certified-archetype",
        "rollback.stop.certified_stable_widening",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RollbackStopConditionKind::StaleReadinessPacket,
        M5RingSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
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
fn evidence_support_ahead() -> M5ResolvedRollbackStopEntry {
    let mut base = clean_evidence_base(
        "evidence:shiproom:support-ahead",
        "launch.ring.public-preview",
        "rollback.stop.public_preview_widening",
        M5LaunchControlRole::ReadinessEvent,
        M5RollbackStopConditionKind::CrashDataLossOrTrustDefect,
        M5RingSurfaceContext::ShiproomSurface,
        "transition-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.stop_condition_active = true;
    base.ring_progression_halted_when_stop_active = false;
    evidence(base)
}

/// Degraded evidence entry: the canonical / accessible / audit resolution-form coverage of the evidence is
/// incomplete.
fn evidence_form_incomplete() -> M5ResolvedRollbackStopEntry {
    let mut base = clean_evidence_base(
        "evidence:release-center:form-incomplete",
        "launch.ring.certified-archetype",
        "rollback.stop.certified_stable_widening",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RollbackStopConditionKind::StaleReadinessPacket,
        M5RingSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5RingResolutionForm::CanonicalObject];
    evidence(base)
}

/// Degraded evidence entry: the evidence scope is unclassified.
fn rollback_stop_condition_unclassified() -> M5ResolvedRollbackStopEntry {
    evidence(clean_evidence_base(
        "evidence:executive-steering:scope-unclassified",
        "launch.ring.design-partner-preview",
        "rollback.stop.design_partner_preview_widening",
        M5LaunchControlRole::CohortMembership,
        M5RollbackStopConditionKind::ConditionUnclassified,
        M5RingSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5RingProgressionRollbackStopRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    ring_progression_entries: Vec<M5ResolvedRingProgressionEntry>,
    rollback_stop_entries: Vec<M5ResolvedRollbackStopEntry>,
) -> M5RingProgressionRollbackStopRegistriesRow {
    M5RingProgressionRollbackStopRegistriesRow {
        consumer_surface,
        qualification: M5LaunchControlQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5LaunchControlWideningStage::ALL.to_vec(),
        required_labels: M5LaunchControlRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5LaunchControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RingAnatomyPart::ALL.to_vec(),
        export_fields: M5RingExportField::ALL.to_vec(),
        downgrade_triggers,
        ring_progression_entries,
        rollback_stop_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_REF,
            M5_RING_PROGRESSION_DOMAIN_SCHEMA_REF,
            M5_ROLLBACK_STOP_DOMAIN_SCHEMA_REF,
        ]),
        advances_a_ring_without_current_known_limits_and_rollback_stop_evidence: false,
        runs_partner_or_public_support_language_ahead_of_ring_proof: false,
        hides_the_known_limits_or_rollback_stop_posture_before_widening: false,
        collapses_distinct_rollback_stop_condition_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5RingProgressionRollbackStopRegistriesRow> {
    use M5LaunchControlConsumerSurface as C;
    use M5LaunchControlDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the canary widening transition's ring-progression rule to one typed object — widening transition, minimum entry evidence, soak-window expectation, widening-allow rationale, known-limits packet, issue-template ref, claim-narrowing action, and rollback-stop reference — from the shared registry and proves the crash / data-loss / trust rollback-stop record for the canary ring; a progression object missing its soak-window expectation and a rollback-stop record that advances the ring while a stop condition is active degrade honestly instead of reading as a clean pass",
            "evidence:m5-launch-control-shiproom:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![
                descriptor_canary_widening_clean(),
                descriptor_object_incomplete(),
            ],
            vec![evidence_dogfood_ring_clean(), evidence_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the broad-internal-dogfood widening transition's ring-progression rule and the stale-readiness-packet rollback-stop record while keeping the rollback-stop visible; a ring advancing without a visible rollback-stop reference and known-limits posture and a resolution-form gap on a rollback-stop record are caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-launch-control-release-center:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::ImpliedGreenWhileGoNoGoOrOrrWasStale,
                D::ProofStale,
            ],
            vec![descriptor_broad_internal_dogfood_widening_clean(), descriptor_widen_fold()],
            vec![evidence_go_no_go_signoff_clean(), evidence_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the design-partner-preview widening transition's ring-progression rule while keeping its partner support language matched to ring proof and reports the certified-stable rollback-stop record; a progression rule that is a hand-copied per-entry assumption and a rollback-stop record on an unclassified rollback-stop condition degrade honestly",
            "evidence:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ReadinessStateUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_design_partner_preview_widening_clean(),
                descriptor_unbound(),
            ],
            vec![rollback_stop_condition_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the extension-author widening transition's ring-progression rule and the repeated-protected-metric-regression rollback-stop record bound to the registry; an unstated registry token on a progression rule is caught before it can drift",
            "evidence:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CohortMembershipUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_extension_author_widening_clean(),
                ring_token_unstated(),
            ],
            vec![evidence_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved ring-progression and rollback-stop truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied ring table; the certified-stable progression rule and the stale-readiness-packet rollback-stop record stay inspectable off-renderer",
            "evidence:m5-launch-control-diagnostics:001",
            vec![
                D::CohortMembershipUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![descriptor_certified_stable_widening_clean()],
            vec![evidence_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved ring-progression and rollback-stop truth, so a hand-copied constant, an unstated registry token, a widen-without-stop attempt, or support language running ahead of proof is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![descriptor_public_preview_widening_clean()],
            vec![evidence_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5RingProgressionRollbackStopRegistriesGovernanceReview {
    M5RingProgressionRollbackStopRegistriesGovernanceReview {
        ring_progression_registry_names_token_role_and_transition: true,
        transition_resolves_to_typed_ring_progression_from_shared_registry: true,
        ring_evidence_and_soak_rows_published: true,
        rings_cannot_advance_without_rollback_stop_and_known_limits: true,
        rollback_stop_keeps_condition_visible_and_halts_active_ring: true,
        support_language_matched_to_ring_proof_for_public_rings: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        ring_or_stop_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RingProgressionRollbackStopRegistriesConsumerProjection {
    M5RingProgressionRollbackStopRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5RingProgressionRollbackStopRegistriesProofFreshness {
    M5RingProgressionRollbackStopRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RingProgressionRollbackStopRegistriesReleasePosture {
    M5RingProgressionRollbackStopRegistriesReleasePosture {
        proof_packet_ref: M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_ARTIFACT_REF.to_owned(),
        ring_control_audit_ref: M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_SCHEMA_REF,
        M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_RING_PROGRESSION_DOMAIN_SCHEMA_REF,
        M5_ROLLBACK_STOP_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 cohort-descriptor and cohort-evidence-packet registries packet.
pub fn seeded_m5_ring_progression_and_rollback_stop_registries(
) -> M5RingProgressionRollbackStopRegistriesPacket {
    M5RingProgressionRollbackStopRegistriesPacket::new(
        M5RingProgressionRollbackStopRegistriesPacketInput {
            packet_id: M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 ring-progression and rollback-stop registries with one typed ring-progression object resolving per widening transition, rings never advancing without a visible known-limits and rollback-stop posture, partner / public support language never running ahead of ring proof, canonical / accessible / audit resolution-form coverage, and the complete transition-identity / active-stop-condition-ledger / rollback-stop-target / protected-metric-regression / packet-freshness / crash-data-loss-or-trust / last-ring-transition-revision rollback-stop record across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5RingProgressionRollbackStopRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_ring_progression_and_rollback_stop_registries_ring_progression_beta_narrowed(
) -> M5RingProgressionRollbackStopRegistriesPacket {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.packet_id =
        "m5-ring-progression-and-rollback-stop-registries:ring-progression-beta:0001".to_owned();
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
pub fn seeded_m5_ring_progression_and_rollback_stop_registries_rollback_stop_preview_narrowed(
) -> M5RingProgressionRollbackStopRegistriesPacket {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.packet_id =
        "m5-ring-progression-and-rollback-stop-registries:rollback-stop-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5LaunchControlQualificationClass::Preview;
    packet
}
