# Migration center / learnability surface (beta)

This page describes the beta-grade migration center and learnability
projection that lives in `aureline-shell`. The migration center is
the one canonical learnability surface a new or migrating user lands
on to find current docs/help anchors, retained migration reports,
known limits, glossary packs, and recovery routes — without forcing
account or marketplace detours, and without ad hoc docs hunting.

The projection lives under the shared contract ref
`shell:migration_center_beta:v1` and is consumed by the live shell,
by the headless inspector
([`aureline_shell_migration_center`](../../../crates/aureline-shell/src/bin/aureline_shell_migration_center.rs)),
and by the support-export wrapper, so UI rows, CLI rows, and
support-export rows always come from the same `page_id`, `section_id`,
and `entry_id`.

Companion artifacts:

- [`/crates/aureline-shell/src/migration_center/mod.rs`](../../../crates/aureline-shell/src/migration_center/mod.rs)
  — Rust module that owns the page model, validator, and seeded
  builder.
- [`/fixtures/ux/m3/migration_center/`](../../../fixtures/ux/m3/migration_center/)
  — minted-from-truth fixtures (page, sections, entries, support
  rows, support export, defects, summary).
- [`/docs/migration/m3/migration_wizard_beta.md`](../../migration/m3/migration_wizard_beta.md)
  — wizard projection the migration center reopens its mapping
  report and rollback checkpoint from.
- [`/docs/migration/m3/incumbent_flow_matrix.md`](../../migration/m3/incumbent_flow_matrix.md)
  — incumbent-flow scoreboard the migration center quotes its
  known-limits lane from.

## Contract surface

The beta migration center emits these record kinds:

- `shell_migration_center_beta_page_record` — the migration-center
  page. Carries the learnability claim, the upstream wizard/corpus
  refs, the sections, the entries, the support rows, the defect
  list, and the summary banner.
- `shell_migration_center_beta_section_record` — one section row,
  one per required section kind.
- `shell_migration_center_beta_entry_record` — one entry-point row.
- `shell_migration_center_beta_support_row_record` — one
  support-export row aligned 1:1 with a live entry.
- `shell_migration_center_beta_defect_record` — one typed validator
  defect.
- `shell_migration_center_beta_support_export_record` — the
  support-export wrapper that quotes the page plus the stable case
  ids reviewers pivot on.

## Required sections

A migration-center page MUST cover every required section kind:

| Section kind | What it holds |
| --- | --- |
| `docs_help_anchors` | Current docs and help anchors users open from the page. |
| `glossary_packs` | Glossary packs that define aureline-specific vocabulary (truth terms, migration terms). |
| `migration_reports` | Reopen links for the retained wizard mapping report and rollback checkpoint. |
| `known_limits` | Known-limits rows pulled from the incumbent-flow scoreboard. |
| `recovery_routes` | Recovery routes owned by the recovery surface (safe mode, restore chooser, support export). |
| `first_run_confusion_exits` | Explicit exits from first-run confusion: command palette, settings, keymap chord. |

The validator surfaces `section_missing_required_kind` when any of
these sections is dropped from the page.

## Entry-point contract

Each entry-point row MUST quote, in closed-vocabulary tokens:

- a `section_kind` and a matching `section_id`;
- an `entry_kind` from the closed set: `docs_help_anchor`,
  `glossary_anchor`, `migration_report_reopen`, `known_limits_lane`,
  `recovery_action`, `first_run_confusion_exit`, `keymap_reference`;
- a `keyboard_reach` posture from the closed set:
  `keyboard_first_command_invocation`,
  `keyboard_reachable_focus_path`, or `keyboard_chord_reference`;
- a `learnability_claim` block with `claim_class`,
  `lifecycle_class`, `freshness_class`, `review_window_days`,
  `evidence_date`, and `as_of` — the same vocabulary the rest of the
  beta program uses;
- the type-specific anchor: a `docs_help_anchor_ref` for docs/help
  anchor rows, a `glossary_pack_ref` for glossary anchors, a
  `migration_report_ref` for migration-report reopens, a
  `known_limit_ref` for known-limits-lane rows, a
  `recovery_route_class` for recovery actions, and a
  `keyboard_chord_ref` for keymap-reference rows.

`requires_account_detour` and `requires_marketplace_detour` MUST be
`false` on claimed beta rows. The validator surfaces
`entry_requires_account_detour` or
`entry_requires_marketplace_detour` when either is set, so the
switching promise is never quietly traded for a marketplace install
or an account sign-in.

## Keyboard reachability

Every entry must be reachable without a pointer:

- **`keyboard_first_command_invocation`** — the entry invokes a
  canonical `command_id` from the
  [`aureline-commands`](../../../crates/aureline-commands) seeded
  registry. Examples used by the seed: `cmd:command_palette.open`,
  `cmd:settings.open`, `cmd:docs.open_in_browser`,
  `cmd:workspace.import_profile`,
  `cmd:workspace.restore_from_checkpoint`.
- **`keyboard_reachable_focus_path`** — the entry sits on a stable
  focus path reachable from the migration-center page. Used for
  glossary anchors, known-limits-lane rows, recovery routes that
  need their own sheet, and docs anchors that open in the in-product
  docs browser.
- **`keyboard_chord_reference`** — the entry quotes a documented
  keymap chord that resolves to a canonical command. Used for the
  keymap-reference row.

The validator surfaces `entry_command_id_missing` when a
`keyboard_first_command_invocation` row drops its command id, and
`entry_keyboard_unreachable` when a `keyboard_chord_reference` row
drops its chord ref.

## Learnability claim

The page-level `learnability_claim` block sets the baseline. Every
entry-point row inherits the same claim vocabulary unless the row is
explicitly downgraded. The seeded beta page seeds:

- `claim_class = beta_claimed`
- `lifecycle_class = beta`
- `freshness_class = current`
- `review_window_days = 90`

A claimed beta row that flips to `freshness_class = review_overdue`
without an explicit downgrade is a contract bug. The validator
surfaces `freshness_review_overdue_on_claimed_row` and
`claim_lifecycle_freshness_drift` to keep the learnability claim
honest.

## Support-export parity

Every entry has a matching support-export row keyed by
`support_row_id`. The support row MUST agree with the live row on
the closed vocabulary tokens listed in
[`fixtures/ux/m3/migration_center/README.md`](../../../fixtures/ux/m3/migration_center/README.md).
Drift is a contract bug the validator rejects with
`support_row_vocabulary_drift`. Support exports MUST set
`raw_private_material_excluded` to `true`; raw bodies are never
exported.

## Verification

```sh
cargo test -p aureline-shell --test migration_center_beta_fixtures
cargo test -p aureline-shell --lib migration_center
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- validate
```
