use aureline_commands::alpha::alpha_command_registry;
use aureline_commands::automation::current_safe_automation_qualification_export;
use aureline_commands::finalize_command_parity::current_finalize_command_parity_export;
use aureline_commands::harden_high_risk_command::current_high_risk_command_hardening_export;
use aureline_commands::m5_capability_state_truth::{
    current_m5_capability_state_truth_export, M5CapabilityStateTruthSupportExport,
    M5_CAPABILITY_STATE_TRUTH_SUPPORT_EXPORT_ID,
};
use aureline_commands::m5_command_governance::{
    current_m5_command_governance_export, M5CommandGovernanceSupportExport,
    M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_ID,
};
use aureline_commands::registry::seeded_registry;
use aureline_commands::stabilize_command_discoverability_records_alias_history::current_command_discoverability_export;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if let Some(rendered) = maybe_render_m5_capability_state_truth(&args) {
        print!("{rendered}");
        return;
    }
    if let Some(rendered) = maybe_render_m5_command_governance(&args) {
        print!("{rendered}");
        return;
    }

    let registry = seeded_registry();
    let alpha_registry = alpha_command_registry();

    let mut out = String::new();
    out.push_str(
        "{\n  \"record_kind\": \"command_registry_enumeration\",\n  \"schema_version\": 1,\n",
    );
    out.push_str("  \"entries\": [\n");
    for (idx, entry) in registry.entries().iter().enumerate() {
        out.push_str("    {\n");
        out.push_str("      \"command_id\": ");
        push_json_string(&mut out, entry.command_id());
        out.push_str(",\n      \"title\": ");
        push_json_string(&mut out, &entry.title);
        out.push_str(",\n      \"summary\": ");
        push_json_string(&mut out, &entry.summary);
        out.push_str(",\n      \"namespace_class\": ");
        push_json_string(&mut out, &entry.namespace_class);
        out.push_str(",\n      \"capability_scope_class\": ");
        push_json_string(&mut out, &entry.descriptor.capability_scope_class);
        out.push_str(",\n      \"preview_class\": ");
        push_json_string(&mut out, &entry.descriptor.preview_class);
        out.push_str(",\n      \"approval_posture_class\": ");
        push_json_string(&mut out, &entry.descriptor.approval_posture_class);
        out.push_str(",\n      \"palette_visibility\": ");
        push_json_string(&mut out, &entry.descriptor.palette_visibility);
        out.push_str(",\n      \"lifecycle_state\": ");
        push_json_string(&mut out, &entry.descriptor.lifecycle_state);
        out.push_str(",\n      \"automation_labels\": ");
        push_json_array(&mut out, &entry.automation_labels);
        out.push_str(",\n      \"enablement\": {\n        \"decision_class\": ");
        push_json_string(
            &mut out,
            entry.seed_enablement_snapshot.decision_class.as_str(),
        );
        out.push_str(",\n        \"disabled_reason_code\": ");
        match &entry.seed_enablement_snapshot.disabled_reason_code {
            Some(code) => push_json_string(&mut out, code.as_str()),
            None => out.push_str("null"),
        }
        out.push_str("\n      }\n");
        out.push_str("    }");
        if idx + 1 != registry.entries().len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ],\n  \"alpha_publication\": {\n");
    out.push_str("    \"registry_id\": ");
    push_json_string(&mut out, &alpha_registry.registry_id);
    out.push_str(",\n    \"descriptor_schema_ref\": ");
    push_json_string(&mut out, &alpha_registry.descriptor_schema_ref);
    out.push_str(",\n    \"invocation_session_schema_ref\": ");
    push_json_string(&mut out, &alpha_registry.invocation_session_schema_ref);
    out.push_str(",\n    \"parity_report_ref\": ");
    push_json_string(&mut out, &alpha_registry.parity_report_ref);
    out.push_str(",\n    \"claimed_command_ids\": ");
    let claimed_command_ids = alpha_registry
        .claimed_commands
        .iter()
        .map(|claim| claim.command_id.clone())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &claimed_command_ids);
    out.push_str(",\n    \"surface_families\": ");
    push_json_array(&mut out, &alpha_registry.surface_family_set);
    out.push_str(",\n    \"disabled_reason_vocabulary_ref\": ");
    push_json_string(&mut out, &alpha_registry.disabled_reason_vocabulary_ref);
    out.push_str("\n  },\n");

    // Project the finalized command-parity lane so the headless surface — itself
    // one of the parity surfaces — can read the same discoverability truth (one
    // canonical record, alias set, footer actions, and query-session privacy
    // posture) that the palette, menus, keybindings, and docs/help project.
    let parity = current_finalize_command_parity_export()
        .expect("checked finalized command parity export validates");
    out.push_str("  \"command_parity_finalization\": {\n");
    out.push_str("    \"packet_id\": ");
    push_json_string(&mut out, &parity.packet_id);
    out.push_str(",\n    \"command_family_id\": ");
    push_json_string(&mut out, &parity.command_family_id);
    out.push_str(",\n    \"claimed_stable\": ");
    out.push_str(if parity.claimed_stable {
        "true"
    } else {
        "false"
    });
    out.push_str(",\n    \"canonical_command_id\": ");
    push_json_string(
        &mut out,
        &parity.discoverability_record.canonical_command_id,
    );
    out.push_str(",\n    \"alias_set\": ");
    push_json_array(&mut out, &parity.discoverability_record.alias_set);
    out.push_str(",\n    \"discoverability_surfaces\": ");
    let discoverability_surfaces = parity
        .projection_rows
        .iter()
        .map(|row| row.surface_class.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &discoverability_surfaces);
    out.push_str(",\n    \"modifier_action_footer\": ");
    let modifier_action_footer = parity
        .footer_contract
        .actions
        .iter()
        .map(|action| action.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &modifier_action_footer);
    out.push_str(",\n    \"query_history_policy\": ");
    push_json_string(
        &mut out,
        parity.query_session_privacy.history_policy.as_str(),
    );
    out.push_str(",\n    \"query_session_local_first\": ");
    out.push_str(if parity.query_session_privacy.local_first {
        "true"
    } else {
        "false"
    });
    out.push_str(",\n    \"evidence_id\": ");
    push_json_string(&mut out, &parity.evidence_export.evidence_id);
    out.push_str("\n  },\n");

    // Project the hardened high-risk command lane so the headless surface — itself
    // one of the parity surfaces — can read the same write-capable safety truth
    // (required preview, attributable approval lineage, and issued rollback handle)
    // that the menu, palette, keybindings, AI tool, and deep links enforce.
    let hardening = current_high_risk_command_hardening_export()
        .expect("checked hardened high-risk command export validates");
    out.push_str("  \"high_risk_command_hardening\": {\n");
    out.push_str("    \"packet_id\": ");
    push_json_string(&mut out, &hardening.packet_id);
    out.push_str(",\n    \"command_family_id\": ");
    push_json_string(&mut out, &hardening.command_family_id);
    out.push_str(",\n    \"claimed_stable\": ");
    out.push_str(if hardening.claimed_stable {
        "true"
    } else {
        "false"
    });
    out.push_str(",\n    \"risk_classes\": ");
    let risk_classes = hardening
        .risk_classes
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &risk_classes);
    out.push_str(",\n    \"preview_required\": ");
    out.push_str(if hardening.preview_contract.required {
        "true"
    } else {
        "false"
    });
    out.push_str(",\n    \"preview_requirements\": ");
    let preview_requirements = hardening
        .preview_contract
        .requirements
        .iter()
        .map(|item| item.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &preview_requirements);
    out.push_str(",\n    \"approval_required\": ");
    out.push_str(if hardening.approval_lineage.required {
        "true"
    } else {
        "false"
    });
    out.push_str(",\n    \"approval_steps\": ");
    let approval_steps = hardening
        .approval_lineage
        .records
        .iter()
        .map(|record| record.step_class.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &approval_steps);
    out.push_str(",\n    \"rollback_posture\": ");
    push_json_string(&mut out, hardening.rollback_handle.posture.as_str());
    out.push_str(",\n    \"rollback_handle_issued\": ");
    out.push_str(if hardening.rollback_handle.issued {
        "true"
    } else {
        "false"
    });
    out.push_str(",\n    \"evidence_id\": ");
    push_json_string(&mut out, &hardening.evidence_export.evidence_id);
    out.push_str("\n  },\n");

    // Project the safe automation qualification lane so command discovery,
    // diagnostics, help, and support exports all read the same label vocabulary
    // and narrowing rules before exposing macro, recipe, or headless affordances.
    let automation = current_safe_automation_qualification_export()
        .expect("checked safe automation qualification export validates");
    out.push_str("  \"safe_automation_qualification\": {\n");
    out.push_str("    \"packet_id\": ");
    push_json_string(&mut out, &automation.packet_id);
    out.push_str(",\n    \"claimed_stable_label_truth\": ");
    out.push_str(if automation.claimed_stable_label_truth {
        "true"
    } else {
        "false"
    });
    out.push_str(",\n    \"controlled_labels\": ");
    let controlled_labels = automation
        .controlled_labels
        .iter()
        .map(|label| label.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &controlled_labels);
    out.push_str(",\n    \"automation_classes\": ");
    let automation_classes = automation
        .automation_classes
        .iter()
        .map(|row| row.object_class.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &automation_classes);
    out.push_str(",\n    \"surface_actions\": ");
    let surface_actions = automation
        .surface_contracts
        .iter()
        .map(|row| row.action_class.as_str().to_owned())
        .collect::<Vec<_>>();
    push_json_array(&mut out, &surface_actions);
    out.push_str(",\n    \"evidence_id\": ");
    push_json_string(&mut out, &automation.evidence_export.evidence_id);
    out.push_str("\n  },\n");

    // Project the stabilized command-discoverability lane so CLI/help, docs/help,
    // onboarding, and support surfaces read the same discoverability source the
    // command registry promotes for protected commands.
    let discoverability = current_command_discoverability_export()
        .expect("checked command discoverability export validates");
    out.push_str("  \"command_discoverability\": {\n");
    out.push_str("    \"packet_id\": ");
    push_json_string(&mut out, &discoverability.packet_id);
    out.push_str(",\n    \"protected_command_count\": ");
    out.push_str(&discoverability.commands.len().to_string());
    out.push_str(",\n    \"stable_command_count\": ");
    out.push_str(
        &discoverability
            .commands
            .iter()
            .filter(|command| command.stable_line_required)
            .count()
            .to_string(),
    );
    out.push_str(",\n    \"query_history_policy\": ");
    push_json_string(
        &mut out,
        discoverability
            .query_session_policy
            .history_policy_class
            .as_str(),
    );
    out.push_str(",\n    \"query_sync_posture\": ");
    push_json_string(
        &mut out,
        discoverability.query_session_policy.sync_posture.as_str(),
    );
    out.push_str(",\n    \"required_surfaces\": ");
    let required_surfaces =
        aureline_commands::CommandDiscoverabilitySurfaceClass::required_coverage()
            .into_iter()
            .map(|surface| surface.as_str().to_owned())
            .collect::<Vec<_>>();
    push_json_array(&mut out, &required_surfaces);
    out.push_str("\n  }\n}\n");
    print!("{out}");
}

fn maybe_render_m5_command_governance(args: &[String]) -> Option<String> {
    if args.first().map(String::as_str) != Some("m5-command-governance") {
        return None;
    }

    let packet = current_m5_command_governance_export()
        .expect("checked M5 command-governance export validates");
    let mode = args.get(1).map(String::as_str).unwrap_or("json");
    Some(match mode {
        "summary" | "summary-md" => packet.render_markdown(),
        "support-export" => {
            serde_json::to_string_pretty(&M5CommandGovernanceSupportExport::from_packet(
                M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_ID.to_string(),
                packet,
            ))
            .expect("support export must serialize")
        }
        _ => serde_json::to_string_pretty(&packet).expect("packet must serialize"),
    })
}

fn maybe_render_m5_capability_state_truth(args: &[String]) -> Option<String> {
    if args.first().map(String::as_str) != Some("m5-capability-state-truth") {
        return None;
    }

    let packet = current_m5_capability_state_truth_export()
        .expect("checked M5 capability-state truth export validates");
    let mode = args.get(1).map(String::as_str).unwrap_or("json");
    Some(match mode {
        "summary" | "summary-md" => packet.render_markdown(),
        "support-export" => {
            serde_json::to_string_pretty(&M5CapabilityStateTruthSupportExport::from_packet(
                M5_CAPABILITY_STATE_TRUTH_SUPPORT_EXPORT_ID.to_string(),
                packet,
            ))
            .expect("support export must serialize")
        }
        _ => serde_json::to_string_pretty(&packet).expect("packet must serialize"),
    })
}

fn push_json_array(out: &mut String, values: &[String]) {
    out.push('[');
    for (idx, value) in values.iter().enumerate() {
        if idx != 0 {
            out.push_str(", ");
        }
        push_json_string(out, value);
    }
    out.push(']');
}

fn push_json_string(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}
