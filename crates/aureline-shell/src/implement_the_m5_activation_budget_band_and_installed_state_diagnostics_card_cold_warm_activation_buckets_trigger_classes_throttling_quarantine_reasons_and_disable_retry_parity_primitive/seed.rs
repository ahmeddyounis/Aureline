//! Canonical seed builders for the M5 activation-budget-band / installed-state-diagnostics-card
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean bands
//! that describe an over-budget artifact still carry cold / warm activation evidence, and clean
//! cards that describe a throttled or quarantined artifact carry the reason and the disable / retry
//! action pair, so performance and quarantine implications are never hidden.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_PACKET_ID: &str =
    "m5-activation-budget-band-installed-state-diagnostics-card-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn band(input: M5ActivationBudgetBandResolutionInput) -> M5ResolvedActivationBudgetBand {
    resolve_activation_budget_band(input).expect("seed activation budget band resolves")
}

fn card(
    input: M5InstalledStateDiagnosticsCardResolutionInput,
) -> M5ResolvedInstalledStateDiagnosticsCard {
    resolve_installed_state_diagnostics_card(input).expect("seed diagnostics card resolves")
}

// -- Clean activation-budget band examples ------------------------------------------------------

/// Clean band: within budget, low cold / warm cost, certified on fresh evidence.
fn band_within_low_clean() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::Low),
        warm_start_evidence: Some(M5ActivationCostLevel::Low),
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

/// Clean band: near budget, medium warm cost, not certified.
fn band_near_medium_clean() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:mid-tool".to_owned(),
        artifact_identity: "mid-tool".to_owned(),
        budget_state: M5ActivationBudgetBandState::NearBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::Medium),
        warm_start_evidence: Some(M5ActivationCostLevel::Low),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

/// Clean band: over budget but honestly shown with cold / warm evidence after runtime degradation.
fn band_over_with_evidence_clean() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:heavy-tool".to_owned(),
        artifact_identity: "heavy-tool".to_owned(),
        budget_state: M5ActivationBudgetBandState::OverBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::High),
        warm_start_evidence: Some(M5ActivationCostLevel::High),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

/// Clean band: within budget but with a high warm-start cost, so the class reads High honestly.
fn band_within_high_warm_clean() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:warm-heavy-tool".to_owned(),
        artifact_identity: "warm-heavy-tool".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::Low),
        warm_start_evidence: Some(M5ActivationCostLevel::High),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

// -- Degraded activation-budget band examples ---------------------------------------------------

/// Degraded band: the artifact identity is unstated.
fn band_identity_unstated() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::Low),
        warm_start_evidence: Some(M5ActivationCostLevel::Low),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

/// Degraded band: the activation-budget band cannot be resolved.
fn band_unknown() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:unknown".to_owned(),
        artifact_identity: "budgetless-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::BudgetUnknown,
        cold_start_evidence: None,
        warm_start_evidence: None,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

/// Degraded band: an over-budget artifact reads as cost-free.
fn band_over_budget_cost_free() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:over-cost-free".to_owned(),
        artifact_identity: "silent-over-budget-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::OverBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::High),
        warm_start_evidence: Some(M5ActivationCostLevel::High),
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: true,
        proof_fresh: true,
    })
}

/// Degraded band: a throttled artifact carries no cold / warm activation evidence.
fn band_evidence_missing_after_degradation() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:evidence-missing".to_owned(),
        artifact_identity: "throttled-no-evidence-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::Throttled,
        cold_start_evidence: None,
        warm_start_evidence: None,
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

/// Degraded band: Certified language is left in place on stale evidence.
fn band_stale_certified() -> M5ResolvedActivationBudgetBand {
    band(M5ActivationBudgetBandResolutionInput {
        band_id: "budget-band:stale-certified".to_owned(),
        artifact_identity: "stale-certified-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        cold_start_evidence: Some(M5ActivationCostLevel::Low),
        warm_start_evidence: Some(M5ActivationCostLevel::Medium),
        certified_or_supported_claimed: true,
        evidence_fresh: false,
        reads_over_budget_as_cost_free: false,
        proof_fresh: true,
    })
}

// -- Clean installed-state diagnostics card examples --------------------------------------------

/// Clean card: healthy, within budget, not quarantined, offering the disable / retry pair.
fn card_healthy_clean() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![
            M5ActivationTriggerClass::OnStartup,
            M5ActivationTriggerClass::OnCommand,
        ],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
            M5DiagnosticsRemediationAction::ViewLogs,
        ],
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Clean card: throttled with an activation-budget reason and the disable / retry pair.
fn card_throttled_with_reason_clean() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:heavy-tool".to_owned(),
        artifact_identity: "heavy-tool".to_owned(),
        budget_state: M5ActivationBudgetBandState::Throttled,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::CompatibleWithWarnings,
        activation_triggers: vec![
            M5ActivationTriggerClass::OnStartup,
            M5ActivationTriggerClass::OnLanguageEvent,
        ],
        exercised_capabilities: vec![
            M5ExercisedCapabilityClass::FileSystemRead,
            M5ExercisedCapabilityClass::Network,
        ],
        throttle_quarantine_reason: Some(M5ThrottleQuarantineReason::ActivationBudgetExceeded),
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
            M5DiagnosticsRemediationAction::DisableGlobal,
            M5DiagnosticsRemediationAction::ViewLogs,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Clean card: quarantined with a crash reason and the disable / retry pair plus release.
fn card_quarantined_with_reason_clean() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:crash-tool".to_owned(),
        artifact_identity: "crash-tool".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::QuarantinedActive,
        compatibility: M5CompatibilityState::DegradedHost,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![
            M5ExercisedCapabilityClass::ProcessSpawn,
            M5ExercisedCapabilityClass::Network,
        ],
        throttle_quarantine_reason: Some(M5ThrottleQuarantineReason::RepeatedCrashes),
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
            M5DiagnosticsRemediationAction::ReleaseQuarantine,
            M5DiagnosticsRemediationAction::ReportIssue,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Clean card: released from a prior quarantine, healthy now, history stays explicit.
fn card_released_history_clean() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:recovered-tool".to_owned(),
        artifact_identity: "recovered-tool".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::ReleasedFromQuarantine,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnCommand],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: true,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

// -- Degraded installed-state diagnostics card examples -----------------------------------------

/// Degraded card: the artifact identity is unstated.
fn card_identity_unstated() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Degraded card: the activation-budget band cannot be resolved.
fn card_budget_unresolved() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:budget-unresolved".to_owned(),
        artifact_identity: "budgetless-installed-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::BudgetUnknown,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Degraded card: the quarantine state cannot be resolved.
fn card_quarantine_unresolved() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:quarantine-unresolved".to_owned(),
        artifact_identity: "quarantine-unknown-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::QuarantineUnknown,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Degraded card: the activation triggers are unstated.
fn card_triggers_unstated() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:triggers-unstated".to_owned(),
        artifact_identity: "triggerless-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Degraded card: the exercised-capability summary is unstated.
fn card_capabilities_unstated() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:capabilities-unstated".to_owned(),
        artifact_identity: "capability-less-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Degraded card: a quarantined artifact reads as healthy.
fn card_quarantine_hidden() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:quarantine-hidden".to_owned(),
        artifact_identity: "hidden-quarantine-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::QuarantinedActive,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::Network],
        throttle_quarantine_reason: Some(M5ThrottleQuarantineReason::ManualQuarantine),
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
            M5DiagnosticsRemediationAction::ReleaseQuarantine,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: true,
        proof_fresh: true,
    })
}

/// Degraded card: a throttled artifact carries no reason.
fn card_reason_missing() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:reason-missing".to_owned(),
        artifact_identity: "reasonless-throttled-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::Throttled,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::Network],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Degraded card: a quarantined artifact offers a retry action without a disable action.
fn card_disable_retry_missing() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:disable-retry-missing".to_owned(),
        artifact_identity: "retry-only-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::QuarantinedActive,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::Network],
        throttle_quarantine_reason: Some(M5ThrottleQuarantineReason::PolicyViolation),
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::ViewLogs,
        ],
        certified_or_supported_claimed: false,
        evidence_fresh: true,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

/// Degraded card: Certified language is left in place on stale evidence.
fn card_stale_certified() -> M5ResolvedInstalledStateDiagnosticsCard {
    card(M5InstalledStateDiagnosticsCardResolutionInput {
        card_id: "diagnostics-card:stale-certified".to_owned(),
        artifact_identity: "stale-certified-installed-artifact".to_owned(),
        budget_state: M5ActivationBudgetBandState::WithinBudget,
        quarantine_state: M5QuarantineState::NotQuarantined,
        compatibility: M5CompatibilityState::Compatible,
        activation_triggers: vec![M5ActivationTriggerClass::OnStartup],
        exercised_capabilities: vec![M5ExercisedCapabilityClass::FileSystemRead],
        throttle_quarantine_reason: None,
        remediation_actions: vec![
            M5DiagnosticsRemediationAction::RetryActivation,
            M5DiagnosticsRemediationAction::DisableWorkspace,
        ],
        certified_or_supported_claimed: true,
        evidence_fresh: false,
        reads_quarantine_as_healthy: false,
        proof_fresh: true,
    })
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ActivationDiagnosticsConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    activation_budget_band_examples: Vec<M5ResolvedActivationBudgetBand>,
    installed_state_diagnostics_card_examples: Vec<M5ResolvedInstalledStateDiagnosticsCard>,
) -> M5ActivationDiagnosticsControlsRow {
    M5ActivationDiagnosticsControlsRow {
        consumer_surface,
        qualification: M5MarketplaceInstallQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5MarketplaceInstallDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5MarketplaceInstallRequiredLabel::Identity,
            M5MarketplaceInstallRequiredLabel::State,
            M5MarketplaceInstallRequiredLabel::KeyboardRoute,
            M5MarketplaceInstallRequiredLabel::CompatibilityAndHost,
            M5MarketplaceInstallRequiredLabel::PermissionAndBudget,
            M5MarketplaceInstallRequiredLabel::PublisherAndSourceClass,
        ],
        accessibility_routes: M5MarketplaceInstallAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ActivationDiagnosticsAnatomyPart::ALL.to_vec(),
        export_fields: M5ActivationDiagnosticsExportField::ALL.to_vec(),
        downgrade_triggers,
        activation_budget_band_examples,
        installed_state_diagnostics_card_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_REF,
            M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
            M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF,
        ]),
        hides_activation_cost_or_over_budget_band: false,
        hides_throttling_or_quarantine_reason: false,
        collapses_disable_and_retry_into_generic_action: false,
        leaves_stale_evidence_certified_or_supported: false,
    }
}

fn controls_rows() -> Vec<M5ActivationDiagnosticsControlsRow> {
    use M5MarketplaceInstallConsumerSurface as C;
    use M5MarketplaceInstallDowngradeTrigger as D;

    vec![
        base_row(
            C::MarketplaceUi,
            "Marketplace catalog owner",
            "The marketplace listing renders one activation-budget band per artifact naming the low / medium / high / over-budget class with cold / warm evidence, and one installed-state diagnostics card naming activation triggers, exercised capabilities, and disable / retry actions, so a performance decision needs no log dig",
            "evidence:m5-activation-diagnostics-marketplace-ui:001",
            vec![
                D::ActivationCostHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![band_within_low_clean(), band_evidence_missing_after_degradation()],
            vec![card_healthy_clean(), card_triggers_unstated()],
        ),
        base_row(
            C::ExtensionsUi,
            "Extensions manager owner",
            "The extensions detail surface reuses the same budget grammar, shows a throttled artifact carrying its activation-budget-exceeded reason and the disable / retry pair, and degrades honestly when an over-budget band reads as cost-free or a throttled card carries no reason",
            "evidence:m5-activation-diagnostics-extensions-ui:001",
            vec![
                D::ActivationCostHidden,
                D::QuarantineHistoryHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![band_near_medium_clean(), band_over_budget_cost_free()],
            vec![card_throttled_with_reason_clean(), card_reason_missing()],
        ),
        base_row(
            C::InstallReviewUi,
            "Install-review owner",
            "The install-review sheet keeps activation cost legible before install, shows a quarantined artifact carrying its crash reason and the disable / retry pair, and degrades honestly when the budget band is unresolved or the disable / retry pair is broken",
            "evidence:m5-activation-diagnostics-install-review-ui:001",
            vec![
                D::ActivationCostHidden,
                D::QuarantineHistoryHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![band_over_with_evidence_clean(), band_unknown()],
            vec![card_quarantined_with_reason_clean(), card_disable_retry_missing()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved band and card truth, so an over-budget-cost-free band, an evidence-missing band, a hidden quarantine, or a stale Certified overclaim is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-activation-diagnostics-support-export:001",
            vec![
                D::ActivationCostHidden,
                D::QuarantineHistoryHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                band_within_high_warm_clean(),
                band_stale_certified(),
                band_identity_unstated(),
            ],
            vec![
                card_quarantine_hidden(),
                card_stale_certified(),
                card_budget_unresolved(),
                card_capabilities_unstated(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product diagnostics owner",
            "In-product listing and diagnostics surfaces reuse the same budget and quarantine grammar, keep a released-from-quarantine artifact's history explicit, and degrade honestly when the quarantine state is unresolved so no stale trust is quietly carried forward",
            "evidence:m5-activation-diagnostics-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::ActivationCostHidden,
                D::ProofStale,
            ],
            vec![band_within_low_clean(), band_identity_unstated()],
            vec![
                card_released_history_clean(),
                card_quarantine_unresolved(),
                card_identity_unstated(),
            ],
        ),
    ]
}

fn governance_review() -> M5ActivationDiagnosticsGovernanceReview {
    M5ActivationDiagnosticsGovernanceReview {
        band_names_budget_class_and_cold_warm_evidence: true,
        over_budget_never_cost_free: true,
        card_names_triggers_and_exercised_capabilities: true,
        card_names_throttle_quarantine_reason_where_applicable: true,
        quarantine_history_always_explicit: true,
        disable_retry_pair_always_intact: true,
        implications_legible_without_logs: true,
        stale_evidence_never_leaves_certified_language: true,
        budget_and_quarantine_language_aligned_across_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ActivationDiagnosticsConsumerProjection {
    M5ActivationDiagnosticsConsumerProjection {
        marketplace_surfaces_consume_activation_budget_vocabulary: true,
        diagnostics_surfaces_consume_budget_and_quarantine_vocabulary: true,
        facts_trace_to_single_component_contract: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ActivationDiagnosticsProofFreshness {
    M5ActivationDiagnosticsProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ActivationDiagnosticsReleasePosture {
    M5ActivationDiagnosticsReleasePosture {
        proof_packet_ref: M5_ACTIVATION_DIAGNOSTICS_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_ACTIVATION_DIAGNOSTICS_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_REF,
        M5_ACTIVATION_DIAGNOSTICS_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
        M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 activation-budget-band / installed-state-diagnostics-card controls packet.
pub fn seeded_m5_activation_diagnostics_controls() -> M5ActivationDiagnosticsControlsPacket {
    M5ActivationDiagnosticsControlsPacket::new(M5ActivationDiagnosticsControlsPacketInput {
        packet_id: M5_ACTIVATION_DIAGNOSTICS_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 activation-budget-band and installed-state-diagnostics-card controls with cold/warm activation buckets, low/medium/high/over-budget classes, activation triggers, exercised capabilities, throttling/quarantine reasons, and disable/retry parity across marketplace, install, diagnostics, help, and export"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5ActivationDiagnosticsVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the marketplace-UI row is held at Beta pending activation-budget parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_activation_diagnostics_controls_marketplace_ui_beta_narrowed(
) -> M5ActivationDiagnosticsControlsPacket {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.packet_id =
        "m5-activation-budget-band-installed-state-diagnostics-card-controls:marketplace-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .expect("marketplace-ui row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Beta;
    packet
}

/// Narrowed variant: the install-review row is narrowed to Preview pending disable / retry parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_activation_diagnostics_controls_installed_state_ui_preview_narrowed(
) -> M5ActivationDiagnosticsControlsPacket {
    let mut packet = seeded_m5_activation_diagnostics_controls();
    packet.packet_id =
        "m5-activation-budget-band-installed-state-diagnostics-card-controls:install-review-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::InstallReviewUi)
        .expect("install-review row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Preview;
    packet
}
