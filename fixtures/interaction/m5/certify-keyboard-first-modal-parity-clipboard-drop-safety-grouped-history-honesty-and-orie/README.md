# M5 keyboard-first interaction-parity certification fixtures

## editor_core_narrows_on_stale_history_proof.json

A narrow-drill fixture for the interaction-parity certification packet. It keeps
the full seeded row set — one certified row per claimed M5 switching / power-user
surface kind (editor core, notebook, data/API, preview, docs, review, runtime,
and a provider-linked companion whose imported proof backs only its imported
claim) — and forces the editor-core row to lose current proof.

The editor-core row (`interaction-cert:editor-core:0001`) claims
`switching_certified` parity, but its grouped-history-continuity proof has aged
outside the freshness window (`proof_currency` is `stale_expired`). Because a
claimed row must auto-narrow when any certified dimension lacks current proof, the
row's effective grade drops to `parity_unverified`, it records the
`stale_verification_proof` downgrade trigger, and it carries a precise narrowed
label naming the stale grouped-history proof. The seeded docs row
(`interaction-cert:docs:0001`) remains narrowed for the same reason on its
orientation-aid dimension, so the fixture demonstrates that two distinct
dimensions — grouped history and orientation aids — both force a claimed row off
its certified lane rather than coasting on adjacent green rows.

The fixture validates against
`schemas/interaction/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.schema.json`
and shares its seeded row set with the checked support export at
`artifacts/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/support_export.json`.

Regenerate with
`cargo run -p aureline-shell --example dump_certify_interaction_parity fixture`.
