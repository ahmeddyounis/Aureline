//! Canonical seed builders for the M5 trust-fact-grid / trust-elevation-sheet controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_PACKET_ID: &str =
    "m5-trust-fact-grid-trust-elevation-sheet-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn grid(input: M5TrustFactGridResolutionInput) -> M5ResolvedTrustFactGrid {
    resolve_trust_fact_grid(input).expect("seed trust-fact grid input resolves")
}

fn sheet(input: M5TrustElevationSheetResolutionInput) -> M5ResolvedTrustElevationSheet {
    resolve_trust_elevation_sheet(input).expect("seed trust-elevation sheet input resolves")
}

// -- Canonical trust-fact grid examples --------------------------------------------------------

/// Clean grid for a fully trusted workspace.
fn grid_trusted_workspace_clean() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:trusted-workspace".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean grid for a restricted workspace naming its narrowed capability.
fn grid_restricted_clean() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:restricted".to_owned(),
        actor_identity: "actor: workspace-config".to_owned(),
        object_identity: "workspace: untrusted-clone".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        root_trust: M5RootTrustState::RootRestricted,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::ExecutionBlocked,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean grid for a mixed-root workspace kept explicit, never uniform.
fn grid_mixed_root_clean() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:mixed-root".to_owned(),
        actor_identity: "actor: inherited-parent".to_owned(),
        object_identity: "workspace: multi-root".to_owned(),
        trust_scope: M5TrustScopeState::MixedRoot,
        root_trust: M5RootTrustState::RootMixedChildren,
        grant_source: M5TrustGrantSourceClass::InheritedParent,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::ReducedMode,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean grid for a policy-blocked workspace naming its policy epoch.
fn grid_policy_blocked_clean() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:policy-blocked".to_owned(),
        actor_identity: "actor: org-policy".to_owned(),
        object_identity: "workspace: managed-app".to_owned(),
        trust_scope: M5TrustScopeState::PolicyBlocked,
        root_trust: M5RootTrustState::RootPolicyBlocked,
        grant_source: M5TrustGrantSourceClass::PolicyManaged,
        grant_actor_stated: true,
        policy_epoch: "org-policy epoch 2026-07".to_owned(),
        capability_narrow: M5CapabilityNarrowState::TaskBlocked,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded grid: the trusted object identity is unstated.
fn grid_object_unstated() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:no-object".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "  ".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded grid: the actor is unstated.
fn grid_actor_unstated() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:no-actor".to_owned(),
        actor_identity: "".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded grid: the trust scope cannot be resolved.
fn grid_scope_unknown() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:scope-unknown".to_owned(),
        actor_identity: "actor: first-party-default".to_owned(),
        object_identity: "workspace: pending".to_owned(),
        trust_scope: M5TrustScopeState::ScopeUnknown,
        root_trust: M5RootTrustState::RootUnknown,
        grant_source: M5TrustGrantSourceClass::FirstPartyDefault,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::CapabilityUnknown,
        capability_narrow_stated: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded grid: the grant source is undisclosed.
fn grid_grant_unstated() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:grant-unstated".to_owned(),
        actor_identity: "actor: unknown".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::GrantSourceUnknown,
        grant_actor_stated: false,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded grid: a narrowed capability is left unnamed.
fn grid_capability_unstated() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:capability-unstated".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: reduced".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::ExtensionBlocked,
        capability_narrow_stated: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded grid: a mixed-root workspace reads as uniform (blanket) trust.
fn grid_mixed_root_collapsed() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:mixed-root-collapsed".to_owned(),
        actor_identity: "actor: inherited-parent".to_owned(),
        object_identity: "workspace: multi-root".to_owned(),
        trust_scope: M5TrustScopeState::MixedRoot,
        root_trust: M5RootTrustState::RootMixedChildren,
        grant_source: M5TrustGrantSourceClass::InheritedParent,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: true,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded grid: no command-backed trust-detail entrypoint is reachable.
fn grid_detail_missing() -> M5ResolvedTrustFactGrid {
    grid(M5TrustFactGridResolutionInput {
        grid_id: "trust-grid:detail-missing".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: false,
        proof_fresh: true,
    })
}

// -- Canonical trust-elevation sheet examples --------------------------------------------------

/// Clean sheet for a lasting workspace-scoped elevation.
fn sheet_trusted_workspace_lasting_clean() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:trusted-workspace-lasting".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean sheet for a one-time, single-root elevation — a trusted root is never presented as a
/// trusted workspace.
fn sheet_trusted_root_one_time_clean() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:trusted-root-one-time".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "root: /src".to_owned(),
        trust_scope: M5TrustScopeState::TrustedRoot,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::OneTimeThisSession,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean sheet for a restricted workspace naming its capability delta and a single-action effect.
fn sheet_restricted_clean() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:restricted".to_owned(),
        actor_identity: "actor: workspace-config".to_owned(),
        object_identity: "workspace: untrusted-clone".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::ExecutionBlocked,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::SingleActionOnly,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean sheet for a mixed-root elevation kept explicit.
fn sheet_mixed_root_clean() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:mixed-root".to_owned(),
        actor_identity: "actor: inherited-parent".to_owned(),
        object_identity: "workspace: multi-root".to_owned(),
        trust_scope: M5TrustScopeState::MixedRoot,
        grant_source: M5TrustGrantSourceClass::InheritedParent,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::ReducedMode,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: the actor is unstated.
fn sheet_actor_unstated() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:no-actor".to_owned(),
        actor_identity: "".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: the grant source is undisclosed.
fn sheet_grant_unstated() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:grant-unstated".to_owned(),
        actor_identity: "actor: unknown".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::GrantSourceUnknown,
        grant_actor_stated: false,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: a policy-managed grant hides its policy epoch.
fn sheet_policy_epoch_missing() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:epoch-missing".to_owned(),
        actor_identity: "actor: org-policy".to_owned(),
        object_identity: "workspace: managed-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::PolicyManaged,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: the capability delta a grant would change is not named.
fn sheet_capability_delta_missing() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:capability-delta-missing".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: reduced".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::ExtensionBlocked,
        capability_delta_stated: false,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: the reduced-mode alternative (what still works without trust) is not named.
fn sheet_reduced_mode_missing() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:reduced-mode-missing".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: false,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: the lasting-versus-one-time effect duration is not named.
fn sheet_effect_unknown() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:effect-unknown".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::EffectUnknown,
        implies_ambient_grant: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: the approval copy implies an ambient / inherited grant beyond the reviewed
/// object and scope.
fn sheet_ambient_scope() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:ambient-scope".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "root: /src".to_owned(),
        trust_scope: M5TrustScopeState::TrustedRoot,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: true,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded sheet: no command-backed trust-detail entrypoint is reachable before approval.
fn sheet_detail_missing() -> M5ResolvedTrustElevationSheet {
    sheet(M5TrustElevationSheetResolutionInput {
        sheet_id: "elevation-sheet:detail-missing".to_owned(),
        actor_identity: "actor: alice@team".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_delta_stated: true,
        reduced_mode_alternative_stated: true,
        effect_class: M5TrustElevationEffectClass::LastingUntilRevoked,
        implies_ambient_grant: false,
        detail_command_available: false,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5TrustFactGridElevationConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    trust_fact_grid_examples: Vec<M5ResolvedTrustFactGrid>,
    trust_elevation_sheet_examples: Vec<M5ResolvedTrustElevationSheet>,
) -> M5TrustFactGridElevationControlsRow {
    M5TrustFactGridElevationControlsRow {
        consumer_surface,
        qualification: M5WorkspaceTrustRepairQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5WorkspaceTrustRepairDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5WorkspaceTrustRepairRequiredLabel::Identity,
            M5WorkspaceTrustRepairRequiredLabel::State,
            M5WorkspaceTrustRepairRequiredLabel::KeyboardRoute,
            M5WorkspaceTrustRepairRequiredLabel::GrantSourceAndScope,
            M5WorkspaceTrustRepairRequiredLabel::CapabilityAndRootScope,
        ],
        accessibility_routes: M5WorkspaceTrustRepairAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5TrustFactGridElevationAnatomyPart::ALL.to_vec(),
        export_fields: M5TrustFactGridElevationExportField::ALL.to_vec(),
        downgrade_triggers,
        trust_fact_grid_examples,
        trust_elevation_sheet_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_REF,
            M5_TRUST_FACT_GRID_SCHEMA_REF,
            M5_TRUST_ELEVATION_SHEET_SCHEMA_REF,
        ]),
        implies_ambient_or_inherited_grant_beyond_reviewed_object: false,
        hides_policy_source_or_capability_delta_in_menus_only: false,
        collapses_reduced_mode_alternative_into_generic_chrome: false,
        collapses_effect_duration_into_generic_grant: false,
    }
}

fn controls_rows() -> Vec<M5TrustFactGridElevationControlsRow> {
    use M5WorkspaceTrustRepairConsumerSurface as C;
    use M5WorkspaceTrustRepairDowngradeTrigger as D;

    vec![
        base_row(
            C::WorkspaceTrustUi,
            "Workspace trust owner",
            "The workspace-trust UI renders one trust-fact grid naming actor, object, scope, policy source, capability, and per-root trust, and one elevation sheet reviewing a workspace-scoped and a root-scoped grant before approval so a trusted root never reads as a trusted workspace",
            "evidence:m5-trust-fact-grid-elevation-workspace-trust-ui:001",
            vec![
                D::RootScopeCollapsedIntoBlanketTrust,
                D::MixedRootShownAsUniformTrust,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![grid_trusted_workspace_clean(), grid_mixed_root_clean()],
            vec![
                sheet_trusted_workspace_lasting_clean(),
                sheet_trusted_root_one_time_clean(),
            ],
        ),
        base_row(
            C::SettingsUi,
            "Settings trust owner",
            "The settings trust pane reuses the same field and delta grammar, names the capability delta a restricted grant changes, and degrades honestly when the capability delta is left unnamed",
            "evidence:m5-trust-fact-grid-elevation-settings-ui:001",
            vec![
                D::NarrowedCapabilityUnstated,
                D::GrantSourceUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![grid_restricted_clean(), grid_capability_unstated()],
            vec![sheet_restricted_clean(), sheet_capability_delta_missing()],
        ),
        base_row(
            C::SafeModeUi,
            "Safe mode owner",
            "Safe mode shows the policy-blocked grid with its policy epoch and a mixed-root elevation reviewing the reduced-mode alternative, degrading honestly when the trust scope cannot be resolved or the reduced-mode path is hidden",
            "evidence:m5-trust-fact-grid-elevation-safe-mode-ui:001",
            vec![
                D::PolicyEpochUnstated,
                D::GenericChromeWordingUsed,
                D::RootScopeCollapsedIntoBlanketTrust,
                D::ProofStale,
            ],
            vec![grid_policy_blocked_clean(), grid_scope_unknown()],
            vec![sheet_mixed_root_clean(), sheet_reduced_mode_missing()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved grid and sheet truth, so a mixed-root workspace collapsed into uniform trust, an approval implying ambient scope, or a missing trust-detail path is visible in evidence rather than hidden",
            "evidence:m5-trust-fact-grid-elevation-support-export:001",
            vec![
                D::MixedRootShownAsUniformTrust,
                D::RootScopeCollapsedIntoBlanketTrust,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![grid_mixed_root_collapsed(), grid_detail_missing()],
            vec![sheet_ambient_scope(), sheet_detail_missing()],
        ),
        base_row(
            C::ProductUi,
            "In-product trust owner",
            "In-product surfaces reuse the same field and delta grammar a user sees in the workspace-trust UI, always offering the command-backed detail path and degrading honestly when object, actor, grant source, or effect duration is unstated",
            "evidence:m5-trust-fact-grid-elevation-product-ui:001",
            vec![
                D::GrantSourceUnstated,
                D::GenericChromeWordingUsed,
                D::RootScopeCollapsedIntoBlanketTrust,
                D::ProofStale,
            ],
            vec![
                grid_trusted_workspace_clean(),
                grid_object_unstated(),
                grid_actor_unstated(),
                grid_grant_unstated(),
            ],
            vec![
                sheet_trusted_workspace_lasting_clean(),
                sheet_grant_unstated(),
                sheet_actor_unstated(),
                sheet_policy_epoch_missing(),
                sheet_effect_unknown(),
            ],
        ),
    ]
}

fn governance_review() -> M5TrustFactGridElevationGovernanceReview {
    M5TrustFactGridElevationGovernanceReview {
        grid_names_actor_object_and_scope: true,
        grid_names_policy_source_and_capability: true,
        elevation_sheet_names_reduced_mode_alternative: true,
        elevation_sheet_names_effect_duration: true,
        no_prompt_implies_ambient_grant_beyond_object: true,
        trust_detail_command_always_reachable: true,
        trust_vocabulary_shared_across_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5TrustFactGridElevationConsumerProjection {
    M5TrustFactGridElevationConsumerProjection {
        trust_prompts_expose_same_fields_and_delta_grammar: true,
        scope_and_source_inspectable_before_approval: true,
        elevation_traces_to_single_component_contract: true,
        support_export_reads_single_trust_source: true,
    }
}

fn proof_freshness() -> M5TrustFactGridElevationProofFreshness {
    M5TrustFactGridElevationProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TrustFactGridElevationReleasePosture {
    M5TrustFactGridElevationReleasePosture {
        proof_packet_ref: M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_REF,
        M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_TRUST_FACT_GRID_SCHEMA_REF,
        M5_TRUST_ELEVATION_SHEET_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 trust-fact-grid / trust-elevation-sheet controls packet.
pub fn seeded_m5_trust_fact_grid_elevation_controls() -> M5TrustFactGridElevationControlsPacket {
    M5TrustFactGridElevationControlsPacket::new(M5TrustFactGridElevationControlsPacketInput {
        packet_id: M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 trust-fact-grid and trust-elevation-sheet controls with actor, object, scope, policy source, capability delta, reduced-mode alternative, lasting-versus-one-time effect, and no-ambient-grant honesty"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5TrustFactGridElevationVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the workspace-trust-UI row is held at Beta pending field / delta parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_trust_fact_grid_elevation_controls_workspace_trust_ui_beta_narrowed(
) -> M5TrustFactGridElevationControlsPacket {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.packet_id =
        "m5-trust-fact-grid-trust-elevation-sheet-controls:workspace-trust-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi)
        .expect("workspace-trust-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Beta;
    packet
}

/// Narrowed variant: the safe-mode-UI row is narrowed to Preview pending reduced-mode parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_trust_fact_grid_elevation_controls_safe_mode_ui_preview_narrowed(
) -> M5TrustFactGridElevationControlsPacket {
    let mut packet = seeded_m5_trust_fact_grid_elevation_controls();
    packet.packet_id =
        "m5-trust-fact-grid-trust-elevation-sheet-controls:safe-mode-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .expect("safe-mode-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Preview;
    packet
}
