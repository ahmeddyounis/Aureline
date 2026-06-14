# M5 macro-replay review: run-capable / cross-file downgrades and recipe-promotion

Aureline's switching promise depends on keyboard-first, recoverable interaction
across every new M5 surface — editor, notebook, data/API, preview, docs, review,
runtime, and companion-adjacent panes. The frozen keyboard-continuity matrix
([`freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md`](freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md))
pins those surfaces to their canonical interaction vocabulary and requires that
*mode changes and macro replay are explicit*. This contract discharges the
macro-replay half of that requirement: it takes the keyboard-mode packets and
makes macro replay **safe** on the new M5 surfaces by routing broad, run-capable,
or cross-file sequences through review, downgrade, or recipe-promotion instead of
replaying hidden side effects.

It does **not** introduce a new general macro language or a new editor core. The
recorded-macro / declarative-recipe boundary in
[`docs/automation/recipe_and_macro_contract.md`](../../automation/recipe_and_macro_contract.md)
still owns macro authoring; this lane only lands the parity and safety controls
that decide *how* a recorded macro is allowed to replay on a claimed M5 surface.

## Truth source

The canonical record is the export-safe packet emitted by
`aureline-shell::ship_macro_replay_review_run_capable_or_cross_file_macro_downgrades_and_recipe_promotion`:

- Schema:
  [`schemas/interaction/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.schema.json`](../../../schemas/interaction/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.schema.json)
- Checked support export:
  [`artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/support_export.json`](../../../artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/support_export.json)
- Markdown summary:
  [`artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.md`](../../../artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.md)
- Protected fixtures:
  [`fixtures/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/`](../../../fixtures/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/)

Product, help, migration guidance, support exports, and release control consume
this packet directly rather than re-deriving macro-replay safety language by hand.

## What a record carries

Each `macro_replay_review_record` binds one claimed M5 surface (keyed by a
`keyboard_surface_kind` and a non-display `subject` whose fingerprint is distinct
from its id) to one replay attempt:

- **Source register** — the register the macro was recorded into
  (`register_token` + `register_class`), preserved so the replay stays
  attributable instead of collapsing into opaque text.
- **Target scope** — a `scope` class plus `files_touched` / `surfaces_spanned`
  counts. The counts must be consistent with the class (a `cross_file_within_surface`
  macro touches ≥ 2 files on one surface; a `cross_surface_span` spans ≥ 2
  surfaces; a `workspace_wide_span` spans ≥ 2 of each).
- **Timing** — `stable_deterministic`, or an unstable `depends_on_async_output` /
  `depends_on_external_state`.
- **Exact command lineage** — an ordered, non-empty `command_lineage`. Every step
  keeps its command-graph `command_id` and `command_revision_ref`, a
  `lineage_class`, a `write_class`, and a `run_capable` flag. AI-tool-handle steps
  cite an `ai_tool_handle_ref`. The lineage is never flattened to opaque text.
- **Verification** — a reopenable proof (`proof_currency` plus, unless missing, a
  `proof_ref` keyed by a distinct `proof_fingerprint_token`).
- **Outcome** — the resolved `outcome` plus exactly the detail field it requires.

## The safety rule

A replay attempt fires zero or more `fired_triggers`. The recorded set must equal
the set computed from the record's scope, lineage, timing, and proof:

| Trigger | Fires when | Minimum outcome |
| --- | --- | --- |
| `cross_file_scope` | scope crosses files (cross-file / cross-surface / workspace) | `review_required_before_apply` |
| `run_capable_or_elevated_command` | a step runs code / a build / a request, or mutates multiple files, network, or settings | `review_required_before_apply` |
| `stale_or_missing_review_proof` | the review proof does not back a current claim for the record's origin | `review_required_before_apply` |
| `cross_surface_or_workspace_scope` | scope crosses surfaces or spans the workspace | `promoted_to_declarative_recipe` |
| `unstable_timing` | timing is not deterministic | `promoted_to_declarative_recipe` |
| `unmapped_or_unsafe_step` | a recorded step does not resolve to a command | `rejected_unsafe_replay` |

The outcome's `safety_rank` (exact `0` < review `1` < downgrade `2` < promote `3`
< reject `4`) must be **at or above** the highest floor imposed by any fired
trigger. Two consequences follow:

- **No silent broad replay.** If any trigger fires, the floor exceeds `0`, so the
  record can never resolve to `exact_replay_local_editor_only`. Cross-file and
  run-capable macros open review, downgrade to observe-no-mutation, promote to a
  recipe, or are rejected — never a hidden side effect.
- **Recipe-promotion or reject for the broadest macros.** Cross-surface /
  workspace spans and unstable-timing macros require recipe-promotion or reject;
  a guarded in-place replay is not enough. Unmapped steps require an outright
  reject rather than a silently approximated sequence.

## Outcomes and their detail fields

| Outcome | Detail field | Meaning |
| --- | --- | --- |
| `exact_replay_local_editor_only` | none | Single-file, single-surface, deterministic, no run-capable step, current proof. The only silent lane. |
| `review_required_before_apply` | `review_reason_label` | Opens a preview / review pass before any apply. |
| `downgraded_to_observer_no_mutation` | `downgrade_target_label` | Observes the recorded sequence without mutating; apply needs explicit confirm. |
| `promoted_to_declarative_recipe` | `promoted_recipe_ref` | Re-authored as a reviewable declarative recipe with exact lineage. |
| `rejected_unsafe_replay` | `rejection_reason_label` | Refused; the recorded sequence cannot be safely or faithfully replayed. |

Detail labels must be precise — a generic non-answer (`"error"`, `"unavailable"`,
`"failed"`, …) is rejected. Help, migration, and support read the outcome and its
detail to distinguish **exact replay**, **downgraded replay**, and **promoted
recipe** paths on every claimed M5 modal workflow.

## Guardrails

- Cross-file / run-capable macros never replay silently.
- Exact command lineage is preserved, never collapsed into opaque text.
- The source register and target scope are preserved on every record.
- Unstable-timing and cross-surface macros take the recipe-promotion path.
- Provider-linked replays never read as a locally verified replay (a
  provider/imported surface carries imported proof; a local surface never leans
  on imported proof).
- No new general macro language or editor core is introduced here.
- No raw keystroke buffers, editor buffer bytes, shell fragments, provider
  payloads, file contents, private paths, or credentials cross the boundary.
