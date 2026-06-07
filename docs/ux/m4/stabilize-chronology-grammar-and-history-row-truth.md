# Stabilize Chronology Grammar And History Row Truth

The stable chronology row is owned by `aureline-chronology` and serialized by
`schemas/ux/chronology-history-row.schema.json`.

Every claimed stable attention or history surface should project through
`ChronologyHistoryRow` before rendering, copying, exporting, support bundling,
or browser/mobile companion delivery. The row grammar is:

`actor action object: outcome`

The grammar sentence is generated from structured fields and validated against
the stored sentence. A surface that wants different visible copy can render a
localized label around the row, but it must not replace the row grammar with
free text in exported or supportable history.

Required fields per row:

- stable `item_id` and shared `canonical_event_id`
- actor kind/ref/label
- stable action verb
- object kind/label
- outcome class/label
- one or more provenance badges with export markers
- timezone-aware absolute timestamp plus local time context
- deterministic relative-age hint with freshness and visible-reason labels
- current follow-up state
- exact reopen target
- provider-owned-object flag

Follow-up controls keep the same meaning everywhere:

- `Acknowledge`: clears unread or attention badge only; the durable row remains.
- `Resolve`: closes Aureline's local follow-up state only.
- `Dismiss`: removes the transient surface only; durable history remains.
- `Snooze`: hides local attention until the declared wake time.
- `Mute`: suppresses future similar local attention according to the recorded scope.

These controls do not close, resolve, publish, or otherwise mutate a
provider-owned object. If a future surface needs a provider mutation, it must
offer a separately reviewed provider command and record that command ref in the
transition.

The checked-in fixtures cover activity center, task/test job history, debug
chronology, provider import, AI run history, policy/admin notice, and recovery
timeline rows. The companion and support export projections preserve the same
grammar sentence, provenance markers, absolute timestamp, relative-age hint,
visible reason, follow-up state, and exact reopen target.
