# M5 keyboard-continuity matrix fixtures

## editor_core_downgrades_on_stale_verification.json

An auto-downgrade drill fixture for the keyboard-continuity matrix packet. Every
claimed M5 surface — editor, notebook, data/API, preview, docs, review, runtime,
and companion — identifies its mode-strip, sequence-guide, clipboard-route,
drag/drop-verb, undo, grouped-history, reopen/recover, and orientation-aid class,
and carries a reopenable verification proof keyed by a non-display fingerprint
distinct from its id.

The editor-core row claims `switching_certified`, but its verification proof has
aged outside its freshness window (`proof_currency` is `stale_expired`). Because
a claimed surface may not outrun current proof, the row auto-downgrades to an
effective grade of `parity_unverified`, records a `stale_verification_proof`
downgrade trigger, and carries a precise downgraded label rather than a generic
provider error. Every other row keeps current, reopenable proof and the honest
axis states it advertises, so its effective grade equals its claim.

The companion row is provider-linked: its `provider_linked_surface` origin pairs
with an `imported_current` proof currency, which backs the provider row's claim
but never a local one — a provider-backed surface never reads as a locally
verified rerun. The runtime row demonstrates honest degradation without a forced
downgrade: it states `no_undo_honest` undo and a `reopen_unavailable_honest`
reopen path while keeping a `parity_partial` claim that equals its effective
grade.

The fixture validates against
`schemas/interaction/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.schema.json`
and shares the seeded surface set with the checked support export at
`artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/support_export.json`.
