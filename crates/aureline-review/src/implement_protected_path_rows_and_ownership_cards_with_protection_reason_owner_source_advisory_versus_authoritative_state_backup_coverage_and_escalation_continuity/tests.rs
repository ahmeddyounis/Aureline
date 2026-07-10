use super::*;

const PACKET_ID: &str = "m5-protected-path-ownership-controls:stable:0001";
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn component_source_refs() -> Vec<String> {
    strings(&[
        M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF,
    ])
}

fn row_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn card_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::OwnerCoverageBackupMissing,
        M5GovernanceComponentDowngradeTrigger::DriCoverageGap,
    ]
}

fn row_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::GovernanceDashboard,
        M5GovernanceComponentConsumerSurface::CliHeadless,
        M5GovernanceComponentConsumerSurface::SupportExport,
    ]
}

fn card_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::OwnerCoveragePanel,
        M5GovernanceComponentConsumerSurface::Shiproom,
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

#[allow(clippy::too_many_arguments)]
fn protected_path_row(
    row_id: &str,
    path_label: &str,
    protection_reason_label: &str,
    owner_source_label: &str,
    enforcement_source: OwnerEnforcementSource,
    evaluation_freshness: EvaluationFreshnessState,
    evaluation_freshness_label: &str,
    rule_source_kind: RuleSourceKind,
    rule_source_ref: &str,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    row_actions: Vec<ProtectedPathRowAction>,
) -> ProtectedPathRow {
    let disclosure = resolve_enforcement_posture(enforcement_source);
    let mut governance_state_vocab = vec![disclosure.governance_vocab];
    if evaluation_freshness.needs_note()
        && !governance_state_vocab.contains(&M5GovernanceStateVocab::Stale)
    {
        governance_state_vocab.push(M5GovernanceStateVocab::Stale);
    }
    ProtectedPathRow {
        component: M5GovernanceComponent::ProtectedPathRow,
        row_id: row_id.to_owned(),
        path_label: path_label.to_owned(),
        protection_reason_label: protection_reason_label.to_owned(),
        owner_source_label: owner_source_label.to_owned(),
        enforcement_source,
        derived_enforcement_posture: disclosure.posture,
        claims_authoritative_enforcement: disclosure.is_authoritative,
        claims_provider_authoritative: disclosure.is_provider_authoritative,
        governance_state_vocab,
        advisory_note: note_if(
            disclosure.needs_advisory_note,
            "Owner rule is an advisory hint; it is not an enforced gate",
        ),
        local_estimate_note: note_if(
            disclosure.needs_local_estimate_note,
            "This match is a local estimate; it is not the provider's final gate",
        ),
        evaluation_freshness,
        evaluation_freshness_label: evaluation_freshness_label.to_owned(),
        stale_evaluation_note: note_if(
            evaluation_freshness.needs_note(),
            &format!(
                "Protection was {}; re-evaluate before trusting the guard",
                evaluation_freshness.as_str()
            ),
        ),
        rule_source_kind,
        rule_source_ref: rule_source_ref.to_owned(),
        context_note: context_note.to_owned(),
        row_actions,
        downgrade_triggers: row_downgrade_triggers(),
        consumer_surfaces: row_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_protection_reason_or_owner_source: false,
        lets_advisory_masquerade_as_authoritative: false,
        lets_local_estimate_read_as_provider_authoritative: false,
        invents_alternate_state_label: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn ownership_card(
    card_id: &str,
    owned_path_label: &str,
    primary_owner_alias: &str,
    backup_owner_alias: &str,
    owner_source_class: OwnerSourceClass,
    owner_source_label: &str,
    enforcement_source: OwnerEnforcementSource,
    coverage_source: OwnerCoverageSource,
    escalation_path_label: &str,
    escalation_boundary_note: &str,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    card_actions: Vec<OwnershipCardAction>,
) -> OwnershipCard {
    let enforcement = resolve_enforcement_posture(enforcement_source);
    let coverage = resolve_owner_coverage_posture(coverage_source);
    let mut governance_state_vocab = vec![enforcement.governance_vocab];
    if !governance_state_vocab.contains(&coverage.governance_vocab) {
        governance_state_vocab.push(coverage.governance_vocab);
    }
    OwnershipCard {
        component: M5GovernanceComponent::OwnershipCard,
        card_id: card_id.to_owned(),
        owned_path_label: owned_path_label.to_owned(),
        primary_owner_alias: primary_owner_alias.to_owned(),
        backup_owner_alias: backup_owner_alias.to_owned(),
        owner_source_class,
        owner_source_label: owner_source_label.to_owned(),
        enforcement_source,
        derived_enforcement_posture: enforcement.posture,
        claims_authoritative_enforcement: enforcement.is_authoritative,
        claims_provider_authoritative: enforcement.is_provider_authoritative,
        coverage_source,
        derived_coverage_posture: coverage.posture,
        derived_continuity_state: coverage.continuity_state,
        claims_clean_coverage: coverage.is_clean_coverage,
        governance_state_vocab,
        advisory_note: note_if(
            enforcement.needs_advisory_note,
            "Owner assignment is an advisory hint; it is not an enforced gate",
        ),
        local_estimate_note: note_if(
            enforcement.needs_local_estimate_note,
            "Owner is a local estimate; it is not the provider's resolved owner",
        ),
        backup_missing_note: note_if(
            coverage.needs_backup_missing_note,
            "Backup coverage is missing; a single owner carries the whole approval burden",
        ),
        unresolved_owner_note: note_if(
            coverage.needs_unresolved_note,
            "Owner is unresolved for this path; do not present it as covered",
        ),
        policy_hidden_note: note_if(
            coverage.needs_policy_hidden_note,
            "Owner is hidden by policy on this build; coverage cannot be confirmed here",
        ),
        escalation_path_label: escalation_path_label.to_owned(),
        escalation_boundary_note: escalation_boundary_note.to_owned(),
        context_note: context_note.to_owned(),
        card_actions,
        downgrade_triggers: card_downgrade_triggers(),
        consumer_surfaces: card_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_owner_source_or_coverage: false,
        lets_advisory_masquerade_as_authoritative: false,
        presents_missing_backup_as_clean_coverage: false,
        invents_alternate_state_label: false,
    }
}

fn protected_path_rows() -> Vec<ProtectedPathRow> {
    use EvaluationFreshnessState as Fresh;
    use M5GovernanceComponentRollbackPosture as Rollback;
    use OwnerEnforcementSource as Src;
    use ProtectedPathRowAction as Action;
    use RuleSourceKind as Rule;

    let full_actions = vec![
        Action::OpenRuleSource,
        Action::InspectEnforcementAuthority,
        Action::ReviewProtectionReason,
        Action::InspectOwnerSource,
        Action::ReviewEvaluationFreshness,
        Action::CopyPathPattern,
    ];

    vec![
        // 1. Provider branch protection / current / branch-protection rule → provider-authoritative.
        protected_path_row(
            "ppr-release-workflow",
            ".github/workflows/release.yml",
            "Release workflow is provider branch-protected: changes require an owner approval",
            "Provider branch protection rule",
            Src::ProviderBranchProtection,
            Fresh::CurrentlyEvaluated,
            "Evaluated against the current head",
            Rule::BranchProtectionRule,
            "rule:branch-protection/release-workflow",
            "Provider-enforced protection; open the branch-protection rule to see who approves",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 2. Provider-resolved CODEOWNERS / imported / codeowners rule → provider-authoritative.
        protected_path_row(
            "ppr-crypto-core",
            "crates/aureline-crypto/**",
            "Cryptography core is CODEOWNERS-guarded: a security owner review is required",
            "Provider-resolved CODEOWNERS",
            Src::ProviderResolvedCodeowners,
            Fresh::Imported,
            "Imported from the last provider scan",
            Rule::CodeownersRule,
            "rule:codeowners/crypto-core",
            "Provider-resolved owners; the imported evaluation may lag the current head",
            Rollback::ProviderMutationAttributable,
            full_actions.clone(),
        ),
        // 3. Local manifest enforced / currently evaluated / manifest entry → locally authoritative.
        protected_path_row(
            "ppr-public-schema",
            "schemas/public/**",
            "Public schema surface is locally protected: a public-surface diff review is required",
            "Local protected-path manifest",
            Src::LocalManifestEnforced,
            Fresh::CurrentlyEvaluated,
            "Evaluated against the current head",
            Rule::ManifestEntry,
            "rule:manifest/public-schema",
            "Locally enforced protection; open the manifest entry to see the governing rule",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 4. Local manifest advisory / stale / protected-path policy → advisory-only.
        protected_path_row(
            "ppr-docs-public",
            "docs/public/**",
            "Public docs are advisory-guarded: an owner review is suggested but not enforced",
            "Local protected-path policy (advisory)",
            Src::LocalManifestAdvisory,
            Fresh::Stale,
            "Evaluation is stale relative to the current head",
            Rule::ProtectedPathPolicy,
            "rule:policy/docs-public",
            "Advisory hint only; do not treat it as an enforced gate",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 5. Local heuristic match / never evaluated / no rule source → local estimate.
        protected_path_row(
            "ppr-inferred-config",
            "config/*.toml",
            "Config files matched a heuristic protection pattern: treat as an estimate",
            "Local heuristic path match",
            Src::LocalHeuristicMatch,
            Fresh::NeverEvaluated,
            "Never evaluated on this build",
            Rule::NoRuleSource,
            "",
            "Local estimate with no governing rule source; confirm before trusting the guard",
            Rollback::LocalContinuePreserved,
            vec![
                Action::OpenRuleSource,
                Action::InspectEnforcementAuthority,
                Action::ReviewProtectionReason,
            ],
        ),
        // 6. Inferred from authorship / unknown freshness / no rule source → local estimate.
        protected_path_row(
            "ppr-inferred-owner",
            "services/billing/**",
            "Billing service owner inferred from recent authorship: not a recorded rule",
            "Inferred from recent authorship",
            Src::InferredFromAuthorship,
            Fresh::Unknown,
            "Freshness unknown",
            Rule::NoRuleSource,
            "",
            "Owner inferred from authorship; this is a local estimate, not an enforced rule",
            Rollback::EvidencePreservedNoRevert,
            vec![
                Action::OpenRuleSource,
                Action::InspectEnforcementAuthority,
                Action::ReviewProtectionReason,
                Action::InspectOwnerSource,
            ],
        ),
    ]
}

fn ownership_cards() -> Vec<OwnershipCard> {
    use M5GovernanceComponentRollbackPosture as Rollback;
    use OwnerCoverageSource as Coverage;
    use OwnerEnforcementSource as Src;
    use OwnerSourceClass as SourceClass;
    use OwnershipCardAction as Action;

    let full_actions = vec![
        Action::InspectOwnerSource,
        Action::ReviewBackupCoverage,
        Action::OpenEscalationPath,
        Action::ReviewContinuityState,
        Action::InspectEnforcementAuthority,
        Action::CopyOwnerAliases,
    ];

    vec![
        // 1. Provider-resolved covered → covered, continuous, provider-authoritative.
        ownership_card(
            "own-crypto-core",
            "crates/aureline-crypto/**",
            "security-team",
            "platform-oncall",
            SourceClass::CodeownersEntry,
            "Provider-resolved CODEOWNERS entry",
            Src::ProviderResolvedCodeowners,
            Coverage::ProviderResolvedCovered,
            "Escalate to the security-team lead via the shiproom",
            "Escalation is a labeled handoff with a return path to this ownership card",
            "Provider-resolved owners with backup; coverage is clean",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 2. Local manifest enforced / primary + backup resolved → covered, locally authoritative.
        ownership_card(
            "own-public-schema",
            "schemas/public/**",
            "api-platform-team",
            "release-oncall",
            SourceClass::OwnershipManifest,
            "Local ownership manifest",
            Src::LocalManifestEnforced,
            Coverage::PrimaryAndBackupResolved,
            "Escalate to the api-platform-team lead via the shiproom",
            "Escalation is a labeled handoff with a return path to this ownership card",
            "Locally enforced owners with backup; coverage is clean",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 3. Local manifest advisory / backup missing → backup missing, advisory-only.
        ownership_card(
            "own-billing",
            "services/billing/**",
            "billing-team",
            "",
            SourceClass::DriRegistry,
            "DRI registry (advisory)",
            Src::LocalManifestAdvisory,
            Coverage::PrimaryOnlyBackupMissing,
            "Escalate to the billing-team DRI via the shiproom",
            "Escalation is a labeled handoff with a return path to this ownership card",
            "Advisory owner with no backup; the single owner carries the whole approval burden",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 4. Local heuristic match / owner unresolved → unresolved, local estimate.
        ownership_card(
            "own-legacy-tools",
            "tools/legacy/**",
            "unassigned-role",
            "",
            SourceClass::Unresolved,
            "Local heuristic match (unresolved)",
            Src::LocalHeuristicMatch,
            Coverage::OwnerUnresolved,
            "Escalate to the platform on-call to assign an owner",
            "Escalation is a labeled handoff with a return path to this ownership card",
            "Owner is unresolved; do not present this path as covered",
            Rollback::EvidencePreservedNoRevert,
            full_actions.clone(),
        ),
        // 5. Inferred from authorship / policy-hidden owner → policy hidden, local estimate.
        ownership_card(
            "own-restricted",
            "services/restricted/**",
            "policy-restricted-role",
            "",
            SourceClass::InferredAuthorship,
            "Inferred authorship (policy-hidden)",
            Src::InferredFromAuthorship,
            Coverage::PolicyHiddenOwner,
            "Escalate to the governance owner to reveal ownership under policy",
            "Escalation is a labeled handoff with a return path to this ownership card",
            "Owner is hidden by policy; coverage cannot be confirmed on this build",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 6. Provider branch protection / provider covered → covered, provider-authoritative.
        ownership_card(
            "own-release-workflow",
            ".github/workflows/release.yml",
            "release-eng-team",
            "release-oncall",
            SourceClass::CodeownersEntry,
            "Provider branch-protection owners",
            Src::ProviderBranchProtection,
            Coverage::ProviderResolvedCovered,
            "Escalate to the release-eng lead via the shiproom",
            "Escalation is a labeled handoff with a return path to this ownership card",
            "Provider-enforced owners with backup; coverage is clean",
            Rollback::ProviderMutationAttributable,
            full_actions,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
        M5GovernanceComponentDowngradeTrigger::OwnerCoverageBackupMissing,
        M5GovernanceComponentDowngradeTrigger::DriCoverageGap,
        M5GovernanceComponentDowngradeTrigger::EscalationHandoffUnavailable,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    M5GovernanceComponentConsumerSurface::ALL.to_vec()
}

fn review() -> ProtectedPathOwnershipReview {
    ProtectedPathOwnershipReview {
        protected_path_row_shows_reason_and_enforcement: true,
        protected_path_row_shows_owner_source: true,
        protected_path_row_offers_open_rule_source: true,
        ownership_card_shows_owners_and_source: true,
        ownership_card_shows_coverage_and_escalation: true,
        ownership_card_uses_export_safe_role_aliases: true,
        enforcement_posture_derived_never_asserted: true,
        advisory_never_shown_as_authoritative: true,
        local_estimate_never_shown_as_provider_authoritative: true,
        missing_backup_never_shown_as_covered: true,
        evaluation_freshness_always_explicit: true,
        every_guarded_path_names_stable_rule_source: true,
        escalation_handoff_always_explicit: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ProtectedPathOwnershipConsumerProjection {
    ProtectedPathOwnershipConsumerProjection {
        review_workspace_reads_single_source: true,
        owner_coverage_panel_reads_single_source: true,
        governance_and_shiproom_read_single_source: true,
        reason_and_owner_visible_before_trust: true,
        coverage_and_escalation_visible_before_trust: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> ProtectedPathOwnershipProofFreshness {
    ProtectedPathOwnershipProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        PROTECTED_PATH_OWNERSHIP_CONTROLS_SCHEMA_REF,
        PROTECTED_PATH_OWNERSHIP_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF,
    ])
}

fn packet() -> ProtectedPathOwnershipControlsPacket {
    ProtectedPathOwnershipControlsPacket::new(ProtectedPathOwnershipControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label:
            "M5 protected-path rows and ownership cards: protection reason, owner source, advisory-versus-authoritative enforcement, backup coverage, and escalation continuity across claimed governed surfaces"
                .to_owned(),
        protected_path_rows: protected_path_rows(),
        ownership_cards: ownership_cards(),
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
fn enforcement_posture_is_derived_not_asserted() {
    use EnforcementPosture as Posture;
    use OwnerEnforcementSource as Src;
    assert_eq!(
        resolve_enforcement_posture(Src::ProviderBranchProtection).posture,
        Posture::ProviderAuthoritative
    );
    assert_eq!(
        resolve_enforcement_posture(Src::ProviderResolvedCodeowners).posture,
        Posture::ProviderAuthoritative
    );
    assert_eq!(
        resolve_enforcement_posture(Src::LocalManifestEnforced).posture,
        Posture::LocallyAuthoritative
    );
    assert_eq!(
        resolve_enforcement_posture(Src::LocalManifestAdvisory).posture,
        Posture::AdvisoryOnly
    );
    assert_eq!(
        resolve_enforcement_posture(Src::LocalHeuristicMatch).posture,
        Posture::LocalEstimate
    );
    assert_eq!(
        resolve_enforcement_posture(Src::InferredFromAuthorship).posture,
        Posture::LocalEstimate
    );
}

#[test]
fn coverage_posture_degrades_explicitly() {
    use OwnerContinuityState as Continuity;
    use OwnerCoveragePosture as Posture;
    use OwnerCoverageSource as Src;
    assert!(resolve_owner_coverage_posture(Src::PrimaryAndBackupResolved).is_clean_coverage);
    assert!(resolve_owner_coverage_posture(Src::ProviderResolvedCovered).is_clean_coverage);
    for source in [
        Src::PrimaryOnlyBackupMissing,
        Src::OwnerUnresolved,
        Src::PolicyHiddenOwner,
    ] {
        assert!(
            !resolve_owner_coverage_posture(source).is_clean_coverage,
            "{source:?} must not read as clean coverage"
        );
    }
    assert_eq!(
        resolve_owner_coverage_posture(Src::PrimaryOnlyBackupMissing).posture,
        Posture::BackupMissing
    );
    assert_eq!(
        resolve_owner_coverage_posture(Src::OwnerUnresolved).continuity_state,
        Continuity::UnresolvedContinuity
    );
}

#[test]
fn advisory_posture_never_claims_authoritative() {
    let disclosure = resolve_enforcement_posture(OwnerEnforcementSource::LocalManifestAdvisory);
    assert!(disclosure.is_advisory);
    assert!(!disclosure.is_authoritative);
    assert_eq!(
        disclosure.governance_vocab,
        M5GovernanceStateVocab::Advisory
    );
}

#[test]
fn advisory_claiming_authoritative_fails() {
    let mut packet = packet();
    let row = packet
        .protected_path_rows
        .iter_mut()
        .find(|row| row.enforcement_source == OwnerEnforcementSource::LocalManifestAdvisory)
        .expect("advisory row present");
    row.claims_authoritative_enforcement = true;
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::AdvisoryClaimsAuthoritative));
}

#[test]
fn local_estimate_claiming_provider_authoritative_fails() {
    let mut packet = packet();
    let card = packet
        .ownership_cards
        .iter_mut()
        .find(|card| card.enforcement_source == OwnerEnforcementSource::LocalHeuristicMatch)
        .expect("local-estimate card present");
    card.claims_provider_authoritative = true;
    assert!(packet.validate().contains(
        &ProtectedPathOwnershipControlsViolation::LocalEstimateClaimsProviderAuthoritative
    ));
}

#[test]
fn missing_backup_presented_as_covered_fails() {
    let mut packet = packet();
    let card = packet
        .ownership_cards
        .iter_mut()
        .find(|card| card.coverage_source == OwnerCoverageSource::PrimaryOnlyBackupMissing)
        .expect("backup-missing card present");
    card.claims_clean_coverage = true;
    let violations = packet.validate();
    assert!(violations
        .contains(&ProtectedPathOwnershipControlsViolation::MissingBackupPresentedAsCovered));
}

#[test]
fn backup_missing_note_required() {
    let mut packet = packet();
    let card = packet
        .ownership_cards
        .iter_mut()
        .find(|card| card.coverage_source == OwnerCoverageSource::PrimaryOnlyBackupMissing)
        .expect("backup-missing card present");
    card.backup_missing_note = String::new();
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::BackupMissingNoteMissing));
}

#[test]
fn person_contact_detail_in_alias_fails() {
    let mut packet = packet();
    packet.ownership_cards[0].primary_owner_alias = "person@example.com".to_owned();
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::PersonContactDetailInAlias));
}

#[test]
fn stale_evaluation_note_required() {
    let mut packet = packet();
    let row = packet
        .protected_path_rows
        .iter_mut()
        .find(|row| row.evaluation_freshness == EvaluationFreshnessState::Stale)
        .expect("stale row present");
    row.stale_evaluation_note = String::new();
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::StaleEvaluationNoteMissing));
}

#[test]
fn open_rule_source_action_required() {
    let mut packet = packet();
    packet.protected_path_rows[0]
        .row_actions
        .retain(|action| *action != ProtectedPathRowAction::OpenRuleSource);
    let violations = packet.validate();
    assert!(
        violations.contains(&ProtectedPathOwnershipControlsViolation::OpenRuleSourceActionMissing)
    );
    assert!(
        violations.contains(&ProtectedPathOwnershipControlsViolation::ComponentActionsIncomplete)
    );
}

#[test]
fn resolvable_rule_source_requires_ref() {
    let mut packet = packet();
    let row = packet
        .protected_path_rows
        .iter_mut()
        .find(|row| row.rule_source_kind.is_resolvable())
        .expect("resolvable rule-source row present");
    row.rule_source_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::RuleSourceRefMissing));
}

#[test]
fn governance_vocab_missing_enforcement_token_fails() {
    let mut packet = packet();
    packet.protected_path_rows[0].governance_state_vocab = vec![M5GovernanceStateVocab::Covered];
    assert!(packet.validate().contains(
        &ProtectedPathOwnershipControlsViolation::GovernanceVocabMissingEnforcementToken
    ));
}

#[test]
fn wrong_component_class_fails() {
    let mut packet = packet();
    packet.protected_path_rows[0].component = M5GovernanceComponent::OwnershipCard;
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::ProtectedPathRowWrongComponentClass));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet.review.missing_backup_never_shown_as_covered = false;
    assert!(packet
        .validate()
        .contains(&ProtectedPathOwnershipControlsViolation::ReviewIncomplete));
}

#[test]
fn enforcement_source_and_posture_fully_covered() {
    let packet = packet();
    assert!(packet.validate().is_empty());
    let mut sources: BTreeSet<OwnerEnforcementSource> = BTreeSet::new();
    let mut postures: BTreeSet<EnforcementPosture> = BTreeSet::new();
    for row in &packet.protected_path_rows {
        sources.insert(row.enforcement_source);
        postures.insert(row.enforcement_disclosure().posture);
    }
    for card in &packet.ownership_cards {
        sources.insert(card.enforcement_source);
        postures.insert(card.enforcement_disclosure().posture);
    }
    assert_eq!(sources.len(), OwnerEnforcementSource::ALL.len());
    assert_eq!(postures.len(), EnforcementPosture::ALL.len());
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for row in packet().protected_path_rows {
        assert!(
            summary.contains(&row.path_label),
            "summary missing row {}",
            row.path_label
        );
    }
    for card in packet().ownership_cards {
        assert!(
            summary.contains(&card.owned_path_label),
            "summary missing card {}",
            card.owned_path_label
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_protected_path_ownership_controls_export()
        .expect("checked protected-path ownership controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed_packet() {
    let seed = packet();
    let checked = current_protected_path_ownership_controls_export()
        .expect("checked protected-path ownership controls export validates");
    assert_eq!(checked, seed);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-ownership-controls/path_row_advisory_local_estimate.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-ownership-controls/ownership_card_backup_missing.json"
        )),
    ] {
        let packet: ProtectedPathOwnershipControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as protected-path ownership packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_PROTECTED_PATH_OWNERSHIP_CONTROLS_ARTIFACTS` so ordinary test runs never
/// touch the working tree. Run in isolation with the env gate set, then run the full suite:
/// `GEN_PROTECTED_PATH_OWNERSHIP_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review
/// implement_protected_path_rows_and_ownership_cards_with_protection_reason_owner_source_advisory_versus_authoritative_state_backup_coverage_and_escalation_continuity::tests::generate_artifacts
/// -- --exact --ignored`
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_PROTECTED_PATH_OWNERSHIP_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-protected-path-ownership-controls-proof");
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
        .join("m5-protected-path-ownership-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: an advisory, stale protected-path row that must never read as an enforced gate.
    let mut advisory = packet.clone();
    advisory.packet_id =
        "m5-protected-path-ownership-controls:fixture:advisory-local-estimate".to_owned();
    advisory.surface_label =
        "M5 protected-path rows: an advisory, stale row never reads as an enforced provider gate"
            .to_owned();
    assert!(advisory.validate().is_empty(), "{:?}", advisory.validate());
    std::fs::write(
        fixture_dir.join("path_row_advisory_local_estimate.json"),
        format!("{}\n", advisory.export_safe_json()),
    )
    .expect("write advisory fixture");

    // Fixture 2: a backup-missing ownership card that must never read as clean coverage.
    let mut backup_missing = packet;
    backup_missing.packet_id =
        "m5-protected-path-ownership-controls:fixture:backup-missing".to_owned();
    backup_missing.surface_label =
        "M5 ownership cards: a backup-missing card never reads as clean coverage".to_owned();
    assert!(
        backup_missing.validate().is_empty(),
        "{:?}",
        backup_missing.validate()
    );
    std::fs::write(
        fixture_dir.join("ownership_card_backup_missing.json"),
        format!("{}\n", backup_missing.export_safe_json()),
    )
    .expect("write backup-missing fixture");
}
