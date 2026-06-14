# M5 grouped-history continuity fixtures

## stale_proof_forces_named_group.json

A drill fixture for the grouped-history continuity packet. It keeps the full
seeded record set — including the clean flat exact-step baseline (editor-core
range edit) and one record for each non-default recovery (notebook reformat →
named exact group, preview view-source → back/forward continuity, docs publish →
labeled compensating action, data/API result → regenerate-from-source, review
thread → checkpoint restore, runtime session → reopen / recover after a
disconnect loss, provider-linked companion thread → reopen / recover after an
intentional close with imported proof) — and adds one extra drill record.

The drill record (`history:editor-core:stale-proof:0001`) is an editor-core range
edit with a single object, a literal inverse, and no breadth. On its own it would
qualify for a flat `exact_step_undo`. But its history proof has aged outside the
freshness window (`proof_currency` is `stale_expired`), so the
`stale_or_missing_history_proof` trigger fires, the required floor rises to
`named_group_exact_undo`, and the record correctly resolves to a named,
re-verified exact-undo group with a precise `group_label` (and a matching
`grouped_exact_undo` undo class). The fixture demonstrates that stale evidence —
not just a broad, cross-surface, non-invertible, generated, checkpoint-only, or
closed-surface entry — forces a history entry off the flat single-step lane, and
that the record stays valid because it records the trigger and the matching
non-default recovery rather than flattening silently.

The fixture validates against
`schemas/interaction/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.schema.json`
and shares its seeded record set with the checked support export at
`artifacts/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont/support_export.json`.

Regenerate with
`cargo run -p aureline-shell --example dump_history_continuity_recovery fixture`.
