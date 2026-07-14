//! Canonical seed builders for the M5 channel-isolation, precedence-review, and rollback-target registries
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean channel and precedence entries prove the supported
//! side-by-side channel, the complete isolation inventory of channel root / state namespace / secrets namespace
//! / services namespace, the never-reused stable durable-state namespace, the disclosed
//! isolated-versus-governed-handoff containment, the published file-association / protocol-handler / deep-link /
//! default-open precedence rule, and the full artifact-graph rollback target across the installer, update,
//! diagnostics, admin, docs, and support surfaces without any hand-copied per-profile assumption, reused stable
//! namespace, ambiguous containment, undisclosed field, narrowed rollback, or presentation-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_PACKET_ID: &str =
    "m5-channel-isolation-precedence-review-and-rollback-targets:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn channel(input: M5ChannelIsolationEntryResolutionInput) -> M5ResolvedChannelIsolationEntry {
    resolve_channel_isolation_entry(input).expect("seed channel-isolation entry resolves")
}

fn precedence(
    input: M5PrecedenceRollbackEntryResolutionInput,
) -> M5ResolvedPrecedenceRollbackEntry {
    resolve_precedence_and_rollback_entry(input).expect("seed precedence-rollback entry resolves")
}

fn all_forms() -> Vec<M5ChannelPresentationForm> {
    M5ChannelPresentationForm::ALL.to_vec()
}

fn all_isolation_fields() -> Vec<M5ChannelIsolationField> {
    M5ChannelIsolationField::ALL.to_vec()
}

fn all_precedence_fields() -> Vec<M5PrecedenceReviewField> {
    M5PrecedenceReviewField::ALL.to_vec()
}

// -- Clean channel-isolation entries ------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_channel_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    chan: M5SideBySideChannel,
    surface_context: M5ChannelSurfaceContext,
    containment: M5ChannelStateContainment,
    channel_root: &str,
    state_namespace_root: &str,
    secrets_namespace_root: &str,
) -> M5ChannelIsolationEntryResolutionInput {
    M5ChannelIsolationEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        channel: chan,
        surface_context,
        presentation_form_coverage: all_forms(),
        channel_root: channel_root.to_owned(),
        state_namespace_root: state_namespace_root.to_owned(),
        secrets_namespace_root: secrets_namespace_root.to_owned(),
        isolation_fields_covered: all_isolation_fields(),
        containment,
        bound_to_registry: true,
        namespace_reuse_used: false,
        namespace_isolation_enforced: true,
        proof_fresh: true,
    }
}

fn channel_stable_installer_clean() -> M5ResolvedChannelIsolationEntry {
    channel(clean_channel_base(
        "channel:stable:installer",
        "profile.side_by_side_stable",
        "channel.side_by_side.stable",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Stable,
        M5ChannelSurfaceContext::InstallerFlow,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\Stable",
        r"%LOCALAPPDATA%\Aureline\Stable\state",
        r"%LOCALAPPDATA%\Aureline\Stable\secrets",
    ))
}

fn channel_preview_update_clean() -> M5ResolvedChannelIsolationEntry {
    channel(clean_channel_base(
        "channel:preview:update",
        "profile.side_by_side_preview",
        "channel.side_by_side.preview",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Preview,
        M5ChannelSurfaceContext::UpdateFlow,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\Preview",
        r"%LOCALAPPDATA%\Aureline\Preview\state",
        r"%LOCALAPPDATA%\Aureline\Preview\secrets",
    ))
}

fn channel_beta_diagnostics_clean() -> M5ResolvedChannelIsolationEntry {
    channel(clean_channel_base(
        "channel:beta:diagnostics",
        "profile.side_by_side_beta",
        "channel.side_by_side.beta",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Beta,
        M5ChannelSurfaceContext::DiagnosticsSurface,
        M5ChannelStateContainment::GovernedHandoff,
        r"%LOCALAPPDATA%\Aureline\Beta",
        r"%LOCALAPPDATA%\Aureline\Beta\state",
        r"%LOCALAPPDATA%\Aureline\Beta\secrets",
    ))
}

fn channel_lts_admin_clean() -> M5ResolvedChannelIsolationEntry {
    channel(clean_channel_base(
        "channel:lts:admin",
        "profile.side_by_side_lts",
        "channel.side_by_side.lts",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Lts,
        M5ChannelSurfaceContext::AdminSurface,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\LTS",
        r"%LOCALAPPDATA%\Aureline\LTS\state",
        r"%LOCALAPPDATA%\Aureline\LTS\secrets",
    ))
}

fn channel_stable_support_clean() -> M5ResolvedChannelIsolationEntry {
    channel(clean_channel_base(
        "channel:stable:support",
        "profile.side_by_side_stable",
        "channel.side_by_side.stable",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Stable,
        M5ChannelSurfaceContext::SupportOrExportForm,
        M5ChannelStateContainment::SharedDisclosed,
        r"%LOCALAPPDATA%\Aureline\Stable",
        r"%LOCALAPPDATA%\Aureline\Stable\state",
        r"%LOCALAPPDATA%\Aureline\Stable\secrets",
    ))
}

// -- Degraded channel-isolation entries ---------------------------------------------------------

/// Degraded channel entry: the isolation inventory is incomplete — the services namespace is not published.
fn channel_inventory_incomplete() -> M5ResolvedChannelIsolationEntry {
    let mut base = clean_channel_base(
        "channel:stable:inventory-incomplete",
        "profile.side_by_side_stable",
        "channel.side_by_side.stable",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Stable,
        M5ChannelSurfaceContext::InstallerFlow,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\Stable",
        r"%LOCALAPPDATA%\Aureline\Stable\state",
        r"%LOCALAPPDATA%\Aureline\Stable\secrets",
    );
    base.isolation_fields_covered = vec![
        M5ChannelIsolationField::ChannelRoot,
        M5ChannelIsolationField::StateNamespace,
        M5ChannelIsolationField::SecretsNamespace,
        // ServicesNamespace is dropped: a background-agent namespace could collide across channels.
    ];
    channel(base)
}

/// Degraded channel entry: a preview channel reused the stable durable-state namespace without a governed
/// handoff, so coexisting installs corrupt one another's durable state.
fn channel_namespace_reused() -> M5ResolvedChannelIsolationEntry {
    let mut base = clean_channel_base(
        "channel:preview:namespace-reused",
        "profile.side_by_side_preview",
        "channel.side_by_side.preview",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Preview,
        M5ChannelSurfaceContext::UpdateFlow,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\Preview",
        r"%LOCALAPPDATA%\Aureline\Preview\state",
        r"%LOCALAPPDATA%\Aureline\Preview\secrets",
    );
    base.namespace_reuse_used = true;
    base.namespace_isolation_enforced = false;
    channel(base)
}

/// Degraded channel entry: the containment is ambiguous, so a coexisting channel could corrupt this channel's
/// durable state.
fn channel_containment_ambiguous() -> M5ResolvedChannelIsolationEntry {
    let mut base = clean_channel_base(
        "channel:beta:containment-ambiguous",
        "profile.side_by_side_beta",
        "channel.side_by_side.beta",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Beta,
        M5ChannelSurfaceContext::DiagnosticsSurface,
        M5ChannelStateContainment::ContainmentAmbiguous,
        r"%LOCALAPPDATA%\Aureline\Beta",
        r"%LOCALAPPDATA%\Aureline\Beta\state",
        r"%LOCALAPPDATA%\Aureline\Beta\secrets",
    );
    base.containment = M5ChannelStateContainment::ContainmentAmbiguous;
    channel(base)
}

/// Degraded channel entry: the behavior is a hand-copied per-profile assumption instead of tracing to the
/// registry.
fn channel_unbound() -> M5ResolvedChannelIsolationEntry {
    let mut base = clean_channel_base(
        "channel:lts:unbound",
        "profile.side_by_side_lts",
        "channel.side_by_side.lts",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Lts,
        M5ChannelSurfaceContext::AdminSurface,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\LTS",
        r"%LOCALAPPDATA%\Aureline\LTS\state",
        r"%LOCALAPPDATA%\Aureline\LTS\secrets",
    );
    base.bound_to_registry = false;
    channel(base)
}

/// Degraded channel entry: the canonical / accessible / audit presentation-form coverage is incomplete.
fn channel_form_incomplete() -> M5ResolvedChannelIsolationEntry {
    let mut base = clean_channel_base(
        "channel:stable:form-incomplete",
        "profile.side_by_side_stable",
        "channel.side_by_side.stable",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Stable,
        M5ChannelSurfaceContext::InstallerFlow,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\Stable",
        r"%LOCALAPPDATA%\Aureline\Stable\state",
        r"%LOCALAPPDATA%\Aureline\Stable\secrets",
    );
    base.presentation_form_coverage = vec![M5ChannelPresentationForm::CanonicalObject];
    channel(base)
}

/// Degraded channel entry: the canonical registry token name is unstated.
fn channel_token_unstated() -> M5ResolvedChannelIsolationEntry {
    let mut base = clean_channel_base(
        "channel:stable:token-unstated",
        "profile.side_by_side_stable",
        "  ",
        M5InstallTopologyRole::WritableStateRoots,
        M5SideBySideChannel::Stable,
        M5ChannelSurfaceContext::SupportOrExportForm,
        M5ChannelStateContainment::Isolated,
        r"%LOCALAPPDATA%\Aureline\Stable",
        r"%LOCALAPPDATA%\Aureline\Stable\state",
        r"%LOCALAPPDATA%\Aureline\Stable\secrets",
    );
    base.token_name = "  ".to_owned();
    channel(base)
}

// -- Clean precedence-rollback entries ----------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_precedence_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    domain: M5PrecedenceReviewDomain,
    surface_context: M5ChannelSurfaceContext,
    posture: M5RollbackCompletenessPosture,
    association_owner: &str,
    rollback_artifact_graph_root: &str,
) -> M5PrecedenceRollbackEntryResolutionInput {
    M5PrecedenceRollbackEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        precedence_domain: domain,
        surface_context,
        presentation_form_coverage: all_forms(),
        association_owner: association_owner.to_owned(),
        rollback_artifact_graph_root: rollback_artifact_graph_root.to_owned(),
        disclosed_fields: all_precedence_fields(),
        rollback_posture: posture,
        rollback_artifact_graph_continuity_documented: true,
        precedence_ownership_disclosed: true,
        proof_fresh: true,
    }
}

fn precedence_file_association_installer_clean() -> M5ResolvedPrecedenceRollbackEntry {
    precedence(clean_precedence_base(
        "precedence:file-association:installer",
        "profile.side_by_side_stable",
        "precedence.file_association",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::FileAssociation,
        M5ChannelSurfaceContext::InstallerFlow,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Stable\rollback\artifact-graph",
    ))
}

fn precedence_protocol_handler_update_clean() -> M5ResolvedPrecedenceRollbackEntry {
    precedence(clean_precedence_base(
        "precedence:protocol-handler:update",
        "profile.side_by_side_preview",
        "precedence.protocol_handler",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::ProtocolHandler,
        M5ChannelSurfaceContext::UpdateFlow,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Preview\rollback\artifact-graph",
    ))
}

fn precedence_deep_link_diagnostics_clean() -> M5ResolvedPrecedenceRollbackEntry {
    precedence(clean_precedence_base(
        "precedence:deep-link:diagnostics",
        "profile.side_by_side_beta",
        "precedence.deep_link",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::DeepLink,
        M5ChannelSurfaceContext::DiagnosticsSurface,
        M5RollbackCompletenessPosture::GovernedPartialDisclosed,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Beta\rollback\artifact-graph",
    ))
}

fn precedence_default_open_admin_clean() -> M5ResolvedPrecedenceRollbackEntry {
    precedence(clean_precedence_base(
        "precedence:default-open:admin",
        "profile.side_by_side_lts",
        "precedence.default_open",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::DefaultOpen,
        M5ChannelSurfaceContext::AdminSurface,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\LTS\rollback\artifact-graph",
    ))
}

fn precedence_file_association_docs_clean() -> M5ResolvedPrecedenceRollbackEntry {
    precedence(clean_precedence_base(
        "precedence:file-association:docs",
        "profile.side_by_side_stable",
        "precedence.file_association",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::FileAssociation,
        M5ChannelSurfaceContext::DiagnosticsSurface,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Stable\rollback\artifact-graph",
    ))
}

fn precedence_default_open_support_clean() -> M5ResolvedPrecedenceRollbackEntry {
    precedence(clean_precedence_base(
        "precedence:default-open:support",
        "profile.side_by_side_stable",
        "precedence.default_open",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::DefaultOpen,
        M5ChannelSurfaceContext::SupportOrExportForm,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Stable\rollback\artifact-graph",
    ))
}

// -- Degraded precedence-rollback entries -------------------------------------------------------

/// Degraded precedence entry: the precedence rule is not inspectable — the inspectable-before-and-after field is
/// dropped and the precedence ownership is left implicit, so handler ownership could become last-writer-wins.
fn precedence_not_inspectable() -> M5ResolvedPrecedenceRollbackEntry {
    let mut base = clean_precedence_base(
        "precedence:file-association:not-inspectable",
        "profile.side_by_side_stable",
        "precedence.file_association",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::FileAssociation,
        M5ChannelSurfaceContext::InstallerFlow,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Stable\rollback\artifact-graph",
    );
    base.disclosed_fields = vec![
        M5PrecedenceReviewField::OwnerChannel,
        M5PrecedenceReviewField::PrecedenceRank,
        M5PrecedenceReviewField::ConflictResolution,
        M5PrecedenceReviewField::RollbackArtifactGraph,
        // InspectableBeforeAndAfter is dropped: ownership cannot be checked after an update / import flow.
    ];
    base.precedence_ownership_disclosed = false;
    precedence(base)
}

/// Degraded precedence entry: the rollback target narrowed to the primary executable only while its
/// artifact-graph continuity is undocumented, so a rollback cannot restore the prior install truthfully.
fn precedence_rollback_incomplete() -> M5ResolvedPrecedenceRollbackEntry {
    let mut base = clean_precedence_base(
        "precedence:protocol-handler:rollback-incomplete",
        "profile.side_by_side_preview",
        "precedence.protocol_handler",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::ProtocolHandler,
        M5ChannelSurfaceContext::UpdateFlow,
        M5RollbackCompletenessPosture::PrimaryExecutableOnly,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Preview\rollback\artifact-graph",
    );
    base.rollback_artifact_graph_continuity_documented = false;
    precedence(base)
}

/// Degraded precedence entry: the precedence domain is unclassified.
fn precedence_domain_unclassified() -> M5ResolvedPrecedenceRollbackEntry {
    precedence(clean_precedence_base(
        "precedence:admin:domain-unclassified",
        "profile.side_by_side_lts",
        "precedence.unknown",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::DomainUnclassified,
        M5ChannelSurfaceContext::AdminSurface,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\LTS\rollback\artifact-graph",
    ))
}

/// Degraded precedence entry: the canonical / accessible / audit presentation-form coverage is incomplete.
fn precedence_form_incomplete() -> M5ResolvedPrecedenceRollbackEntry {
    let mut base = clean_precedence_base(
        "precedence:file-association:form-incomplete",
        "profile.side_by_side_stable",
        "precedence.file_association",
        M5InstallTopologyRole::RollbackTarget,
        M5PrecedenceReviewDomain::FileAssociation,
        M5ChannelSurfaceContext::DiagnosticsSurface,
        M5RollbackCompletenessPosture::FullArtifactGraphBound,
        "channel.stable",
        r"%LOCALAPPDATA%\Aureline\Stable\rollback\artifact-graph",
    );
    base.presentation_form_coverage = vec![M5ChannelPresentationForm::CanonicalObject];
    precedence(base)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ChannelConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    channel_isolation_entries: Vec<M5ResolvedChannelIsolationEntry>,
    precedence_rollback_entries: Vec<M5ResolvedPrecedenceRollbackEntry>,
) -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsRow {
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsRow {
        consumer_surface,
        qualification: M5InstallTopologyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5InstallTopologyDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5InstallTopologyRequiredLabel::Identity,
            M5InstallTopologyRequiredLabel::SemanticRole,
            M5InstallTopologyRequiredLabel::RegistryReference,
            M5InstallTopologyRequiredLabel::StateRoot,
            M5InstallTopologyRequiredLabel::RollbackTarget,
        ],
        accessibility_routes: M5InstallTopologyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ChannelAnatomyPart::ALL.to_vec(),
        export_fields: M5ChannelExportField::ALL.to_vec(),
        downgrade_triggers,
        channel_isolation_entries,
        precedence_rollback_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
        ]),
        preview_or_beta_reused_stable_state_namespace: false,
        handler_ownership_became_last_writer_wins: false,
        rollback_targeted_primary_executable_only: false,
        channel_precedence_or_rollback_drifted_from_matrix: false,
    }
}

fn registry_rows() -> Vec<M5ChannelIsolationPrecedenceReviewAndRollbackTargetsRow> {
    use M5InstallTopologyConsumerSurface as C;
    use M5InstallTopologyDowngradeTrigger as D;

    vec![
        base_row(
            C::Installer,
            "Installer/side-by-side-channel owner",
            "The installer resolves the stable channel to one inspectable object — the channel, the channel / state-namespace / secrets-namespace roots, and the isolated channel-root / state-namespace / secrets-namespace / services-namespace inventory — from the shared registry and publishes the file-association precedence rule bound to the full rollback artifact graph; an isolation inventory that omits the services namespace and a precedence rule that hides the before/after inspectability and ownership degrade honestly instead of reading as a clean pass",
            "evidence:m5-coexistence-installer:001",
            vec![
                D::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
                D::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
                D::ProofStale,
            ],
            vec![
                channel_stable_installer_clean(),
                channel_inventory_incomplete(),
            ],
            vec![
                precedence_file_association_installer_clean(),
                precedence_not_inspectable(),
            ],
        ),
        base_row(
            C::UpdaterService,
            "Updater/channel-coexistence owner",
            "The updater resolves the preview channel and the protocol-handler precedence rule; a preview channel that reused the stable durable-state namespace without a governed handoff and a rollback target narrowed to the primary executable while its artifact-graph continuity is undocumented are caught before an update can corrupt a coexisting channel or restore an install untruthfully",
            "evidence:m5-coexistence-updater:001",
            vec![
                D::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
                D::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
                D::ProofStale,
            ],
            vec![channel_preview_update_clean(), channel_namespace_reused()],
            vec![
                precedence_protocol_handler_update_clean(),
                precedence_rollback_incomplete(),
            ],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the beta channel and its published precedence rule without manual reconstruction; a channel whose containment is ambiguous — so a coexisting channel could corrupt its durable state — is caught instead of reading as a clean pass",
            "evidence:m5-coexistence-diagnostics:001",
            vec![
                D::StateRootUnstated,
                D::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
                D::ProofStale,
            ],
            vec![
                channel_beta_diagnostics_clean(),
                channel_containment_ambiguous(),
            ],
            vec![precedence_deep_link_diagnostics_clean()],
        ),
        base_row(
            C::Admin,
            "Admin surface owner",
            "Admin resolves the LTS channel and the default-open precedence rule while preserving one registry-bound source; a hand-copied per-profile assumption and a precedence rule on an unclassified domain degrade honestly",
            "evidence:m5-coexistence-admin:001",
            vec![
                D::StateRootBoundaryDriftedByTopology,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![channel_lts_admin_clean(), channel_unbound()],
            vec![
                precedence_default_open_admin_clean(),
                precedence_domain_unclassified(),
            ],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the same resolved channel-isolation and published-precedence truth the resolvers produced across the canonical, accessible, and audit presentation forms rather than a hand-copied channel-root table",
            "evidence:m5-coexistence-docs-help:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
                D::ProofStale,
            ],
            vec![
                channel_stable_installer_clean(),
                channel_form_incomplete(),
            ],
            vec![
                precedence_file_association_docs_clean(),
                precedence_form_incomplete(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved channel-isolation and precedence truth, so a hand-copied constant, an unstated registry token, an ambiguous containment, or a preview channel reusing the stable namespace is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-coexistence-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
                D::ProofStale,
            ],
            vec![
                channel_stable_support_clean(),
                channel_token_unstated(),
            ],
            vec![precedence_default_open_support_clean()],
        ),
    ]
}

fn governance_review() -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsGovernanceReview {
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsGovernanceReview {
        registry_names_token_role_and_channel: true,
        profile_isolates_all_canonical_channels: true,
        all_isolation_fields_published: true,
        preview_never_reuses_stable_state_namespace: true,
        containment_explicit_and_distinguishable: true,
        precedence_published_across_domains: true,
        every_entry_covers_all_presentation_forms: true,
        rollback_binds_full_artifact_graph: true,
        behavior_bound_to_registry_not_hand_copied: true,
        installer_update_diagnostics_admin_read_single_source: true,
        coexistence_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsConsumerProjection {
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsConsumerProjection {
        installer_and_update_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        updater_and_precedence_review_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsProofFreshness {
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsReleasePosture {
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsReleasePosture {
        proof_packet_ref: M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_ARTIFACT_REF
            .to_owned(),
        channel_isolation_audit_ref:
            M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_SCHEMA_REF,
        M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 channel-isolation, precedence-review, and rollback-target registries packet.
pub fn seeded_m5_channel_isolation_precedence_review_and_rollback_targets(
) -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket {
    M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket::new(
        M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacketInput {
            packet_id:
                M5_CHANNEL_ISOLATION_PRECEDENCE_REVIEW_AND_ROLLBACK_TARGETS_PACKET_ID.to_owned(),
            registries_label:
                "M5 side-by-side channel-isolation, association-precedence-review, and full artifact-graph rollback-target registries enforcing isolated channel roots and mutable-state namespaces of channel root / state namespace / secrets namespace / services namespace, a never-reused stable durable-state namespace, explicit isolated-versus-governed-handoff containment, published file-association / protocol-handler / deep-link / default-open precedence rules, and full artifact-graph rollback targets across the installer, update, diagnostics, admin, docs, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5ChannelIsolationPrecedenceReviewAndRollbackTargetsVocabularySet::canonical(),
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

/// Narrowed variant: the admin row is held at Beta pending side-by-side LTS-channel parity on every platform;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_channel_isolation_precedence_review_and_rollback_targets_side_by_side_channel_beta_narrowed(
) -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.packet_id =
        "m5-channel-isolation-precedence-review-and-rollback-targets:side-by-side-channel-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::Admin)
        .expect("admin row present");
    row.qualification = M5InstallTopologyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the updater row is narrowed to Preview pending offline / air-gap coexistence parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_channel_isolation_precedence_review_and_rollback_targets_offline_airgap_bundle_preview_narrowed(
) -> M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket {
    let mut packet = seeded_m5_channel_isolation_precedence_review_and_rollback_targets();
    packet.packet_id =
        "m5-channel-isolation-precedence-review-and-rollback-targets:offline-airgap-bundle-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::UpdaterService)
        .expect("updater-service row present");
    row.qualification = M5InstallTopologyQualificationClass::Preview;
    packet
}
