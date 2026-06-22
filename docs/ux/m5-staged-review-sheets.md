# M5 staged-review (commit) sheets across mutation flows

One canonical, export-safe model for the **commit sheet itself** — the surface
every consequential M5 mutation flow stops at *before* it changes
remote/provider/admin/package/request/import state. One review model is reused
across provider publish-later, admin/source-management, request replay/mutation,
package install/update/remove, and import/export/publish flows, instead of each
domain minting a one-off confirm dialog.

Where [the field/control-row model](m5-field-control-rows.md) freezes a single
field's label, source, and validation anchor,
[the form-validation model](m5-form-validation-and-blocked-submit.md) freezes how
field validity rolls up into a blocked-submit reason,
[the structured-input / staged-review model](m5-structured-input-and-staged-review.md)
freezes the per-surface honesty claim of a whole form, and
[the draft-state model](m5-draft-state-and-autosave.md) freezes what survives an
interruption, this model freezes the **preview-before-commit** truth a mutation
must hold at the moment of commit: the target scope is named, the
included/excluded/blocked/hidden counts reconcile, every omitted default and side
effect is disclosed, the rollback/export path is visible, and the commit button
names the scope and effect rather than a generic Continue.

- Schema: [`schemas/ux/m5-staged-review-sheets.schema.json`](../../schemas/ux/m5-staged-review-sheets.schema.json)
- Canonical support export: [`artifacts/ux/m5-staged-review-sheets/support_export.json`](../../artifacts/ux/m5-staged-review-sheets/support_export.json)
- Report: [`artifacts/ux/m5-staged-review-sheets/report.md`](../../artifacts/ux/m5-staged-review-sheets/report.md)
- Perturbation corpus: [`fixtures/ux/m5-staged-review-sheets/`](../../fixtures/ux/m5-staged-review-sheets/)
- Rust truth source: `crates/aureline-ui/src/m5_staged_review_sheets`
- Validator: `tools/release/staged_review_sheets.py`

## What a sheet carries

Each `ReviewSheetRecord` (`#/$defs/sheet`) is one staged-review sheet, identified
by `sheet_id`, its `flow`, its `lane`, its `origin`, and its `claim_posture`, plus
the commit-review contract it must hold (`sheet`):

- **Target scope** (`sheet.scope`) — the `scope_kind` (`single_object`,
  `multi_object_explicit`, `query_backed`, `workspace_wide`), whether the scope is
  `scope_declared`, and a reviewer-facing `scope_label`. The user always sees
  *what* the commit acts on; a query-backed or workspace-wide action can never hide
  its breadth behind a collapsed list.
- **Omitted defaults** (`sheet.omitted_defaults_disclosed`,
  `sheet.omitted_default_count`) — how many default values are being applied
  silently, disclosed with a count rather than hidden.
- **Included / excluded / blocked / hidden counts** (`sheet.counts`) — a reconciled
  `MemberCounts` block where `included + excluded + blocked + hidden ==
  total_matched`, plus the per-object `members[]` and whether the counts are
  `counts_visible`. A multi-object action surfaces its counts; a query-backed/broad
  action that collapses members must disclose a `hidden` count.
- **Side-effect summary** (`sheet.side_effects`, `sheet.side_effects_disclosed`,
  `sheet.side_effect_summary_labeled`) — every `SideEffectDescriptor`
  (`reversible_local`, `reversible_with_export`, `irreversible_confirmed`,
  `external_publish`, `policy_governed`) disclosed before commit, with an aggregate
  summary the reviewer can read.
- **Recoverability** (`sheet.recoverability`) — the aggregate
  `recoverability_class` (`fully_reversible`, `reversible_via_export`,
  `partially_reversible`, `irreversible`), whether a `rollback_path_present` and an
  `export_path_present`, and a `recovery_label`. An irreversible or external-publish
  effect must carry an export/backup path before commit.
- **Commit action** (`sheet.commit`) — a `commit_action_is_specific` confirm that
  names the scope/effect rather than a generic Continue, plus an action-specific
  cancel.

`declared_freshness_state` with `freshness_state_visible`/`superseded_state_marked`
and a `verification` proof age a stale, superseded, or unproven scope snapshot down
instead of letting it read as fresh; `declared_reopen_target` declares what a
cancel/reopen returns the user to; `lineage` (including `target_ref`,
`provider_ref`, `source_artifact_ref`, `rollback_plan_ref`, `export_bundle_ref`,
and `reopen_backlink_ref`) attributes the sheet; and `renderings[]` lists the
consumer surfaces and the claim each one shows.

## Effective claim

`ReviewSheetRecord::narrow` re-derives a `SheetClaim` per sheet so a sheet can never
read wider than its evidence:

| Claim | Meaning |
| --- | --- |
| `sheet_certified` | Full scope-explicit, count-reconciled, side-effect-disclosed, rollback-visible commit-review contract. |
| `sheet_narrowed` | A first-party sheet held below certified by a labelled, recoverable gap (an unlabelled member class or summary, a non-specific cancel, an unlabelled recoverability posture or freshness state, a superseded/stale scope, a stale/missing proof); the scope stays reopenable. |
| `sheet_review_overlay` | A read-only review of an imported/migrated/restored state; attributable but never a local apply. |
| `sheet_unsafe` | The commit-review contract is broken; the sheet falls back to an explicit blocked state with a reopen/keyboard recovery path instead of a clean-but-false commit. |
| `sheet_labs_not_claimed` | Labs/unadvertised; makes no public claim and is never widened. |

A higher-rank claim asserts more authority, so a narrowing or floor moves strictly
lower, and a rendering that shows wider than the effective claim is itself a floor
(`sheet_overclaims`).

### Floor reasons (drop to `sheet_unsafe`)

These break the commit-review contract outright: `target_scope_hidden`,
`member_counts_inconsistent`, `hidden_members_uncounted`,
`included_excluded_blocked_counts_hidden`, `omitted_defaults_hidden`,
`side_effect_undisclosed`, `blocked_prereq_hidden`,
`rollback_consequences_hidden`, `generic_continue_action`,
`imported_review_reads_as_apply`, `reopen_path_lost`, `sheet_overclaims`, and
`sheet_backing_missing`. A floored sheet keeps a reopen/keyboard recovery fallback
(`declared_reopen_target` of `scope_only`/`none_keyboard_fallback` or a
`reopen_backlink_ref`) rather than a misleading clean commit.

### Narrowing reasons (hold at `sheet_narrowed`, stay usable)

`member_classes_unlabeled`, `side_effect_summary_unlabeled`,
`cancel_action_unlabeled`, `recoverability_class_unlabeled`, `freshness_unlabeled`,
`superseded_scope_not_marked`, `scope_stale`, `verification_proof_stale`, and
`verification_proof_missing`. On an imported/restore review overlay, any non-floor
gap drops the sheet below the overlay because the overlay is already the minimal
honest claim.

## Guardrails enforced by the validator

`M5StagedReviewSheetSetPacket::validate` (Rust) and
`tools/release/staged_review_sheets.py validate` (the CI gate) both refuse a packet
that:

- hides a sheet's target scope, lets its included/excluded/blocked/hidden counts
  disagree with the total matched, or leaves a query-backed/broad action's
  collapsed members uncounted;
- hides the included/excluded/blocked counts on a multi-object action, hides omitted
  defaults, leaves a side effect undisclosed, or buries a blocked prerequisite;
- hides the rollback/export consequence, lets a consequential commit hide behind a
  generic Continue, lets an imported/restore review read as a local apply, loses the
  reopen-to-scope path, or lets a rendering overclaim;
- floors a sheet that loses its reopen/keyboard recovery fallback;
- fails to represent every mutation flow, lane, scope kind, member class, side-effect
  class, or consumer surface, or contains no sheet that demonstrates the
  auto-narrowing rule;
- leaks raw credential/secret material across the export boundary.

## Scope and effect are always named, and an import review is never an apply

The model keeps the commit's *breadth* and its *consequence* explicit and
independent. `scope_kind` records how the target set was chosen — and a multi-object
or query-backed scope must surface a reconciled count block, while a scope that
collapses members must disclose how many are `hidden`. `recoverability_class` and
the per-effect `side_effects[]` record the consequence — and an `irreversible` or
`external_publish` commit must carry an export/backup path before it runs. An
imported/migrated/restored review stays an `imported_review` overlay: it is
attributable and reopenable but never reads as a local apply, and any non-floor gap
on it drops it below the overlay rather than holding it.

## Regenerating the artifacts

```bash
# Canonical support export + report (Rust seed is the source of truth)
cargo run -p aureline-ui --example dump_m5_staged_review_sheets \
  > artifacts/ux/m5-staged-review-sheets/support_export.json
cargo run -p aureline-ui --example dump_m5_staged_review_sheets report \
  > artifacts/ux/m5-staged-review-sheets/report.md

# Perturbation corpus
python3 tools/release/staged_review_sheets.py emit-corpus

# Verify everything (schema, re-derivation, corpus)
python3 tools/release/staged_review_sheets.py self-test
cargo test -p aureline-ui m5_staged_review
```

The Rust seed builder, the checked-in support export, and the Python re-derivation
are kept byte-aligned: a Rust test asserts the checked-in export equals the in-crate
builder, and the Python `self-test` re-derives every sheet and corpus case so the
artifacts can never imply a wider claim than the current evidence backs.
