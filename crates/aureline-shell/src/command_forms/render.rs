//! Markdown renderer for the command-forms catalog. Quotes every record
//! deterministically so the published report stays bit-for-bit equal to
//! the seeded catalog.

use super::{CommandFormBundle, CommandFormsCatalog};

/// Renders the catalog as a markdown report.
pub fn render_catalog_markdown(catalog: &CommandFormsCatalog) -> String {
    let mut out = String::new();
    out.push_str("# Command parameter forms and invocation review sheets\n\n");
    out.push_str(
        "This report is the published parity surface for the schema-driven \
parameter form and invocation review sheet records. Every bundle below is \
projected from the same command descriptor, so palette parameter forms, \
CLI inspect surfaces, AI tool envelopes, automation-recipe step editors, \
request / run / debug / template / repair workspaces, and voice grammars \
render identical typed fields, source-layer labels, validation findings, \
restart classes, and review semantics.\n\n",
    );
    out.push_str("Catalog id: `");
    out.push_str(&catalog.catalog_id);
    out.push_str("`\n\n");
    out.push_str("Source descriptor schema: `");
    out.push_str(&catalog.source_descriptor_schema_ref);
    out.push_str("`\n\n");
    out.push_str("Source form schema: `");
    out.push_str(&catalog.source_form_schema_ref);
    out.push_str("`\n\n");
    out.push_str("Source review-sheet schema: `");
    out.push_str(&catalog.source_review_schema_ref);
    out.push_str("`\n\n");
    out.push_str("Generated at: `");
    out.push_str(&catalog.generated_at);
    out.push_str("`\n\n");

    for bundle in &catalog.bundles {
        render_bundle(bundle, &mut out);
    }
    out
}

fn render_bundle(bundle: &CommandFormBundle, out: &mut String) {
    out.push_str("## Scenario: `");
    out.push_str(&bundle.scenario_id);
    out.push_str("`\n\n");
    out.push_str("Bundle id: `");
    out.push_str(&bundle.bundle_id);
    out.push_str("`\n\n");
    out.push_str("Command: `");
    out.push_str(&bundle.form_state.command_id);
    out.push_str("` (revision `");
    out.push_str(&bundle.form_state.command_revision_ref);
    out.push_str("`)\n\n");

    out.push_str("### Parameter form\n\n");
    out.push_str("- Form surface: `");
    out.push_str(bundle.form_state.form_surface_class.as_str());
    out.push_str("`\n");
    out.push_str("- Client scope: `");
    out.push_str(bundle.form_state.client_scope.as_str());
    out.push_str("`\n");
    out.push_str("- Trust state: `");
    out.push_str(bundle.form_state.policy_context.trust_state.as_str());
    out.push_str("`\n");
    out.push_str("- Overall validation severity: `");
    out.push_str(
        bundle
            .form_state
            .validation_rollup
            .overall_severity
            .as_str(),
    );
    out.push_str("`\n\n");

    out.push_str("| Field | Kind | Required | State | Source layer | Visibility | Redaction | Restart/reload |\n");
    out.push_str("|---|---|---|---|---|---|---|---|\n");
    for field in &bundle.form_state.fields {
        out.push_str("| `");
        out.push_str(&field.argument_name);
        out.push_str("` | `");
        out.push_str(field.argument_kind.as_str());
        out.push_str("` | ");
        out.push_str(if field.is_required { "yes" } else { "no" });
        out.push_str(" | `");
        out.push_str(field.field_state_class.as_str());
        out.push_str("` | ");
        out.push_str(field.source_layer_class.display_label());
        out.push_str(" | `");
        out.push_str(field.value_visibility.as_str());
        out.push_str("` | `");
        out.push_str(field.redaction_class.as_str());
        out.push_str("` | `");
        out.push_str(field.restart_or_reload_class.as_str());
        out.push_str("` |\n");
    }
    out.push('\n');

    out.push_str("### Invocation review sheet\n\n");
    out.push_str("- Review surface: `");
    out.push_str(bundle.review_sheet.review_surface_class.as_str());
    out.push_str("`\n");
    out.push_str("- Capability scope: `");
    out.push_str(bundle.review_sheet.capability_scope_class.as_str());
    out.push_str("`\n");
    out.push_str("- Preview class: `");
    out.push_str(bundle.review_sheet.preview_class_declared.as_str());
    out.push_str("`\n");
    out.push_str("- Approval posture: `");
    out.push_str(bundle.review_sheet.approval_posture_class.as_str());
    out.push_str("`\n");
    out.push_str("- Execution intent: `");
    out.push_str(bundle.review_sheet.execution_intent.as_str());
    out.push_str("`\n");
    out.push_str("- Rollback: `");
    out.push_str(bundle.review_sheet.rollback_class.as_str());
    out.push_str("`\n");
    out.push_str("- Preview/dry-run: `");
    out.push_str(bundle.review_sheet.preview_or_dry_run_class.as_str());
    out.push_str("` (available: ");
    out.push_str(if bundle.review_sheet.preview_or_dry_run_available {
        "yes"
    } else {
        "no"
    });
    out.push_str(")\n");
    out.push_str("- Invocable from this sheet: ");
    out.push_str(if bundle.review_sheet.is_invocable() {
        "yes"
    } else {
        "no"
    });
    out.push_str("\n\n");

    if !bundle.review_sheet.scope_axes.is_empty() {
        out.push_str("Scope axes:\n\n");
        for axis in &bundle.review_sheet.scope_axes {
            out.push_str("- `");
            out.push_str(axis.axis_class.as_str());
            out.push_str("`: included=");
            out.push_str(&axis.included_count.to_string());
            out.push_str(", excluded=");
            out.push_str(&axis.excluded_count.to_string());
            out.push_str(", hidden/blocked=");
            out.push_str(&axis.hidden_or_blocked_count.to_string());
            out.push_str(" (count truth `");
            out.push_str(axis.count_truth_class.as_str());
            out.push_str("`)\n");
        }
        out.push('\n');
    }

    if !bundle.review_sheet.side_effects.is_empty() {
        out.push_str("Side effects:\n\n");
        for se in &bundle.review_sheet.side_effects {
            out.push_str("- `");
            out.push_str(se.as_str());
            out.push_str("`\n");
        }
        out.push('\n');
    }

    if !bundle.review_sheet.blocked_prerequisites.is_empty() {
        out.push_str("Blocked prerequisites:\n\n");
        for prereq in &bundle.review_sheet.blocked_prerequisites {
            out.push_str("- `");
            out.push_str(prereq.class.as_str());
            out.push_str("` -> repair hook `");
            out.push_str(&prereq.repair_hook_ref.hook_kind);
            out.push_str("` (id `");
            out.push_str(&prereq.repair_hook_ref.hook_id);
            out.push_str("`)");
            if let Some(reason) = &prereq.disabled_reason_code {
                out.push_str(", disabled_reason=`");
                out.push_str(reason);
                out.push('`');
            }
            out.push('\n');
        }
        out.push('\n');
    }

    if let Some(summary) = &bundle.review_sheet.secret_handling_summary {
        out.push_str("Secret handling: any-secret-bearing=");
        out.push_str(if summary.any_secret_bearing_field {
            "yes"
        } else {
            "no"
        });
        out.push_str(", all-handle-only=");
        out.push_str(if summary.all_handle_only { "yes" } else { "no" });
        out.push_str(", runtime-reveal-armed=");
        out.push_str(if summary.any_runtime_reveal_armed {
            "yes"
        } else {
            "no"
        });
        out.push_str(", redaction=`");
        out.push_str(summary.redaction_class.as_str());
        out.push_str("`\n\n");
    }
}
