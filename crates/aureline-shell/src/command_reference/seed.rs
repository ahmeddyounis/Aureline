//! Deterministic seeded catalog used by the live shell detail
//! surface, the markdown parity report, and the JSON fixtures
//! checked in under
//! `fixtures/ux/m3/command_reference_and_discoverability/`.
//!
//! Every field is pinned to a constant so the seed mints the
//! same record on every host, every revision, every OS.

use super::{
    AliasKind, AliasLifecycleState, AliasReference, ArgumentSchemaSlot, AutomationEligibility,
    AutomationLabel, AvailabilitySection, CommandReferenceCatalog, CommandReferenceEntry,
    DeprecationRecord, DiscoverabilityLink, ImportImpactClass, KeybindingFact, KeybindingState,
    PlatformVariant, ReferenceIdempotencyClass, ReferenceLifecycleState, ReferencePreviewClass,
    ReferenceRiskClass, ReferenceSurfaceFamily, SearchIndexToken, SearchTokenClass,
    COMMAND_REFERENCE_CATALOG_ID, COMMAND_REFERENCE_CATALOG_RECORD_KIND,
    COMMAND_REFERENCE_DESCRIPTOR_SCHEMA_REF, COMMAND_REFERENCE_ENTRY_RECORD_KIND,
    COMMAND_REFERENCE_GENERATED_AT, COMMAND_REFERENCE_SCHEMA_VERSION,
    COMMAND_REFERENCE_SHARED_CONTRACT_REF,
};

fn open_folder_entry() -> CommandReferenceEntry {
    CommandReferenceEntry {
        record_kind: COMMAND_REFERENCE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: COMMAND_REFERENCE_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_REFERENCE_SHARED_CONTRACT_REF.to_owned(),
        command_id: "cmd:workspace.open_folder".to_owned(),
        command_revision_ref: "cmd-rev:workspace.open_folder:2026.04.21-01".to_owned(),
        canonical_verb: "workspace.open_folder".to_owned(),
        primary_label_ref: "label:workspace.open_folder:primary".to_owned(),
        title: "Open Folder".to_owned(),
        summary: "Open a folder as the active workspace root.".to_owned(),
        lifecycle_state: ReferenceLifecycleState::Stable,
        origin_class: "core".to_owned(),
        risk_class: ReferenceRiskClass::ReversibleLocalMutation,
        preview_class: ReferencePreviewClass::NoPreviewRequired,
        idempotency_class: ReferenceIdempotencyClass::Idempotent,
        supports_dry_run: false,
        aliases: vec![
            AliasReference {
                alias_id: "alias:workspace.open_folder:cli_open".to_owned(),
                alias_kind: AliasKind::AlternateCliVerb,
                lifecycle_state: AliasLifecycleState::Active,
                introduced_version: Some("2026.04".to_owned()),
                retirement_version: None,
                replacement_command_id: None,
                replacement_note_ref: None,
                import_impact_class: Some(ImportImpactClass::NoActionRequired),
            },
            AliasReference {
                alias_id: "alias:workspace.open_folder:legacy_file_open_folder".to_owned(),
                alias_kind: AliasKind::LegacyCommandId,
                lifecycle_state: AliasLifecycleState::Deprecated,
                introduced_version: Some("2025.10".to_owned()),
                retirement_version: Some("2026.10".to_owned()),
                replacement_command_id: Some("cmd:workspace.open_folder".to_owned()),
                replacement_note_ref: Some(
                    "docs:anchor:migration:legacy_file_open_folder".to_owned(),
                ),
                import_impact_class: Some(ImportImpactClass::RewriteRecipe),
            },
        ],
        deprecation: DeprecationRecord::active(),
        argument_schema: vec![
            ArgumentSchemaSlot {
                argument_name: "workspace_scope_ref".to_owned(),
                argument_kind: "workspace_scope_ref".to_owned(),
                is_required: true,
                default_provenance_when_omitted: None,
                narration_label_ref: "label:workspace.open_folder.argument:workspace_scope_ref"
                    .to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            },
            ArgumentSchemaSlot {
                argument_name: "add_to_workspace".to_owned(),
                argument_kind: "bool".to_owned(),
                is_required: false,
                default_provenance_when_omitted: Some("default_from_descriptor".to_owned()),
                narration_label_ref: "label:workspace.open_folder.argument:add_to_workspace"
                    .to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            },
        ],
        availability: AvailabilitySection {
            supported_surfaces: vec![
                ReferenceSurfaceFamily::CommandPalette,
                ReferenceSurfaceFamily::MenuOrButton,
                ReferenceSurfaceFamily::KeybindingHelp,
                ReferenceSurfaceFamily::CliHeadless,
                ReferenceSurfaceFamily::AiToolSurface,
                ReferenceSurfaceFamily::DocsHelp,
                ReferenceSurfaceFamily::Onboarding,
            ],
            trust_gate_class: "trusted_workspace_required".to_owned(),
            policy_gate_class: "no_policy_gate".to_owned(),
            dependency_presence_class: "no_dependency_required".to_owned(),
            current_disabled_reason_codes: Vec::new(),
            current_disabled_reason_explanation_refs: Vec::new(),
        },
        keybindings: vec![
            KeybindingFact {
                chord_ref: "chord:cmd+o".to_owned(),
                platform_variant: PlatformVariant::Macos,
                binding_state: KeybindingState::Default,
                shadowed_by_chord_ref: None,
                shadowed_by_command_id: None,
            },
            KeybindingFact {
                chord_ref: "chord:ctrl+o".to_owned(),
                platform_variant: PlatformVariant::Windows,
                binding_state: KeybindingState::Default,
                shadowed_by_chord_ref: None,
                shadowed_by_command_id: None,
            },
            KeybindingFact {
                chord_ref: "chord:ctrl+o".to_owned(),
                platform_variant: PlatformVariant::Linux,
                binding_state: KeybindingState::Default,
                shadowed_by_chord_ref: None,
                shadowed_by_command_id: None,
            },
        ],
        automation: AutomationEligibility {
            headless_eligible: true,
            recipe_eligible: true,
            macro_eligible: true,
            ai_eligible: true,
            automation_labels: vec![
                AutomationLabel::HeadlessSafe,
                AutomationLabel::RecipeSafe,
                AutomationLabel::MacroSafe,
                AutomationLabel::AiCallableWithApproval,
            ],
        },
        search_index: vec![
            SearchIndexToken {
                token_class: SearchTokenClass::HumanLabel,
                value: "Open Folder".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CommandId,
                value: "cmd:workspace.open_folder".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CanonicalVerb,
                value: "workspace.open_folder".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::AliasId,
                value: "alias:workspace.open_folder:cli_open".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::AliasId,
                value: "alias:workspace.open_folder:legacy_file_open_folder".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::KeySequence,
                value: "chord:cmd+o".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::KeySequence,
                value: "chord:ctrl+o".to_owned(),
            },
        ],
        discoverability_links: vec![
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::DocsHelp,
                anchor_ref: "docs:anchor:workspace:open_folder_overview".to_owned(),
            },
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::Onboarding,
                anchor_ref: "onboarding:tip:workspace:open_folder".to_owned(),
            },
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::CommandPalette,
                anchor_ref: "palette:row:workspace.open_folder".to_owned(),
            },
        ],
        docs_help_anchor_ref: "docs:anchor:workspace:open_folder_overview".to_owned(),
        migration_notes_refs: vec![
            "docs:anchor:migration:legacy_file_open_folder".to_owned(),
        ],
        generated_at: COMMAND_REFERENCE_GENERATED_AT.to_owned(),
    }
}

fn clone_repository_entry() -> CommandReferenceEntry {
    CommandReferenceEntry {
        record_kind: COMMAND_REFERENCE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: COMMAND_REFERENCE_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_REFERENCE_SHARED_CONTRACT_REF.to_owned(),
        command_id: "cmd:workspace.clone_repository".to_owned(),
        command_revision_ref: "cmd-rev:workspace.clone_repository:2026.04.22-01".to_owned(),
        canonical_verb: "workspace.clone_repository".to_owned(),
        primary_label_ref: "label:workspace.clone_repository:primary".to_owned(),
        title: "Clone Repository".to_owned(),
        summary: "Clone a remote repository into a new workspace.".to_owned(),
        lifecycle_state: ReferenceLifecycleState::Stable,
        origin_class: "core".to_owned(),
        risk_class: ReferenceRiskClass::RecoverableDurableMutation,
        preview_class: ReferencePreviewClass::StructuredDiffPreview,
        idempotency_class: ReferenceIdempotencyClass::NonIdempotentObservableOnly,
        supports_dry_run: true,
        aliases: vec![AliasReference {
            alias_id: "alias:workspace.clone_repository:cli_clone".to_owned(),
            alias_kind: AliasKind::AlternateCliVerb,
            lifecycle_state: AliasLifecycleState::Active,
            introduced_version: Some("2026.04".to_owned()),
            retirement_version: None,
            replacement_command_id: None,
            replacement_note_ref: None,
            import_impact_class: Some(ImportImpactClass::NoActionRequired),
        }],
        deprecation: DeprecationRecord::active(),
        argument_schema: vec![
            ArgumentSchemaSlot {
                argument_name: "remote_url".to_owned(),
                argument_kind: "remote_url".to_owned(),
                is_required: true,
                default_provenance_when_omitted: None,
                narration_label_ref: "label:workspace.clone_repository.argument:remote_url"
                    .to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: vec!["restricted".to_owned()],
            },
            ArgumentSchemaSlot {
                argument_name: "destination_scope_ref".to_owned(),
                argument_kind: "workspace_scope_ref".to_owned(),
                is_required: true,
                default_provenance_when_omitted: None,
                narration_label_ref:
                    "label:workspace.clone_repository.argument:destination_scope_ref".to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            },
        ],
        availability: AvailabilitySection {
            supported_surfaces: vec![
                ReferenceSurfaceFamily::CommandPalette,
                ReferenceSurfaceFamily::MenuOrButton,
                ReferenceSurfaceFamily::KeybindingHelp,
                ReferenceSurfaceFamily::CliHeadless,
                ReferenceSurfaceFamily::AiToolSurface,
                ReferenceSurfaceFamily::DocsHelp,
            ],
            trust_gate_class: "trusted_workspace_required".to_owned(),
            policy_gate_class: "network_egress_allowed".to_owned(),
            dependency_presence_class: "provider_linked_required".to_owned(),
            current_disabled_reason_codes: vec![
                "provider_not_linked".to_owned(),
                "network_egress_blocked".to_owned(),
            ],
            current_disabled_reason_explanation_refs: vec![
                "reason:workspace.clone_repository:provider_not_linked".to_owned(),
                "reason:workspace.clone_repository:network_egress_blocked".to_owned(),
            ],
        },
        keybindings: vec![KeybindingFact {
            chord_ref: "chord:unassigned".to_owned(),
            platform_variant: PlatformVariant::All,
            binding_state: KeybindingState::Unassigned,
            shadowed_by_chord_ref: None,
            shadowed_by_command_id: None,
        }],
        automation: AutomationEligibility {
            headless_eligible: true,
            recipe_eligible: true,
            macro_eligible: false,
            ai_eligible: true,
            automation_labels: vec![
                AutomationLabel::HeadlessSafe,
                AutomationLabel::RecipeSafe,
                AutomationLabel::AiCallableWithApproval,
            ],
        },
        search_index: vec![
            SearchIndexToken {
                token_class: SearchTokenClass::HumanLabel,
                value: "Clone Repository".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CommandId,
                value: "cmd:workspace.clone_repository".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CanonicalVerb,
                value: "workspace.clone_repository".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::AliasId,
                value: "alias:workspace.clone_repository:cli_clone".to_owned(),
            },
        ],
        discoverability_links: vec![
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::DocsHelp,
                anchor_ref: "docs:anchor:workspace:clone_repository_overview".to_owned(),
            },
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::CommandPalette,
                anchor_ref: "palette:row:workspace.clone_repository".to_owned(),
            },
        ],
        docs_help_anchor_ref: "docs:anchor:workspace:clone_repository_overview".to_owned(),
        migration_notes_refs: Vec::new(),
        generated_at: COMMAND_REFERENCE_GENERATED_AT.to_owned(),
    }
}

fn import_profile_entry() -> CommandReferenceEntry {
    CommandReferenceEntry {
        record_kind: COMMAND_REFERENCE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: COMMAND_REFERENCE_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_REFERENCE_SHARED_CONTRACT_REF.to_owned(),
        command_id: "cmd:workspace.import_profile".to_owned(),
        command_revision_ref: "cmd-rev:workspace.import_profile:2026.04.22-01".to_owned(),
        canonical_verb: "workspace.import_profile".to_owned(),
        primary_label_ref: "label:workspace.import_profile:primary".to_owned(),
        title: "Import Profile".to_owned(),
        summary: "Import a profile bundle and apply it as a workspace preset.".to_owned(),
        lifecycle_state: ReferenceLifecycleState::Beta,
        origin_class: "core".to_owned(),
        risk_class: ReferenceRiskClass::RecoverableDurableMutation,
        preview_class: ReferencePreviewClass::StructuredDiffPreview,
        idempotency_class: ReferenceIdempotencyClass::NonIdempotentObservableOnly,
        supports_dry_run: true,
        aliases: vec![AliasReference {
            alias_id: "alias:workspace.import_profile:cli_import_profile".to_owned(),
            alias_kind: AliasKind::AlternateCliVerb,
            lifecycle_state: AliasLifecycleState::Active,
            introduced_version: Some("2026.04".to_owned()),
            retirement_version: None,
            replacement_command_id: None,
            replacement_note_ref: None,
            import_impact_class: Some(ImportImpactClass::NoActionRequired),
        }],
        deprecation: DeprecationRecord::active(),
        argument_schema: vec![
            ArgumentSchemaSlot {
                argument_name: "import_source_ref".to_owned(),
                argument_kind: "import_source_ref".to_owned(),
                is_required: true,
                default_provenance_when_omitted: None,
                narration_label_ref: "label:workspace.import_profile.argument:import_source_ref"
                    .to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            },
            ArgumentSchemaSlot {
                argument_name: "apply_scope".to_owned(),
                argument_kind: "enum".to_owned(),
                is_required: false,
                default_provenance_when_omitted: Some("default_from_descriptor".to_owned()),
                narration_label_ref: "label:workspace.import_profile.argument:apply_scope"
                    .to_owned(),
                enum_value_refs: vec![
                    "profile_only".to_owned(),
                    "profile_and_keymap".to_owned(),
                    "profile_keymap_and_settings".to_owned(),
                ],
                policy_pinned_when_trust_state_is: Vec::new(),
            },
            ArgumentSchemaSlot {
                argument_name: "create_restore_checkpoint".to_owned(),
                argument_kind: "bool".to_owned(),
                is_required: false,
                default_provenance_when_omitted: Some("default_from_descriptor".to_owned()),
                narration_label_ref:
                    "label:workspace.import_profile.argument:create_restore_checkpoint".to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            },
        ],
        availability: AvailabilitySection {
            supported_surfaces: vec![
                ReferenceSurfaceFamily::CommandPalette,
                ReferenceSurfaceFamily::MenuOrButton,
                ReferenceSurfaceFamily::KeybindingHelp,
                ReferenceSurfaceFamily::CliHeadless,
                ReferenceSurfaceFamily::AiToolSurface,
                ReferenceSurfaceFamily::DocsHelp,
            ],
            trust_gate_class: "trusted_workspace_required".to_owned(),
            policy_gate_class: "labs_beta_gate".to_owned(),
            dependency_presence_class: "no_dependency_required".to_owned(),
            current_disabled_reason_codes: vec!["labs_beta_disabled".to_owned()],
            current_disabled_reason_explanation_refs: vec![
                "reason:workspace.import_profile:labs_beta_disabled".to_owned(),
            ],
        },
        keybindings: vec![KeybindingFact {
            chord_ref: "chord:unassigned".to_owned(),
            platform_variant: PlatformVariant::All,
            binding_state: KeybindingState::Unassigned,
            shadowed_by_chord_ref: None,
            shadowed_by_command_id: None,
        }],
        automation: AutomationEligibility {
            headless_eligible: true,
            recipe_eligible: true,
            macro_eligible: false,
            ai_eligible: true,
            automation_labels: vec![
                AutomationLabel::HeadlessSafe,
                AutomationLabel::RecipeSafe,
                AutomationLabel::AiCallableWithApproval,
            ],
        },
        search_index: vec![
            SearchIndexToken {
                token_class: SearchTokenClass::HumanLabel,
                value: "Import Profile".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CommandId,
                value: "cmd:workspace.import_profile".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CanonicalVerb,
                value: "workspace.import_profile".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::AliasId,
                value: "alias:workspace.import_profile:cli_import_profile".to_owned(),
            },
        ],
        discoverability_links: vec![
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::DocsHelp,
                anchor_ref: "docs:anchor:migration:import_profile_overview".to_owned(),
            },
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::Onboarding,
                anchor_ref: "onboarding:tip:migration:import_profile".to_owned(),
            },
        ],
        docs_help_anchor_ref: "docs:anchor:migration:import_profile_overview".to_owned(),
        migration_notes_refs: vec![
            "docs:anchor:migration:import_profile_overview".to_owned(),
        ],
        generated_at: COMMAND_REFERENCE_GENERATED_AT.to_owned(),
    }
}

fn restore_from_checkpoint_entry() -> CommandReferenceEntry {
    CommandReferenceEntry {
        record_kind: COMMAND_REFERENCE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: COMMAND_REFERENCE_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_REFERENCE_SHARED_CONTRACT_REF.to_owned(),
        command_id: "cmd:workspace.restore_from_checkpoint".to_owned(),
        command_revision_ref: "cmd-rev:workspace.restore_from_checkpoint:2026.04.22-01".to_owned(),
        canonical_verb: "workspace.restore_from_checkpoint".to_owned(),
        primary_label_ref: "label:workspace.restore_from_checkpoint:primary".to_owned(),
        title: "Restore From Checkpoint".to_owned(),
        summary: "Roll the workspace back to a recorded checkpoint.".to_owned(),
        lifecycle_state: ReferenceLifecycleState::Beta,
        origin_class: "core".to_owned(),
        risk_class: ReferenceRiskClass::DestructiveBulkMutation,
        preview_class: ReferencePreviewClass::DestructiveBulkMutationPreview,
        idempotency_class: ReferenceIdempotencyClass::NonIdempotentDestructive,
        supports_dry_run: true,
        aliases: vec![AliasReference {
            alias_id: "alias:workspace.restore_from_checkpoint:cli_restore".to_owned(),
            alias_kind: AliasKind::AlternateCliVerb,
            lifecycle_state: AliasLifecycleState::Active,
            introduced_version: Some("2026.04".to_owned()),
            retirement_version: None,
            replacement_command_id: None,
            replacement_note_ref: None,
            import_impact_class: Some(ImportImpactClass::NoActionRequired),
        }],
        deprecation: DeprecationRecord::active(),
        argument_schema: vec![
            ArgumentSchemaSlot {
                argument_name: "checkpoint_ref".to_owned(),
                argument_kind: "checkpoint_ref".to_owned(),
                is_required: true,
                default_provenance_when_omitted: None,
                narration_label_ref:
                    "label:workspace.restore_from_checkpoint.argument:checkpoint_ref".to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            },
            ArgumentSchemaSlot {
                argument_name: "confirm_destructive".to_owned(),
                argument_kind: "bool".to_owned(),
                is_required: true,
                default_provenance_when_omitted: None,
                narration_label_ref:
                    "label:workspace.restore_from_checkpoint.argument:confirm_destructive"
                        .to_owned(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            },
        ],
        availability: AvailabilitySection {
            supported_surfaces: vec![
                ReferenceSurfaceFamily::CommandPalette,
                ReferenceSurfaceFamily::KeybindingHelp,
                ReferenceSurfaceFamily::CliHeadless,
                ReferenceSurfaceFamily::DocsHelp,
            ],
            trust_gate_class: "trusted_workspace_required".to_owned(),
            policy_gate_class: "destructive_action_review_required".to_owned(),
            dependency_presence_class: "checkpoint_present_required".to_owned(),
            current_disabled_reason_codes: vec![
                "no_checkpoint_available".to_owned(),
                "destructive_review_pending".to_owned(),
            ],
            current_disabled_reason_explanation_refs: vec![
                "reason:workspace.restore_from_checkpoint:no_checkpoint_available".to_owned(),
                "reason:workspace.restore_from_checkpoint:destructive_review_pending".to_owned(),
            ],
        },
        keybindings: vec![KeybindingFact {
            chord_ref: "chord:unassigned".to_owned(),
            platform_variant: PlatformVariant::All,
            binding_state: KeybindingState::Unassigned,
            shadowed_by_chord_ref: None,
            shadowed_by_command_id: None,
        }],
        automation: AutomationEligibility {
            headless_eligible: true,
            recipe_eligible: false,
            macro_eligible: false,
            ai_eligible: false,
            automation_labels: vec![AutomationLabel::HeadlessSafe, AutomationLabel::AiNotCallable],
        },
        search_index: vec![
            SearchIndexToken {
                token_class: SearchTokenClass::HumanLabel,
                value: "Restore From Checkpoint".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CommandId,
                value: "cmd:workspace.restore_from_checkpoint".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CanonicalVerb,
                value: "workspace.restore_from_checkpoint".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::AliasId,
                value: "alias:workspace.restore_from_checkpoint:cli_restore".to_owned(),
            },
        ],
        discoverability_links: vec![
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::DocsHelp,
                anchor_ref: "docs:anchor:workspace:restore_from_checkpoint_overview".to_owned(),
            },
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::CommandPalette,
                anchor_ref: "palette:row:workspace.restore_from_checkpoint".to_owned(),
            },
        ],
        docs_help_anchor_ref: "docs:anchor:workspace:restore_from_checkpoint_overview".to_owned(),
        migration_notes_refs: Vec::new(),
        generated_at: COMMAND_REFERENCE_GENERATED_AT.to_owned(),
    }
}

fn command_palette_open_entry() -> CommandReferenceEntry {
    CommandReferenceEntry {
        record_kind: COMMAND_REFERENCE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: COMMAND_REFERENCE_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_REFERENCE_SHARED_CONTRACT_REF.to_owned(),
        command_id: "cmd:command_palette.open".to_owned(),
        command_revision_ref: "cmd-rev:command_palette.open:2026.04.22-01".to_owned(),
        canonical_verb: "command_palette.open".to_owned(),
        primary_label_ref: "label:command_palette.open:primary".to_owned(),
        title: "Open Command Palette".to_owned(),
        summary: "Open the command palette to search and run any command.".to_owned(),
        lifecycle_state: ReferenceLifecycleState::Stable,
        origin_class: "core".to_owned(),
        risk_class: ReferenceRiskClass::InertMetadataOnly,
        preview_class: ReferencePreviewClass::NoPreviewRequired,
        idempotency_class: ReferenceIdempotencyClass::Idempotent,
        supports_dry_run: false,
        aliases: vec![AliasReference {
            alias_id: "alias:command_palette.open:vscode_show_all_commands".to_owned(),
            alias_kind: AliasKind::AlternatePalettePhrasing,
            lifecycle_state: AliasLifecycleState::Active,
            introduced_version: Some("2026.04".to_owned()),
            retirement_version: None,
            replacement_command_id: None,
            replacement_note_ref: Some(
                "docs:anchor:migration:command_palette_show_all_commands".to_owned(),
            ),
            import_impact_class: Some(ImportImpactClass::NoActionRequired),
        }],
        deprecation: DeprecationRecord::active(),
        argument_schema: Vec::new(),
        availability: AvailabilitySection {
            supported_surfaces: vec![
                ReferenceSurfaceFamily::CommandPalette,
                ReferenceSurfaceFamily::KeybindingHelp,
                ReferenceSurfaceFamily::DocsHelp,
                ReferenceSurfaceFamily::Onboarding,
            ],
            trust_gate_class: "no_trust_gate".to_owned(),
            policy_gate_class: "no_policy_gate".to_owned(),
            dependency_presence_class: "no_dependency_required".to_owned(),
            current_disabled_reason_codes: Vec::new(),
            current_disabled_reason_explanation_refs: Vec::new(),
        },
        keybindings: vec![
            KeybindingFact {
                chord_ref: "chord:cmd+shift+p".to_owned(),
                platform_variant: PlatformVariant::Macos,
                binding_state: KeybindingState::Default,
                shadowed_by_chord_ref: None,
                shadowed_by_command_id: None,
            },
            KeybindingFact {
                chord_ref: "chord:ctrl+shift+p".to_owned(),
                platform_variant: PlatformVariant::Windows,
                binding_state: KeybindingState::Default,
                shadowed_by_chord_ref: None,
                shadowed_by_command_id: None,
            },
            KeybindingFact {
                chord_ref: "chord:ctrl+shift+p".to_owned(),
                platform_variant: PlatformVariant::Linux,
                binding_state: KeybindingState::Default,
                shadowed_by_chord_ref: None,
                shadowed_by_command_id: None,
            },
        ],
        automation: AutomationEligibility {
            headless_eligible: false,
            recipe_eligible: false,
            macro_eligible: false,
            ai_eligible: false,
            automation_labels: vec![AutomationLabel::UiOnly, AutomationLabel::AiNotCallable],
        },
        search_index: vec![
            SearchIndexToken {
                token_class: SearchTokenClass::HumanLabel,
                value: "Open Command Palette".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CommandId,
                value: "cmd:command_palette.open".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::CanonicalVerb,
                value: "command_palette.open".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::AliasId,
                value: "alias:command_palette.open:vscode_show_all_commands".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::KeySequence,
                value: "chord:cmd+shift+p".to_owned(),
            },
            SearchIndexToken {
                token_class: SearchTokenClass::KeySequence,
                value: "chord:ctrl+shift+p".to_owned(),
            },
        ],
        discoverability_links: vec![
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::DocsHelp,
                anchor_ref: "docs:anchor:command_palette:open_overview".to_owned(),
            },
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::Onboarding,
                anchor_ref: "onboarding:tip:command_palette:open".to_owned(),
            },
            DiscoverabilityLink {
                surface_family: ReferenceSurfaceFamily::KeybindingHelp,
                anchor_ref: "keybinding_help:row:command_palette.open".to_owned(),
            },
        ],
        docs_help_anchor_ref: "docs:anchor:command_palette:open_overview".to_owned(),
        migration_notes_refs: vec![
            "docs:anchor:migration:command_palette_show_all_commands".to_owned(),
        ],
        generated_at: COMMAND_REFERENCE_GENERATED_AT.to_owned(),
    }
}

/// Returns the seeded command-reference catalog the shell, parity
/// report, fixtures, and beta-contract doc consume.
pub fn seeded_command_reference_catalog() -> CommandReferenceCatalog {
    let mut entries = vec![
        open_folder_entry(),
        clone_repository_entry(),
        import_profile_entry(),
        restore_from_checkpoint_entry(),
        command_palette_open_entry(),
    ];
    entries.sort_by(|left, right| left.command_id.cmp(&right.command_id));

    CommandReferenceCatalog {
        record_kind: COMMAND_REFERENCE_CATALOG_RECORD_KIND.to_owned(),
        schema_version: COMMAND_REFERENCE_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_REFERENCE_SHARED_CONTRACT_REF.to_owned(),
        catalog_id: COMMAND_REFERENCE_CATALOG_ID.to_owned(),
        source_descriptor_schema_ref: COMMAND_REFERENCE_DESCRIPTOR_SCHEMA_REF.to_owned(),
        entries,
        generated_at: COMMAND_REFERENCE_GENERATED_AT.to_owned(),
    }
}
