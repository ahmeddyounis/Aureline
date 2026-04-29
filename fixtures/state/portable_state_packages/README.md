# Portable-state package fixtures

These fixtures are short, reviewable scenarios that anchor the
portable-state package, checksum/redaction manifest, and cross-machine
import contract frozen in
[`/docs/state/portable_state_package_contract.md`](../../../docs/state/portable_state_package_contract.md)
and validated by the portable-state manifest schema:

- [`/schemas/state/portable_state_manifest.schema.json`](../../../schemas/state/portable_state_manifest.schema.json)

Each fixture is one `portable_state_manifest_record` body. The
companion persistence-inspector contract and its export-sheet /
restore-provenance schema sit one layer above this manifest:

- [`/schemas/state/portable_state_package.schema.json`](../../../schemas/state/portable_state_package.schema.json)
  validates the inspector, export-sheet, and restore-provenance
  records; this manifest validates the package body those surfaces
  describe.

## Scope rules

- Fixtures validate against the portable-state manifest schema; they
  do not encode raw secrets, raw absolute paths, raw hostnames, raw
  command lines, raw logs, raw provider payloads, or raw source
  content. Per-section bodies are referenced by opaque ref only and
  validate under their own contracts.
- Every fixture MUST exercise exactly one cross-machine
  `import_posture_class` (`exact`, `compatible`, `downgraded`, or
  `inspect_only`) and MUST cite the motivating section of the package
  contract.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- Excluded live authority and machine-local artifacts stay explicit
  under `machine_local_exclusions[]` instead of vanishing silently.
- Redaction manifests stay reviewable: every fixture lists the
  enforced floors (raw secrets, credentials, command lines, source
  content, provider payloads) and the typed rules that scope the
  redaction to specific section classes.

## Index

| Fixture | Posture | Key coverage |
|---|---|---|
| [`exact_user_portable_export.yaml`](./exact_user_portable_export.yaml) | `exact` | user-portable export between two installations of the same producer build; selected profile defaults, keybindings, snippets, themes, command aliases, saved view, and approved docs pack travel intact |
| [`compatible_migration_handoff.yaml`](./compatible_migration_handoff.yaml) | `compatible` | migration handoff with declared keybindings and workspace-manifest equivalence maps; rollback checkpoint and preserved prior artifacts keep pre-apply bodies inspectable |
| [`downgraded_workspace_layout_export.yaml`](./downgraded_workspace_layout_export.yaml) | `downgraded` | workspace layout export across channel-floor violation and missing-extension dependency; saved view falls onto a safe placeholder; rollback checkpoint stays reachable |
| [`inspect_only_support_review.yaml`](./inspect_only_support_review.yaml) | `inspect_only` | support-review export off the producing installation; apply is disabled with a typed reason; compare-against-prior-export and stricter re-export remain admitted |

## Coverage contract

The shared fixture set MUST keep:

- at least one fixture for each `import_posture_class`
  (`exact`, `compatible`, `downgraded`, `inspect_only`);
- at least one fixture for each `package_purpose` represented in the
  set (`user_portable_export`, `migration_handoff_export`,
  `workspace_layout_export`, `support_review_export`); the remaining
  purpose `restore_compare_export` may be added when a motivating
  scenario lands;
- at least one fixture that exercises `compatibility_range` with
  `platform_class_allowance = named_classes_only` so destination-side
  platform-class admission is challengeable;
- at least one fixture that exercises a non-empty
  `machine_local_exclusions[]` with reasons that span
  `credential_store_only`, `local_absolute_path`,
  `display_hint_best_effort_only`, `contains_live_handle`, and
  `policy_excludes_export`; and
- at least one fixture where the redaction manifest carries an
  `advisory` rule (admitted only on `support_review_export`) so the
  difference between enforced and advisory enforcement remains
  reviewable.
