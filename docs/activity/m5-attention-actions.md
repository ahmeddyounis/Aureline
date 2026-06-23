# Attention-action semantics

This document describes the working engine that applies the distinct **dismiss,
snooze, acknowledge, mute, and resolve** actions to a durable attention object and
computes exactly what each one means for retention, the badge, exact reopen
continuity, cross-client fanout, support export, and audit history.

Where the [attention-routing matrix](./m5-attention-routing.md) *names and freezes
the object model* — including the action/retention-semantics object family — and the
[envelope-routing contract](./m5-envelope-routing.md) *routes a fresh envelope to its
surfaces*, this lane *implements what happens after a person acts on an existing
durable object*. The five actions are not one generic "close": each carries a
distinct retention, badge, resume, and audit meaning, and none of them erases the
durable record or reissues the original side effect.

The track invariant this lane protects: **attention is routed, typed,
privacy-aware, and reopen-safe.** Clearing a badge never erases the durable record;
every surface reopens the authoritative object rather than reissuing a blind side
effect; suppression and quiet-hours state stay separate from audit history; and the
activity center, OS notification, companion, and operator surfaces share one action
model rather than inventing local variants.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update in
the same change.

## The five distinct actions

[`apply_attention_action`](../../crates/aureline-activity/src/m5_attention_actions/mod.rs)
is a pure function of an
[`AttentionItem`](../../crates/aureline-activity/src/m5_attention_actions/mod.rs) — a
durable attention object already present in the activity center — and an
[`AttentionActionClass`](../../crates/aureline-activity/src/m5_attention_actions/mod.rs).
It returns an [`AttentionActionOutcome`](../../crates/aureline-activity/src/m5_attention_actions/mod.rs)
that records the resulting lifecycle state, retention class, badge effect and exact
count delta, the resume condition (for snooze and mute only), the per-surface
propagation, and a short reviewable support-export sentence. The same `(item,
action)` yields the same outcome byte-for-byte in support export and CLI / headless
diagnostics.

| Action | Resulting state | Retention | Badge effect | Resume | Scope | Suppression |
| --- | --- | --- | --- | --- | --- | --- |
| `dismiss` | dismissed | `durable_until_archived` | `clear_keep_record` | none | this item | no |
| `snooze` | snoozed | `suppression_state_separate` | `clear_until_resume` | timer / predicate | this item | yes |
| `acknowledge` | acknowledged | `durable_until_resolved` | `clear_mark_read` | none | this item | no |
| `mute` | suppressed | `suppression_state_separate` | `clear_and_suppress_source` | until unmuted | this source | yes |
| `resolve` | resolved | `durable_until_archived` | `clear_on_resolve` | none | this item | no |

- **Dismiss** clears the badge while keeping the durable record; the underlying
  event is neither read nor resolved.
- **Snooze** defers the item with a resume condition; it leaves the badge now and
  returns automatically when the condition fires.
- **Acknowledge** marks the item read; the underlying work stays open and durable
  until it resolves.
- **Mute** suppresses the whole source from the badge and out-of-window fanout until
  it is unmuted; existing and future events still accrue durably.
- **Resolve** closes the item because its underlying object changed or the user
  marked it done; it is retained as resolved history.

Snooze and mute are the only actions that record a deferral / suppression marker, and
that marker lives in a **separate ledger** rather than overwriting audit history.
Every action is audit-append-only.

## Exact reopen continuity

Every outcome reopens the **same authoritative target** through the **same anchor**
and the **same stable action target** as its source item — `reopen_target`,
`reopen_anchor_ref`, and `action_target_id` are copied from the item, and
`reopen_continuity_preserved` is computed from that equality. Acting never reissues
the original notification's side effect (`replays_side_effects` is always false), so
a support export can explain what happened without replaying it.

## One action model across surfaces

The action propagates to every surface the item fans out to, and each surface
reflects the same resulting state and the same stable action target:

- the **in-app activity center** applies the action authoritatively, transitioning
  the durable record (`apply_authoritative`);
- the **dock / taskbar badge** drops the item's contribution from the deduped count
  (`clear_count`);
- the **OS notification** is withdrawn without replaying any side effect
  (`withdraw_no_replay`);
- the **browser / mobile companions** and the **operator dashboard** reflect the new
  state from the same action target and never re-execute the action
  (`reflect_state_no_replay`).

No surface replays a side effect, so OS notification, in-app rows, companion
summaries, and operator surfaces share one action model rather than inventing local
variants.

## The honesty rules, enforced

The canonical [`attention_actions_bundle`](../../crates/aureline-activity/src/m5_attention_actions/mod.rs)
computes each invariant's `holds` flag from the built action definitions, items, and
outcomes, so an inconsistent edit flips an invariant and fails the freeze gate:

- `action.five_distinct_actions` / `action.badge_effects_distinct` /
  `action.semantics_distinct` — the five actions carry distinct resulting states,
  badge effects, and full `(state, badge, resume, scope)` signatures.
- `action.keeps_underlying_record` — every action clears the badge but keeps the
  underlying durable record.
- `action.exact_reopen_continuity` — every outcome reopens the same authoritative
  target, anchor, and action target as its source item.
- `action.no_side_effect_replay` — no action and no surface propagation replays the
  original side effect.
- `action.surface_parity` — every outcome applies authoritatively in-app and on the
  badge and reflects the same action target on every surface.
- `action.suppression_separate_from_audit` — snooze and mute record their deferral
  separately from audit history; every action is audit-append-only.
- `action.resume_condition_present_iff_required` — a resume condition is present
  exactly when the action defers (snooze and mute).
- `action.badge_clears_never_negative` — the badge count after equals the count
  before minus the item's contribution, never negative, never increasing.
- `action.support_export_explains_without_replay` — every outcome carries a non-empty
  support-export note and replays no side effect.
- `action.security_not_silenceable` — a security advisory is never dismissed,
  snoozed, or muted; only acknowledged or resolved.
- `action.matrix_bound` — every action token, resulting state, retention class, and
  reopen target is one the attention-routing matrix defines.

## Companion artifacts

- [`/schemas/activity/m5-attention-actions.schema.json`](../../schemas/activity/m5-attention-actions.schema.json)
  — boundary schema for `m5_attention_actions_bundle`.
- [`/fixtures/activity/m5-attention-actions/canonical_bundle.json`](../../fixtures/activity/m5-attention-actions/canonical_bundle.json)
  — the published canonical bundle; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/activity/m5-attention-actions.md`](../../artifacts/activity/m5-attention-actions.md)
  — the human-readable companion (action, item, and worked outcome tables).
- `crates/aureline-activity/src/m5_attention_actions/` — the action definitions, the
  attention-item corpus, the action engine, the invariants, and the canonical
  builder.
- `crates/aureline-activity/tests/m5_attention_actions.rs` — the freeze gate.
- `cargo run -p aureline-activity --example dump_m5_attention_actions` — the headless
  emitter that regenerates the fixture (`-- --lines` for the human-readable
  projection).
