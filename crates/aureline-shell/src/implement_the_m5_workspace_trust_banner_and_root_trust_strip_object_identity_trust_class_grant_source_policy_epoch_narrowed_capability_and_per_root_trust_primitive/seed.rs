//! Canonical seed builders for the M5 workspace-trust-banner / root-trust-strip controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_PACKET_ID: &str =
    "m5-workspace-trust-banner-root-trust-strip-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn banner(input: M5WorkspaceTrustBannerResolutionInput) -> M5ResolvedWorkspaceTrustBanner {
    resolve_workspace_trust_banner(input).expect("seed workspace-trust banner input resolves")
}

fn strip(input: M5RootTrustStripResolutionInput) -> M5ResolvedRootTrustStrip {
    resolve_root_trust_strip(input).expect("seed root-trust strip input resolves")
}

// -- Canonical workspace-trust banner examples ------------------------------------------------

/// Clean banner for a fully trusted workspace.
fn banner_trusted_workspace_clean() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:trusted-workspace".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
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

/// Clean banner for a restricted workspace naming its narrowed capability.
fn banner_restricted_clean() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:restricted".to_owned(),
        object_identity: "workspace: untrusted-clone".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
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

/// Clean banner for a mixed-root workspace kept explicit, never uniform.
fn banner_mixed_root_clean() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:mixed-root".to_owned(),
        object_identity: "workspace: multi-root".to_owned(),
        trust_scope: M5TrustScopeState::MixedRoot,
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

/// Clean banner for a policy-blocked workspace naming its policy epoch.
fn banner_policy_blocked_clean() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:policy-blocked".to_owned(),
        object_identity: "workspace: managed-app".to_owned(),
        trust_scope: M5TrustScopeState::PolicyBlocked,
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

/// Degraded banner: the trust scope cannot be resolved.
fn banner_scope_unknown() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:scope-unknown".to_owned(),
        object_identity: "workspace: pending".to_owned(),
        trust_scope: M5TrustScopeState::ScopeUnknown,
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

/// Degraded banner: the grant source is undisclosed.
fn banner_grant_unstated() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:grant-unstated".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
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

/// Degraded banner: a policy-managed grant hides its policy epoch.
fn banner_policy_epoch_missing() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:epoch-missing".to_owned(),
        object_identity: "workspace: managed-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
        grant_source: M5TrustGrantSourceClass::PolicyManaged,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        capability_narrow: M5CapabilityNarrowState::FullCapability,
        capability_narrow_stated: true,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded banner: a narrowed capability is left unnamed.
fn banner_narrowed_capability_unstated() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:capability-unstated".to_owned(),
        object_identity: "workspace: reduced".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
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

/// Degraded banner: a mixed-root workspace reads as uniform (blanket) trust.
fn banner_mixed_root_collapsed() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:mixed-root-collapsed".to_owned(),
        object_identity: "workspace: multi-root".to_owned(),
        trust_scope: M5TrustScopeState::MixedRoot,
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

/// Degraded banner: no command-backed trust-detail entrypoint is reachable.
fn banner_detail_missing() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:detail-missing".to_owned(),
        object_identity: "workspace: acme-app".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
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

/// Degraded banner: the trusted object identity is unstated.
fn banner_object_identity_unstated() -> M5ResolvedWorkspaceTrustBanner {
    banner(M5WorkspaceTrustBannerResolutionInput {
        banner_id: "trust-banner:no-identity".to_owned(),
        object_identity: "  ".to_owned(),
        trust_scope: M5TrustScopeState::TrustedWorkspace,
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

// -- Canonical root-trust strip examples ------------------------------------------------------

/// Clean strip for a trusted root.
fn strip_root_trusted_clean() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:trusted".to_owned(),
        root_identity: "root: /src".to_owned(),
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for a restricted root.
fn strip_root_restricted_clean() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:restricted".to_owned(),
        root_identity: "root: /vendor".to_owned(),
        root_trust: M5RootTrustState::RootRestricted,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for a root with mixed children, kept explicit.
fn strip_root_mixed_children_clean() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:mixed-children".to_owned(),
        root_identity: "root: /packages".to_owned(),
        root_trust: M5RootTrustState::RootMixedChildren,
        grant_source: M5TrustGrantSourceClass::InheritedParent,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for a policy-blocked root naming its policy epoch.
fn strip_root_policy_blocked_clean() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:policy-blocked".to_owned(),
        root_identity: "root: /managed".to_owned(),
        root_trust: M5RootTrustState::RootPolicyBlocked,
        grant_source: M5TrustGrantSourceClass::PolicyManaged,
        grant_actor_stated: true,
        policy_epoch: "org-policy epoch 2026-07".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: the per-root trust cannot be resolved.
fn strip_root_unknown() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:unknown".to_owned(),
        root_identity: "root: /pending".to_owned(),
        root_trust: M5RootTrustState::RootUnknown,
        grant_source: M5TrustGrantSourceClass::FirstPartyDefault,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: the grant source is undisclosed.
fn strip_grant_unstated() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:grant-unstated".to_owned(),
        root_identity: "root: /src".to_owned(),
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::GrantSourceUnknown,
        grant_actor_stated: false,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: a per-root trust reads as uniform with its siblings.
fn strip_collapsed() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:collapsed".to_owned(),
        root_identity: "root: /vendor".to_owned(),
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: true,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: no command-backed trust-detail entrypoint is reachable.
fn strip_detail_missing() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:detail-missing".to_owned(),
        root_identity: "root: /src".to_owned(),
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: false,
        proof_fresh: true,
    })
}

/// Degraded strip: the root identity is unstated.
fn strip_root_identity_unstated() -> M5ResolvedRootTrustStrip {
    strip(M5RootTrustStripResolutionInput {
        strip_id: "root-strip:no-identity".to_owned(),
        root_identity: "".to_owned(),
        root_trust: M5RootTrustState::RootTrusted,
        grant_source: M5TrustGrantSourceClass::UserExplicit,
        grant_actor_stated: true,
        policy_epoch: "".to_owned(),
        part_of_mixed_root: true,
        reads_as_uniform_with_siblings: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5WorkspaceTrustRootConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    workspace_trust_banner_examples: Vec<M5ResolvedWorkspaceTrustBanner>,
    root_trust_strip_examples: Vec<M5ResolvedRootTrustStrip>,
) -> M5WorkspaceTrustRootControlsRow {
    M5WorkspaceTrustRootControlsRow {
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
        anatomy_parts: M5WorkspaceTrustRootAnatomyPart::ALL.to_vec(),
        export_fields: M5WorkspaceTrustRootExportField::ALL.to_vec(),
        downgrade_triggers,
        workspace_trust_banner_examples,
        root_trust_strip_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_REF,
            M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
            M5_ROOT_TRUST_STRIP_SCHEMA_REF,
        ]),
        implies_blanket_trust_across_roots_or_routes: false,
        hides_grant_source_or_policy_epoch_in_menus_only: false,
        collapses_mixed_root_into_uniform_trust: false,
        hides_narrowed_capability_behind_generic_chrome: false,
    }
}

fn controls_rows() -> Vec<M5WorkspaceTrustRootControlsRow> {
    use M5WorkspaceTrustRepairConsumerSurface as C;
    use M5WorkspaceTrustRepairDowngradeTrigger as D;

    vec![
        base_row(
            C::WorkspaceTrustUi,
            "Workspace trust owner",
            "The workspace-trust UI renders one banner naming the trusted object, trust class, grant source, and policy epoch, and one root-trust strip per root so a mixed-root workspace never reads as blanket trust",
            "evidence:m5-workspace-trust-root-workspace-trust-ui:001",
            vec![
                D::GrantSourceUnstated,
                D::MixedRootShownAsUniformTrust,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![banner_trusted_workspace_clean(), banner_mixed_root_clean()],
            vec![strip_root_trusted_clean(), strip_root_mixed_children_clean()],
        ),
        base_row(
            C::SettingsUi,
            "Settings trust owner",
            "The settings trust pane reuses the same trust/root vocabulary, names the narrowed capability a restricted workspace removes, and degrades honestly when the grant source is undisclosed",
            "evidence:m5-workspace-trust-root-settings-ui:001",
            vec![
                D::NarrowedCapabilityUnstated,
                D::GrantSourceUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![banner_restricted_clean(), banner_narrowed_capability_unstated()],
            vec![strip_root_restricted_clean(), strip_grant_unstated()],
        ),
        base_row(
            C::SafeModeUi,
            "Safe mode owner",
            "Safe mode shows the policy-blocked banner with its policy epoch and the per-root trust strip, degrading honestly when a policy epoch or per-root trust cannot be resolved",
            "evidence:m5-workspace-trust-root-safe-mode-ui:001",
            vec![
                D::PolicyEpochUnstated,
                D::RootScopeCollapsedIntoBlanketTrust,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![banner_policy_blocked_clean(), banner_policy_epoch_missing()],
            vec![strip_root_policy_blocked_clean(), strip_root_unknown()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved banner and strip truth, so a mixed-root workspace collapsed into uniform trust or a missing trust-detail path is visible in evidence rather than hidden",
            "evidence:m5-workspace-trust-root-support-export:001",
            vec![
                D::MixedRootShownAsUniformTrust,
                D::RootScopeCollapsedIntoBlanketTrust,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![banner_mixed_root_collapsed(), banner_scope_unknown()],
            vec![strip_collapsed(), strip_detail_missing()],
        ),
        base_row(
            C::ProductUi,
            "In-product trust owner",
            "In-product surfaces reuse the same trust/root vocabulary a user sees in the workspace-trust UI, always offering the command-backed detail path and degrading honestly when object or root identity is unstated",
            "evidence:m5-workspace-trust-root-product-ui:001",
            vec![
                D::GrantSourceUnstated,
                D::GenericChromeWordingUsed,
                D::RootScopeCollapsedIntoBlanketTrust,
                D::ProofStale,
            ],
            vec![
                banner_trusted_workspace_clean(),
                banner_detail_missing(),
                banner_object_identity_unstated(),
                banner_grant_unstated(),
            ],
            vec![strip_root_trusted_clean(), strip_root_identity_unstated()],
        ),
    ]
}

fn governance_review() -> M5WorkspaceTrustRootGovernanceReview {
    M5WorkspaceTrustRootGovernanceReview {
        banner_names_object_identity_and_trust_class: true,
        banner_names_grant_source_and_policy_epoch: true,
        root_strip_names_per_root_trust: true,
        mixed_root_always_explicit_never_uniform: true,
        narrowed_capability_always_named: true,
        trust_detail_command_always_reachable: true,
        trust_vocabulary_shared_across_shell_and_workspace: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5WorkspaceTrustRootConsumerProjection {
    M5WorkspaceTrustRootConsumerProjection {
        shell_surfaces_consume_trust_scope_vocabulary: true,
        workspace_surfaces_consume_root_trust_vocabulary: true,
        trust_detail_traces_to_single_component_contract: true,
        support_export_reads_single_trust_source: true,
    }
}

fn proof_freshness() -> M5WorkspaceTrustRootProofFreshness {
    M5WorkspaceTrustRootProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WorkspaceTrustRootReleasePosture {
    M5WorkspaceTrustRootReleasePosture {
        proof_packet_ref: M5_WORKSPACE_TRUST_ROOT_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_WORKSPACE_TRUST_ROOT_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_REF,
        M5_WORKSPACE_TRUST_ROOT_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
        M5_ROOT_TRUST_STRIP_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 workspace-trust-banner / root-trust-strip controls packet.
pub fn seeded_m5_workspace_trust_root_controls() -> M5WorkspaceTrustRootControlsPacket {
    M5WorkspaceTrustRootControlsPacket::new(M5WorkspaceTrustRootControlsPacketInput {
        packet_id: M5_WORKSPACE_TRUST_ROOT_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 workspace-trust-banner and root-trust-strip controls with object identity, trust class, grant source, policy epoch, narrowed-capability, per-root trust, and mixed-root honesty"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5WorkspaceTrustRootVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the workspace-trust-UI row is held at Beta pending trust/root parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_workspace_trust_root_controls_workspace_trust_ui_beta_narrowed(
) -> M5WorkspaceTrustRootControlsPacket {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.packet_id =
        "m5-workspace-trust-banner-root-trust-strip-controls:workspace-trust-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi)
        .expect("workspace-trust-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Beta;
    packet
}

/// Narrowed variant: the safe-mode-UI row is narrowed to Preview pending root-trust-strip parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_workspace_trust_root_controls_safe_mode_ui_preview_narrowed(
) -> M5WorkspaceTrustRootControlsPacket {
    let mut packet = seeded_m5_workspace_trust_root_controls();
    packet.packet_id =
        "m5-workspace-trust-banner-root-trust-strip-controls:safe-mode-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .expect("safe-mode-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Preview;
    packet
}
