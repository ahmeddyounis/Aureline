use super::*;

const PACKET_ID: &str = "m5-public-surface-diff-merge-control-controls:stable:0001";
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn component_source_refs() -> Vec<String> {
    strings(&[
        M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF,
    ])
}

fn card_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::PublicSurfaceDiffUnavailable,
        M5GovernanceComponentDowngradeTrigger::MigrationEvidenceMissing,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn banner_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn card_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::ReleaseCandidate,
        M5GovernanceComponentConsumerSurface::CliHeadless,
        M5GovernanceComponentConsumerSurface::SupportExport,
    ]
}

fn banner_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::Shiproom,
        M5GovernanceComponentConsumerSurface::GovernanceDashboard,
        M5GovernanceComponentConsumerSurface::SupportExport,
    ]
}

fn note_if(needed: bool, text: &str) -> String {
    if needed {
        text.to_owned()
    } else {
        String::new()
    }
}

/// Collects the frozen governance token the shared confirmation-locus resolver derives, so the vocab
/// never borrows another state's label but always carries the token it must.
fn locus_vocab(locus: &ConfirmationLocusDisclosure) -> Vec<M5GovernanceStateVocab> {
    let mut vocab = Vec::new();
    if let Some(token) = locus.governance_vocab {
        vocab.push(token);
    }
    vocab
}

#[allow(clippy::too_many_arguments)]
fn diff_card(
    card_id: &str,
    change_title_label: &str,
    surface_classes: Vec<PublicSurfaceClass>,
    surface_class_label: &str,
    stability_class: StabilityClass,
    stability_label: &str,
    surface_change_source: SurfaceChangeSource,
    delta_disclosure_label: &str,
    confirmation_locus_source: ConfirmationLocusSource,
    migration_note: &str,
    migration_evidence_ref: &str,
    diff_evidence_kind: DiffEvidenceKind,
    diff_evidence_ref: &str,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    card_actions: Vec<PublicSurfaceDiffCardAction>,
) -> PublicSurfaceDiffCard {
    let locus = resolve_confirmation_locus(confirmation_locus_source);
    let change = resolve_surface_change(surface_change_source);
    PublicSurfaceDiffCard {
        component: M5GovernanceComponent::PublicSurfaceDiffCard,
        card_id: card_id.to_owned(),
        change_title_label: change_title_label.to_owned(),
        surface_classes,
        surface_class_label: surface_class_label.to_owned(),
        stability_class,
        stability_label: stability_label.to_owned(),
        surface_change_source,
        derived_surface_change: change.posture,
        claims_breaking: change.is_breaking,
        delta_disclosure_label: delta_disclosure_label.to_owned(),
        confirmation_locus_source,
        derived_confirmation_locus: locus.posture,
        claims_provider_confirmed: locus.is_provider_confirmed,
        governance_state_vocab: locus_vocab(&locus),
        local_estimate_note: note_if(
            locus.needs_local_estimate_note,
            "Diff is a local estimate; it is not provider-confirmed",
        ),
        machine_generated_note: note_if(
            locus.needs_machine_generated_note,
            "Diff was machine-generated locally; open the diff evidence to confirm",
        ),
        not_evaluated_note: note_if(
            locus.needs_not_evaluated_note,
            "This surface diff was not evaluated on this build; do not read it as evaluated",
        ),
        stale_note: note_if(
            locus.needs_stale_note,
            "Diff is stale relative to the current base/head; regenerate before trusting",
        ),
        breaking_note: note_if(
            change.needs_breaking_note,
            "This change breaks the public contract; a migration path is required",
        ),
        deprecation_note: note_if(
            change.needs_deprecation_note,
            "This change announces a deprecation; a removal timeline and migration are required",
        ),
        removal_note: note_if(
            change.needs_removal_note,
            "This change removes a public surface; a migration path is required",
        ),
        migration_note: migration_note.to_owned(),
        migration_evidence_ref: migration_evidence_ref.to_owned(),
        diff_evidence_kind,
        diff_evidence_ref: diff_evidence_ref.to_owned(),
        context_note: context_note.to_owned(),
        card_actions,
        downgrade_triggers: card_downgrade_triggers(),
        consumer_surfaces: card_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_surface_class_or_stability: false,
        lets_stable_breaking_change_hide_without_migration: false,
        lets_local_estimate_read_as_provider_confirmed: false,
        invents_alternate_state_label: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn merge_banner(
    banner_id: &str,
    gate_title_label: &str,
    blocker_class: MergeBlockerClass,
    blocker_reason_label: &str,
    required_checks: Vec<RequiredCheck>,
    protection_state: ProtectionState,
    protection_state_label: &str,
    bypass_policy: BypassPolicyClass,
    bypass_policy_label: &str,
    confirmation_locus_source: ConfirmationLocusSource,
    mergeability_label: &str,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    banner_actions: Vec<MergeControlBannerAction>,
) -> MergeControlBanner {
    let locus = resolve_confirmation_locus(confirmation_locus_source);
    MergeControlBanner {
        component: M5GovernanceComponent::MergeControlBanner,
        banner_id: banner_id.to_owned(),
        gate_title_label: gate_title_label.to_owned(),
        blocker_class,
        blocker_reason_label: blocker_reason_label.to_owned(),
        required_checks,
        required_checks_label: "Required checks and their blocking state are listed explicitly"
            .to_owned(),
        protection_state,
        protection_state_label: protection_state_label.to_owned(),
        bypass_policy,
        bypass_policy_label: bypass_policy_label.to_owned(),
        confirmation_locus_source,
        derived_confirmation_locus: locus.posture,
        claims_provider_confirmed: locus.is_provider_confirmed,
        claims_evaluated_here: locus.is_evaluated_here,
        mergeability_label: mergeability_label.to_owned(),
        export_parity_label: "Export packet mirrors the rendered merge-control state".to_owned(),
        governance_state_vocab: locus_vocab(&locus),
        local_estimate_note: note_if(
            locus.needs_local_estimate_note,
            "Mergeability is a local estimate; it is not provider-confirmed",
        ),
        machine_generated_note: note_if(
            locus.needs_machine_generated_note,
            "Gate state was machine-generated locally; confirm with the provider before merge",
        ),
        not_evaluated_note: note_if(
            locus.needs_not_evaluated_note,
            "This merge gate was not evaluated on this build; do not read it as evaluated",
        ),
        stale_note: note_if(
            locus.needs_stale_note,
            "Merge gate is stale relative to the current base/head; re-evaluate first",
        ),
        context_note: context_note.to_owned(),
        banner_actions,
        downgrade_triggers: banner_downgrade_triggers(),
        consumer_surfaces: banner_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_blocker_reason_or_bypass_policy: false,
        lets_local_estimate_read_as_provider_mergeable: false,
        names_generic_blocker_instead_of_current_gate: false,
        invents_alternate_state_label: false,
    }
}

fn required_check(
    check_label: &str,
    state: RequiredCheckState,
    is_blocking: bool,
) -> RequiredCheck {
    RequiredCheck {
        check_label: check_label.to_owned(),
        check_state: state,
        is_blocking,
    }
}

fn diff_cards() -> Vec<PublicSurfaceDiffCard> {
    use ConfirmationLocusSource as Locus;
    use DiffEvidenceKind as Evidence;
    use M5GovernanceComponentRollbackPosture as Rollback;
    use PublicSurfaceClass as Surface;
    use PublicSurfaceDiffCardAction as Action;
    use StabilityClass as Stability;
    use SurfaceChangeSource as Change;

    let full_actions = vec![
        Action::OpenDiffEvidence,
        Action::InspectSurfaceChange,
        Action::ReviewMigrationNote,
        Action::InspectStabilityLabel,
        Action::CompareBaseHead,
        Action::CopySurfaceDigest,
    ];

    vec![
        // 1. Stable command/flag removed / provider-reported → removal, provider-confirmed.
        diff_card(
            "psd-command-removed",
            "Remove the deprecated `aureline verify` command",
            vec![Surface::Command, Surface::CliFlag],
            "Command and CLI flag",
            Stability::Stable,
            "Stable, externally depended-on command surface",
            Change::SurfaceRemoved,
            "Command `aureline verify` and its `--strict` flag are removed",
            Locus::ProviderReportedState,
            "Migrate callers to `aureline review --verify`; the old command is gone",
            "evidence:migration/command-removed",
            Evidence::MachineGeneratedDiff,
            "evidence:diff/command-removed",
            "Stable command removed; open the machine-generated diff and migration path",
            Rollback::ReturnPathPreserved,
            full_actions.clone(),
        ),
        // 2. Stable schema/manifest incompatible change / machine-generated → breaking, machine-gen.
        diff_card(
            "psd-schema-breaking",
            "Tighten the run-config schema to reject unknown keys",
            vec![Surface::Schema, Surface::Manifest],
            "Schema and manifest",
            Stability::Stable,
            "Stable, externally depended-on schema surface",
            Change::IncompatibleSignatureChange,
            "Schema `run-config` now rejects previously accepted unknown keys",
            Locus::MachineGeneratedLocally,
            "Update manifests to drop unknown keys before upgrading",
            "evidence:migration/schema-breaking",
            Evidence::MigrationGuide,
            "evidence:diff/schema-breaking",
            "Stable schema breaking change; machine-generated diff and migration guide are linked",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 3. Beta SDK/WIT/token compatible addition / provider-confirmed → compatible, provider.
        diff_card(
            "psd-sdk-addition",
            "Add an optional `dry_run` field to the SDK plan surface",
            vec![Surface::SdkWitSurface, Surface::Token],
            "SDK/WIT surface and token",
            Stability::Beta,
            "Beta SDK/WIT surface",
            Change::BackwardCompatibleAddition,
            "SDK plan surface gains an optional `dry_run` field and a new token",
            Locus::ProviderConfirmedGate,
            "",
            "",
            Evidence::CompatibilityReport,
            "evidence:diff/sdk-addition",
            "Beta SDK addition; backward-compatible, provider-confirmed",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 4. Experimental message-id/automation deprecation / local estimate → deprecation, local.
        diff_card(
            "psd-message-deprecation",
            "Deprecate the legacy `job.enqueued` message id",
            vec![Surface::MessageId, Surface::AutomationContract],
            "Message id and automation contract",
            Stability::Experimental,
            "Experimental message-id surface",
            Change::DeprecationAnnounced,
            "Message id `job.enqueued` and its automation contract are deprecated",
            Locus::LocalHeuristicEstimate,
            "Move automations to `job.accepted`; `job.enqueued` will be removed later",
            "evidence:migration/message-deprecation",
            Evidence::ChangelogEntry,
            "evidence:diff/message-deprecation",
            "Experimental message-id deprecation; migration path is provided as a local estimate",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 5. Internal compatibility-claim clarifying change / stale → compatible, stale.
        diff_card(
            "psd-compat-claim",
            "Clarify the compatibility claim wording without behavior change",
            vec![Surface::CompatibilityClaim],
            "Compatibility claim",
            Stability::Internal,
            "Internal compatibility-claim surface",
            Change::ClarifyingCompatibleChange,
            "Compatibility claim wording is clarified; no behavior changes",
            Locus::StaleAgainstBaseHead,
            "",
            "",
            Evidence::NoDiffEvidence,
            "",
            "Internal compatibility-claim clarification; stale relative to the current base/head",
            Rollback::EvidencePreservedNoRevert,
            full_actions.clone(),
        ),
        // 6. Stable command behavior break / not evaluated here → breaking, not-evaluated.
        diff_card(
            "psd-command-behavior",
            "Change `aureline run` to fail fast on missing config",
            vec![Surface::Command],
            "Command",
            Stability::Stable,
            "Stable, externally depended-on command surface",
            Change::SemanticBehaviorBreak,
            "Command `aureline run` now fails fast instead of prompting on missing config",
            Locus::NotEvaluatedHere,
            "Ensure config exists before invoking `aureline run`; behavior changed",
            "evidence:migration/command-behavior",
            Evidence::MachineGeneratedDiff,
            "evidence:diff/command-behavior",
            "Stable command behavior break; diff and migration are linked though not evaluated here",
            Rollback::ReturnPathPreserved,
            vec![
                Action::OpenDiffEvidence,
                Action::InspectSurfaceChange,
                Action::ReviewMigrationNote,
                Action::InspectStabilityLabel,
            ],
        ),
    ]
}

fn merge_banners() -> Vec<MergeControlBanner> {
    use BypassPolicyClass as Bypass;
    use ConfirmationLocusSource as Locus;
    use M5GovernanceComponentRollbackPosture as Rollback;
    use MergeBlockerClass as Blocker;
    use MergeControlBannerAction as Action;
    use ProtectionState as Protection;
    use RequiredCheckState as Check;

    let full_actions = vec![
        Action::InspectMergeGate,
        Action::ReviewRequiredChecks,
        Action::ReviewBypassPolicy,
        Action::InspectProtectionState,
        Action::OpenBlockerEvidence,
        Action::CopyMergeControlSummary,
    ];

    vec![
        // 1. Required check failing / provider-enforced / no bypass → provider-confirmed.
        merge_banner(
            "mcb-check-failing",
            "Merge blocked: a required check is failing",
            Blocker::RequiredCheckFailing,
            "Required check `ci/build` is failing; fix the build before merge",
            vec![
                required_check("ci/build", Check::Failing, true),
                required_check("ci/lint", Check::Passing, false),
            ],
            Protection::ProviderEnforced,
            "Provider enforces branch protection on this base",
            Bypass::NoBypassAllowed,
            "No bypass is allowed for this gate",
            Locus::ProviderConfirmedGate,
            "Provider-confirmed: the provider reports this merge as blocked",
            "Provider-confirmed blocked merge; the failing check names the current gate",
            Rollback::ProviderMutationAttributable,
            full_actions.clone(),
        ),
        // 2. Required review missing / ruleset-enforced / admin bypass → provider-confirmed.
        merge_banner(
            "mcb-review-missing",
            "Merge blocked: a required review is missing",
            Blocker::RequiredReviewMissing,
            "A required review from the api-platform owners is missing",
            vec![
                required_check("required-review", Check::Missing, true),
                required_check("ci/build", Check::Passing, false),
            ],
            Protection::RulesetEnforced,
            "A ruleset enforces the required-review gate",
            Bypass::AdminBypassAllowed,
            "An admin bypass is allowed but would be recorded",
            Locus::ProviderReportedState,
            "Provider-confirmed: the provider reports the review as missing",
            "Provider-confirmed missing review; admin bypass is recorded if used",
            Rollback::ProviderMutationAttributable,
            full_actions.clone(),
        ),
        // 3. Branch protection / advisory-only / emergency bypass → local estimate.
        merge_banner(
            "mcb-branch-protection",
            "Merge estimate: a branch-protection rule may block",
            Blocker::BranchProtectionRule,
            "A branch-protection rule requires a linear history; the local branch diverges",
            vec![
                required_check("linear-history", Check::Pending, true),
                required_check("ci/build", Check::Passing, false),
            ],
            Protection::AdvisoryOnly,
            "Protection is advisory in this local estimate; the provider is authoritative",
            Bypass::EmergencyBypassAllowed,
            "An emergency bypass is allowed under the incident policy",
            Locus::LocalHeuristicEstimate,
            "Local estimate: the provider has not confirmed mergeability",
            "Local-estimate branch-protection blocker; confirm with the provider before merge",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 4. Ruleset violation / not configured / bypass used → stale.
        merge_banner(
            "mcb-ruleset-violation",
            "Merge blocked: a ruleset violation was recorded",
            Blocker::RulesetViolation,
            "A ruleset requires signed commits; an unsigned commit is present",
            vec![
                required_check("signed-commits", Check::Failing, true),
                required_check("ci/build", Check::Passing, false),
            ],
            Protection::NotConfigured,
            "No local protection is configured; the recorded state is stale",
            Bypass::BypassUsed,
            "A bypass was used on a prior attempt and is recorded",
            Locus::StaleAgainstBaseHead,
            "Stale: the merge gate was evaluated against an older base/head",
            "Stale ruleset-violation blocker; re-evaluate against the current base/head",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 5. Merge conflict / provider-enforced / no bypass → not evaluated here.
        merge_banner(
            "mcb-merge-conflict",
            "Merge gate not evaluated: a merge conflict may exist",
            Blocker::MergeConflict,
            "A merge conflict prevents a clean merge into the base",
            vec![
                required_check("mergeable", Check::Missing, true),
                required_check("ci/build", Check::Pending, false),
            ],
            Protection::ProviderEnforced,
            "Provider enforces branch protection on this base",
            Bypass::NoBypassAllowed,
            "No bypass is allowed for this gate",
            Locus::NotEvaluatedHere,
            "Not evaluated here: mergeability was not computed on this build",
            "Not-evaluated merge-conflict blocker; do not read it as an evaluated verdict",
            Rollback::EvidencePreservedNoRevert,
            full_actions.clone(),
        ),
        // 6. No blocker / provider-enforced / no bypass → provider-confirmed mergeable.
        merge_banner(
            "mcb-mergeable",
            "Merge allowed: the provider confirms the gate is clear",
            Blocker::NoBlocker,
            "No current blocker; all required checks pass and the review is satisfied",
            vec![
                required_check("ci/build", Check::Passing, true),
                required_check("required-review", Check::Passing, true),
            ],
            Protection::ProviderEnforced,
            "Provider enforces branch protection on this base",
            Bypass::NoBypassAllowed,
            "No bypass is needed; the gate is clear",
            Locus::ProviderConfirmedGate,
            "Provider-confirmed: the provider reports this merge as allowed",
            "Provider-confirmed mergeable gate; no current blocker",
            Rollback::ProviderMutationAttributable,
            full_actions,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::PublicSurfaceDiffUnavailable,
        M5GovernanceComponentDowngradeTrigger::MigrationEvidenceMissing,
        M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
        M5GovernanceComponentDowngradeTrigger::EscalationHandoffUnavailable,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    M5GovernanceComponentConsumerSurface::ALL.to_vec()
}

fn review() -> PublicSurfaceMergeControlReview {
    PublicSurfaceMergeControlReview {
        diff_card_shows_surface_class_and_stability: true,
        diff_card_shows_schema_or_command_delta: true,
        diff_card_offers_open_diff_evidence: true,
        merge_banner_shows_blocker_reason_and_bypass: true,
        merge_banner_shows_required_checks_and_protection: true,
        merge_banner_shows_export_parity: true,
        confirmation_locus_derived_never_asserted: true,
        local_or_machine_never_shown_as_provider_confirmed: true,
        not_evaluated_here_never_shown_as_evaluated: true,
        stable_breaking_change_never_hides_without_migration: true,
        merge_blocker_never_generic: true,
        migration_and_evidence_required_for_stable_change: true,
        stale_relative_to_base_head_always_explicit: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> PublicSurfaceMergeControlConsumerProjection {
    PublicSurfaceMergeControlConsumerProjection {
        review_workspace_reads_single_source: true,
        release_candidate_reads_single_source: true,
        governance_and_shiproom_read_single_source: true,
        surface_class_and_delta_visible_before_merge: true,
        blocker_and_bypass_visible_before_merge: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> PublicSurfaceMergeControlProofFreshness {
    PublicSurfaceMergeControlProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_SCHEMA_REF,
        PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF,
    ])
}

fn packet() -> PublicSurfaceMergeControlControlsPacket {
    PublicSurfaceMergeControlControlsPacket::new(PublicSurfaceMergeControlControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label:
            "M5 public-surface diff cards and merge-control banners: surface class, stability label, schema-or-command delta disclosure, blocker reason, bypass policy, and migration-note continuity across claimed release-bearing changes"
                .to_owned(),
        public_surface_diff_cards: diff_cards(),
        merge_control_banners: merge_banners(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        review: review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn confirmation_locus_is_derived_not_asserted() {
    use ConfirmationLocusPosture as Posture;
    use ConfirmationLocusSource as Src;
    assert_eq!(
        resolve_confirmation_locus(Src::ProviderConfirmedGate).posture,
        Posture::ProviderConfirmed
    );
    assert_eq!(
        resolve_confirmation_locus(Src::ProviderReportedState).posture,
        Posture::ProviderConfirmed
    );
    assert_eq!(
        resolve_confirmation_locus(Src::MachineGeneratedLocally).posture,
        Posture::MachineGeneratedLocal
    );
    assert_eq!(
        resolve_confirmation_locus(Src::LocalHeuristicEstimate).posture,
        Posture::LocalEstimate
    );
    assert_eq!(
        resolve_confirmation_locus(Src::NotEvaluatedHere).posture,
        Posture::NotEvaluatedHere
    );
    assert_eq!(
        resolve_confirmation_locus(Src::StaleAgainstBaseHead).posture,
        Posture::StaleRelativeToHead
    );
}

#[test]
fn only_provider_locus_is_provider_confirmed() {
    for source in ConfirmationLocusSource::ALL {
        let disclosure = resolve_confirmation_locus(source);
        let expected = matches!(
            source,
            ConfirmationLocusSource::ProviderConfirmedGate
                | ConfirmationLocusSource::ProviderReportedState
        );
        assert_eq!(
            disclosure.is_provider_confirmed, expected,
            "{source:?} provider-confirmed mismatch"
        );
    }
    // AC-2: a local estimate and a machine-generated diff are never provider-confirmed.
    assert!(
        !resolve_confirmation_locus(ConfirmationLocusSource::LocalHeuristicEstimate)
            .is_provider_confirmed
    );
    assert!(
        !resolve_confirmation_locus(ConfirmationLocusSource::MachineGeneratedLocally)
            .is_provider_confirmed
    );
    // A not-evaluated-here gate is never evaluated here.
    assert!(
        !resolve_confirmation_locus(ConfirmationLocusSource::NotEvaluatedHere).is_evaluated_here
    );
}

#[test]
fn surface_change_never_collapses_breaking_into_compatible() {
    use SurfaceChangePosture as Posture;
    use SurfaceChangeSource as Src;
    assert!(resolve_surface_change(Src::BackwardCompatibleAddition).is_compatible);
    assert!(resolve_surface_change(Src::ClarifyingCompatibleChange).is_compatible);
    for source in [Src::IncompatibleSignatureChange, Src::SurfaceRemoved] {
        assert!(
            resolve_surface_change(source).is_breaking,
            "{source:?} must read as breaking"
        );
    }
    assert_eq!(
        resolve_surface_change(Src::SurfaceRemoved).posture,
        Posture::Removal
    );
    assert!(resolve_surface_change(Src::DeprecationAnnounced).requires_migration_note);
    assert!(!resolve_surface_change(Src::BackwardCompatibleAddition).requires_migration_note);
}

#[test]
fn local_estimate_banner_claiming_provider_confirmed_fails() {
    let mut packet = packet();
    let banner = packet
        .merge_control_banners
        .iter_mut()
        .find(|banner| {
            banner.confirmation_locus_source == ConfirmationLocusSource::LocalHeuristicEstimate
        })
        .expect("local-estimate banner present");
    banner.claims_provider_confirmed = true;
    let violations = packet.validate();
    assert!(violations.contains(
        &PublicSurfaceMergeControlControlsViolation::LocalOrMachineClaimsProviderConfirmed
    ));
}

#[test]
fn machine_generated_card_claiming_provider_confirmed_fails() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| {
            card.confirmation_locus_source == ConfirmationLocusSource::MachineGeneratedLocally
        })
        .expect("machine-generated card present");
    card.claims_provider_confirmed = true;
    let violations = packet.validate();
    assert!(violations.contains(
        &PublicSurfaceMergeControlControlsViolation::LocalOrMachineClaimsProviderConfirmed
    ));
}

#[test]
fn not_evaluated_here_claiming_evaluated_fails() {
    let mut packet = packet();
    let banner = packet
        .merge_control_banners
        .iter_mut()
        .find(|banner| {
            banner.confirmation_locus_source == ConfirmationLocusSource::NotEvaluatedHere
        })
        .expect("not-evaluated banner present");
    banner.claims_evaluated_here = true;
    let violations = packet.validate();
    assert!(violations
        .contains(&PublicSurfaceMergeControlControlsViolation::NotEvaluatedClaimsEvaluated));
}

#[test]
fn stable_breaking_change_without_migration_fails() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| {
            card.stability_class == StabilityClass::Stable && card.change_disclosure().is_breaking
        })
        .expect("stable breaking card present");
    card.migration_note = String::new();
    assert!(packet.validate().contains(
        &PublicSurfaceMergeControlControlsViolation::StableChangeMissingMigrationOrEvidence
    ));
}

#[test]
fn stable_breaking_change_without_evidence_fails() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| {
            card.stability_class == StabilityClass::Stable && card.change_disclosure().is_breaking
        })
        .expect("stable breaking card present");
    card.migration_evidence_ref = String::new();
    assert!(packet.validate().contains(
        &PublicSurfaceMergeControlControlsViolation::StableChangeMissingMigrationOrEvidence
    ));
}

#[test]
fn removal_note_required() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| card.surface_change_source == SurfaceChangeSource::SurfaceRemoved)
        .expect("removal card present");
    card.removal_note = String::new();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::RemovalNoteMissing));
}

#[test]
fn deprecation_note_required() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| card.surface_change_source == SurfaceChangeSource::DeprecationAnnounced)
        .expect("deprecation card present");
    card.deprecation_note = String::new();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::DeprecationNoteMissing));
}

#[test]
fn stale_note_required() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| {
            card.confirmation_locus_source == ConfirmationLocusSource::StaleAgainstBaseHead
        })
        .expect("stale card present");
    card.stale_note = String::new();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::StaleNoteMissing));
}

#[test]
fn machine_generated_note_required() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| {
            card.confirmation_locus_source == ConfirmationLocusSource::MachineGeneratedLocally
        })
        .expect("machine-generated card present");
    card.machine_generated_note = String::new();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::MachineGeneratedNoteMissing));
}

#[test]
fn governance_vocab_missing_locus_token_fails() {
    let mut packet = packet();
    let banner = packet
        .merge_control_banners
        .iter_mut()
        .find(|banner| {
            banner.confirmation_locus_source == ConfirmationLocusSource::LocalHeuristicEstimate
        })
        .expect("local-estimate banner present");
    banner.governance_state_vocab.clear();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::GovernanceVocabMissingLocusToken));
}

#[test]
fn merge_blocker_reason_required_when_blocking() {
    let mut packet = packet();
    let banner = packet
        .merge_control_banners
        .iter_mut()
        .find(|banner| banner.blocker_class.is_blocking())
        .expect("blocking banner present");
    banner.blocker_reason_label = String::new();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::MergeBlockerReasonMissing));
}

#[test]
fn required_check_incomplete_fails() {
    let mut packet = packet();
    packet.merge_control_banners[0].required_checks[0].check_label = String::new();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::RequiredCheckIncomplete));
}

#[test]
fn open_diff_evidence_action_required() {
    let mut packet = packet();
    packet.public_surface_diff_cards[0]
        .card_actions
        .retain(|action| *action != PublicSurfaceDiffCardAction::OpenDiffEvidence);
    let violations = packet.validate();
    assert!(violations
        .contains(&PublicSurfaceMergeControlControlsViolation::OpenDiffEvidenceActionMissing));
    assert!(violations
        .contains(&PublicSurfaceMergeControlControlsViolation::ComponentActionsIncomplete));
}

#[test]
fn resolvable_diff_evidence_requires_ref() {
    let mut packet = packet();
    let card = packet
        .public_surface_diff_cards
        .iter_mut()
        .find(|card| card.diff_evidence_kind.is_resolvable())
        .expect("resolvable diff-evidence card present");
    card.diff_evidence_ref = String::new();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::DiffEvidenceRefMissing));
}

#[test]
fn wrong_component_class_fails() {
    let mut packet = packet();
    packet.public_surface_diff_cards[0].component = M5GovernanceComponent::MergeControlBanner;
    assert!(packet.validate().contains(
        &PublicSurfaceMergeControlControlsViolation::PublicSurfaceDiffCardWrongComponentClass
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet.review.merge_blocker_never_generic = false;
    assert!(packet
        .validate()
        .contains(&PublicSurfaceMergeControlControlsViolation::ReviewIncomplete));
}

#[test]
fn cards_alone_cover_every_surface_class_and_change_posture() {
    let packet = packet();
    assert!(packet.validate().is_empty());
    let mut classes: BTreeSet<PublicSurfaceClass> = BTreeSet::new();
    let mut postures: BTreeSet<SurfaceChangePosture> = BTreeSet::new();
    for card in &packet.public_surface_diff_cards {
        for class in &card.surface_classes {
            classes.insert(*class);
        }
        postures.insert(card.change_disclosure().posture);
    }
    // AC-1: every public surface a change can materially affect, and every change posture, is
    // distinguishable from the diff cards alone.
    assert_eq!(classes.len(), PublicSurfaceClass::ALL.len());
    assert_eq!(postures.len(), SurfaceChangePosture::ALL.len());
}

#[test]
fn banners_cover_every_blocker_and_bypass_class() {
    let packet = packet();
    let mut blockers: BTreeSet<MergeBlockerClass> = BTreeSet::new();
    let mut bypasses: BTreeSet<BypassPolicyClass> = BTreeSet::new();
    for banner in &packet.merge_control_banners {
        blockers.insert(banner.blocker_class);
        bypasses.insert(banner.bypass_policy);
    }
    assert_eq!(blockers.len(), MergeBlockerClass::ALL.len());
    assert_eq!(bypasses.len(), BypassPolicyClass::ALL.len());
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for card in packet().public_surface_diff_cards {
        assert!(
            summary.contains(&card.change_title_label),
            "summary missing card {}",
            card.card_id
        );
    }
    for banner in packet().merge_control_banners {
        assert!(
            summary.contains(&banner.gate_title_label),
            "summary missing banner {}",
            banner.banner_id
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_public_surface_merge_control_controls_export()
        .expect("checked public-surface merge-control controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed_packet() {
    let seed = packet();
    let checked = current_public_surface_merge_control_controls_export()
        .expect("checked public-surface merge-control controls export validates");
    assert_eq!(checked, seed);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-public-surface-diff-merge-control-controls/public_surface_diff_stable_breaking.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-public-surface-diff-merge-control-controls/merge_control_banner_local_estimate.json"
        )),
    ] {
        let packet: PublicSurfaceMergeControlControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as public-surface merge-control packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_PUBLIC_SURFACE_DIFF_MERGE_CONTROL_CONTROLS_ARTIFACTS` so ordinary test runs
/// never touch the working tree. Run in isolation with the env gate set, then run the full suite:
/// `GEN_PUBLIC_SURFACE_DIFF_MERGE_CONTROL_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review
/// implement_public_surface_diff_cards_and_merge_control_banners_with_surface_class_stability_label_schema_or_command_delta_disclosure_blocker_reason_bypass_policy_and_migration_note_continuity::tests::generate_artifacts
/// -- --exact --ignored`
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_PUBLIC_SURFACE_DIFF_MERGE_CONTROL_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-public-surface-diff-merge-control-controls-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-public-surface-diff-merge-control-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: a stable breaking public-surface change that must carry migration and evidence.
    let mut stable_breaking = packet.clone();
    stable_breaking.packet_id =
        "m5-public-surface-diff-merge-control-controls:fixture:stable-breaking".to_owned();
    stable_breaking.surface_label =
        "M5 public-surface diff cards: a stable breaking change never hides without migration"
            .to_owned();
    assert!(
        stable_breaking.validate().is_empty(),
        "{:?}",
        stable_breaking.validate()
    );
    std::fs::write(
        fixture_dir.join("public_surface_diff_stable_breaking.json"),
        format!("{}\n", stable_breaking.export_safe_json()),
    )
    .expect("write stable-breaking fixture");

    // Fixture 2: a local-estimate merge banner that must never read as provider-confirmed.
    let mut local_estimate = packet;
    local_estimate.packet_id =
        "m5-public-surface-diff-merge-control-controls:fixture:local-estimate".to_owned();
    local_estimate.surface_label =
        "M5 merge-control banners: a local-estimate gate never reads as provider-confirmed"
            .to_owned();
    assert!(
        local_estimate.validate().is_empty(),
        "{:?}",
        local_estimate.validate()
    );
    std::fs::write(
        fixture_dir.join("merge_control_banner_local_estimate.json"),
        format!("{}\n", local_estimate.export_safe_json()),
    )
    .expect("write local-estimate fixture");
}
