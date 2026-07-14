//! Canonical seed builders for the M5 acquisition-evidence and partial-recovery registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean acquisition-evidence and partial-recovery entries are
//! built so the one stable acquisition-evidence packet resolving per acquisition path, the evidence staying
//! visible with no partial content presented as a healthy full checkout, partial-not-full status disclosed before
//! any partial-describing packet, the canonical / accessible / audit resolution forms, and the complete
//! recovery-action-kind / recovery-site / state-consequence / lineage-consequence / explicit-action-requirement /
//! attribution partial-recovery object are proven across the acquisition-engine, git, trust, diagnostics, CLI, and
//! support surfaces without any hand-copied per-entry assumption, state-discarding recovery action, overclaimed
//! full checkout, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_PACKET_ID: &str =
    "m5-acquisition-evidence-and-partial-recovery-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn evidence(
    input: M5AcquisitionEvidenceEntryResolutionInput,
) -> M5ResolvedAcquisitionEvidenceEntry {
    resolve_acquisition_evidence_entry(input).expect("seed acquisition-evidence entry resolves")
}

fn recovery(input: M5PartialRecoveryEntryResolutionInput) -> M5ResolvedPartialRecoveryEntry {
    resolve_partial_recovery_entry(input).expect("seed partial-recovery entry resolves")
}

fn all_forms() -> Vec<M5RecoveryResolutionForm> {
    M5RecoveryResolutionForm::ALL.to_vec()
}

// -- Clean acquisition-evidence entries (stable packet, visible, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_evidence_base(
    entry_id: &str,
    acquisition_path_id: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    evidence_kind: M5AcquisitionEvidenceKind,
    surface_context: M5RecoverySurfaceContext,
    transcript_ref: &str,
    warnings_and_retries_ref: &str,
    resulting_root_identity_ref: &str,
    omitted_or_unfetched_ref: &str,
    bootstrap_checkpoint_ref: &str,
    evidence_provenance: &str,
) -> M5AcquisitionEvidenceEntryResolutionInput {
    M5AcquisitionEvidenceEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        acquisition_path_id: acquisition_path_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        evidence_kind,
        surface_context,
        resolution_form_coverage: all_forms(),
        transcript_ref: transcript_ref.to_owned(),
        warnings_and_retries_ref: warnings_and_retries_ref.to_owned(),
        resulting_root_identity_ref: resulting_root_identity_ref.to_owned(),
        omitted_or_unfetched_ref: omitted_or_unfetched_ref.to_owned(),
        bootstrap_checkpoint_ref: bootstrap_checkpoint_ref.to_owned(),
        evidence_provenance: evidence_provenance.to_owned(),
        bound_to_registry: true,
        partial_state_visible: true,
        describes_partial_state: false,
        partial_not_full_disclosed: true,
        proof_fresh: true,
    }
}

fn evidence_acq_transcript_clean() -> M5ResolvedAcquisitionEvidenceEntry {
    evidence(clean_evidence_base(
        "evidence:acquisition:clone-fetch-transcript",
        "entry.acme.open-local",
        "acquisition.evidence.clone_fetch_transcript",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::CloneFetchTranscript,
        M5RecoverySurfaceContext::ShellSurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/none-observed",
        "root-identity.acme/full-head",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    ))
}

fn evidence_git_warnings_clean() -> M5ResolvedAcquisitionEvidenceEntry {
    evidence(clean_evidence_base(
        "evidence:git:warnings-and-retries",
        "entry.acme.clone-remote",
        "acquisition.evidence.warnings_and_retries",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::WarningsAndRetries,
        M5RecoverySurfaceContext::EntrySurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/two-retries-recorded",
        "root-identity.acme/full-head",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    ))
}

fn evidence_diagnostics_root_clean() -> M5ResolvedAcquisitionEvidenceEntry {
    evidence(clean_evidence_base(
        "evidence:diagnostics:resulting-root-identity",
        "entry.acme.open-archive",
        "acquisition.evidence.resulting_root_identity",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::ResultingRootIdentity,
        M5RecoverySurfaceContext::DiagnosticsSurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/none-observed",
        "root-identity.acme/archive-root",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    ))
}

fn evidence_admin_omitted_clean() -> M5ResolvedAcquisitionEvidenceEntry {
    // An omitted-or-unfetched-state packet describes partial state and discloses partial-not-full status.
    let mut base = clean_evidence_base(
        "evidence:admin:omitted-or-unfetched-state",
        "entry.acme.import-bundle",
        "acquisition.evidence.omitted_or_unfetched_state",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::OmittedOrUnfetchedState,
        M5RecoverySurfaceContext::AdminSurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/interrupted-fetch",
        "root-identity.acme/partial-head",
        "omitted.acme/lfs-and-submodule-unfetched",
        "checkpoint.acme/partial-at-tree",
        "evidence-provenance.acme.v3",
    );
    base.describes_partial_state = true;
    base.partial_not_full_disclosed = true;
    evidence(base)
}

fn evidence_support_checkpoint_clean() -> M5ResolvedAcquisitionEvidenceEntry {
    // A bootstrap-checkpoint packet describes partial state and discloses partial-not-full status.
    let mut base = clean_evidence_base(
        "evidence:support:bootstrap-checkpoint",
        "entry.acme.resume-snapshot",
        "acquisition.evidence.bootstrap_checkpoint",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::BootstrapCheckpoint,
        M5RecoverySurfaceContext::SupportOrExportForm,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/interrupted-fetch",
        "root-identity.acme/partial-head",
        "omitted.acme/pack-window-unfetched",
        "checkpoint.acme/partial-at-pack",
        "evidence-provenance.acme.v3",
    );
    base.describes_partial_state = true;
    base.partial_not_full_disclosed = true;
    evidence(base)
}

// -- Degraded acquisition-evidence entries ------------------------------------------------------

/// Degraded evidence entry: the resolved acquisition-evidence packet is incomplete — the clone / fetch transcript
/// reference is unstated.
fn evidence_packet_incomplete() -> M5ResolvedAcquisitionEvidenceEntry {
    let mut base = clean_evidence_base(
        "evidence:acquisition:incomplete",
        "entry.acme.open-local",
        "acquisition.evidence.clone_fetch_transcript",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::CloneFetchTranscript,
        M5RecoverySurfaceContext::ShellSurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/none-observed",
        "root-identity.acme/full-head",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    );
    base.transcript_ref = "   ".to_owned();
    evidence(base)
}

/// Degraded evidence entry: a partial-describing packet would present partial content as a healthy full checkout
/// before partial-not-full status is disclosed.
fn evidence_overclaim() -> M5ResolvedAcquisitionEvidenceEntry {
    let mut base = clean_evidence_base(
        "evidence:trust:overclaim",
        "entry.acme.import-bundle",
        "acquisition.evidence.omitted_or_unfetched_state",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::OmittedOrUnfetchedState,
        M5RecoverySurfaceContext::DiagnosticsSurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/interrupted-fetch",
        "root-identity.acme/partial-head",
        "omitted.acme/lfs-and-submodule-unfetched",
        "checkpoint.acme/partial-at-tree",
        "evidence-provenance.acme.v3",
    );
    base.describes_partial_state = true;
    base.partial_not_full_disclosed = false;
    evidence(base)
}

/// Degraded evidence entry: the behavior is a hand-copied per-entry assumption instead of tracing to the registry.
fn evidence_unbound() -> M5ResolvedAcquisitionEvidenceEntry {
    let mut base = clean_evidence_base(
        "evidence:diagnostics:unbound",
        "entry.acme.open-archive",
        "acquisition.evidence.resulting_root_identity",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::ResultingRootIdentity,
        M5RecoverySurfaceContext::AdminSurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/none-observed",
        "root-identity.acme/archive-root",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    );
    base.bound_to_registry = false;
    evidence(base)
}

/// Degraded evidence entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn evidence_form_incomplete() -> M5ResolvedAcquisitionEvidenceEntry {
    let mut base = clean_evidence_base(
        "evidence:git:form-incomplete",
        "entry.acme.clone-remote",
        "acquisition.evidence.warnings_and_retries",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::WarningsAndRetries,
        M5RecoverySurfaceContext::EntrySurface,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/two-retries-recorded",
        "root-identity.acme/full-head",
        "omitted.acme/none-omitted",
        "checkpoint.acme/complete",
        "evidence-provenance.acme.v3",
    );
    base.resolution_form_coverage = vec![M5RecoveryResolutionForm::CanonicalObject];
    evidence(base)
}

/// Degraded evidence entry: the canonical registry token name is unstated.
fn evidence_token_unstated() -> M5ResolvedAcquisitionEvidenceEntry {
    let mut base = clean_evidence_base(
        "evidence:support:token-unstated",
        "entry.acme.resume-snapshot",
        "  ",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5AcquisitionEvidenceKind::BootstrapCheckpoint,
        M5RecoverySurfaceContext::SupportOrExportForm,
        "transcript.acme/clone-fetch-log",
        "warnings.acme/interrupted-fetch",
        "root-identity.acme/partial-head",
        "omitted.acme/pack-window-unfetched",
        "checkpoint.acme/partial-at-pack",
        "evidence-provenance.acme.v3",
    );
    base.token_name = "  ".to_owned();
    base.describes_partial_state = true;
    base.partial_not_full_disclosed = true;
    evidence(base)
}

// -- Clean partial-recovery entries -------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_recovery_base(
    entry_id: &str,
    source_ref: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    recovery_class: M5PartialRecoveryClass,
    surface_context: M5RecoverySurfaceContext,
    recovery_action_kind: &str,
    recovery_site: &str,
    state_consequence: &str,
    lineage_consequence: &str,
    explicit_action_requirement: &str,
    attribution_ref: &str,
) -> M5PartialRecoveryEntryResolutionInput {
    M5PartialRecoveryEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        source_ref: source_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        recovery_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        recovery_action_kind: recovery_action_kind.to_owned(),
        recovery_site: recovery_site.to_owned(),
        state_consequence: state_consequence.to_owned(),
        lineage_consequence: lineage_consequence.to_owned(),
        explicit_action_requirement: explicit_action_requirement.to_owned(),
        attribution_ref: attribution_ref.to_owned(),
        identifies_recovery_site_and_state_effect: true,
        action_is_truthfully_typed: true,
        is_state_mutating_action: false,
        explicit_discard_or_cleanup_action_present: false,
        schedules_deferred_cleanup: false,
        cleanup_is_disclosed: false,
        discards_state_without_explicit_action: false,
        proof_fresh: true,
    }
}

fn recovery_resume_shell_clean() -> M5ResolvedPartialRecoveryEntry {
    // A resume-acquisition action mutates state and requires an explicit discard or cleanup action.
    let mut base = clean_recovery_base(
        "recovery:acquisition:resume-acquisition",
        "entry.acme.clone-remote",
        "partial.recovery.resume_acquisition",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::ResumeAcquisition,
        M5RecoverySurfaceContext::ShellSurface,
        "recovery-action.resume-from-checkpoint",
        "site.worktree",
        "consequence.continues-partial-state",
        "consequence.preserves-transcript-lineage",
        "action.explicit-resume-required",
        "attribution.acquisition-engine",
    );
    base.is_state_mutating_action = true;
    base.explicit_discard_or_cleanup_action_present = true;
    recovery(base)
}

fn recovery_discard_entry_clean() -> M5ResolvedPartialRecoveryEntry {
    // A discard-partial-state action mutates state, is gated, and schedules a disclosed cleanup follow-up.
    let mut base = clean_recovery_base(
        "recovery:git:discard-partial-state",
        "entry.acme.clone-remote",
        "partial.recovery.discard_partial_state",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::DiscardPartialState,
        M5RecoverySurfaceContext::EntrySurface,
        "recovery-action.discard-and-clean",
        "site.git-dir",
        "consequence.removes-partial-state",
        "consequence.archives-transcript-lineage",
        "action.explicit-discard-required",
        "attribution.git-service",
    );
    base.is_state_mutating_action = true;
    base.explicit_discard_or_cleanup_action_present = true;
    base.schedules_deferred_cleanup = true;
    base.cleanup_is_disclosed = true;
    recovery(base)
}

fn recovery_openro_diag_clean() -> M5ResolvedPartialRecoveryEntry {
    // An open-read-only-partial-root action is read-only; it mutates nothing and needs no gate.
    recovery(clean_recovery_base(
        "recovery:trust:open-read-only-partial-root",
        "entry.acme.open-archive",
        "partial.recovery.open_read_only_partial_root",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::OpenReadOnlyPartialRoot,
        M5RecoverySurfaceContext::DiagnosticsSurface,
        "recovery-action.open-partial-root-read-only",
        "site.presentation-only",
        "consequence.no-state-change",
        "consequence.preserves-transcript-lineage",
        "action.none-read-only",
        "attribution.trust-service",
    ))
}

fn recovery_inert_admin_clean() -> M5ResolvedPartialRecoveryEntry {
    // An inert status report presents only; it mutates nothing and needs no gate.
    recovery(clean_recovery_base(
        "recovery:diagnostics:inert-status-report",
        "entry.acme.import-bundle",
        "partial.recovery.inert_status_report",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::InertStatusReport,
        M5RecoverySurfaceContext::AdminSurface,
        "recovery-action.report-partial-status",
        "site.presentation-only",
        "consequence.no-state-change",
        "consequence.preserves-transcript-lineage",
        "action.none-inert",
        "attribution.diagnostics",
    ))
}

fn recovery_resume_support_clean() -> M5ResolvedPartialRecoveryEntry {
    let mut base = clean_recovery_base(
        "recovery:support:resume-acquisition",
        "entry.acme.clone-remote",
        "partial.recovery.resume_acquisition",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::ResumeAcquisition,
        M5RecoverySurfaceContext::SupportOrExportForm,
        "recovery-action.resume-from-checkpoint",
        "site.network",
        "consequence.continues-partial-state",
        "consequence.preserves-transcript-lineage",
        "action.explicit-resume-required",
        "attribution.support-export",
    );
    base.is_state_mutating_action = true;
    base.explicit_discard_or_cleanup_action_present = true;
    recovery(base)
}

// -- Degraded partial-recovery entries ----------------------------------------------------------

/// Degraded recovery entry: a state-mutating action would discard partial state during acquisition — the partial
/// state or transcript lineage is discarded merely because an acquisition was interrupted, so the action reads as
/// unsafe.
fn recovery_discard_without_action() -> M5ResolvedPartialRecoveryEntry {
    let mut base = clean_recovery_base(
        "recovery:acquisition:discard-without-action",
        "entry.acme.clone-remote",
        "partial.recovery.discard_partial_state",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::DiscardPartialState,
        M5RecoverySurfaceContext::ShellSurface,
        "recovery-action.discard-and-clean",
        "site.git-dir",
        "consequence.removes-partial-state",
        "consequence.archives-transcript-lineage",
        "action.explicit-discard-required",
        "attribution.acquisition-engine",
    );
    base.is_state_mutating_action = true;
    base.explicit_discard_or_cleanup_action_present = true;
    base.discards_state_without_explicit_action = true;
    recovery(base)
}

/// Degraded recovery entry: the canonical / accessible / audit resolution-form coverage of the recovery action is
/// incomplete.
fn recovery_form_incomplete() -> M5ResolvedPartialRecoveryEntry {
    let mut base = clean_recovery_base(
        "recovery:git:form-incomplete",
        "entry.acme.clone-remote",
        "partial.recovery.discard_partial_state",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::DiscardPartialState,
        M5RecoverySurfaceContext::EntrySurface,
        "recovery-action.discard-and-clean",
        "site.git-dir",
        "consequence.removes-partial-state",
        "consequence.archives-transcript-lineage",
        "action.explicit-discard-required",
        "attribution.git-service",
    );
    base.is_state_mutating_action = true;
    base.explicit_discard_or_cleanup_action_present = true;
    base.resolution_form_coverage = vec![M5RecoveryResolutionForm::CanonicalObject];
    recovery(base)
}

/// Degraded recovery entry: the partial-recovery class is unclassified.
fn recovery_class_unclassified() -> M5ResolvedPartialRecoveryEntry {
    recovery(clean_recovery_base(
        "recovery:diagnostics:class-unclassified",
        "entry.acme.import-bundle",
        "partial.recovery.unknown",
        M5RepositoryBootstrapRole::ResumableAcquisition,
        M5PartialRecoveryClass::RecoveryUnclassified,
        M5RecoverySurfaceContext::AdminSurface,
        "recovery-action.report-partial-status",
        "site.presentation-only",
        "consequence.no-state-change",
        "consequence.preserves-transcript-lineage",
        "action.none-inert",
        "attribution.diagnostics",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5AcquisitionEvidencePartialRecoveryRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    acquisition_evidence_entries: Vec<M5ResolvedAcquisitionEvidenceEntry>,
    partial_recovery_entries: Vec<M5ResolvedPartialRecoveryEntry>,
) -> M5AcquisitionEvidencePartialRecoveryRegistriesRow {
    M5AcquisitionEvidencePartialRecoveryRegistriesRow {
        consumer_surface,
        qualification: M5RepositoryBootstrapQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5RepositoryBootstrapDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5RepositoryBootstrapRequiredLabel::Identity,
            M5RepositoryBootstrapRequiredLabel::SemanticRole,
            M5RepositoryBootstrapRequiredLabel::RegistryReference,
            M5RepositoryBootstrapRequiredLabel::CredentialPosture,
            M5RepositoryBootstrapRequiredLabel::CheckoutPlan,
        ],
        accessibility_routes: M5RepositoryBootstrapAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RecoveryAnatomyPart::ALL.to_vec(),
        export_fields: M5RecoveryExportField::ALL.to_vec(),
        downgrade_triggers,
        acquisition_evidence_entries,
        partial_recovery_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_REF,
            M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
            M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
        ]),
        presents_partial_acquisition_as_healthy_full_checkout: false,
        discards_partial_state_or_lineage_without_explicit_action: false,
        hides_what_a_recovery_action_would_do_or_its_state_or_lineage_effect: false,
        leaves_partial_or_interrupted_state_invisible_or_unrecoverable: false,
    }
}

fn registry_rows() -> Vec<M5AcquisitionEvidencePartialRecoveryRegistriesRow> {
    use M5RepositoryBootstrapConsumerSurface as C;
    use M5RepositoryBootstrapDowngradeTrigger as D;

    vec![
        base_row(
            C::AcquisitionEngine,
            "Acquisition-engine owner",
            "The acquisition engine resolves the clone-fetch-transcript evidence kind to one stable packet — transcript reference, warnings and retries, resulting root identity, omitted-or-unfetched state, bootstrap checkpoint, and evidence provenance — from the shared registry and derives the resume-acquisition partial-recovery action gated behind an explicit resume; an evidence packet missing its transcript reference and a discard action that would remove partial state merely because an acquisition was interrupted degrade honestly instead of reading as a clean pass",
            "evidence:m5-repository-bootstrap-acquisition-engine:001",
            vec![
                D::StagedTrustRuleUnstated,
                D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
                D::ProofStale,
            ],
            vec![evidence_acq_transcript_clean(), evidence_packet_incomplete()],
            vec![recovery_resume_shell_clean(), recovery_discard_without_action()],
        ),
        base_row(
            C::GitService,
            "Git-service owner",
            "The git service resolves the warnings-and-retries evidence kind while keeping the partial state visible, and renders the discard-partial-state partial-recovery action gated behind an explicit discard with a disclosed cleanup; a resolution-form gap on an evidence packet and on a recovery action is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-repository-bootstrap-git-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![evidence_git_warnings_clean(), evidence_form_incomplete()],
            vec![recovery_discard_entry_clean(), recovery_form_incomplete()],
        ),
        base_row(
            C::TrustService,
            "Trust-service owner",
            "The trust service reports the resulting-root-identity evidence kind and the open-read-only-partial-root partial-recovery action without manual reconstruction; a partial-describing evidence packet that would present partial content as a healthy full checkout before partial-not-full status is disclosed is caught as an overclaim",
            "evidence:m5-repository-bootstrap-trust-service:001",
            vec![
                D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![evidence_diagnostics_root_clean(), evidence_overclaim()],
            vec![recovery_openro_diag_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics resolves the omitted-or-unfetched-state evidence kind while keeping it visible and bound to the registry, and renders the inert-status-report partial-recovery action; an evidence packet that is a hand-copied per-entry assumption and a recovery action on an unclassified class degrade honestly",
            "evidence:m5-repository-bootstrap-diagnostics:001",
            vec![
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![evidence_admin_omitted_clean(), evidence_unbound()],
            vec![recovery_inert_admin_clean(), recovery_class_unclassified()],
        ),
        base_row(
            C::CliExport,
            "CLI-export owner",
            "The CLI export renders the same resolved acquisition-evidence and partial-recovery truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied recovery table",
            "evidence:m5-repository-bootstrap-cli-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![evidence_diagnostics_root_clean(), evidence_form_incomplete()],
            vec![recovery_discard_entry_clean(), recovery_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved acquisition-evidence and partial-recovery truth without embedding raw secrets, so a hand-copied constant, an unstated registry token, a partial content presented as a healthy full checkout, or a partial state left invisible is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-repository-bootstrap-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
                D::ProofStale,
            ],
            vec![evidence_support_checkpoint_clean(), evidence_token_unstated()],
            vec![recovery_resume_support_clean()],
        ),
    ]
}

fn governance_review() -> M5AcquisitionEvidencePartialRecoveryRegistriesGovernanceReview {
    M5AcquisitionEvidencePartialRecoveryRegistriesGovernanceReview {
        evidence_registry_names_token_role_and_kind: true,
        entry_flow_resolves_to_stable_evidence_from_shared_registry: true,
        transcript_warnings_root_omitted_checkpoint_and_provenance_published: true,
        acquisition_evidence_stays_visible_no_full_checkout_overclaim: true,
        partial_recovery_identifies_action_and_consequence: true,
        state_mutating_recovery_requires_explicit_discard_or_cleanup: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        acquisition_git_trust_diagnostics_read_single_source: true,
        evidence_or_recovery_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AcquisitionEvidencePartialRecoveryRegistriesConsumerProjection {
    M5AcquisitionEvidencePartialRecoveryRegistriesConsumerProjection {
        acquisition_and_git_consume_shared_registries: true,
        trust_and_diagnostics_consume_shared_registries: true,
        cli_and_support_export_consume_shared_registries: true,
        docs_help_and_workspace_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5AcquisitionEvidencePartialRecoveryRegistriesProofFreshness {
    M5AcquisitionEvidencePartialRecoveryRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AcquisitionEvidencePartialRecoveryRegistriesReleasePosture {
    M5AcquisitionEvidencePartialRecoveryRegistriesReleasePosture {
        proof_packet_ref: M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        repository_bootstrap_audit_ref:
            M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_SCHEMA_REF,
        M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 acquisition-evidence and partial-recovery registries packet.
pub fn seeded_m5_acquisition_evidence_and_partial_recovery_registries(
) -> M5AcquisitionEvidencePartialRecoveryRegistriesPacket {
    M5AcquisitionEvidencePartialRecoveryRegistriesPacket::new(
        M5AcquisitionEvidencePartialRecoveryRegistriesPacketInput {
            packet_id: M5_ACQUISITION_EVIDENCE_PARTIAL_RECOVERY_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 acquisition-evidence and partial-recovery registries with one stable acquisition-evidence packet resolving per acquisition path, the evidence staying visible with no partial content presented as a healthy full checkout and partial-not-full status disclosed before any partial-describing packet, canonical / accessible / audit resolution-form coverage, and the complete recovery-action-kind / recovery-site / state-consequence / lineage-consequence / explicit-action-requirement / attribution partial-recovery object across acquisition-engine, git, trust, diagnostics, CLI, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5AcquisitionEvidencePartialRecoveryRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the trust-service row is held at Beta pending resume-partial follow-up parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_acquisition_evidence_and_partial_recovery_registries_resume_partial_beta_narrowed(
) -> M5AcquisitionEvidencePartialRecoveryRegistriesPacket {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.packet_id =
        "m5-acquisition-evidence-and-partial-recovery-registries:resume-partial-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::TrustService)
        .expect("trust-service row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending discard-cleanup scheduling parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_acquisition_evidence_and_partial_recovery_registries_discard_cleanup_preview_narrowed(
) -> M5AcquisitionEvidencePartialRecoveryRegistriesPacket {
    let mut packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
    packet.packet_id =
        "m5-acquisition-evidence-and-partial-recovery-registries:discard-cleanup-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Preview;
    packet
}
