//! Canonical seed builders for the M5 migration-bridge-card primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical migration-bridge-card primitive packet.
pub const M5_MIGRATION_BRIDGE_CARD_PACKET_ID: &str =
    "m5-migration-bridge-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked migration-bridge-card resolution case from a full imported-behavior state.
#[allow(clippy::too_many_arguments)]
fn bridge_case(
    mapping_class: M5MigrationMappingClass,
    source_tool: M5SourceToolClass,
    old_path_ref: &str,
    new_command_ref: Option<&str>,
    affected_scope: &str,
    unsupported_edge_cases: &[&str],
    import_created_durable_change: bool,
    rollback_checkpoint_ref: Option<&str>,
    bridge_identity_ref: &str,
) -> M5MigrationBridgeCardResolutionCase {
    M5MigrationBridgeCardResolutionCase::resolved(M5MigrationBridgeCardResolutionInput {
        mapping_class,
        source_tool,
        old_path_ref: old_path_ref.to_owned(),
        new_command_ref: new_command_ref.map(str::to_owned),
        affected_scope: affected_scope.to_owned(),
        unsupported_edge_cases: unsupported_edge_cases
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        import_created_durable_change,
        rollback_checkpoint_ref: rollback_checkpoint_ref.map(str::to_owned),
        bridge_identity_ref: bridge_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full bridge-card anatomy, mapping-class,
/// source-tool, bridge-posture, action, export-field, and accessibility parity every consumer
/// carries.
fn base_row(
    consumer_surface: M5MigrationBridgeConsumerSurface,
    qualification: M5TeachingQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    bridge_examples: Vec<M5MigrationBridgeCardResolutionCase>,
) -> M5MigrationBridgeConsumerRow {
    M5MigrationBridgeConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TeachingSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TeachingDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5MigrationBridgeAnatomyPart::ALL.to_vec(),
        mapping_classes: M5MigrationMappingClass::ALL.to_vec(),
        source_tools: M5SourceToolClass::ALL.to_vec(),
        bridge_postures: M5MigrationBridgePosture::ALL.to_vec(),
        bridge_actions: M5MigrationBridgeAction::ALL.to_vec(),
        export_fields: M5MigrationBridgeExportField::ALL.to_vec(),
        accessibility_routes: M5TeachingAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TeachingConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TeachingDowngradeTrigger::MigrationMappingUnstated,
            M5TeachingDowngradeTrigger::SourceToolUnstated,
            M5TeachingDowngradeTrigger::AlternateStateLabelInvented,
            M5TeachingDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF,
            M5_MIGRATION_BRIDGE_CARD_IMPORTER_OUTCOME_REF,
            M5_MIGRATION_BRIDGE_CARD_ROLLBACK_CHECKPOINT_REF,
        ]),
        bridge_examples,
        masks_mapping_state: false,
        overstates_as_exact_parity: false,
        drops_affected_scope_or_edge_cases: false,
        severs_import_rollback_linkage: false,
    }
}

fn rows() -> Vec<M5MigrationBridgeConsumerRow> {
    use M5MigrationBridgeConsumerSurface as Surface;
    use M5MigrationMappingClass as Mapping;
    use M5SourceToolClass as Source;
    use M5TeachingQualificationClass as Qual;

    vec![
        // 1. Migration report panel — an exact one-to-one mapping the import applied durably
        //    (undo + review available), and an unsupported behavior with no native command that
        //    can only be inspected and reported.
        base_row(
            Surface::MigrationReportPanel,
            Qual::Stable,
            "Migration report panel owner",
            "The migration report panel renders the shared migration bridge card so an exact one-to-one mapping the import applied durably is shown as exact parity with the old shortcut, the new command, the affected scope, and available undo / review actions, and an unsupported behavior with no native command is shown honestly as unsupported-no-mapping — never implied to be parity — with its uncovered edge cases named and a report action",
            "evidence:m5-bridge-card-migration-report-panel:001",
            vec![
                bridge_case(
                    Mapping::Exact,
                    Source::RivalIde,
                    "Ctrl+Shift+P (rival IDE command palette)",
                    Some("command:command-palette.open"),
                    "The command palette open shortcut for the current profile",
                    &[],
                    true,
                    Some("checkpoint:import:command-palette-shortcut:0001"),
                    "bridge:migration-report:command-palette",
                ),
                bridge_case(
                    Mapping::Unsupported,
                    Source::ModalEditor,
                    ":source (modal editor vimscript source command)",
                    None,
                    "A vimscript configuration block from the imported modal editor",
                    &[
                        "Arbitrary vimscript execution has no Aureline equivalent",
                        "Custom vimscript functions are not importable",
                    ],
                    false,
                    None,
                    "bridge:migration-report:vimscript-source",
                ),
            ],
        ),
        // 2. Import diff row — a native Aureline equivalent applied durably (undo + review), and
        //    a partial mapping applied durably whose uncovered edge cases are named alongside an
        //    undo / review and a report action.
        base_row(
            Surface::ImportDiffRow,
            Qual::Stable,
            "Import diff row owner",
            "The import diff row renders the shared migration bridge card so a native Aureline equivalent applied durably is shown as native-equivalent with undo / review actions, and a partial mapping applied durably is shown honestly as partial-coverage — never as exact parity — naming the imported keys it does cover, the affected scope, the edge cases it does not cover, and undo / review / report actions",
            "evidence:m5-bridge-card-import-diff-row:001",
            vec![
                bridge_case(
                    Mapping::Native,
                    Source::LegacyEditor,
                    "editor.action.formatDocument (legacy editor keybinding)",
                    Some("command:editor.format-document"),
                    "The format-document keybinding for the imported keymap",
                    &[],
                    true,
                    Some("checkpoint:import:format-document-keybinding:0001"),
                    "bridge:import-diff:format-document",
                ),
                bridge_case(
                    Mapping::Partial,
                    Source::ImportedKeymap,
                    "3w (imported repeat-count motion)",
                    Some("command:editor.move-word-forward"),
                    "The word-forward motion for the imported keymap",
                    &["Repeat-count prefixes are honored only for motions, not operators"],
                    true,
                    Some("checkpoint:import:word-motion-keybinding:0001"),
                    "bridge:import-diff:repeat-count-motion",
                ),
            ],
        ),
        // 3. First-run switch summary — a bridge approximation that changed nothing durable (no
        //    undo needed, only inspect / open native), and a shimmed compatibility applied
        //    durably whose shim edge cases are named with undo / review / report actions.
        base_row(
            Surface::FirstRunSwitchSummary,
            Qual::Stable,
            "First-run switch summary owner",
            "The first-run switch summary renders the shared migration bridge card so a bridge that only approximates the imported behavior and changed nothing durable is shown honestly as bridged-approximation — offering inspect and open-native actions but no undo it does not need — and a shimmed compatibility applied durably is shown as shimmed-compatibility with its shim edge cases named and undo / review / report actions available",
            "evidence:m5-bridge-card-first-run-switch-summary:001",
            vec![
                bridge_case(
                    Mapping::Bridge,
                    Source::RivalIde,
                    "Alt+Click multi-cursor (rival IDE gesture)",
                    Some("command:editor.add-cursor-at-pointer"),
                    "The multi-cursor pointer gesture; no durable setting is written",
                    &[],
                    false,
                    None,
                    "bridge:first-run:multi-cursor-gesture",
                ),
                bridge_case(
                    Mapping::Shimmed,
                    Source::MigratedWorkflowConfig,
                    "tasks.json build task (migrated workflow config)",
                    Some("command:tasks.run-build"),
                    "The build-task runner mapped through the workflow-config shim",
                    &["The shim ignores nested per-folder task profiles"],
                    true,
                    Some("checkpoint:import:build-task-config:0001"),
                    "bridge:first-run:build-task-shim",
                ),
            ],
        ),
        // 4. Keybinding migration notice — a partial keybinding mapping applied durably (undo /
        //    review / report), and a native keybinding equivalent applied durably (undo /
        //    review).
        base_row(
            Surface::KeybindingMigrationNotice,
            Qual::Stable,
            "Keybinding migration notice owner",
            "The keybinding migration notice renders the shared migration bridge card so a partial keybinding mapping applied durably is shown honestly as partial-coverage with the edge cases it does not cover named and undo / review / report actions available, and a native keybinding equivalent applied durably is shown as native-equivalent with undo / review actions — every durable keybinding import keeps its undo path",
            "evidence:m5-bridge-card-keybinding-migration-notice:001",
            vec![
                bridge_case(
                    Mapping::Partial,
                    Source::ModalEditor,
                    "Ctrl+V visual-block (modal editor)",
                    Some("command:editor.column-selection"),
                    "The column-selection keybinding for the current profile",
                    &["Visual-block insert-at-end is only partially supported"],
                    true,
                    Some("checkpoint:import:column-selection-keybinding:0001"),
                    "bridge:keybinding:visual-block",
                ),
                bridge_case(
                    Mapping::Native,
                    Source::RivalIde,
                    "F2 rename symbol (rival IDE)",
                    Some("command:editor.rename-symbol"),
                    "The rename-symbol keybinding for the current profile",
                    &[],
                    true,
                    Some("checkpoint:import:rename-symbol-keybinding:0001"),
                    "bridge:keybinding:rename-symbol",
                ),
            ],
        ),
        // 5. Support migration export — a bridge approximation applied durably (proving undo
        //    stays available for an approximated durable change), and an unsupported behavior
        //    with no native command whose edge cases survive the export.
        base_row(
            Surface::SupportMigrationExport,
            Qual::Stable,
            "Support migration export owner",
            "The support migration export renders the shared migration bridge card so a bridge approximation that changed a durable snippet is shown as bridged-approximation with undo / review actions still available — proving undo survives export for an approximated durable change — and an unsupported behavior with no native command is exported honestly as unsupported-no-mapping with its uncovered edge cases intact and no raw imported config leaking",
            "evidence:m5-bridge-card-support-migration-export:001",
            vec![
                bridge_case(
                    Mapping::Bridge,
                    Source::LegacyEditor,
                    "snippet prefix expansion (legacy editor snippets)",
                    Some("command:snippets.insert"),
                    "One imported snippet body, approximated into a native snippet",
                    &[],
                    true,
                    Some("checkpoint:import:snippet-body:0001"),
                    "bridge:support:snippet-expansion",
                ),
                bridge_case(
                    Mapping::Unsupported,
                    Source::UnknownSource,
                    "proprietary plugin hook (unknown source)",
                    None,
                    "A proprietary plugin hook from an unrecognized source",
                    &["The proprietary plugin API is not importable into Aureline"],
                    false,
                    None,
                    "bridge:support:proprietary-plugin-hook",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5MigrationBridgeCardGovernanceReview {
    M5MigrationBridgeCardGovernanceReview {
        bridge_card_shows_old_path: true,
        bridge_card_shows_new_command: true,
        bridge_card_shows_mapping_state: true,
        bridge_card_shows_affected_scope: true,
        bridge_card_shows_unsupported_edge_cases: true,
        imported_users_never_mistake_partial_for_exact: true,
        bridge_card_never_masks_mapping_state: true,
        undo_review_available_where_import_changed_durable_behavior: true,
        bridge_card_preserves_import_rollback_linkage: true,
        bridge_card_names_imported_source_tool: true,
        users_understand_imported_behavior_without_detached_docs: true,
        bridge_cards_stable_across_deployment_lines: true,
        bridge_cards_stable_across_consumer_surfaces: true,
        every_bridge_card_declares_accessibility_route: true,
        support_export_reconstructs_bridge_truth: true,
        later_rows_cannot_invent_parallel_bridge_vocabulary: true,
    }
}

fn consumer_projection() -> M5MigrationBridgeCardConsumerProjection {
    M5MigrationBridgeCardConsumerProjection {
        migration_surfaces_consume_bridge_vocabulary: true,
        bridge_posture_reads_single_source: true,
        action_set_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5MigrationBridgeCardProofFreshness {
    M5MigrationBridgeCardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MigrationBridgeCardReleasePosture {
    M5MigrationBridgeCardReleasePosture {
        release_packet_ref: M5_MIGRATION_BRIDGE_CARD_ARTIFACT_REF.to_owned(),
        migration_bridge_audit_ref: M5_MIGRATION_BRIDGE_CARD_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF,
        M5_MIGRATION_BRIDGE_CARD_DOC_REF,
        M5_MIGRATION_BRIDGE_CARD_COMPONENT_MATRIX_REF,
        M5_MIGRATION_BRIDGE_CARD_IMPORTER_OUTCOME_REF,
        M5_MIGRATION_BRIDGE_CARD_ROLLBACK_CHECKPOINT_REF,
    ])
}

/// Builds the canonical M5 migration-bridge-card packet.
pub fn seeded_m5_migration_bridge_card_packet() -> M5MigrationBridgeCardPacket {
    M5MigrationBridgeCardPacket::new(M5MigrationBridgeCardPacketInput {
        packet_id: M5_MIGRATION_BRIDGE_CARD_PACKET_ID.to_owned(),
        matrix_label:
            "M5 migration-bridge-card primitive: migration mapping class, imported source tool, old-path reference, new-command reference, affected scope, unsupported edge cases, import rollback linkage, derived bridge posture (exact-parity/native-equivalent/bridged-approximation/shimmed-compatibility/partial-coverage/unsupported-no-mapping), and bounded view-mapping-details/open-native-command/undo-import-changes/review-import-checkpoint/report-unsupported-edge-case actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5MigrationBridgeCardVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the keybinding migration notice consumer is held at Beta because a slice
/// of imported keybindings does not yet render the source-tool cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed(
) -> M5MigrationBridgeCardPacket {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.packet_id =
        "m5-migration-bridge-card-primitive:keybinding-migration-notice-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5MigrationBridgeConsumerSurface::KeybindingMigrationNotice
        })
        .expect("keybinding-migration-notice row present");
    row.qualification = M5TeachingQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support migration export consumer is narrowed to Preview pending
/// affected-scope / rollback-linkage parity proof across every deployment; every consumer stays
/// visible.
pub fn seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed(
) -> M5MigrationBridgeCardPacket {
    let mut packet = seeded_m5_migration_bridge_card_packet();
    packet.packet_id =
        "m5-migration-bridge-card-primitive:support-migration-export-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5MigrationBridgeConsumerSurface::SupportMigrationExport
        })
        .expect("support-migration-export row present");
    row.qualification = M5TeachingQualificationClass::Preview;
    packet
}
