# Schema/example drift gate

This gate keeps schema changes, reference example payloads, and any docs snippets
that embed those payloads from silently diverging.

Canonical sources:

- Source map: `artifacts/ci/contract_example_sources.yaml`
- Rules: `tools/ci/contract_example_drift_rules.md`
- Example index: `artifacts/contracts/example_pack_index.yaml`
- Validator entry point: `tools/check_schema_example_drift.py`
- CI wrapper: `ci/check_schema_example_drift.sh`
- GitHub Actions gate: `.github/workflows/check_schema_example_drift.yml`

## What the gate enforces

For every protected contract family listed in the source map:

1. **Explicit review is required for protected schema/example changes.**
   If a protected schema file or protected canonical example payload changes, the
   source map must be updated in the same change.
2. **Canonical examples stay schema-valid.**
   Each protected example payload is validated against its active schema (after
   stripping fixture-only metadata keys like `$schema` and `__fixture__`).
3. **Docs snippets stay traceable to canonical sources.**
   Any docs snippet registered in the source map must match the canonical
   example payload it cites (using the configured normalization rules).

## Running locally

```bash
./ci/check_schema_example_drift.sh
```

Artifacts land under `target/schema-example-drift/`:

- `schema_example_drift_summary.txt`
- `schema_example_drift_report.json`

## Worked docs snippet (canonical source binding)

The snippet below is a docs-embeddable rendering of the canonical example at
`fixtures/contracts/reference_examples/commands/command_descriptor_workspace_open_folder.json`
after stripping `$schema` and `__fixture__`.

<!-- aureline-snippet: id=contract_example:command_descriptor_workspace_open_folder kind=contract_example -->
```json
{
  "accessibility_label_path": {
    "keyboard_shortcut_narration_ref": "label:workspace.open_folder:shortcut_narration",
    "long_description_ref": "label:workspace.open_folder:accessibility_long",
    "primary_label_ref": "label:workspace.open_folder:accessibility_primary",
    "role_class": "command_that_opens_dialog",
    "short_label_ref": "label:workspace.open_folder:accessibility_short"
  },
  "ai_tool_surfacing_class": "ai_callable_reversible_mutation",
  "aliases": [
    {
      "alias_id": "alias:workspace.open_folder:legacy_file_open_folder",
      "alias_kind": "legacy_command_id"
    },
    {
      "alias_id": "alias:workspace.open_folder:cli_open",
      "alias_kind": "alternate_cli_verb"
    },
    {
      "alias_id": "alias:workspace.open_folder:ai_tool_open_workspace",
      "alias_kind": "ai_tool_handle"
    }
  ],
  "approval_posture_class": "no_approval_required",
  "capability_scope_class": "reversible_local_mutation",
  "canonical_verb": "workspace.open_folder",
  "client_scopes": [
    "desktop_product",
    "cli",
    "companion_surface",
    "remote_agent",
    "sdk_or_api"
  ],
  "command_descriptor_schema_version": 1,
  "command_id": "cmd:workspace.open_folder",
  "command_revision_ref": "cmd-rev:workspace.open_folder:2026.04.21-01",
  "declared_freshness_class": "authoritative_live",
  "default_enablement_repair_hook_ref": null,
  "docs_help_anchor_ref": {
    "anchor_id": "docs:anchor:workspace:open_folder_overview",
    "anchor_kind": "docs_page_anchor",
    "pack_id": "pack:project:aureline:01"
  },
  "lifecycle_state": "stable",
  "minted_at": "2026-04-21T12:05:00Z",
  "palette_visibility": "always_visible",
  "policy_context": {
    "execution_context_id": "exec:workspace-command-registry:01",
    "policy_epoch": "pe:2026-04-21:01",
    "trust_state": "trusted"
  },
  "preview_class": "no_preview_required",
  "primary_label_ref": "label:workspace.open_folder:primary",
  "record_kind": "command_descriptor_record",
  "redaction_class": "metadata_safe_default",
  "release_channel": "stable_channel",
  "result_contract": {
    "artifact_kind_ref": "artifact-kind:workspace:open_folder_journal_entry",
    "evidence_ref_class_required": [
      "mutation_journal_entry_ref"
    ],
    "result_contract_class": "journal_entry_appended_ref",
    "typed_value_shape_ref": null
  },
  "shortcut_narration_hint": {
    "chord_class_hint": "modifier_plus_key",
    "when_bound_narration_ref": "label:workspace.open_folder:shortcut_bound_narration",
    "when_unbound_narration_ref": "label:workspace.open_folder:shortcut_unbound_narration"
  },
  "support_class": "standard_support",
  "typed_arguments": [
    {
      "argument_kind": "workspace_scope_ref",
      "argument_name": "workspace_scope_ref",
      "default_provenance_when_omitted": null,
      "enum_value_refs": [],
      "is_required": true,
      "maximum_inclusive": null,
      "minimum_inclusive": null,
      "narration_label_ref": "label:workspace.open_folder:arg:workspace_scope_ref",
      "policy_pinned_when_trust_state_is": []
    },
    {
      "argument_kind": "boolean_flag",
      "argument_name": "add_to_workspace",
      "default_provenance_when_omitted": "default_from_descriptor",
      "enum_value_refs": [],
      "is_required": false,
      "maximum_inclusive": null,
      "minimum_inclusive": null,
      "narration_label_ref": "label:workspace.open_folder:arg:add_to_workspace",
      "policy_pinned_when_trust_state_is": []
    }
  ],
  "ui_slot_hints": [
    {
      "contextual_filter_class_ref": null,
      "menu_path_refs": [],
      "primary_or_secondary_toolbar_position_hint": "primary",
      "ui_slot_class": "command_palette",
      "weight_hint": 900
    },
    {
      "contextual_filter_class_ref": null,
      "menu_path_refs": [
        "menu:file",
        "menu:file:open"
      ],
      "primary_or_secondary_toolbar_position_hint": "primary",
      "ui_slot_class": "global_application_menu",
      "weight_hint": 900
    },
    {
      "contextual_filter_class_ref": "filter:explorer:folder_row",
      "menu_path_refs": [
        "menu:explorer:open_as_workspace"
      ],
      "primary_or_secondary_toolbar_position_hint": "secondary",
      "ui_slot_class": "explorer_context_menu",
      "weight_hint": 500
    },
    {
      "contextual_filter_class_ref": null,
      "menu_path_refs": [],
      "primary_or_secondary_toolbar_position_hint": "overflow_only",
      "ui_slot_class": "keybinding_help",
      "weight_hint": 700
    },
    {
      "contextual_filter_class_ref": null,
      "menu_path_refs": [],
      "primary_or_secondary_toolbar_position_hint": "overflow_only",
      "ui_slot_class": "cli_help",
      "weight_hint": 700
    }
  ]
}
```
<!-- /aureline-snippet -->

## Fixture scenarios

The repository includes deterministic failing scenarios under
`fixtures/ci/schema_example_drift_cases/`. Run one by passing `--scenario`:

```bash
python3 tools/check_schema_example_drift.py --scenario fixtures/ci/schema_example_drift_cases/schema_bump_without_example_review.yaml
```

