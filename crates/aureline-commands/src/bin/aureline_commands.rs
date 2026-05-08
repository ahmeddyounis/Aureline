use aureline_commands::registry::seeded_registry;

fn main() {
    let registry = seeded_registry();

    let mut out = String::new();
    out.push_str("{\n  \"record_kind\": \"command_registry_enumeration\",\n  \"schema_version\": 1,\n");
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
        out.push_str(",\n      \"enablement\": {\n        \"decision_class\": ");
        push_json_string(&mut out, &entry.seed_enablement_snapshot.decision_class);
        out.push_str(",\n        \"disabled_reason_code\": ");
        match &entry.seed_enablement_snapshot.disabled_reason_code {
            Some(code) => push_json_string(&mut out, code),
            None => out.push_str("null"),
        }
        out.push_str("\n      }\n");
        out.push_str("    }");
        if idx + 1 != registry.entries().len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]\n}\n");
    print!("{out}");
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
