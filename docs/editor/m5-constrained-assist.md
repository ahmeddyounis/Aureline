# Constrained-file and degraded-provider assist-narrowing model

One canonical, frozen, export-safe model that states how every editor assist
micro-surface **narrows, downgrades, blocks, or routes elsewhere** when file state
or provider certainty means Aureline cannot safely offer the same completion /
hint / hover / refactor behavior it offers on an ordinary source file. Where the
[editor-assist matrix](m5-editor-assist.md) freezes the per-**surface** degraded
policy, the [completion-row model](m5-completion-rows.md) the shared *suggestion
row*, and the [hover/peek model](m5-hover-peek.md) the contextual inspectors, this
model freezes the orthogonal axis those three assume: the canonical
**constrained-file state classes** and the **degraded-provider** posture, projected
once into every assist channel.

Before this model, each pane decided locally what to do with a generated,
protected, read-only, projection, captured-evidence, partially-indexed, restricted,
or large file — one greyed completion silently, another offered an apply that could
never land, a third dropped a side-effectful refactor with no explanation. The
model folds all of that into one governed narrowing model so that, for every
constrained state and channel, the editor exposes a **degraded-state verdict**, an
**inspectable reason**, a **next-safe-action route**, and **keyboard reachability**.

- Schema: [`schemas/editor/m5-constrained-assist.schema.json`](../../schemas/editor/m5-constrained-assist.schema.json)
- Canonical fixture: [`fixtures/editor/m5-constrained-assist/canonical_model.json`](../../fixtures/editor/m5-constrained-assist/canonical_model.json)
- Rust truth source: `crates/aureline-editor/src/m5_constrained_assist`
- Headless emitter: `cargo run --bin aureline_m5_constrained_assist`
- Freeze gate: `cargo test -p aureline-editor --test m5_constrained_assist_replay`

The model **reuses** the assist and surface contracts rather than forking them:
each profile embeds the canonical `AssistSourceDescriptor` for provider provenance,
reuses the `AssistChannelClass`, `AssistDegradeClass`, and `EditorSurfaceClass`
catalogs from the editor-assist matrix, and reuses the language router's
`DegradedStateClass` for the provider axis. The eight constrained-file state
classes each **document the canonical landed class they project** — the
writable-boundary, generated-artifact, managed-zone, projection, captured-evidence,
partial-index, restricted-mode, and large-file-posture state classes — they are not
redefined here.

## The constrained-file state classes

Each state resolves into a `ConstrainedStateProfile` with one narrowing cell per
channel, a primary next-safe-action route, and the provider posture that travels
with it.

| State | Projects | Primary next-safe action | Blocks direct apply |
|---|---|---|---|
| `read_only_boundary` | writable-boundary read-only posture | duplicate to an editable copy | yes |
| `generated_artifact` | generated-artifact state class | open generator source | yes |
| `managed_region` | managed-zone state class | regenerate from source | yes |
| `projection_view` | projection state class | edit the underlying source | yes |
| `captured_evidence` | captured-evidence state class | inspect only | yes |
| `partial_index` | partial-index state class | wait for the index | no |
| `restricted_mode` | restricted / protected-path state class | request approval | yes |
| `large_file` | large-file posture state class | open in the full editor | yes |

## The narrowing cell

Every `AssistNarrowingCell` carries the truth that keeps a narrowed affordance
honest:

| Field | Why |
|---|---|
| `channel`, `applicable` | Which assist channel, and whether it is offered at all on this state. |
| `degrade_class` | The canonical degraded-state verdict (`full_fidelity` / `source_labeled_fallback` / `read_only_no_apply` / `suppressed_large_file` / `pending_partial_index` / `blocked_unavailable`). |
| `apply_blocked` | Whether a direct in-buffer apply is blocked here. |
| `narrow_reason` | The closed reason the channel is narrowed (`write_routes_through_generator`, `projection_edits_route_to_source`, `write_requires_approval`, `write_boundary_read_only`, `snapshot_immutable`, `index_still_building`, `suppressed_for_safety`, `provider_degraded_fallback`). |
| `next_safe_action`, `next_safe_action_command_ref` | The nearest safe action and the command that reaches it. |
| `keyboard_reachable` | Every offered cell stays keyboard-reachable. |
| `disabled_state_diagnostic` | The inspectable *why*, shown as a tooltip, disabled-state diagnostic, and support-export line. |

## The per-channel narrowing matrix

Generated and pinned in the fixture. Columns are the nine assist channels; cells
are the resolved `degrade_class`.

| State \ channel | completion | signature | snippet | code lens | inlay | hover | peek | inline AI | decoration |
|---|---|---|---|---|---|---|---|---|---|
| read_only_boundary | read_only_no_apply | full | read_only_no_apply | read_only_no_apply | full | full | full | read_only_no_apply | full |
| generated_artifact | read_only_no_apply | full | read_only_no_apply | read_only_no_apply | full | full | full | read_only_no_apply | full |
| managed_region | read_only_no_apply | full | read_only_no_apply | read_only_no_apply | full | full | full | read_only_no_apply | full |
| projection_view | read_only_no_apply | full | read_only_no_apply | read_only_no_apply | full | full | full | read_only_no_apply | full |
| captured_evidence | blocked_unavailable | full | blocked_unavailable | read_only_no_apply | full | full | full | blocked_unavailable | full |
| partial_index | pending_partial_index | pending_partial_index | full | pending_partial_index | pending_partial_index | pending_partial_index | pending_partial_index | full | full |
| restricted_mode | read_only_no_apply | full | read_only_no_apply | read_only_no_apply | full | full | full | read_only_no_apply | full |
| large_file | suppressed | suppressed | suppressed | suppressed | suppressed | suppressed | suppressed | suppressed | source_labeled_fallback |

Three rules drive the matrix:

- **Editing-truth decorations stay full fidelity** on every state except
  `large_file`, where the file is not fully parsed and they narrow to a labeled
  lexical fallback rather than being dropped.
- **Reading channels (hover, peek, signature, inlay) stay full fidelity** on the
  write-blocking states — reading a generated, projected, read-only, or restricted
  file is fine; the constraint is on *applying*.
- **Apply-capable and actionable channels narrow** with a reason and a route: to
  `read_only_no_apply` on the write-blocking states, to `blocked_unavailable` on the
  immutable captured-evidence snapshot, to `pending_partial_index` while the index
  builds, and to `suppressed_large_file` in large-file mode.

## Next-safe-action routes

Whenever direct assist or apply is narrowed, a closed `NextSafeActionClass` names
the nearest safe thing the user can do and the command that reaches it:

`open_generator_source`, `regenerate_from_source`, `duplicate_editable_copy`,
`request_approval_review`, `edit_underlying_source`, `wait_for_index`,
`open_in_full_editor`, `reconnect_provider`, and `view_only_no_action`. The last is
the honest terminal answer — there is no safe mutation route on an immutable
snapshot, but reading and copying remain.

## The degraded-provider axis

The same narrowing applies when only the **provider**, not the file, is degraded.
`DegradedProviderCase`s prove it on otherwise-ordinary writable files:

| Case | Channel | Verdict | Route |
|---|---|---|---|
| provider unavailable | completion | source_labeled_fallback | reconnect provider |
| scope narrowed (index warming) | hover | pending_partial_index | wait for index |
| stale awaiting refresh | signature help | source_labeled_fallback | reconnect provider |

Each is source-labeled (never silently styled as live), discloses its reason, and
offers a route.

## Consumer-surface proofs

The claimed surfaces bind back to the shared state vocabulary rather than inventing
local special cases:

| Surface | Exhibited state | Channel | Verdict | Route |
|---|---|---|---|---|
| notebook cell | partial_index | completion | pending_partial_index | wait for index |
| generated file | generated_artifact | completion | read_only_no_apply | open generator source |
| request / response artifact | captured_evidence | completion | blocked_unavailable | inspect only |
| docs-code block | projection_view | completion | read_only_no_apply | edit underlying source |
| protected config | restricted_mode | completion | read_only_no_apply | request approval |

## Honesty invariants

The model proves 17 invariants over its own data (see
[the release artifact](../../artifacts/editor/m5-constrained-assist.md)), including
that every state resolves one cell per channel, that every narrowed channel
discloses an inspectable reason, that every blocked apply offers a concrete route,
that no apply-capable channel is silently hidden (the guardrail), that offered cells
stay keyboard-reachable, that large-file suppresses semantic and apply channels,
that partial-index narrows semantics to a labeled pending state, that generated /
managed / restricted states route writes to regenerate / approval, that
captured-evidence is inspect-only while still reading, that editing-truth
decorations survive except in large-file mode, that degraded-provider cases are
source-labeled and routed, and that the claimed consumer surfaces reuse the shared
vocabulary.

## What this model is not

- **Not a live binding.** The resolved profiles are the declared policy; wiring each
  live assist surface to render the narrowing is incremental follow-up.
- **Not a redefinition of the state classes.** The constrained-file state classes
  are projected from their canonical landed definitions; their own contracts remain
  authoritative.
- **Not generic file-state redesign or Project Doctor / repair work.** The model
  stays inside assist-surface narrowing for already-claimed constrained states.
