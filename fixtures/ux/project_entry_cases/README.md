# Project-entry chooser-row and open-flow sheet fixtures

This corpus is the worked-example projection of
[`/docs/ux/project_entry_contract.md`](../../../docs/ux/project_entry_contract.md)
and the two boundary schemas:

- [`/schemas/ux/entry_chooser_row.schema.json`](../../../schemas/ux/entry_chooser_row.schema.json)
- [`/schemas/ux/open_flow_sheet.schema.json`](../../../schemas/ux/open_flow_sheet.schema.json)

Each fixture is one record. Row records (`entry_chooser_row_record`)
validate against the row schema; sheet records
(`open_flow_sheet_record`) validate against the sheet schema. A row
record names its paired sheet via `open_flow_sheet_ref`; the
referenced sheet fixture lives in the same directory.

## Row + sheet pairings

| Row fixture | Paired sheet fixture |
|---|---|
| `entry_row_open_local_file_start_center.json` | `entry_sheet_open_local_file_no_review.json` |
| `entry_row_open_local_folder_drag_drop.json` | `entry_sheet_open_local_target_folder.json` |
| `entry_row_open_workspace_palette.json` | (re-uses `entry_sheet_open_local_target_folder.json` semantics — workspace variant) |
| `entry_row_clone_remote_repository_start_center.json` | `entry_sheet_clone_remote_target.json` |
| `entry_row_start_from_template_snapshot.json` | `entry_sheet_start_from_template_or_prebuild.json` |
| `entry_row_import_handoff_archive_compare.json` | `entry_sheet_import_artifact_compare.json` |
| `entry_row_recent_work_remote_resume.json` | `entry_sheet_restore_or_resume_managed_cloud.json` |
| `entry_row_restore_last_session.json` | (re-uses `entry_sheet_restore_or_resume_managed_cloud.json` semantics — restore variant) |
| `entry_row_add_root_workspace_switcher.json` | `entry_sheet_add_root_to_active_workspace.json` |
| `entry_row_deep_link_intent_review.json` | `entry_sheet_deep_link_intent_review.json` |
| `entry_row_cli_headless_open_preview.json` | (re-uses `entry_sheet_open_local_target_folder.json` semantics — CLI text-block render) |

## Coverage matrix

Row kinds exercised: `file`, `folder`, `workspace`, `remote`,
`template`, `archive_or_handoff`, `recent`, `restore`,
`deep_link_intent_review`. The `recent` row resolves to the
`resume` verb (the recent's underlying target_kind is
`managed_cloud_workspace`); the `restore` row resolves to the
`restore` verb.

Surface classes exercised: `start_center`,
`workspace_switcher_palette`, `command_palette`,
`drag_drop_preview`, `cli_headless_preview`,
`deep_link_intent_review`.

Verbs exercised: `open` (file / folder / workspace / CLI / deep
link variants), `clone`, `import`, `add_root`, `restore`, `resume`,
`start_from_snapshot`. All seven verbs from
[`project_entry_contract.md` §3](../../../docs/ux/project_entry_contract.md)
are covered.

Sheet classes exercised: `open_local_target`,
`open_local_target_no_review_required`, `clone_remote_target`,
`import_artifact`, `add_root_to_active_workspace`,
`restore_or_resume`, `start_from_template_or_prebuild`,
`deep_link_intent_review`.

Trust postures exercised: `trust_pending_until_admission`,
`trust_never_implied_by_clone`, `trust_unchanged_until_admit`,
`trust_per_root_admission`, `trust_inherited_from_target`,
`trust_revalidated_at_resume`. All six §4.4 values covered.

Profile-default classes exercised: `default_profile`,
`sticky_for_target_or_default`, `sticky_for_target`, `unchanged`,
`depends_on_artifact_class`, `default_or_template_default`. The
`last_active_profile` and `locked_profile_required` variants are
reserved for downstream deployment-profile fixtures.

## Pre-commit invariants

Every sheet fixture asserts the seven §5.3 pre-commit invariants
as boolean `true` on `pre_commit_invariants`:

- `no_durable_write_before_commit`
- `no_trust_change_before_commit`
- `no_runtime_attach_before_commit`
- `no_profile_retarget_before_commit`
- `no_verb_mutation_after_activation`
- `fallback_always_reachable`
- `keyboard_reachable`

A sheet that emits any of these as `false` is non-conforming and
the surface MUST not commit.

## Adding a fixture

1. Pick the smallest scenario that exercises a single new value
   (a new row kind, surface class, verb, sheet class, trust
   posture, or profile-default class).
2. Write a row fixture and (when applicable) a paired sheet
   fixture; reference the sheet from the row via
   `open_flow_sheet_ref`.
3. Add the pairing to the table above and the value to the
   coverage matrix.
4. Validate both fixtures against the relevant boundary schema.
