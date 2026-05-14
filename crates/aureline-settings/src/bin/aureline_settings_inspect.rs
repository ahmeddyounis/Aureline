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
