# M5 Focus-Return and Stable-Selection Contract

This document is the contract for the M5 focus-and-selection packet that makes focus
movement and dense-surface navigation explicit across the claimed M5 shell zones,
dense collections, overlays, multi-window layouts, and follow/presentation flows.
Where the frozen dynamic-surface matrix freezes *which* focus-return dispositions an
accessibility object may admit, and the
[live-announcement grammar](./m5-announcement-grammar.md) governs *how* a focus or
selection change is narrated, this contract supplies the concrete *focus and selection
behavior* a keyboard or assistive user can rely on: dialogs, sheets, palettes,
popovers, rename fields, inspector promotions, streamed lists, multi-window restores,
and follow/presentation modes preserve stable item identity and predictable return
paths.

- Record kind: `m5_focus_and_selection_contract`
- Schema: [`schemas/a11y/m5-focus-selection.schema.json`](../../schemas/a11y/m5-focus-selection.schema.json)
- Canonical support export: [`artifacts/a11y/m5-focus-return-proof/support_export.json`](../../artifacts/a11y/m5-focus-return-proof/support_export.json)
- Governance summary artifact: [`artifacts/a11y/m5-focus-return-proof/focus-return-proof.md`](../../artifacts/a11y/m5-focus-return-proof/focus-return-proof.md)
- Fixtures: [`fixtures/a11y/m5-focus-return/`](../../fixtures/a11y/m5-focus-return/)
- Producer: `aureline_shell::focus::current_stable_m5_focus_selection_export`
- Headless emitter: `aureline_shell_m5_focus_return`
- Focus / zoom / pointer-independence contract this lane reuses: [`docs/accessibility/focus_zoom_and_pointer_independence_contract.md`](../accessibility/focus_zoom_and_pointer_independence_contract.md)

## Why this contract exists

Keyboard and assistive users lose trust when focus jumps unpredictably, when row
identity resets under virtualization, or when an overlay closes without returning them
to a safe working context. Before this contract, which surface returned focus where —
and whether a refreshed or restored collection kept the user's current item — was
implicit per surface. This contract makes focus and selection a single governed
packet: one zone per governed surface, each stating an explicit focus-return target
and a safe fallback, a stable-item-identity rule, and (for dense collections) a
roving-tabindex rule. The same zones are reused by support exports, docs/help, and
assistive-tech conformance packets, so a focus-teleport or selection-drift regression
is debuggable from the export alone.

## Governed focus zones

The contract carries one zone row for each governed focus zone:

| Zone | Row | Interaction model | Roving tabindex |
| --- | --- | --- | --- |
| `modal_dialog` | `focus-zone:modal-dialog` | transient overlay | — |
| `sheet` | `focus-zone:sheet` | transient overlay | — |
| `command_palette` | `focus-zone:command-palette` | transient overlay | — |
| `popover` | `focus-zone:popover` | transient overlay | — |
| `rename_field` | `focus-zone:rename-field` | transient overlay | — |
| `inspector_promotion` | `focus-zone:inspector-promotion` | transient overlay | — |
| `dense_collection` | `focus-zone:dense-collection` | dense collection | yes |
| `streamed_list` | `focus-zone:streamed-list` | dense collection | yes |
| `shell_zone` | `focus-zone:shell-zone` | shell zone | — |
| `multi_window_layout` | `focus-zone:multi-window-layout` | multi-window layout | — |
| `follow_presentation` | `focus-zone:follow-presentation` | follow/presentation | — |

## What each zone binds

Each `focus_zone_contract` binds a stable `zone_id` (prefixed `focus-zone:`) to:

- **An explicit focus-return rule** — `focus_return` names a real `return_target_ref`,
  a `primary_disposition` for when the invoking object still exists, and a
  `safe_fallback_disposition` for when it no longer exists. The fallback always locates
  a new real owner (`returned_nearest_safe_ancestor`,
  `returned_current_batch_or_detail_owner`, or `returned_placeholder_announced`) — it
  can never return to the exact prior owner or to a non-interactive surface, so focus
  never teleports to an unrelated surface or vanishes. A `returned_placeholder_announced`
  fallback must set `announces_return`.
- **A stable-item-identity rule** — `stable_identity` records a non-row-index
  `identity_strategy` (`stable_key`, `content_hash`, or `path_or_uri`), preserves both
  focus and selection, and lists the `preserved_across` async-update classes
  (`virtualization`, `refresh`, `streaming_insert`, `filtering`, `sort_change`,
  `multi_window_restore`, `layout_adjustment`, `overlay_teardown`). Each interaction
  model requires a specific set of those classes, so virtualization, refresh, and
  restore can never degrade into row-index-based focus loss or selection drift.
- **A roving-tabindex rule (dense collections only)** — `roving_tabindex` is present
  for, and only for, a dense collection. It pins a `single_tab_stop`, lists predictable
  `navigation_keys` (at least `arrow_up_down` and `home_end`), and sets
  `multi_selection_narrowing_announced` so a selection narrowing is announced, never
  silent.
- **A safe working context** — `durable_fallback` names the reopenable surface
  (`activity_row`, `status_detail`, `selection_summary`, …) the user returns to, so an
  overlay never closes without a recoverable working context.

## Keyboard-completeness guardrail

`keyboard_complete_claim` may be `true` only when the zone states and proves its
focus-return and stable-item-identity behavior — and, for a dense collection, its
roving-tabindex behavior. The validator rejects any zone that claims keyboard
completeness without those rules being well-formed, so a new overlay, sheet, or
collection surface cannot claim keyboard completeness it cannot back.

## Controlled vocabulary reuse

The focus-return-disposition tokens are reused verbatim from the frozen dynamic-surface
matrix through the `shared_vocabulary_set` block, which must match the matrix's
canonical token lists, and the durable-fallback-surface tokens from the announcement
grammar. The focus-shaped vocabularies this lane adds — `zone_kind`,
`interaction_model`, `async_update_class`, `identity_strategy`, and
`collection_nav_key` — are frozen in the `focus_vocabulary_set` block. No surface mints
a parallel synonym for a governed zone kind or identity strategy.

## Auto-narrowing on degraded bridge or stale proof

A zone whose assistive-tech proof has gone stale narrows its qualification (for example
Stable to Beta) and drops its keyboard-complete claim while keeping its focus-return
rule, stable-item-identity rule, roving tabindex, and `proof_stale` downgrade trigger
intact. A zone whose OS accessibility bridge is unavailable narrows (for example Stable
to Preview), drops its `non_visual_fidelity` to `degraded_accessible`, and drops its
keyboard-complete claim while keeping its stable-item-identity rule and
`bridge_unavailable` trigger — a restored window still preserves item identity rather
than degrading into row-index focus loss. The `proof_stale_narrowed.json` and
`bridge_unavailable_narrowed.json` fixtures exercise both paths: the narrowing is always
a disclosed claim change, never a hidden zone.

## Consumers

`shell` returns focus on overlay teardown; `search`/command palette returns focus to its
invoker; `review` preserves row identity across refresh; the data grid uses roving
tabindex; `notifications` route focus-return targets; and `presentation`/follow mode
preserves context across multi-window restore. Support exports, docs/help, and
assistive-tech conformance packets reuse the same contract. The `consumer_projection`
block records that every one of those consumers routes through this contract rather than
improvising per-surface focus handling.

## Regenerating the contract

The seed builders in `aureline_shell::focus` are the single producer of the checked-in
support export and fixtures. Regenerate with the headless emitter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- support-export \
  > artifacts/a11y/m5-focus-return-proof/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- markdown \
  > artifacts/a11y/m5-focus-return-proof/focus-return-proof.md
cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- fixture-proof-stale-narrowed \
  > fixtures/a11y/m5-focus-return/proof_stale_narrowed.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- fixture-bridge-unavailable-narrowed \
  > fixtures/a11y/m5-focus-return/bridge_unavailable_narrowed.json
```

The `checked_support_export_matches_seed` test fails if the checked-in export drifts
from the seed builder, so the artifact, the fixtures, and the in-code contract stay in
lockstep.
