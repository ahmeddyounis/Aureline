# State migration-case fixtures

These fixtures are short, reviewable scenarios that anchor the shared
migration and restore vocabulary frozen in
[`/docs/state/migration_and_restore_playbook.md`](../../../docs/state/migration_and_restore_playbook.md)
and validated by
[`/schemas/state/restore_provenance.schema.json`](../../../schemas/state/restore_provenance.schema.json).

Each fixture is one `state_restore_provenance_record` rendered as a
worked migration or restore case. The set exists so reviewers can diff a
closed fidelity label, a typed downgrade reason, a typed failure state,
and an intentional exclusion story without reverse-engineering it from
artifact-specific docs.

## Scope rules

- Fixtures validate against the shared restore-provenance schema; they
  do not encode raw profile bytes, raw layout databases, credential
  bodies, live capability tickets, or runtime-specific engine state.
- A new fixture MUST exercise at least one fidelity label, one state
  plane, one downgrade reason or failure state, or one preserved prior-
  artifact rule from the shared playbook, and MUST cite the motivating
  section.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- `portable_settings`, `local_context`,
  `workspace_shared_manifest`, and `non_portable_live_authority` are
  boundaries, not permission to flatten unlike things into one blob.

## Index

| Fixture | Result | Key coverage |
|---|---|---|
| [`profile_restore_exact.json`](./profile_restore_exact.json) | `exact` | exact portable-settings restore with explicit secret/live-handle exclusions |
| [`profile_restore_compatible_schema_shift.json`](./profile_restore_compatible_schema_shift.json) | `compatible` | schema translation plus preserved prior artifact for compare/export |
| [`layout_restore_layout_only_missing_dependencies.json`](./layout_restore_layout_only_missing_dependencies.json) | `layout_only` | missing extension, missing remote session, unsupported display topology, and no-rerun live-handle exclusions |
| [`support_bundle_manual_review_workspace_conflict.json`](./support_bundle_manual_review_workspace_conflict.json) | `manual_review` | workspace-manifest conflict, missing extension dependency, manual repair escalation, and preserved compare/export refs |

## Coverage contract

The shared fixture set MUST keep:

- at least one case for each fidelity label
  (`exact`, `compatible`, `layout_only`, `manual_review`);
- at least one case that makes intentional secret or live-handle
  exclusions explicit without pretending they restored;
- at least one case that preserves a prior artifact because schema
  meaning changed;
- at least one case that stops at manual review instead of silently
  overwriting workspace-shared truth.
