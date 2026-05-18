//! Deterministic markdown rendering for the command-reference parity
//! report under `artifacts/ux/m3/command_reference_parity_report.md`.
//!
//! The renderer reads one [`CommandReferenceCatalog`] and emits the
//! same markdown body the shell parity inspector renders for the
//! command detail surface.

use std::fmt::Write;

use super::{AliasLifecycleState, CommandReferenceCatalog, CommandReferenceEntry};

/// Renders the catalog as the published markdown parity report.
pub fn render_catalog_markdown(catalog: &CommandReferenceCatalog) -> String {
    let mut out = String::new();
    out.push_str("# Command reference parity report\n\n");
    out.push_str(
        "Generated from the seeded command-reference catalog in\n\
         [`crate::command_reference`](../../../crates/aureline-shell/src/command_reference/mod.rs).\n\
         Regenerate by running the fixture-protected integration test:\n\n",
    );
    out.push_str("```sh\n");
    out.push_str("cargo test -p aureline-shell --test command_reference_fixtures\n");
    out.push_str("```\n\n");

    writeln!(out, "- Catalog id: `{}`", catalog.catalog_id).unwrap();
    writeln!(
        out,
        "- Shared contract ref: `{}`",
        catalog.shared_contract_ref
    )
    .unwrap();
    writeln!(
        out,
        "- Descriptor schema ref: `{}`",
        catalog.source_descriptor_schema_ref
    )
    .unwrap();
    writeln!(out, "- Entries: `{}`", catalog.entries.len()).unwrap();
    writeln!(
        out,
        "- High-risk entries: `{}`",
        catalog.high_risk_count()
    )
    .unwrap();
    writeln!(
        out,
        "- Deprecated entries: `{}`",
        catalog.deprecated_count()
    )
    .unwrap();
    writeln!(out, "- Generated at: `{}`", catalog.generated_at).unwrap();
    out.push('\n');

    out.push_str("## Catalog summary\n\n");
    out.push_str(
        "| Command | Lifecycle | Risk | Preview | Idempotency | Deprecation | Aliases |\n\
         | ------- | --------- | ---- | ------- | ----------- | ----------- | -------:|\n",
    );
    for entry in &catalog.entries {
        let dep = match entry.deprecation.state {
            AliasLifecycleState::Active => "active".to_owned(),
            AliasLifecycleState::Deprecated => format!(
                "deprecated -> {}",
                entry
                    .deprecation
                    .replacement_command_id
                    .clone()
                    .unwrap_or_else(|| "-".to_owned())
            ),
            AliasLifecycleState::Retired => format!(
                "retired -> {}",
                entry
                    .deprecation
                    .replacement_command_id
                    .clone()
                    .unwrap_or_else(|| "-".to_owned())
            ),
        };
        writeln!(
            out,
            "| `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} |",
            entry.command_id,
            entry.lifecycle_state.as_str(),
            entry.risk_class.as_str(),
            entry.preview_class.as_str(),
            entry.idempotency_class.as_str(),
            dep,
            entry.aliases.len(),
        )
        .unwrap();
    }
    out.push('\n');

    out.push_str("## Per-command detail\n\n");
    for entry in &catalog.entries {
        render_entry(&mut out, entry);
    }

    out.push_str("## Verification\n\n");
    out.push_str("```sh\n");
    out.push_str("cargo test -p aureline-shell --test command_reference_fixtures\n");
    out.push_str("```\n");
    out
}

fn render_entry(out: &mut String, entry: &CommandReferenceEntry) {
    writeln!(
        out,
        "### `{}` -- {}",
        entry.command_id, entry.title
    )
    .unwrap();
    out.push('\n');
    writeln!(out, "{}", entry.summary).unwrap();
    out.push('\n');

    writeln!(
        out,
        "- Lifecycle: `{}`",
        entry.lifecycle_state.as_str()
    )
    .unwrap();
    writeln!(out, "- Origin: `{}`", entry.origin_class).unwrap();
    writeln!(
        out,
        "- Risk class: `{}`",
        entry.risk_class.as_str()
    )
    .unwrap();
    writeln!(
        out,
        "- Preview class: `{}`",
        entry.preview_class.as_str()
    )
    .unwrap();
    writeln!(
        out,
        "- Idempotency: `{}`",
        entry.idempotency_class.as_str()
    )
    .unwrap();
    writeln!(out, "- Supports dry run: `{}`", entry.supports_dry_run).unwrap();
    writeln!(
        out,
        "- Descriptor revision: `{}`",
        entry.command_revision_ref
    )
    .unwrap();
    writeln!(
        out,
        "- Primary label ref: `{}`",
        entry.primary_label_ref
    )
    .unwrap();
    writeln!(
        out,
        "- Docs/help anchor: `{}`",
        entry.docs_help_anchor_ref
    )
    .unwrap();
    out.push('\n');

    out.push_str("#### Aliases\n\n");
    if entry.aliases.is_empty() {
        out.push_str("- none\n\n");
    } else {
        for alias in &entry.aliases {
            writeln!(
                out,
                "- `{}` (`{}`, `{}`){}{}{}",
                alias.alias_id,
                alias.alias_kind.as_str(),
                alias.lifecycle_state.as_str(),
                alias
                    .introduced_version
                    .as_deref()
                    .map(|v| format!(", introduced {v}"))
                    .unwrap_or_default(),
                alias
                    .retirement_version
                    .as_deref()
                    .map(|v| format!(", retires {v}"))
                    .unwrap_or_default(),
                alias
                    .import_impact_class
                    .map(|v| format!(", import impact `{}`", v.as_str()))
                    .unwrap_or_default(),
            )
            .unwrap();
        }
        out.push('\n');
    }

    out.push_str("#### Deprecation\n\n");
    writeln!(
        out,
        "- State: `{}`",
        entry.deprecation.state.as_str()
    )
    .unwrap();
    if let Some(v) = entry.deprecation.deprecated_in_version.as_deref() {
        writeln!(out, "- Deprecated in: `{v}`").unwrap();
    }
    if let Some(v) = entry.deprecation.retires_in_version.as_deref() {
        writeln!(out, "- Retires in: `{v}`").unwrap();
    }
    if let Some(v) = entry.deprecation.replacement_command_id.as_deref() {
        writeln!(out, "- Replacement command: `{v}`").unwrap();
    }
    if let Some(v) = entry.deprecation.import_impact_class {
        writeln!(out, "- Import impact: `{}`", v.as_str()).unwrap();
    }
    if let Some(v) = entry.deprecation.migration_note_ref.as_deref() {
        writeln!(out, "- Migration note: `{v}`").unwrap();
    }
    out.push('\n');

    out.push_str("#### Argument schema\n\n");
    if entry.argument_schema.is_empty() {
        out.push_str("- none\n\n");
    } else {
        out.push_str("| Argument | Kind | Required | Default provenance |\n");
        out.push_str("| -------- | ---- | -------- | ------------------ |\n");
        for slot in &entry.argument_schema {
            writeln!(
                out,
                "| `{}` | `{}` | `{}` | `{}` |",
                slot.argument_name,
                slot.argument_kind,
                slot.is_required,
                slot.default_provenance_when_omitted
                    .as_deref()
                    .unwrap_or("-"),
            )
            .unwrap();
        }
        out.push('\n');
    }

    out.push_str("#### Availability\n\n");
    writeln!(
        out,
        "- Trust gate: `{}`",
        entry.availability.trust_gate_class
    )
    .unwrap();
    writeln!(
        out,
        "- Policy gate: `{}`",
        entry.availability.policy_gate_class
    )
    .unwrap();
    writeln!(
        out,
        "- Dependency presence: `{}`",
        entry.availability.dependency_presence_class
    )
    .unwrap();
    let supported: Vec<&str> = entry
        .availability
        .supported_surfaces
        .iter()
        .map(|surface| surface.as_str())
        .collect();
    writeln!(out, "- Supported surfaces: `{}`", supported.join(", ")).unwrap();
    if entry
        .availability
        .current_disabled_reason_codes
        .is_empty()
    {
        out.push_str("- Current disabled reasons: none\n");
    } else {
        writeln!(
            out,
            "- Current disabled reasons: `{}`",
            entry
                .availability
                .current_disabled_reason_codes
                .join(", ")
        )
        .unwrap();
    }
    out.push('\n');

    out.push_str("#### Keybindings\n\n");
    out.push_str("| Chord | Platform | State | Shadowed by |\n");
    out.push_str("| ----- | -------- | ----- | ----------- |\n");
    for binding in &entry.keybindings {
        let shadow = match (
            binding.shadowed_by_chord_ref.as_deref(),
            binding.shadowed_by_command_id.as_deref(),
        ) {
            (Some(chord), Some(cmd)) => format!("`{chord}` (`{cmd}`)"),
            (Some(chord), None) => format!("`{chord}`"),
            (None, Some(cmd)) => format!("`{cmd}`"),
            (None, None) => "-".to_owned(),
        };
        writeln!(
            out,
            "| `{}` | `{}` | `{}` | {} |",
            binding.chord_ref,
            binding.platform_variant.as_str(),
            binding.binding_state.as_str(),
            shadow,
        )
        .unwrap();
    }
    out.push('\n');

    out.push_str("#### Automation\n\n");
    let labels: Vec<&str> = entry
        .automation
        .automation_labels
        .iter()
        .map(|label| label.as_str())
        .collect();
    writeln!(
        out,
        "- Headless eligible: `{}`",
        entry.automation.headless_eligible
    )
    .unwrap();
    writeln!(
        out,
        "- Recipe eligible: `{}`",
        entry.automation.recipe_eligible
    )
    .unwrap();
    writeln!(
        out,
        "- Macro eligible: `{}`",
        entry.automation.macro_eligible
    )
    .unwrap();
    writeln!(
        out,
        "- AI eligible: `{}`",
        entry.automation.ai_eligible
    )
    .unwrap();
    writeln!(out, "- Automation labels: `{}`", labels.join(", ")).unwrap();
    out.push('\n');

    out.push_str("#### Search index\n\n");
    out.push_str("| Token class | Value |\n");
    out.push_str("| ----------- | ----- |\n");
    for token in &entry.search_index {
        writeln!(
            out,
            "| `{}` | `{}` |",
            token.token_class.as_str(),
            token.value
        )
        .unwrap();
    }
    out.push('\n');

    out.push_str("#### Discoverability links\n\n");
    for link in &entry.discoverability_links {
        writeln!(
            out,
            "- `{}` -> `{}`",
            link.surface_family.as_str(),
            link.anchor_ref
        )
        .unwrap();
    }
    out.push('\n');
}
