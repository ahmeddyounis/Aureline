//! Deterministic command-reference catalog projected from the canonical
//! discoverability export.
//!
//! The catalog intentionally derives from
//! [`aureline_commands::current_command_discoverability_export`] rather than a
//! handwritten per-surface copy, so command reference/help, CLI help, docs/help
//! search, onboarding, and support export all read the same command
//! discoverability truth.

use aureline_commands::{
    current_command_discoverability_export, DiscoverabilityAliasRecord,
    DiscoverabilityAutomationSupportClass, DiscoverabilityCurrentKeybindingRecord,
    ProtectedCommandDiscoverabilityRecord,
};

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

fn lifecycle_state(value: &str) -> ReferenceLifecycleState {
    match value {
        "alpha" => ReferenceLifecycleState::Alpha,
        "beta" | "preview" => ReferenceLifecycleState::Beta,
        "stable" => ReferenceLifecycleState::Stable,
        "lts_facing" => ReferenceLifecycleState::LtsFacing,
        "deprecated" => ReferenceLifecycleState::Deprecated,
        "labs" => ReferenceLifecycleState::Labs,
        _ => ReferenceLifecycleState::Beta,
    }
}

fn risk_class(record: &ProtectedCommandDiscoverabilityRecord) -> ReferenceRiskClass {
    if record
        .automation_support
        .contains(&DiscoverabilityAutomationSupportClass::UiOnly)
        && record.preview_class == "no_preview_required"
    {
        return ReferenceRiskClass::ReversibleLocalMutation;
    }

    match record.capability_scope_class.as_str() {
        "inert_metadata_only" => ReferenceRiskClass::InertMetadataOnly,
        "reversible_local_read" => ReferenceRiskClass::ReversibleLocalRead,
        "reversible_local_mutation" => ReferenceRiskClass::ReversibleLocalMutation,
        "recoverable_durable_mutation" => ReferenceRiskClass::RecoverableDurableMutation,
        "irreversible_high_blast_mutation" => ReferenceRiskClass::IrreversiblePublish,
        "externally_visible_mutation"
        | "credential_or_secret_bearing"
        | "managed_workspace_control"
        | "policy_authoring_or_waiver" => ReferenceRiskClass::IrreversiblePublish,
        _ => ReferenceRiskClass::ReversibleLocalMutation,
    }
}

fn preview_class(value: &str) -> ReferencePreviewClass {
    match value {
        "no_preview_required" => ReferencePreviewClass::NoPreviewRequired,
        "structured_diff_preview" => ReferencePreviewClass::StructuredDiffPreview,
        "destructive_bulk_mutation_preview" => {
            ReferencePreviewClass::DestructiveBulkMutationPreview
        }
        "policy_authoring_or_waiver_preview" => {
            ReferencePreviewClass::PolicyAuthoringOrWaiverPreview
        }
        "irreversible_publish_preview" => ReferencePreviewClass::IrreversiblePublishPreview,
        _ => ReferencePreviewClass::StructuredDiffPreview,
    }
}

fn idempotency_class(record: &ProtectedCommandDiscoverabilityRecord) -> ReferenceIdempotencyClass {
    match record.capability_scope_class.as_str() {
        "inert_metadata_only" | "reversible_local_read" => ReferenceIdempotencyClass::Idempotent,
        "reversible_local_mutation" => ReferenceIdempotencyClass::IdempotentWithVisibleRedirect,
        "recoverable_durable_mutation" => {
            ReferenceIdempotencyClass::NonIdempotentObservableOnly
        }
        _ => ReferenceIdempotencyClass::NonIdempotentDestructive,
    }
}

fn alias_kind(value: &str) -> AliasKind {
    match value {
        "legacy_command_id" => AliasKind::LegacyCommandId,
        "alternate_palette_phrasing" => AliasKind::AlternatePalettePhrasing,
        "alternate_cli_verb" => AliasKind::AlternateCliVerb,
        "ai_tool_handle" => AliasKind::AiToolHandle,
        "keybinding_target" => AliasKind::KeybindingTarget,
        _ => AliasKind::AlternatePalettePhrasing,
    }
}

fn alias_lifecycle_state(value: &str) -> AliasLifecycleState {
    match value {
        "active" => AliasLifecycleState::Active,
        "deprecated" => AliasLifecycleState::Deprecated,
        "retired" => AliasLifecycleState::Retired,
        _ => AliasLifecycleState::Active,
    }
}

fn import_impact(alias: &DiscoverabilityAliasRecord) -> Option<ImportImpactClass> {
    match alias.alias_kind.as_str() {
        "keybinding_target" => Some(ImportImpactClass::RebindKeymap),
        "ai_tool_handle" => Some(ImportImpactClass::AiToolHandleRenames),
        "legacy_command_id" => Some(ImportImpactClass::RewriteRecipe),
        "alternate_cli_verb" => Some(ImportImpactClass::NoActionRequired),
        _ => {
            if alias.alias_state == "retired" {
                Some(ImportImpactClass::RemoveInvocation)
            } else {
                Some(ImportImpactClass::NoActionRequired)
            }
        }
    }
}

fn alias_reference(alias: &DiscoverabilityAliasRecord) -> AliasReference {
    AliasReference {
        alias_id: alias.alias_id.clone(),
        alias_kind: alias_kind(&alias.alias_kind),
        lifecycle_state: alias_lifecycle_state(&alias.alias_state),
        introduced_version: alias.introduced_ref.clone(),
        retirement_version: alias.retired_ref.clone().or_else(|| {
            (alias.alias_state != "active").then(|| "release:aureline:next".to_owned())
        }),
        replacement_command_id: alias.replacement_command_id.clone(),
        replacement_note_ref: alias.notes_ref.clone(),
        import_impact_class: import_impact(alias),
    }
}

fn deprecation(record: &ProtectedCommandDiscoverabilityRecord) -> DeprecationRecord {
    if record.lifecycle_state == "deprecated" {
        DeprecationRecord {
            state: AliasLifecycleState::Deprecated,
            deprecated_in_version: Some(record.command_revision_ref.clone()),
            retires_in_version: Some("release:aureline:next".to_owned()),
            replacement_command_id: record.replacement_command_id.clone(),
            import_impact_class: Some(ImportImpactClass::RewriteRecipe),
            migration_note_ref: record
                .alias_records
                .iter()
                .find_map(|alias| alias.notes_ref.clone()),
        }
    } else {
        DeprecationRecord::active()
    }
}

fn supported_surfaces(record: &ProtectedCommandDiscoverabilityRecord) -> Vec<ReferenceSurfaceFamily> {
    let mut surfaces = vec![
        ReferenceSurfaceFamily::CommandPalette,
        ReferenceSurfaceFamily::KeybindingHelp,
        ReferenceSurfaceFamily::DocsHelp,
        ReferenceSurfaceFamily::Onboarding,
    ];
    if !record.current_keybindings.is_empty() {
        surfaces.push(ReferenceSurfaceFamily::MenuOrButton);
    }
    if record
        .automation_support
        .iter()
        .any(|support| *support == DiscoverabilityAutomationSupportClass::HeadlessSafe)
    {
        surfaces.push(ReferenceSurfaceFamily::CliHeadless);
    }
    if record.ai_tool_surfacing_class != "not_ai_callable" {
        surfaces.push(ReferenceSurfaceFamily::AiToolSurface);
    }
    surfaces.sort();
    surfaces.dedup();
    surfaces
}

fn availability(record: &ProtectedCommandDiscoverabilityRecord) -> AvailabilitySection {
    AvailabilitySection {
        supported_surfaces: supported_surfaces(record),
        trust_gate_class: if record
            .disabled_reason_explanation_refs
            .iter()
            .any(|reason| reason.contains("trust"))
        {
            "trust_gate_present".to_owned()
        } else {
            "no_trust_gate".to_owned()
        },
        policy_gate_class: if record
            .disabled_reason_explanation_refs
            .iter()
            .any(|reason| reason.contains("policy"))
        {
            "policy_gate_present".to_owned()
        } else {
            "no_policy_gate".to_owned()
        },
        dependency_presence_class: if record
            .disabled_reason_explanation_refs
            .iter()
            .any(|reason| reason.contains("provider") || reason.contains("dependency"))
        {
            "dependency_required".to_owned()
        } else {
            "no_dependency_required".to_owned()
        },
        current_disabled_reason_codes: record
            .disabled_reason_explanation_refs
            .iter()
            .map(|reason| {
                reason
                    .split(':')
                    .next_back()
                    .unwrap_or(reason.as_str())
                    .to_owned()
            })
            .collect(),
        current_disabled_reason_explanation_refs: record.disabled_reason_explanation_refs.clone(),
    }
}

fn platform_variant(value: &str) -> PlatformVariant {
    match value {
        "macos" => PlatformVariant::Macos,
        "windows" => PlatformVariant::Windows,
        "linux" => PlatformVariant::Linux,
        _ => PlatformVariant::All,
    }
}

fn binding_state(binding: &DiscoverabilityCurrentKeybindingRecord) -> KeybindingState {
    match binding.display_state.as_str() {
        "unassigned" => KeybindingState::Unassigned,
        "conflict" => KeybindingState::Conflict,
        _ => {
            if binding.imported_from_ref.is_some() {
                KeybindingState::OverridingUserBinding
            } else {
                KeybindingState::Default
            }
        }
    }
}

fn keybindings(record: &ProtectedCommandDiscoverabilityRecord) -> Vec<KeybindingFact> {
    record
        .current_keybindings
        .iter()
        .map(|binding| KeybindingFact {
            chord_ref: chord_ref(&binding.keybinding_ref),
            platform_variant: platform_variant(&binding.platform_class),
            binding_state: binding_state(binding),
            shadowed_by_chord_ref: None,
            shadowed_by_command_id: None,
        })
        .collect()
}

fn chord_ref(value: &str) -> String {
    value
        .rsplit(':')
        .next()
        .map(|chord| format!("chord:{chord}"))
        .unwrap_or_else(|| value.to_owned())
}

fn automation(record: &ProtectedCommandDiscoverabilityRecord) -> AutomationEligibility {
    let labels = record
        .automation_support
        .iter()
        .filter_map(|support| match support {
            DiscoverabilityAutomationSupportClass::UiOnly => Some(AutomationLabel::UiOnly),
            DiscoverabilityAutomationSupportClass::HeadlessSafe => {
                Some(AutomationLabel::HeadlessSafe)
            }
            DiscoverabilityAutomationSupportClass::RecipeSafe => Some(AutomationLabel::RecipeSafe),
            DiscoverabilityAutomationSupportClass::MacroSafe => Some(AutomationLabel::MacroSafe),
            DiscoverabilityAutomationSupportClass::ApprovalRequired => {
                Some(AutomationLabel::AiCallableWithApproval)
            }
            DiscoverabilityAutomationSupportClass::Unknown => None,
        })
        .collect::<Vec<_>>();
    AutomationEligibility {
        headless_eligible: record
            .automation_support
            .contains(&DiscoverabilityAutomationSupportClass::HeadlessSafe),
        recipe_eligible: record
            .automation_support
            .contains(&DiscoverabilityAutomationSupportClass::RecipeSafe),
        macro_eligible: record
            .automation_support
            .contains(&DiscoverabilityAutomationSupportClass::MacroSafe),
        ai_eligible: record.ai_tool_surfacing_class != "not_ai_callable",
        automation_labels: labels,
    }
}

fn search_index(record: &ProtectedCommandDiscoverabilityRecord) -> Vec<SearchIndexToken> {
    let mut tokens = vec![
        SearchIndexToken {
            token_class: SearchTokenClass::HumanLabel,
            value: record.title.clone(),
        },
        SearchIndexToken {
            token_class: SearchTokenClass::CommandId,
            value: record.command_id.clone(),
        },
        SearchIndexToken {
            token_class: SearchTokenClass::CanonicalVerb,
            value: record.canonical_verb.clone(),
        },
    ];
    tokens.extend(
        record
            .promoted_alias_ids
            .iter()
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .map(|value| SearchIndexToken {
                token_class: SearchTokenClass::AliasId,
                value,
            }),
    );
    tokens.extend(
        record
            .current_keybindings
            .iter()
            .filter(|binding| !binding.keybinding_ref.trim().is_empty())
            .cloned()
            .map(|binding| SearchIndexToken {
                token_class: SearchTokenClass::KeySequence,
                value: chord_ref(&binding.keybinding_ref),
            }),
    );
    tokens
}

fn discoverability_links(record: &ProtectedCommandDiscoverabilityRecord) -> Vec<DiscoverabilityLink> {
    vec![
        DiscoverabilityLink {
            surface_family: ReferenceSurfaceFamily::DocsHelp,
            anchor_ref: record.projection_refs.docs_help_page_ref.clone(),
        },
        DiscoverabilityLink {
            surface_family: ReferenceSurfaceFamily::Onboarding,
            anchor_ref: record.projection_refs.onboarding_hint_ref.clone(),
        },
        DiscoverabilityLink {
            surface_family: ReferenceSurfaceFamily::CommandPalette,
            anchor_ref: record.projection_refs.palette_row_ref.clone(),
        },
    ]
}

fn migration_notes(record: &ProtectedCommandDiscoverabilityRecord) -> Vec<String> {
    let mut notes = record
        .alias_records
        .iter()
        .filter_map(|alias| alias.notes_ref.clone())
        .collect::<Vec<_>>();
    notes.push(record.projection_refs.migration_bridge_card_ref.clone());
    notes.sort();
    notes.dedup();
    notes
}

fn entry(record: &ProtectedCommandDiscoverabilityRecord) -> CommandReferenceEntry {
    CommandReferenceEntry {
        record_kind: COMMAND_REFERENCE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: COMMAND_REFERENCE_SCHEMA_VERSION,
        shared_contract_ref: COMMAND_REFERENCE_SHARED_CONTRACT_REF.to_owned(),
        command_id: record.command_id.clone(),
        command_revision_ref: record.command_revision_ref.clone(),
        canonical_verb: record.canonical_verb.clone(),
        primary_label_ref: record.accessibility.primary_label_ref.clone(),
        title: record.title.clone(),
        summary: record.summary.clone(),
        lifecycle_state: lifecycle_state(&record.lifecycle_state),
        origin_class: record.origin_class.clone(),
        risk_class: risk_class(record),
        preview_class: preview_class(&record.preview_class),
        idempotency_class: idempotency_class(record),
        supports_dry_run: record.preview_class != "no_preview_required",
        aliases: record.alias_records.iter().map(alias_reference).collect(),
        deprecation: deprecation(record),
        argument_schema: record
            .typed_arguments
            .iter()
            .map(|argument| ArgumentSchemaSlot {
                argument_name: argument.argument_name.clone(),
                argument_kind: argument.argument_kind.clone(),
                is_required: argument.is_required,
                default_provenance_when_omitted: None,
                narration_label_ref: argument.narration_label_ref.clone(),
                enum_value_refs: Vec::new(),
                policy_pinned_when_trust_state_is: Vec::new(),
            })
            .collect(),
        availability: availability(record),
        keybindings: keybindings(record),
        automation: automation(record),
        search_index: search_index(record),
        discoverability_links: discoverability_links(record),
        docs_help_anchor_ref: record.docs_help_anchor_ref.clone(),
        migration_notes_refs: migration_notes(record),
        generated_at: COMMAND_REFERENCE_GENERATED_AT.to_owned(),
    }
}

/// Returns the seeded command-reference catalog derived from the canonical
/// discoverability packet.
pub fn seeded_command_reference_catalog() -> CommandReferenceCatalog {
    let packet = current_command_discoverability_export()
        .expect("checked discoverability export must validate before command-reference projection");
    let entries = packet
        .commands
        .iter()
        .filter(|record| record.lifecycle_state != "labs")
        .map(entry)
        .collect();
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
