//! Canonical seed builders for the M5 restricted-capability-row / narrowed-capability-summary
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_PACKET_ID: &str =
    "m5-restricted-capability-row-narrowed-capability-summary-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn row(input: M5RestrictedCapabilityRowResolutionInput) -> M5ResolvedRestrictedCapabilityRow {
    resolve_restricted_capability_row(input).expect("seed restricted-capability row input resolves")
}

fn summary(
    input: M5NarrowedCapabilitySummaryResolutionInput,
) -> M5ResolvedNarrowedCapabilitySummary {
    resolve_narrowed_capability_summary(input).expect("seed narrowed-capability summary resolves")
}

use M5RestrictedActionFamily as F;

// -- Canonical restricted-capability row examples ----------------------------------------------

/// Clean row for a restricted workspace naming its narrowed capability, blocked families, and
/// still-safe actions.
fn row_restricted_clean() -> M5ResolvedRestrictedCapabilityRow {
    row(M5RestrictedCapabilityRowResolutionInput {
        row_id: "restricted-row:restricted-workspace".to_owned(),
        object_identity: "workspace: untrusted-clone".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        root_trust: M5RootTrustState::RootRestricted,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::ExecutionBlocked,
        capability_narrow_stated: true,
        blocked_action_families: vec![F::CodeExecution, F::TaskAutomation, F::DebuggerAttach],
        still_safe_actions: vec![F::ReadOnlyNavigation, F::FileEditing],
        approval_allowed: true,
        reads_as_generic_unavailable: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean row for a policy-blocked object naming its policy-managed source; approval is not allowed.
fn row_policy_blocked_clean() -> M5ResolvedRestrictedCapabilityRow {
    row(M5RestrictedCapabilityRowResolutionInput {
        row_id: "restricted-row:policy-blocked".to_owned(),
        object_identity: "workspace: managed-app".to_owned(),
        trust_scope: M5TrustScopeState::PolicyBlocked,
        root_trust: M5RootTrustState::RootPolicyBlocked,
        grant_source: M5TrustGrantSourceClass::PolicyManaged,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::ExtensionBlocked,
        capability_narrow_stated: true,
        blocked_action_families: vec![F::ExtensionActivation, F::OutboundRequests],
        still_safe_actions: vec![F::ReadOnlyNavigation],
        approval_allowed: false,
        reads_as_generic_unavailable: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean row for a mixed-root workspace kept explicit, never uniform.
fn row_mixed_root_clean() -> M5ResolvedRestrictedCapabilityRow {
    row(M5RestrictedCapabilityRowResolutionInput {
        row_id: "restricted-row:mixed-root".to_owned(),
        object_identity: "workspace: multi-root".to_owned(),
        trust_scope: M5TrustScopeState::MixedRoot,
        root_trust: M5RootTrustState::RootMixedChildren,
        grant_source: M5TrustGrantSourceClass::InheritedParent,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::ReducedMode,
        capability_narrow_stated: true,
        blocked_action_families: vec![F::CodeExecution, F::WorkspaceSettingsWrite],
        still_safe_actions: vec![F::ReadOnlyNavigation, F::FileEditing],
        approval_allowed: true,
        reads_as_generic_unavailable: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean row for a task-blocked reduced mode.
fn row_task_blocked_clean() -> M5ResolvedRestrictedCapabilityRow {
    row(M5RestrictedCapabilityRowResolutionInput {
        row_id: "restricted-row:task-blocked".to_owned(),
        object_identity: "workspace: safe-mode-session".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        root_trust: M5RootTrustState::RootRestricted,
        grant_source: M5TrustGrantSourceClass::FirstPartyDefault,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::TaskBlocked,
        capability_narrow_stated: true,
        blocked_action_families: vec![F::TaskAutomation],
        still_safe_actions: vec![F::ReadOnlyNavigation, F::FileEditing],
        approval_allowed: true,
        reads_as_generic_unavailable: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: the restricted object identity is unstated.
fn row_object_unstated() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:no-object".to_owned();
    input.object_identity = "  ".to_owned();
    row(input)
}

/// Degraded row: the restriction scope cannot be resolved.
fn row_scope_unknown() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:scope-unknown".to_owned();
    input.trust_scope = M5TrustScopeState::ScopeUnknown;
    row(input)
}

/// Degraded row: the grant source that imposed the restriction is undisclosed.
fn row_source_unstated() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:source-unstated".to_owned();
    input.grant_source = M5TrustGrantSourceClass::GrantSourceUnknown;
    row(input)
}

/// Degraded row: why the restriction exists is not stated.
fn row_reason_unstated() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:reason-unstated".to_owned();
    input.restriction_reason_stated = false;
    row(input)
}

/// Degraded row: the narrowed capability is left unnamed.
fn row_capability_unstated() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:capability-unstated".to_owned();
    input.capability_narrow = M5CapabilityNarrowState::ExtensionBlocked;
    input.capability_narrow_stated = false;
    row(input)
}

/// Degraded row: no blocked action family is enumerated.
fn row_blocked_unstated() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:blocked-unstated".to_owned();
    input.blocked_action_families = Vec::new();
    row(input)
}

/// Degraded row: no still-safe action is named.
fn row_still_safe_unstated() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:still-safe-unstated".to_owned();
    input.still_safe_actions = Vec::new();
    row(input)
}

/// Degraded row: the restriction collapses into generic "unavailable" copy.
fn row_generic_unavailable() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:generic-unavailable".to_owned();
    input.reads_as_generic_unavailable = true;
    row(input)
}

/// Degraded row: a mixed-root restriction reads as uniform across roots.
fn row_mixed_root_collapsed() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:mixed-root-collapsed".to_owned();
    input.object_identity = "workspace: multi-root".to_owned();
    input.trust_scope = M5TrustScopeState::MixedRoot;
    input.root_trust = M5RootTrustState::RootMixedChildren;
    input.reads_as_uniform_trust = true;
    row(input)
}

/// Degraded row: no command-backed recovery path is reachable.
fn row_recovery_missing() -> M5ResolvedRestrictedCapabilityRow {
    let mut input = clean_restricted_input();
    input.row_id = "restricted-row:recovery-missing".to_owned();
    input.detail_command_available = false;
    row(input)
}

/// A fully valid restricted-workspace row input the degrade builders mutate a single field of.
fn clean_restricted_input() -> M5RestrictedCapabilityRowResolutionInput {
    M5RestrictedCapabilityRowResolutionInput {
        row_id: "restricted-row:clean".to_owned(),
        object_identity: "workspace: untrusted-clone".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        root_trust: M5RootTrustState::RootRestricted,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::ExecutionBlocked,
        capability_narrow_stated: true,
        blocked_action_families: vec![F::CodeExecution, F::TaskAutomation],
        still_safe_actions: vec![F::ReadOnlyNavigation, F::FileEditing],
        approval_allowed: true,
        reads_as_generic_unavailable: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

// -- Canonical narrowed-capability summary examples --------------------------------------------

/// A fully valid restricted-workspace summary input the builders mutate a single field of.
fn clean_summary_input() -> M5NarrowedCapabilitySummaryResolutionInput {
    M5NarrowedCapabilitySummaryResolutionInput {
        summary_id: "narrowed-summary:clean".to_owned(),
        object_identity: "workspace: untrusted-clone".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::ExecutionBlocked,
        capability_narrow_stated: true,
        blocked_action_families: vec![F::CodeExecution, F::TaskAutomation],
        still_safe_actions: vec![F::ReadOnlyNavigation, F::FileEditing],
        approval_allowed: true,
        reads_as_generic_unavailable: false,
        collapses_blocked_families: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean summary for a restricted workspace.
fn summary_restricted_clean() -> M5ResolvedNarrowedCapabilitySummary {
    summary(clean_summary_input())
}

/// Clean summary for a policy-blocked object.
fn summary_policy_blocked_clean() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:policy-blocked".to_owned();
    input.object_identity = "workspace: managed-app".to_owned();
    input.trust_scope = M5TrustScopeState::PolicyBlocked;
    input.grant_source = M5TrustGrantSourceClass::PolicyManaged;
    input.capability_narrow = M5CapabilityNarrowState::ExtensionBlocked;
    input.blocked_action_families = vec![F::ExtensionActivation, F::OutboundRequests];
    input.still_safe_actions = vec![F::ReadOnlyNavigation];
    input.approval_allowed = false;
    summary(input)
}

/// Clean summary for a mixed-root workspace.
fn summary_mixed_root_clean() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:mixed-root".to_owned();
    input.object_identity = "workspace: multi-root".to_owned();
    input.trust_scope = M5TrustScopeState::MixedRoot;
    input.grant_source = M5TrustGrantSourceClass::InheritedParent;
    input.capability_narrow = M5CapabilityNarrowState::ReducedMode;
    input.blocked_action_families = vec![F::CodeExecution, F::WorkspaceSettingsWrite];
    summary(input)
}

/// Degraded summary: the posture object identity is unstated.
fn summary_posture_unstated() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:no-object".to_owned();
    input.object_identity = "".to_owned();
    summary(input)
}

/// Degraded summary: the capability posture cannot be resolved.
fn summary_posture_unresolved() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:posture-unresolved".to_owned();
    input.trust_scope = M5TrustScopeState::ScopeUnknown;
    summary(input)
}

/// Degraded summary: distinct blocked families collapsed into a generic count.
fn summary_blocked_collapsed() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:blocked-collapsed".to_owned();
    input.collapses_blocked_families = true;
    summary(input)
}

/// Degraded summary: no still-safe action is named.
fn summary_still_safe_unstated() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:still-safe-unstated".to_owned();
    input.still_safe_actions = Vec::new();
    summary(input)
}

/// Degraded summary: the summary collapses into generic "unavailable" copy.
fn summary_generic_unavailable() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:generic-unavailable".to_owned();
    input.reads_as_generic_unavailable = true;
    summary(input)
}

/// Degraded summary: no command-backed recovery path is reachable.
fn summary_recovery_missing() -> M5ResolvedNarrowedCapabilitySummary {
    let mut input = clean_summary_input();
    input.summary_id = "narrowed-summary:recovery-missing".to_owned();
    input.detail_command_available = false;
    summary(input)
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5RestrictedCapabilityConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    restricted_capability_row_examples: Vec<M5ResolvedRestrictedCapabilityRow>,
    narrowed_capability_summary_examples: Vec<M5ResolvedNarrowedCapabilitySummary>,
) -> M5RestrictedCapabilityControlsRow {
    M5RestrictedCapabilityControlsRow {
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
        anatomy_parts: M5RestrictedCapabilityAnatomyPart::ALL.to_vec(),
        export_fields: M5RestrictedCapabilityExportField::ALL.to_vec(),
        downgrade_triggers,
        restricted_capability_row_examples,
        narrowed_capability_summary_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_REF,
            M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF,
        ]),
        collapses_restricted_into_generic_unavailable: false,
        hides_blocked_families_or_still_safe_actions: false,
        routes_recovery_through_docs_or_logs_only: false,
        implies_blanket_restriction_across_roots_or_routes: false,
    }
}

fn controls_rows() -> Vec<M5RestrictedCapabilityControlsRow> {
    use M5WorkspaceTrustRepairConsumerSurface as C;
    use M5WorkspaceTrustRepairDowngradeTrigger as D;

    vec![
        base_row(
            C::WorkspaceTrustUi,
            "Workspace trust owner",
            "The workspace-trust UI renders one restricted-capability row enumerating blocked action families, still-safe actions, and why the restriction exists for a restricted and a mixed-root workspace, plus a narrowed-capability summary keeping mixed-root restriction explicit rather than uniform",
            "evidence:m5-restricted-capability-workspace-trust-ui:001",
            vec![
                D::NarrowedCapabilityUnstated,
                D::MixedRootShownAsUniformTrust,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![row_restricted_clean(), row_mixed_root_clean()],
            vec![summary_restricted_clean(), summary_mixed_root_clean()],
        ),
        base_row(
            C::SettingsUi,
            "Settings trust owner",
            "The settings trust pane reuses the same field and recovery grammar for a policy-blocked object, names the narrowed capability a restriction removes, and degrades honestly when the narrowed capability is unnamed or the summary collapses distinct blocked families",
            "evidence:m5-restricted-capability-settings-ui:001",
            vec![
                D::NarrowedCapabilityUnstated,
                D::GrantSourceUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![row_policy_blocked_clean(), row_capability_unstated()],
            vec![summary_policy_blocked_clean(), summary_blocked_collapsed()],
        ),
        base_row(
            C::SafeModeUi,
            "Safe mode owner",
            "Safe mode shows the task-blocked restricted row with its still-safe actions and a restricted summary, degrading honestly when the restriction scope cannot be resolved or a still-safe action is not named",
            "evidence:m5-restricted-capability-safe-mode-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::NarrowedCapabilityUnstated,
                D::ProofStale,
                D::GrantSourceUnstated,
            ],
            vec![row_task_blocked_clean(), row_scope_unknown()],
            vec![summary_restricted_clean(), summary_still_safe_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved row and summary truth, so a restriction collapsed into generic unavailable copy or a missing command-backed recovery path is visible in evidence rather than hidden",
            "evidence:m5-restricted-capability-support-export:001",
            vec![
                D::GenericChromeWordingUsed,
                D::RootScopeCollapsedIntoBlanketTrust,
                D::NarrowedCapabilityUnstated,
                D::ProofStale,
            ],
            vec![row_generic_unavailable(), row_recovery_missing()],
            vec![summary_generic_unavailable(), summary_recovery_missing()],
        ),
        base_row(
            C::ProductUi,
            "In-product restricted owner",
            "In-product surfaces reuse the same restriction, still-safe, and command-backed recovery grammar a user sees in the workspace-trust UI, always keeping inspect-trust reachable and degrading honestly when object, source, reason, blocked families, still-safe actions, or per-root scope is unstated",
            "evidence:m5-restricted-capability-product-ui:001",
            vec![
                D::GrantSourceUnstated,
                D::GenericChromeWordingUsed,
                D::MixedRootShownAsUniformTrust,
                D::ProofStale,
            ],
            vec![
                row_restricted_clean(),
                row_object_unstated(),
                row_source_unstated(),
                row_reason_unstated(),
                row_blocked_unstated(),
                row_still_safe_unstated(),
                row_mixed_root_collapsed(),
            ],
            vec![
                summary_restricted_clean(),
                summary_posture_unstated(),
                summary_posture_unresolved(),
            ],
        ),
    ]
}

fn governance_review() -> M5RestrictedCapabilityGovernanceReview {
    M5RestrictedCapabilityGovernanceReview {
        row_names_object_scope_and_source: true,
        row_enumerates_blocked_and_still_safe: true,
        no_surface_collapses_into_generic_unavailable: true,
        command_backed_recovery_always_reachable: true,
        recovery_choices_consistent_across_consumers: true,
        no_surface_implies_blanket_restriction: true,
        restricted_vocabulary_shared_across_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RestrictedCapabilityConsumerProjection {
    M5RestrictedCapabilityConsumerProjection {
        restricted_surfaces_expose_same_fields_and_recovery: true,
        still_safe_actions_legible_without_docs: true,
        restricted_traces_to_single_component_contract: true,
        support_export_reads_single_restricted_source: true,
    }
}

fn proof_freshness() -> M5RestrictedCapabilityProofFreshness {
    M5RestrictedCapabilityProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RestrictedCapabilityReleasePosture {
    M5RestrictedCapabilityReleasePosture {
        proof_packet_ref: M5_RESTRICTED_CAPABILITY_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_RESTRICTED_CAPABILITY_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_REF,
        M5_RESTRICTED_CAPABILITY_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 restricted-capability-row / narrowed-capability-summary controls packet.
pub fn seeded_m5_restricted_capability_controls() -> M5RestrictedCapabilityControlsPacket {
    M5RestrictedCapabilityControlsPacket::new(M5RestrictedCapabilityControlsPacketInput {
        packet_id: M5_RESTRICTED_CAPABILITY_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 restricted-capability rows and narrowed-capability summaries with blocked action families, still-safe actions, restriction reason, and command-backed recovery paths"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5RestrictedCapabilityVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the workspace-trust-UI row is held at Beta pending recovery parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_restricted_capability_controls_workspace_trust_ui_beta_narrowed(
) -> M5RestrictedCapabilityControlsPacket {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.packet_id =
        "m5-restricted-capability-row-narrowed-capability-summary-controls:workspace-trust-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi)
        .expect("workspace-trust-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Beta;
    packet
}

/// Narrowed variant: the safe-mode-UI row is narrowed to Preview pending still-safe parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_restricted_capability_controls_safe_mode_ui_preview_narrowed(
) -> M5RestrictedCapabilityControlsPacket {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.packet_id =
        "m5-restricted-capability-row-narrowed-capability-summary-controls:safe-mode-ui-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .expect("safe-mode-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Preview;
    packet
}
