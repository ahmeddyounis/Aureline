//! Canonical seed builders for the M5 mention-resolver / slash-command-row primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical mention/command-primitive packet.
pub const M5_MENTION_SLASH_COMMAND_PACKET_ID: &str =
    "m5-mention-resolver-slash-command-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked mention resolution case from a full mention state.
#[allow(clippy::too_many_arguments)]
fn mention_case(
    mention_token: &str,
    scope_note: &str,
    candidate_count: usize,
    has_exact_stable_target: bool,
    target_is_pinned: bool,
    target_object_id: Option<&str>,
    target_preview_label: Option<&str>,
    in_scope: bool,
    deferred: bool,
) -> M5MentionResolverResolutionCase {
    M5MentionResolverResolutionCase::resolved(M5MentionResolverResolutionInput {
        mention_token: mention_token.to_owned(),
        scope_note: scope_note.to_owned(),
        candidate_count,
        has_exact_stable_target,
        target_is_pinned,
        target_object_id: target_object_id.map(str::to_owned),
        target_preview_label: target_preview_label.map(str::to_owned),
        in_scope,
        deferred,
    })
}

/// Builds a worked slash-command-row resolution case from a full command state.
#[allow(clippy::too_many_arguments)]
fn slash_case(
    command_id: &str,
    command_label: &str,
    capability_class: M5SlashCommandCapabilityClass,
    help_path: &str,
    state: M5SlashCommandState,
    requires_approval: bool,
    disabled_reason: Option<&str>,
    alias_of: Option<&str>,
) -> M5SlashCommandRowResolutionCase {
    M5SlashCommandRowResolutionCase::resolved(M5SlashCommandRowResolutionInput {
        command_id: command_id.to_owned(),
        command_label: command_label.to_owned(),
        capability_class,
        help_path: help_path.to_owned(),
        state,
        requires_approval,
        disabled_reason: disabled_reason.map(str::to_owned),
        alias_of: alias_of.map(str::to_owned),
    })
}

/// A base row with the shared fields filled in and the full mention / slash anatomy,
/// resolution, state, capability, posture, action, export-field, and accessibility parity
/// every consumer carries.
fn base_row(
    consumer_surface: M5MentionSlashCommandConsumerSurface,
    qualification: M5ComposerQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    mention_examples: Vec<M5MentionResolverResolutionCase>,
    slash_examples: Vec<M5SlashCommandRowResolutionCase>,
) -> M5MentionSlashCommandRow {
    M5MentionSlashCommandRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComposerSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComposerDeploymentLine::ALL.to_vec(),
        mention_anatomy_parts: M5MentionResolverAnatomyPart::ALL.to_vec(),
        slash_anatomy_parts: M5SlashCommandRowAnatomyPart::ALL.to_vec(),
        mention_resolutions: M5MentionResolution::ALL.to_vec(),
        mention_actions: M5MentionResolverAction::ALL.to_vec(),
        slash_command_states: M5SlashCommandState::ALL.to_vec(),
        capability_classes: M5SlashCommandCapabilityClass::ALL.to_vec(),
        slash_row_postures: M5SlashCommandRowPosture::ALL.to_vec(),
        slash_actions: M5SlashCommandRowAction::ALL.to_vec(),
        mention_export_fields: M5MentionResolverExportField::ALL.to_vec(),
        slash_export_fields: M5SlashCommandRowExportField::ALL.to_vec(),
        accessibility_routes: M5ComposerAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ComposerConsumerSurface::InlineComposerUi,
            M5ComposerConsumerSurface::ComposerPanelUi,
            M5ComposerConsumerSurface::PatchReviewUi,
            M5ComposerConsumerSurface::SupportExport,
            M5ComposerConsumerSurface::CliInspect,
            M5ComposerConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5ComposerDowngradeTrigger::MentionLeftUnresolved,
            M5ComposerDowngradeTrigger::RouteOrProviderMasked,
            M5ComposerDowngradeTrigger::SendReviewGateBypassed,
            M5ComposerDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MENTION_SLASH_COMMAND_SCHEMA_REF,
            M5_MENTION_SLASH_COMMAND_COMMAND_GRAPH_REF,
            M5_MENTION_SLASH_COMMAND_MENTION_PROVENANCE_REF,
        ]),
        mention_examples,
        slash_examples,
        masks_command_identity_or_capability: false,
        hides_mention_resolution_or_ambiguity: false,
        invents_private_command_grammar: false,
        bypasses_ambiguity_or_approval_gate: false,
    }
}

fn rows() -> Vec<M5MentionSlashCommandRow> {
    use M5SlashCommandCapabilityClass as Cap;
    use M5SlashCommandState as State;

    let mut rows = Vec::new();

    // 1. Inline composer — a mention that binds to an exact stable symbol and an ambiguous
    //    file mention that blocks send with review; a ready read-only query command and an
    //    approval-gated external-side-effect command.
    rows.push(base_row(
        M5MentionSlashCommandConsumerSurface::InlineComposer,
        M5ComposerQualificationClass::Stable,
        "Inline composer owner",
        "The inline composer renders the shared mention resolver and slash-command row so an `@`-mention that matches an exact stable symbol binds uniquely with its exact-target preview, an ambiguous file mention blocks send and offers a choose-candidate review instead of binding to the wrong target, a read-only query command reads as ready-invocable, and an external-side-effect command reads as approval-gated with a request-approval action",
        "evidence:m5-mention-slash-inline:001",
        vec![
            mention_case(
                "@parse_config",
                "scope: active file symbols",
                1,
                true,
                false,
                Some("obj.symbol.parse_config"),
                Some("fn parse_config"),
                true,
                false,
            ),
            mention_case(
                "@config",
                "scope: workspace files",
                3,
                false,
                false,
                None,
                None,
                true,
                false,
            ),
        ],
        vec![
            slash_case(
                "cmd.ai.explain",
                "Explain selection",
                Cap::ReadOnlyQuery,
                "docs/help/commands/ai-explain.md",
                State::Available,
                false,
                None,
                None,
            ),
            slash_case(
                "cmd.ai.publish-review",
                "Publish review comment",
                Cap::ExternalSideEffect,
                "docs/help/commands/publish-review.md",
                State::RequiresApproval,
                false,
                None,
                None,
            ),
        ],
    ));

    // 2. Command palette — a mention that binds to a pinned object and an unresolved missing
    //    mention; a disabled scoped-mutation command with an explanation and a
    //    deprecated/aliased meta command that redirects to its canonical id.
    rows.push(base_row(
        M5MentionSlashCommandConsumerSurface::CommandPalette,
        M5ComposerQualificationClass::Stable,
        "Command palette owner",
        "The command palette renders the same mention resolver and slash-command row so a pinned `@`-mention binds to its pinned object with its exact-target preview, an unresolved mention reads as unresolved-missing and blocks send with an edit action, a disabled scoped-mutation command names its unmet-precondition reason, and a deprecated command redirects to its canonical id — the same availability, authority, and disabled reasons the palette shows for non-AI commands",
        "evidence:m5-mention-slash-palette:001",
        vec![
            mention_case(
                "@pinned-runbook",
                "scope: pinned objects",
                1,
                true,
                true,
                Some("obj.doc.runbook-pinned"),
                Some("runbook: incident-response"),
                true,
                false,
            ),
            mention_case(
                "@does-not-exist",
                "scope: workspace files",
                0,
                false,
                false,
                None,
                None,
                true,
                false,
            ),
        ],
        vec![
            slash_case(
                "cmd.workspace.rename-symbol",
                "Rename symbol",
                Cap::ScopedMutation,
                "docs/help/commands/rename-symbol.md",
                State::DisabledUnmetPrecondition,
                false,
                Some("no symbol under the cursor"),
                None,
            ),
            slash_case(
                "cmd.help.legacy-index",
                "Open legacy index",
                Cap::MetaHelp,
                "docs/help/commands/legacy-index.md",
                State::DeprecatedAliased,
                false,
                None,
                Some("cmd.help.index"),
            ),
        ],
    ));

    // 3. Automation recipe — an out-of-scope denied mention and a deferred-pending mention; a
    //    policy-hidden privileged-admin command and an unknown command.
    rows.push(base_row(
        M5MentionSlashCommandConsumerSurface::AutomationRecipe,
        M5ComposerQualificationClass::Stable,
        "Automation recipe owner",
        "The automation recipe renders the same mention resolver and slash-command row so an `@`-mention outside the recipe scope reads as out-of-scope-denied with a reveal-scope action, a deferred mention reads as deferred-pending and blocks send, a privileged-admin command hidden by policy reads as policy-hidden with its reason, and an unknown command reads as unknown-rejected rather than silently binding or invoking",
        "evidence:m5-mention-slash-automation:001",
        vec![
            mention_case(
                "@other-tenant-file",
                "scope: this recipe workspace",
                2,
                false,
                false,
                None,
                None,
                false,
                false,
            ),
            mention_case(
                "@indexing-target",
                "scope: workspace files",
                4,
                false,
                false,
                None,
                None,
                true,
                true,
            ),
        ],
        vec![
            slash_case(
                "cmd.admin.rotate-tenant-keys",
                "Rotate tenant keys",
                Cap::PrivilegedAdmin,
                "docs/help/commands/rotate-tenant-keys.md",
                State::PolicyHidden,
                false,
                Some("hidden by tenant policy on this deployment line"),
                None,
            ),
            slash_case(
                "cmd.unknown.legacy",
                "Unknown legacy command",
                Cap::ReadOnlyQuery,
                "docs/help/commands/unknown.md",
                State::UnknownCommand,
                false,
                None,
                None,
            ),
        ],
    ));

    // 4. CLI / headless — a single-candidate unique mention and an ambiguous mention; an
    //    approval-escalated repository-mutation command (available but requires approval) and
    //    a ready scoped-mutation command.
    rows.push(base_row(
        M5MentionSlashCommandConsumerSurface::CliHeadless,
        M5ComposerQualificationClass::Stable,
        "CLI / headless owner",
        "The CLI / headless surface renders the same mention resolver and slash-command row so a single-candidate `@`-mention binds uniquely with its exact-target preview, an ambiguous mention blocks send and needs explicit review, an available repository-mutation command that still requires approval reads as approval-gated rather than plainly ready, and a scoped-mutation command reads as ready-invocable — the same postures a headless reviewer reads elsewhere",
        "evidence:m5-mention-slash-cli:001",
        vec![
            mention_case(
                "@main.rs",
                "scope: open files",
                1,
                false,
                false,
                Some("obj.file.main-rs"),
                Some("src/main.rs"),
                true,
                false,
            ),
            mention_case(
                "@handler",
                "scope: repository symbols",
                5,
                false,
                false,
                None,
                None,
                true,
                false,
            ),
        ],
        vec![
            slash_case(
                "cmd.repo.apply-migration",
                "Apply migration",
                Cap::RepositoryMutation,
                "docs/help/commands/apply-migration.md",
                State::Available,
                true,
                None,
                None,
            ),
            slash_case(
                "cmd.workspace.format",
                "Format workspace",
                Cap::ScopedMutation,
                "docs/help/commands/format-workspace.md",
                State::Available,
                false,
                None,
                None,
            ),
        ],
    ));

    // 5. Support export — an exact-stable unique mention and an unresolved missing mention; a
    //    disabled repository-mutation command with an explanation and a ready meta/help
    //    command — the same mention/command vocabulary a support reviewer reconstructs from
    //    the export alone.
    rows.push(base_row(
        M5MentionSlashCommandConsumerSurface::SupportExport,
        M5ComposerQualificationClass::Stable,
        "Support export owner",
        "The support export renders the same mention resolver and slash-command row so a resolved mention's stable target id, exact-target preview, and scope note are reconstructable from the export alone, an unresolved mention reads as unresolved-missing, a disabled repository-mutation command carries its explanation, and a meta/help command reads as ready-invocable with its help path",
        "evidence:m5-mention-slash-support:001",
        vec![
            mention_case(
                "@evidence-run-42",
                "scope: evidence packets",
                1,
                true,
                false,
                Some("obj.evidence.run-42"),
                Some("evidence packet: run-42"),
                true,
                false,
            ),
            mention_case(
                "@ghost-symbol",
                "scope: repository symbols",
                0,
                false,
                false,
                None,
                None,
                true,
                false,
            ),
        ],
        vec![
            slash_case(
                "cmd.repo.rewrite-history",
                "Rewrite history",
                Cap::RepositoryMutation,
                "docs/help/commands/rewrite-history.md",
                State::DisabledUnmetPrecondition,
                false,
                Some("blocked while a background agent holds the branch"),
                None,
            ),
            slash_case(
                "cmd.help.index",
                "Open command index",
                Cap::MetaHelp,
                "docs/help/commands/index.md",
                State::Available,
                false,
                None,
                None,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5MentionSlashCommandGovernanceReview {
    M5MentionSlashCommandGovernanceReview {
        one_primitive_carries_mention_and_command_truth: true,
        mention_prefers_exact_stable_objects: true,
        ambiguous_binding_blocks_send_with_review: true,
        unresolved_binding_never_silently_bound: true,
        mention_scope_note_always_preserved: true,
        exact_target_preview_always_shown: true,
        slash_reuses_stable_command_ids: true,
        disabled_state_always_explained: true,
        approval_semantics_match_command_graph: true,
        availability_matches_non_ai_surfaces: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5MentionSlashCommandConsumerProjection {
    M5MentionSlashCommandConsumerProjection {
        composition_and_palette_surfaces_consume_shared_primitive: true,
        mention_resolution_reads_single_source: true,
        slash_command_posture_reads_single_source: true,
        command_graph_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5MentionSlashCommandProofFreshness {
    M5MentionSlashCommandProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MentionSlashCommandReleasePosture {
    M5MentionSlashCommandReleasePosture {
        release_packet_ref: M5_MENTION_SLASH_COMMAND_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_MENTION_SLASH_COMMAND_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MENTION_SLASH_COMMAND_SCHEMA_REF,
        M5_MENTION_SLASH_COMMAND_DOC_REF,
        M5_MENTION_SLASH_COMMAND_COMPONENT_MATRIX_REF,
        M5_MENTION_SLASH_COMMAND_COMMAND_GRAPH_REF,
        M5_MENTION_SLASH_COMMAND_MENTION_PROVENANCE_REF,
    ])
}

/// Builds the canonical M5 mention-resolver / slash-command-row packet.
pub fn seeded_m5_mention_slash_command_packet() -> M5MentionSlashCommandPacket {
    M5MentionSlashCommandPacket::new(M5MentionSlashCommandPacketInput {
        packet_id: M5_MENTION_SLASH_COMMAND_PACKET_ID.to_owned(),
        matrix_label:
            "M5 mention resolver and slash-command row primitive: mention token, resolution, exact-target preview, scope note, candidate count, command id, capability class, help path, availability state, row posture, approval semantics, disabled-state explanation, and bounded open/choose/edit/remove/reveal and invoke/request-approval/open-help/view-canonical/explain-disabled actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5MentionSlashCommandVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the automation recipe is narrowed to Preview pending mention-scope
/// parity proof across every headless recipe path; every consumer stays visible.
pub fn seeded_m5_mention_slash_command_automation_recipe_preview_narrowed(
) -> M5MentionSlashCommandPacket {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.packet_id =
        "m5-mention-resolver-slash-command-row-primitive:automation-recipe-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MentionSlashCommandConsumerSurface::AutomationRecipe)
        .expect("automation-recipe row present");
    row.qualification = M5ComposerQualificationClass::Preview;
    packet
}

/// Narrowed variant: the CLI / headless surface is held at Beta because a slice of headless
/// commands do not yet render the disabled-reason cue on every profile; every consumer stays
/// visible.
pub fn seeded_m5_mention_slash_command_cli_headless_beta_narrowed() -> M5MentionSlashCommandPacket {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.packet_id =
        "m5-mention-resolver-slash-command-row-primitive:cli-headless-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MentionSlashCommandConsumerSurface::CliHeadless)
        .expect("cli-headless row present");
    row.qualification = M5ComposerQualificationClass::Beta;
    packet
}
