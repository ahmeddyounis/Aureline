# M5 drag/drop transfer-safety fixtures

## stale_proof_forces_disclosure.json

A drill fixture for the transfer-safety packet. It keeps the full seeded record
set — including the clean inline-commit baselines (editor text fragment reorder,
notebook cell reorder) and one record for each non-default resolution (data/API
result-row drop → explicit verb-choice disclosure, review work item →
cross-window continuity, large preview-runtime asset drop → progress / cancel /
summary, artifact import into a notebook → confirmed before mutation, runtime
asset with a destructive default verb → rejected, provider-linked companion
artifact detach → cross-window continuity with imported proof) — and adds one
extra drill record.

The drill record (`transfer:notebook:stale-proof:0001`) is a notebook cell
reorder with a single explicit move verb, same-pane scope, and a trivial payload.
On its own it would qualify for `disclosed_inline_commit`. But its transfer proof
has aged outside the freshness window (`proof_currency` is `stale_expired`), so
the `stale_or_missing_transfer_proof` trigger fires, the required floor rises to
`explicit_verb_choice_disclosed`, and the record correctly resolves to an explicit
verb-choice disclosure with a precise `verb_choice_label`. The fixture
demonstrates that stale evidence — not just a multi-verb, cross-window, large, or
import drop — forces a transfer off the inline-commit lane, and that the record
stays valid because it records the trigger and the matching non-default resolution
rather than committing silently.

The fixture validates against
`schemas/interaction/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.schema.json`
and shares its seeded record set with the checked support export at
`artifacts/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/support_export.json`.
