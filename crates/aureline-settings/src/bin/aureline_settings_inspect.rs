//! CLI consumer for the schema-backed settings inspector.

use aureline_settings::experiments::{
    inspect_default_inventory, project_cli_inventory as project_experiments_cli_inventory,
    project_support_export as project_experiments_support_export,
};
use aureline_settings::inspector::{
    inspect_all_settings, inspect_setting, preview_write, project_cli_inspect,
    project_support_export, SettingWritePreviewRequest, SettingsInspectionContext, WriteActorClass,
    WriteReasonClass,
};
use aureline_settings::repair_review::{
    build_repair_plan, project_review_sheet,
    project_support_export as project_repair_support_export, ImportedProfileFragmentRef,
    MigrationStepRef, RepairActionClass, RepairUserDecision, SettingsRepairPlan,
    SettingsRepairPlanRequest,
};
use aureline_settings::sync::{
    build_review_row, project_review_page,
    project_support_export as project_sync_beta_support_export, DeviceParticipationState,
    IdentityModeClass, LastWriterBreadcrumb, SyncBetaDeviceRecord, SyncConflictReviewBetaRequest,
};
use aureline_settings::ui::{
    inspect_setting_pane, project_page_from_records, project_settings_ui_beta_page,
    project_support_export as project_ui_beta_support_export, project_write_composer,
};
use aureline_settings::{
    EffectiveSettingsResolver, PolicyConstraint, SchemaRegistry, ScopeOverlay, SettingScope,
    SettingValue,
};
use serde::Serialize;

#[derive(Serialize)]
struct CliInspectEnvelope<T> {
    record_kind: &'static str,
    shared_contract_ref: &'static str,
    cli_projection: T,
    effective_setting: serde_json::Value,
}

#[derive(Serialize)]
struct CliExperimentsInventoryEnvelope<T> {
    record_kind: &'static str,
    shared_contract_ref: &'static str,
    cli_projection: T,
    experiments_inventory: serde_json::Value,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let resolver = seeded_resolver()?;
    let context = seeded_context(&resolver);

    match args.first().map(String::as_str) {
        Some("experiments-inventory") => {
            let inventory = inspect_default_inventory()?;
            let cli = project_experiments_cli_inventory(&inventory);
            let envelope = CliExperimentsInventoryEnvelope {
                record_kind: "experiments_inventory_cli_envelope",
                shared_contract_ref: "settings:experiments_inventory_alpha:v1",
                cli_projection: cli,
                experiments_inventory: serde_json::to_value(inventory)?,
            };
            print_json(&envelope)?;
        }
        Some("experiments-support-export") => {
            let inventory = inspect_default_inventory()?;
            let export =
                project_experiments_support_export("support-export:experiments:alpha", &inventory);
            print_json(&export)?;
        }
        Some("support-export") => {
            let records = inspect_all_settings(&resolver, &context)?;
            let export = project_support_export("support-export:settings:alpha", records);
            print_json(&export)?;
        }
        Some("effective-record") => {
            let setting_id = args
                .get(1)
                .map(String::as_str)
                .unwrap_or("security.ai.egress_policy");
            let record = resolver.resolve_record(setting_id)?;
            print_json(&record)?;
        }
        Some("ui-beta-page") => {
            let page = project_settings_ui_beta_page(&resolver, &context, "all", "All settings")?;
            print_json(&page)?;
        }
        Some("ui-beta-inspector") => {
            let setting_id = args
                .get(1)
                .map(String::as_str)
                .unwrap_or("security.ai.egress_policy");
            let pane = inspect_setting_pane(&resolver, setting_id, &context)?;
            print_json(&pane)?;
        }
        Some("ui-beta-write-composer") => {
            let composer = project_write_composer(
                &resolver,
                SettingWritePreviewRequest {
                    setting_id: "security.ai.egress_policy".to_owned(),
                    target_scope: SettingScope::UserGlobal,
                    proposed_value: SettingValue::String("any_hosted_provider".to_owned()),
                    actor_class: WriteActorClass::UserCommand,
                    reason_class: WriteReasonClass::UserEdit,
                    checkpoint_ref: None,
                    approval_ticket_ref: None,
                },
                &context,
            );
            print_json(&composer)?;
        }
        Some("sync-beta-review") => {
            let (page, _packets) = sync_beta_review_artifacts(&resolver, &context)?;
            print_json(&page)?;
        }
        Some("sync-beta-support-export") => {
            let (page, packets) = sync_beta_review_artifacts(&resolver, &context)?;
            let export = project_sync_beta_support_export(
                "support-export:settings-sync-beta:001",
                page,
                packets,
            );
            print_json(&export)?;
        }
        Some("repair-plan-reset-value") => {
            let plan = build_reset_value_plan(&resolver, &context)?;
            print_json(&plan)?;
        }
        Some("repair-plan-reset-section") => {
            let plan = build_reset_section_plan(&resolver, &context, false)?;
            print_json(&plan)?;
        }
        Some("repair-plan-reset-section-checkpointed") => {
            let plan = build_reset_section_plan(&resolver, &context, true)?;
            print_json(&plan)?;
        }
        Some("repair-plan-repair-drift") => {
            let plan = build_repair_drift_plan(&resolver, &context)?;
            print_json(&plan)?;
        }
        Some("repair-plan-reapply-profile-fragment") => {
            let plan = build_reapply_profile_fragment_plan(&resolver, &context, true)?;
            print_json(&plan)?;
        }
        Some("repair-plan-revert-migration-step") => {
            let plan = build_revert_migration_step_plan(&resolver, &context)?;
            print_json(&plan)?;
        }
        Some("repair-plan-adjacent-refused") => {
            let plan = build_adjacent_refused_plan(&resolver, &context)?;
            print_json(&plan)?;
        }
        Some("repair-plan-policy-owned-refused") => {
            let plan = build_policy_owned_refused_plan(&resolver, &context)?;
            print_json(&plan)?;
        }
        Some("repair-review-sheet") => {
            let plan = build_reset_value_plan(&resolver, &context)?;
            let sheet = project_review_sheet(plan);
            print_json(&sheet)?;
        }
        Some("repair-support-export") => {
            let plans = vec![
                with_decision(
                    build_reset_value_plan(&resolver, &context)?,
                    "accepted",
                ),
                with_decision(
                    build_reset_section_plan(&resolver, &context, true)?,
                    "accepted",
                ),
                with_decision(
                    build_reapply_profile_fragment_plan(&resolver, &context, true)?,
                    "accepted",
                ),
                with_decision(
                    build_revert_migration_step_plan(&resolver, &context)?,
                    "declined",
                ),
                build_adjacent_refused_plan(&resolver, &context)?,
                build_policy_owned_refused_plan(&resolver, &context)?,
            ];
            let export = project_repair_support_export("support:repair:001", plans);
            print_json(&export)?;
        }
        Some("ui-beta-support-export") => {
            let records = inspect_all_settings(&resolver, &context)?;
            let page = project_page_from_records(records.clone(), "all", "All settings");
            let export = project_ui_beta_support_export(
                "support-export:settings-ui-beta:001",
                page,
                records,
            );
            print_json(&export)?;
        }
        Some("preview-write") => {
            let preview = preview_write(
                &resolver,
                SettingWritePreviewRequest {
                    setting_id: "security.ai.egress_policy".to_owned(),
                    target_scope: SettingScope::Workspace,
                    proposed_value: SettingValue::String(
                        "approved_hosted_providers_only".to_owned(),
                    ),
                    actor_class: WriteActorClass::UserCommand,
                    reason_class: WriteReasonClass::UserEdit,
                    checkpoint_ref: Some("checkpoint:settings:egress:workspace:001".to_owned()),
                    approval_ticket_ref: Some("approval:settings:egress:001".to_owned()),
                },
                &context,
            );
            print_json(&preview)?;
        }
        Some("inspect") => {
            let setting_id = args
                .get(1)
                .map(String::as_str)
                .unwrap_or("security.ai.egress_policy");
            print_inspect(&resolver, &context, setting_id)?;
        }
        Some(setting_id) => {
            print_inspect(&resolver, &context, setting_id)?;
        }
        None => {
            print_inspect(&resolver, &context, "security.ai.egress_policy")?;
        }
    }

    Ok(())
}

fn print_inspect(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    setting_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let record = inspect_setting(resolver, setting_id, context)?;
    let cli = project_cli_inspect(&record);
    let envelope = CliInspectEnvelope {
        record_kind: "settings_cli_inspect_envelope",
        shared_contract_ref: "settings:effective_inspector_alpha:v1",
        cli_projection: cli,
        effective_setting: serde_json::to_value(record)?,
    };
    print_json(&envelope)
}

fn print_json(value: &impl Serialize) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn sync_beta_review_artifacts(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Result<
    (
        aureline_settings::sync::SyncConflictReviewBetaPage,
        Vec<aureline_settings::inspector::conflict::SyncConflictPacket>,
    ),
    Box<dyn std::error::Error>,
> {
    let local = SyncBetaDeviceRecord {
        device_id: "dev-laptop-primary-0001".to_owned(),
        device_label: Some("Dev laptop".to_owned()),
        device_class: "personal_laptop".to_owned(),
        os_family_class: "macos".to_owned(),
        identity_mode: IdentityModeClass::AccountFreeLocal,
        participation_state: DeviceParticipationState::Active,
        revocation_reason: None,
        lineage_cursor: Some("lc-0001-000000000517".to_owned()),
        last_seen_at: Some("2026-04-20T09:00:00Z".to_owned()),
        last_seen_source: Some("local_heartbeat".to_owned()),
        trust_state: Some("trusted".to_owned()),
    };
    let active_remote = SyncBetaDeviceRecord {
        device_id: "dev-desktop-home-0002".to_owned(),
        device_label: Some("Home desktop".to_owned()),
        device_class: "personal_workstation".to_owned(),
        os_family_class: "macos".to_owned(),
        identity_mode: IdentityModeClass::AccountFreeLocal,
        participation_state: DeviceParticipationState::Active,
        revocation_reason: None,
        lineage_cursor: Some("lc-0002-000000000142".to_owned()),
        last_seen_at: Some("2026-04-19T22:11:00Z".to_owned()),
        last_seen_source: Some("push".to_owned()),
        trust_state: Some("trusted".to_owned()),
    };
    let paused_remote = SyncBetaDeviceRecord {
        device_id: "dev-corp-vm-0003".to_owned(),
        device_label: Some("Corp dev VM".to_owned()),
        device_class: "remote_dev_vm".to_owned(),
        os_family_class: "linux".to_owned(),
        identity_mode: IdentityModeClass::ManagedConvenience,
        participation_state: DeviceParticipationState::Paused,
        revocation_reason: Some("user_paused".to_owned()),
        lineage_cursor: Some("lc-0003-000000000084".to_owned()),
        last_seen_at: Some("2026-04-12T19:42:00Z".to_owned()),
        last_seen_source: Some("push".to_owned()),
        trust_state: Some("restricted".to_owned()),
    };

    let stale_row = build_review_row(
        resolver,
        context,
        SyncConflictReviewBetaRequest {
            setting_id: "editor.tab_size".to_owned(),
            local_device: local.clone(),
            remote_device: active_remote.clone(),
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::Integer(8),
            import_continuity: false,
            remote_bundle_epoch: Some(100),
            local_bundle_epoch_floor: Some(200),
            last_writer: Some(LastWriterBreadcrumb {
                device_id: "dev-laptop-primary-0001".to_owned(),
                device_label: Some("Dev laptop".to_owned()),
                actor_class: "user_keystroke".to_owned(),
                revision_ref: "settings-rev:00517".to_owned(),
                winning_scope: "workspace".to_owned(),
                at: Some("2026-04-18T14:05:31Z".to_owned()),
                mutation_journal_ref: Some("mjr-laptop-primary-0001-000000000517".to_owned()),
            }),
            rollback_checkpoint_ref: None,
            approval_ticket_ref: None,
        },
    )?;
    let policy_row = build_review_row(
        resolver,
        context,
        SyncConflictReviewBetaRequest {
            setting_id: "security.ai.egress_policy".to_owned(),
            local_device: local.clone(),
            remote_device: active_remote.clone(),
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::String("any_hosted_provider".to_owned()),
            import_continuity: false,
            remote_bundle_epoch: Some(142),
            local_bundle_epoch_floor: Some(140),
            last_writer: Some(LastWriterBreadcrumb {
                device_id: "dev-laptop-primary-0001".to_owned(),
                device_label: Some("Dev laptop".to_owned()),
                actor_class: "admin_policy_injector".to_owned(),
                revision_ref: "settings-rev:00042".to_owned(),
                winning_scope: "admin_policy_narrowing".to_owned(),
                at: Some("2026-04-19T10:14:22Z".to_owned()),
                mutation_journal_ref: Some("mjr-admin-policy-bundle-v3-000000000042".to_owned()),
            }),
            rollback_checkpoint_ref: None,
            approval_ticket_ref: None,
        },
    )?;
    let disabled_row = build_review_row(
        resolver,
        context,
        SyncConflictReviewBetaRequest {
            setting_id: "editor.format_on_save".to_owned(),
            local_device: local.clone(),
            remote_device: paused_remote.clone(),
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::Boolean(false),
            import_continuity: false,
            remote_bundle_epoch: Some(84),
            local_bundle_epoch_floor: Some(80),
            last_writer: None,
            rollback_checkpoint_ref: None,
            approval_ticket_ref: None,
        },
    )?;

    let rows = vec![stale_row, policy_row, disabled_row];
    let packets = rows
        .iter()
        .filter_map(|row| {
            row.source_packet_ref.as_ref().and_then(|_| {
                aureline_settings::inspector::conflict::inspect_sync_conflict(
                    resolver,
                    aureline_settings::inspector::conflict::SyncConflictReviewRequest {
                        setting_id: row.setting_id.clone(),
                        current_device:
                            aureline_settings::inspector::conflict::SyncConflictDevice::new(
                                row.local_device.device_id.clone(),
                            ),
                        conflicting_device:
                            aureline_settings::inspector::conflict::SyncConflictDevice::new(
                                row.remote_device.device_id.clone(),
                            ),
                        conflicting_scope: scope_from_token(&row.conflicting_scope),
                        conflicting_value: rebuild_value_for_setting(&row.setting_id),
                    },
                )
                .ok()
                .flatten()
            })
        })
        .collect::<Vec<_>>();

    let page = project_review_page(
        "sync-review-001",
        "Sync conflict review",
        local,
        rows,
        vec![paused_remote],
    );
    Ok((page, packets))
}

fn scope_from_token(token: &str) -> SettingScope {
    match token {
        "user_global" => SettingScope::UserGlobal,
        "workspace" => SettingScope::Workspace,
        "language_override" => SettingScope::LanguageOverride,
        "machine_specific" => SettingScope::MachineSpecific,
        "session_override" => SettingScope::SessionOverride,
        "folder_or_module_override" => SettingScope::FolderOrModuleOverride,
        "imported_profile_default" => SettingScope::ImportedProfileDefault,
        "channel_or_experiment_default" => SettingScope::ChannelOrExperimentDefault,
        "built_in_default" => SettingScope::BuiltInDefault,
        "admin_policy_narrowing" => SettingScope::AdminPolicyNarrowing,
        _ => SettingScope::UserGlobal,
    }
}

fn rebuild_value_for_setting(setting_id: &str) -> SettingValue {
    match setting_id {
        "editor.tab_size" => SettingValue::Integer(8),
        "security.ai.egress_policy" => SettingValue::String("any_hosted_provider".to_owned()),
        "editor.format_on_save" => SettingValue::Boolean(false),
        _ => SettingValue::String(String::new()),
    }
}

fn seeded_resolver() -> Result<EffectiveSettingsResolver, Box<dyn std::error::Error>> {
    let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());

    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
    user.set_value("editor.tab_size", SettingValue::Integer(8));
    user.set_value(
        "security.ai.egress_policy",
        SettingValue::String("any_hosted_provider".to_owned()),
    );
    resolver.set_overlay(user)?;

    let mut workspace = ScopeOverlay::new(SettingScope::Workspace, "Workspace settings");
    workspace.set_value("editor.tab_size", SettingValue::Integer(2));
    resolver.set_overlay(workspace)?;

    let mut policy =
        ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
    policy.set_policy_constraint(
        "security.ai.egress_policy",
        PolicyConstraint::SingleValue {
            value: SettingValue::String("approved_hosted_providers_only".to_owned()),
        },
    );
    resolver.set_overlay(policy)?;

    Ok(resolver)
}

fn seeded_context(resolver: &EffectiveSettingsResolver) -> SettingsInspectionContext {
    let mut context = SettingsInspectionContext::new()
        .with_last_applied_revision("editor.tab_size", "settings-rev:00041")
        .with_last_applied_revision("security.ai.egress_policy", "settings-rev:00042");
    for def in resolver.registry().definitions() {
        for dependency in &def.capability_dependencies {
            context = context.with_capability_state(dependency, true, "available");
        }
    }
    context
}

fn with_decision(mut plan: SettingsRepairPlan, decision: &str) -> SettingsRepairPlan {
    plan.user_decision = decision.to_owned();
    plan
}

fn build_reset_value_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Result<SettingsRepairPlan, Box<dyn std::error::Error>> {
    let request = SettingsRepairPlanRequest {
        plan_id: "repair:reset-tab-size:workspace:001".to_owned(),
        action_class: RepairActionClass::ResetCurrentValue,
        target_scope: SettingScope::Workspace,
        section_id: None,
        imported_profile_fragment: None,
        migration_step: None,
        actor_class: WriteActorClass::UserCommand,
        reason_class: WriteReasonClass::UserEdit,
        reason_note: Some(
            "Reset workspace tab size back to the user-default value.".to_owned(),
        ),
        selected_setting_ids: vec!["editor.tab_size".to_owned()],
        proposed_values: vec![("editor.tab_size".to_owned(), SettingValue::Integer(4))],
        checkpoint_ref: None,
        approval_ticket_ref: None,
        user_decision: RepairUserDecision::Pending,
    };
    Ok(build_repair_plan(resolver, context, request)?)
}

fn build_reset_section_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    with_checkpoint: bool,
) -> Result<SettingsRepairPlan, Box<dyn std::error::Error>> {
    let request = SettingsRepairPlanRequest {
        plan_id: if with_checkpoint {
            "repair:reset-section-editor:user_global:002".to_owned()
        } else {
            "repair:reset-section-editor:user_global:001".to_owned()
        },
        action_class: RepairActionClass::ResetSection,
        target_scope: SettingScope::UserGlobal,
        section_id: Some("editor".to_owned()),
        imported_profile_fragment: None,
        migration_step: None,
        actor_class: WriteActorClass::UserCommand,
        reason_class: WriteReasonClass::UserEdit,
        reason_note: Some(
            "Reset every editor.* setting at user_global back to inherited sources."
                .to_owned(),
        ),
        selected_setting_ids: vec![
            "editor.tab_size".to_owned(),
            "editor.format_on_save".to_owned(),
        ],
        proposed_values: vec![
            ("editor.tab_size".to_owned(), SettingValue::Integer(4)),
            (
                "editor.format_on_save".to_owned(),
                SettingValue::Boolean(false),
            ),
        ],
        checkpoint_ref: if with_checkpoint {
            Some("checkpoint:settings:user_global:editor:001".to_owned())
        } else {
            None
        },
        approval_ticket_ref: None,
        user_decision: RepairUserDecision::Pending,
    };
    Ok(build_repair_plan(resolver, context, request)?)
}

fn build_repair_drift_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Result<SettingsRepairPlan, Box<dyn std::error::Error>> {
    let request = SettingsRepairPlanRequest {
        plan_id: "repair:drift-tab-size:user_global:001".to_owned(),
        action_class: RepairActionClass::RepairDrift,
        target_scope: SettingScope::UserGlobal,
        section_id: None,
        imported_profile_fragment: None,
        migration_step: None,
        actor_class: WriteActorClass::UserCommand,
        reason_class: WriteReasonClass::UserEdit,
        reason_note: Some(
            "Restore tab size at user_global to the last-known intended value.".to_owned(),
        ),
        selected_setting_ids: vec!["editor.tab_size".to_owned()],
        proposed_values: vec![("editor.tab_size".to_owned(), SettingValue::Integer(4))],
        checkpoint_ref: None,
        approval_ticket_ref: None,
        user_decision: RepairUserDecision::Pending,
    };
    Ok(build_repair_plan(resolver, context, request)?)
}

fn build_reapply_profile_fragment_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    with_checkpoint: bool,
) -> Result<SettingsRepairPlan, Box<dyn std::error::Error>> {
    let request = SettingsRepairPlanRequest {
        plan_id: "repair:reapply-fragment:editor-cleanup:001".to_owned(),
        action_class: RepairActionClass::ReapplyImportedProfileFragment,
        target_scope: SettingScope::ImportedProfileDefault,
        section_id: None,
        imported_profile_fragment: Some(ImportedProfileFragmentRef {
            profile_id: "profile:portable:dev-laptop".to_owned(),
            fragment_id: "fragment:editor-cleanup".to_owned(),
            fragment_label: "Editor cleanup".to_owned(),
            source_label: "Imported profile: Dev laptop".to_owned(),
        }),
        migration_step: None,
        actor_class: WriteActorClass::ImportedProfile,
        reason_class: WriteReasonClass::Import,
        reason_note: Some(
            "Re-apply the editor-cleanup fragment from the imported portable profile.".to_owned(),
        ),
        selected_setting_ids: vec![
            "editor.tab_size".to_owned(),
            "editor.format_on_save".to_owned(),
        ],
        proposed_values: vec![
            ("editor.tab_size".to_owned(), SettingValue::Integer(4)),
            (
                "editor.format_on_save".to_owned(),
                SettingValue::Boolean(true),
            ),
        ],
        checkpoint_ref: if with_checkpoint {
            Some("checkpoint:settings:imported_profile:editor-cleanup:001".to_owned())
        } else {
            None
        },
        approval_ticket_ref: None,
        user_decision: RepairUserDecision::Pending,
    };
    Ok(build_repair_plan(resolver, context, request)?)
}

fn build_revert_migration_step_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Result<SettingsRepairPlan, Box<dyn std::error::Error>> {
    let request = SettingsRepairPlanRequest {
        plan_id: "repair:revert-migration:editor-tab-size:001".to_owned(),
        action_class: RepairActionClass::RevertMigrationStep,
        target_scope: SettingScope::UserGlobal,
        section_id: None,
        imported_profile_fragment: None,
        migration_step: Some(MigrationStepRef {
            migration_id: "migration:editor.tab_size:v1-to-v2".to_owned(),
            from_version: "settings_definition:v1".to_owned(),
            to_version: "settings_definition:v2".to_owned(),
            transform_class: "narrow_enum".to_owned(),
            is_lossy: false,
            rollback_supported: true,
        }),
        actor_class: WriteActorClass::WorkspaceMigration,
        reason_class: WriteReasonClass::Automation,
        reason_note: Some(
            "Revert the editor.tab_size migration step using the checkpoint captured before apply.".to_owned(),
        ),
        selected_setting_ids: vec!["editor.tab_size".to_owned()],
        proposed_values: vec![("editor.tab_size".to_owned(), SettingValue::Integer(4))],
        checkpoint_ref: Some(
            "checkpoint:settings:migration:editor.tab_size:v1-to-v2:001".to_owned(),
        ),
        approval_ticket_ref: None,
        user_decision: RepairUserDecision::Pending,
    };
    Ok(build_repair_plan(resolver, context, request)?)
}

fn build_adjacent_refused_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Result<SettingsRepairPlan, Box<dyn std::error::Error>> {
    let request = SettingsRepairPlanRequest {
        plan_id: "repair:adjacent-refused:workspace:001".to_owned(),
        action_class: RepairActionClass::ResetCurrentValue,
        target_scope: SettingScope::Workspace,
        section_id: None,
        imported_profile_fragment: None,
        migration_step: None,
        actor_class: WriteActorClass::UserCommand,
        reason_class: WriteReasonClass::UserEdit,
        reason_note: Some(
            "Attempt to reset only editor.tab_size; the plan would have touched editor.format_on_save too."
                .to_owned(),
        ),
        selected_setting_ids: vec!["editor.tab_size".to_owned()],
        proposed_values: vec![
            ("editor.tab_size".to_owned(), SettingValue::Integer(4)),
            (
                "editor.format_on_save".to_owned(),
                SettingValue::Boolean(false),
            ),
        ],
        checkpoint_ref: None,
        approval_ticket_ref: None,
        user_decision: RepairUserDecision::Declined,
    };
    Ok(build_repair_plan(resolver, context, request)?)
}

fn build_policy_owned_refused_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Result<SettingsRepairPlan, Box<dyn std::error::Error>> {
    let request = SettingsRepairPlanRequest {
        plan_id: "repair:policy-owned-refused:001".to_owned(),
        action_class: RepairActionClass::ResetCurrentValue,
        target_scope: SettingScope::AdminPolicyNarrowing,
        section_id: None,
        imported_profile_fragment: None,
        migration_step: None,
        actor_class: WriteActorClass::UserCommand,
        reason_class: WriteReasonClass::UserEdit,
        reason_note: Some(
            "Attempt to repair an admin-policy-owned value from a user-initiated plan.".to_owned(),
        ),
        selected_setting_ids: vec!["security.ai.egress_policy".to_owned()],
        proposed_values: vec![(
            "security.ai.egress_policy".to_owned(),
            SettingValue::String("any_hosted_provider".to_owned()),
        )],
        checkpoint_ref: None,
        approval_ticket_ref: None,
        user_decision: RepairUserDecision::Declined,
    };
    Ok(build_repair_plan(resolver, context, request)?)
}
