//! Canonical seed builders for the M5 write-scope-preview-tree primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical write-scope-preview-tree primitive packet.
pub const M5_WRITE_SCOPE_PREVIEW_TREE_PACKET_ID: &str =
    "m5-write-scope-preview-tree-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked write-scope-preview-tree resolution case from a full change scope.
#[allow(clippy::too_many_arguments)]
fn tree_case(
    write_scope_class: M5WriteScopeClass,
    mutation_class: M5MutationClass,
    total_file_count: u32,
    included_file_count: u32,
    excluded_file_count: u32,
    distinct_workspace_root_count: u32,
    touches_generated_or_managed: bool,
    has_out_of_workspace_target: bool,
    has_conflict: bool,
    has_policy_blocked: bool,
    scope_is_reviewable: bool,
    apply_path_ready: bool,
    scope_label: &str,
) -> M5WriteScopePreviewTreeResolutionCase {
    M5WriteScopePreviewTreeResolutionCase::resolved(M5WriteScopePreviewTreeResolutionInput {
        write_scope_class,
        mutation_class,
        total_file_count,
        included_file_count,
        excluded_file_count,
        distinct_workspace_root_count,
        touches_generated_or_managed,
        has_out_of_workspace_target,
        has_conflict,
        has_policy_blocked,
        scope_is_reviewable,
        apply_path_ready,
        scope_label: scope_label.to_owned(),
    })
}

/// Builds a worked write-scope-file-node resolution case from a full change state.
#[allow(clippy::too_many_arguments)]
fn node_case(
    change_type: M5WriteScopeChangeType,
    change_actor: M5WriteScopeChangeActor,
    content_class: M5WriteScopeFileContentClass,
    managed_caveat: M5ManagedFileCaveat,
    is_policy_blocked: bool,
    is_read_only: bool,
    has_conflict: bool,
    is_out_of_workspace: bool,
    opt_out_of_apply: bool,
    diff_available: bool,
    node_label: &str,
) -> M5WriteScopeFileNodeResolutionCase {
    M5WriteScopeFileNodeResolutionCase::resolved(M5WriteScopeFileNodeResolutionInput {
        change_type,
        change_actor,
        content_class,
        managed_caveat,
        is_policy_blocked,
        is_read_only,
        has_conflict,
        is_out_of_workspace,
        opt_out_of_apply,
        diff_available,
        node_label: node_label.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full tree / node anatomy, scope-class,
/// mutation, caveat, change-type, change-actor, content-class, bucket, posture, disposition,
/// exclusion-reason, action, export-field, and accessibility parity every consumer carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5WriteScopeConsumerSurface,
    qualification: M5HistoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    tree_examples: Vec<M5WriteScopePreviewTreeResolutionCase>,
    node_examples: Vec<M5WriteScopeFileNodeResolutionCase>,
) -> M5WriteScopePreviewTreeRow {
    M5WriteScopePreviewTreeRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5HistorySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5HistoryDeploymentLine::ALL.to_vec(),
        tree_anatomy_parts: M5WriteScopeTreeAnatomyPart::ALL.to_vec(),
        node_anatomy_parts: M5WriteScopeNodeAnatomyPart::ALL.to_vec(),
        write_scope_classes: M5WriteScopeClass::ALL.to_vec(),
        mutation_classes: M5MutationClass::ALL.to_vec(),
        managed_caveats: M5ManagedFileCaveat::ALL.to_vec(),
        change_types: M5WriteScopeChangeType::ALL.to_vec(),
        change_actors: M5WriteScopeChangeActor::ALL.to_vec(),
        content_classes: M5WriteScopeFileContentClass::ALL.to_vec(),
        file_count_buckets: M5WriteScopeFileCountBucket::ALL.to_vec(),
        tree_postures: M5WriteScopeTreePosture::ALL.to_vec(),
        node_dispositions: M5WriteScopeNodeDisposition::ALL.to_vec(),
        exclusion_reasons: M5WriteScopeExclusionReason::ALL.to_vec(),
        tree_actions: M5WriteScopeTreeAction::ALL.to_vec(),
        node_actions: M5WriteScopeNodeAction::ALL.to_vec(),
        tree_export_fields: M5WriteScopeTreeExportField::ALL.to_vec(),
        node_export_fields: M5WriteScopeNodeExportField::ALL.to_vec(),
        accessibility_routes: M5HistoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5HistoryConsumerSurface::EditorTimelineUi,
            M5HistoryConsumerSurface::CheckpointInspectorUi,
            M5HistoryConsumerSurface::RestoreReviewUi,
            M5HistoryConsumerSurface::RefactorPreviewUi,
            M5HistoryConsumerSurface::AiApplyReviewUi,
            M5HistoryConsumerSurface::RecoveryCenterUi,
            M5HistoryConsumerSurface::SupportExport,
            M5HistoryConsumerSurface::CliInspect,
            M5HistoryConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5HistoryDowngradeTrigger::FileOrObjectIdentityUnstated,
            M5HistoryDowngradeTrigger::GeneratedOrManagedCaveatHidden,
            M5HistoryDowngradeTrigger::WriteScopeUnderstated,
            M5HistoryDowngradeTrigger::TimestampOrActorUnstated,
            M5HistoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF,
            M5_WRITE_SCOPE_PREVIEW_TREE_WRITE_BOUNDARY_REF,
            M5_WRITE_SCOPE_PREVIEW_TREE_REFACTOR_PREVIEW_REF,
        ]),
        tree_examples,
        node_examples,
        flattens_into_generic_file_list: false,
        drops_ineligible_files: false,
        understates_write_scope: false,
        hides_actor_provenance: false,
    }
}

fn rows() -> Vec<M5WriteScopePreviewTreeRow> {
    use M5ManagedFileCaveat as Caveat;
    use M5MutationClass as Mutation;
    use M5WriteScopeChangeActor as Actor;
    use M5WriteScopeChangeType as Change;
    use M5WriteScopeClass as Scope;
    use M5WriteScopeFileContentClass as Content;

    vec![
        // 1. Rename preview — a focused single-file rename that applies cleanly, and a
        //    cross-package rename that reads as a broad scope and can narrow; an included
        //    text rename and a read-only-excluded target that discloses its reason.
        base_row(
            M5WriteScopeConsumerSurface::RenamePreview,
            M5HistoryQualificationClass::Stable,
            "Rename preview owner",
            "The rename preview renders the shared write-scope tree and file nodes so a focused single-file rename reads as a focused scope with a single-file bucket, and a cross-package rename reads honestly as a broad scope over its full file count with narrowing on offer — every node naming its change type and human actor, and a read-only target disclosing exactly why it is held out of the apply",
            "evidence:m5-write-scope-rename:001",
            vec![
                tree_case(
                    Scope::SingleFile,
                    Mutation::TextEdit,
                    1,
                    1,
                    0,
                    1,
                    false,
                    false,
                    false,
                    false,
                    true,
                    true,
                    "rename scope: widget.rs",
                ),
                tree_case(
                    Scope::CrossPackage,
                    Mutation::MultiFileRefactor,
                    12,
                    11,
                    1,
                    1,
                    false,
                    false,
                    false,
                    false,
                    true,
                    true,
                    "rename scope: Widget across crate (12 files)",
                ),
            ],
            vec![
                node_case(
                    Change::Renamed,
                    Actor::HumanEdit,
                    Content::TextSource,
                    Caveat::Unmanaged,
                    false,
                    false,
                    false,
                    false,
                    false,
                    true,
                    "src/widget.rs",
                ),
                node_case(
                    Change::Renamed,
                    Actor::HumanEdit,
                    Content::TextSource,
                    Caveat::ProtectedReadonly,
                    false,
                    true,
                    false,
                    false,
                    false,
                    true,
                    "vendor/lib/widget.rs",
                ),
            ],
        ),
        // 2. Refactor preview — a generated-tree refactor that discloses it reaches managed
        //    files and defaults to exclude-generated; a generated node opted out with its
        //    reason.
        base_row(
            M5WriteScopeConsumerSurface::RefactorPreview,
            M5HistoryQualificationClass::Stable,
            "Refactor preview owner",
            "The refactor preview renders the shared write-scope tree and file nodes so a refactor that regenerates a managed tree reads as a generated/managed scope, offers exclude-generated, and keeps every generated file visible with an explicit opted-out reason rather than silently rewriting or dropping it",
            "evidence:m5-write-scope-refactor:001",
            vec![tree_case(
                Scope::GeneratedTree,
                Mutation::GeneratedArtifact,
                8,
                6,
                2,
                1,
                true,
                false,
                false,
                false,
                true,
                true,
                "refactor scope: regenerate api (8 files)",
            )],
            vec![node_case(
                Change::Modified,
                Actor::RefactorEngine,
                Content::GeneratedOutput,
                Caveat::GeneratedFile,
                false,
                false,
                false,
                false,
                true,
                true,
                "gen/api_bindings.rs",
            )],
        ),
        // 3. Search/replace preview — a whole-directory replace over many files that reads as
        //    a broad scope in a large bucket and can narrow; a binary file kept in scope with
        //    no diff jump.
        base_row(
            M5WriteScopeConsumerSurface::SearchReplacePreview,
            M5HistoryQualificationClass::Stable,
            "Search/replace preview owner",
            "The search/replace preview renders the shared write-scope tree and file nodes so a whole-directory replace across many files reads honestly as a broad scope in a large file-count bucket with narrowing on offer, and a binary match stays visible in the preview as a binary-included node even when no textual diff jump is available",
            "evidence:m5-write-scope-replace:001",
            vec![tree_case(
                Scope::WholeDirectory,
                Mutation::TextEdit,
                60,
                60,
                0,
                2,
                false,
                false,
                false,
                false,
                true,
                true,
                "replace scope: rename symbol (60 files)",
            )],
            vec![node_case(
                Change::Modified,
                Actor::HumanEdit,
                Content::BinaryBlob,
                Caveat::Unmanaged,
                false,
                false,
                false,
                false,
                false,
                false,
                "assets/logo.bin",
            )],
        ),
        // 4. Import preview — an out-of-workspace import that reads as an out-of-workspace
        //    scope; an out-of-workspace node opted out with its reason and a metadata-only
        //    node kept in scope.
        base_row(
            M5WriteScopeConsumerSurface::ImportPreview,
            M5HistoryQualificationClass::Stable,
            "Import preview owner",
            "The import preview renders the shared write-scope tree and file nodes so a sync that would write outside the workspace root reads as an out-of-workspace scope, the out-of-workspace file is opted out with its explicit reason, and a metadata-only rename stays visible and in scope rather than being dropped as ineligible",
            "evidence:m5-write-scope-import:001",
            vec![tree_case(
                Scope::OutOfWorkspace,
                Mutation::ConfigMigration,
                4,
                3,
                1,
                1,
                false,
                true,
                false,
                false,
                true,
                true,
                "import scope: sync settings (4 files)",
            )],
            vec![
                node_case(
                    Change::Created,
                    Actor::ImportBridge,
                    Content::TextSource,
                    Caveat::Unmanaged,
                    false,
                    false,
                    false,
                    true,
                    true,
                    true,
                    "external/home_config.toml",
                ),
                node_case(
                    Change::Renamed,
                    Actor::ImportBridge,
                    Content::MetadataOnly,
                    Caveat::Unmanaged,
                    false,
                    false,
                    false,
                    false,
                    false,
                    true,
                    "config/app.toml",
                ),
            ],
        ),
        // 5. AI apply preview — an apply blocked behind a pending conflict that reads as a
        //    conflict scope and offers resolve-conflict; a conflict-held node with its
        //    reason.
        base_row(
            M5WriteScopeConsumerSurface::AiApplyPreview,
            M5HistoryQualificationClass::Stable,
            "AI apply preview owner",
            "The AI apply preview renders the shared write-scope tree and file nodes so an apply blocked behind a pending conflict reads as a conflict scope, offers resolve-conflict rather than a false apply, and the conflicted file node is held out with an explicit conflict reason and its AI-agent provenance intact",
            "evidence:m5-write-scope-ai-apply:001",
            vec![tree_case(
                Scope::MultiFile,
                Mutation::MultiFileRefactor,
                5,
                4,
                1,
                1,
                false,
                false,
                true,
                false,
                true,
                true,
                "ai apply scope: extract module (5 files)",
            )],
            vec![node_case(
                Change::Modified,
                Actor::AiAgent,
                Content::TextSource,
                Caveat::Unmanaged,
                false,
                false,
                true,
                false,
                false,
                true,
                "src/module.rs",
            )],
        ),
        // 6. Repair preview — a repair whose apply path is unavailable that reads as a blocked
        //    scope; a policy-blocked node kept visible with its explicit reason.
        base_row(
            M5WriteScopeConsumerSurface::RepairPreview,
            M5HistoryQualificationClass::Stable,
            "Repair preview owner",
            "The repair preview renders the shared write-scope tree and file nodes so a repair whose apply path is unavailable reads as a blocked scope that only inspects and expands, and a policy-blocked file stays visible in the preview with an explicit policy-blocked reason rather than vanishing from the change list",
            "evidence:m5-write-scope-repair:001",
            vec![tree_case(
                Scope::MultiFile,
                Mutation::RepairTransaction,
                3,
                2,
                1,
                1,
                false,
                false,
                false,
                true,
                false,
                false,
                "repair scope: restore transaction (3 files)",
            )],
            vec![node_case(
                Change::Deleted,
                Actor::RepairEngine,
                Content::TextSource,
                Caveat::Unmanaged,
                true,
                false,
                false,
                false,
                false,
                true,
                "src/legacy.rs",
            )],
        ),
    ]
}

fn governance_review() -> M5WriteScopePreviewTreeGovernanceReview {
    M5WriteScopePreviewTreeGovernanceReview {
        one_primitive_carries_tree_and_node_truth: true,
        write_scope_never_understated: true,
        file_count_bucket_always_shown: true,
        workspace_root_grouping_always_shown: true,
        actor_provenance_always_attributable: true,
        generated_readonly_conflict_never_flattened: true,
        ineligible_files_never_dropped: true,
        exclusion_reason_always_explicit: true,
        scope_inspectable_and_narrowable: true,
        diff_jump_reachable_where_available: true,
        support_export_reconstructs_tree_and_node_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5WriteScopePreviewTreeConsumerProjection {
    M5WriteScopePreviewTreeConsumerProjection {
        change_surfaces_consume_shared_primitive: true,
        tree_posture_reads_single_source: true,
        node_disposition_reads_single_source: true,
        actions_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5WriteScopePreviewTreeProofFreshness {
    M5WriteScopePreviewTreeProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WriteScopePreviewTreeReleasePosture {
    M5WriteScopePreviewTreeReleasePosture {
        release_packet_ref: M5_WRITE_SCOPE_PREVIEW_TREE_ARTIFACT_REF.to_owned(),
        recovery_audit_ref: M5_WRITE_SCOPE_PREVIEW_TREE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_DOC_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_COMPONENT_MATRIX_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_WRITE_BOUNDARY_REF,
        M5_WRITE_SCOPE_PREVIEW_TREE_REFACTOR_PREVIEW_REF,
    ])
}

/// Builds the canonical M5 write-scope-preview-tree packet.
pub fn seeded_m5_write_scope_preview_tree_packet() -> M5WriteScopePreviewTreePacket {
    M5WriteScopePreviewTreePacket::new(M5WriteScopePreviewTreePacketInput {
        packet_id: M5_WRITE_SCOPE_PREVIEW_TREE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 write-scope-preview-tree primitive: file-count buckets, workspace-root grouping, write-scope class, change type, actor provenance, generated/read-only/conflict/exclusion truth, selectable and narrowable scope, and diff-jump affordances with bounded inspect/expand/jump/narrow/exclude/apply/resolve and view-provenance/jump/toggle/reason/resolve actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5WriteScopePreviewTreeVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the import preview consumer is narrowed to Preview pending
/// out-of-workspace-scope parity proof across every headless import path; every consumer
/// stays visible.
pub fn seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed(
) -> M5WriteScopePreviewTreePacket {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.packet_id =
        "m5-write-scope-preview-tree-primitive:import-preview-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WriteScopeConsumerSurface::ImportPreview)
        .expect("import-preview row present");
    row.qualification = M5HistoryQualificationClass::Preview;
    packet
}

/// Narrowed variant: the AI apply preview consumer is held at Beta because a slice of
/// AI-apply previews do not yet render the actor-provenance cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed(
) -> M5WriteScopePreviewTreePacket {
    let mut packet = seeded_m5_write_scope_preview_tree_packet();
    packet.packet_id =
        "m5-write-scope-preview-tree-primitive:ai-apply-preview-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WriteScopeConsumerSurface::AiApplyPreview)
        .expect("ai-apply-preview row present");
    row.qualification = M5HistoryQualificationClass::Beta;
    packet
}
