# M5 macro-replay review fixtures

## stale_proof_forces_review.json

A drill fixture for the macro-replay review packet. It keeps the full seeded
record set — including the clean editor-core exact-replay baseline and one record
for each non-exact outcome (run-capable notebook → review, cross-file data/API →
observe-no-mutation downgrade, cross-surface run-capable preview → recipe
promotion, docs with an unmapped step → reject, workspace-wide review → recipe
promotion, unstable-timing runtime → recipe promotion, provider-linked companion
→ review) — and adds one extra drill record.

The drill record (`macro-replay:editor-core:stale-proof:0001`) is a single-file,
single-surface, deterministic editor macro with no run-capable step. On its own it
would qualify for `exact_replay_local_editor_only`. But its review proof has aged
outside the freshness window (`proof_currency` is `stale_expired`), so the
`stale_or_missing_review_proof` trigger fires, the required floor rises to
`review_required_before_apply`, and the record correctly resolves to a review
outcome with a precise `review_reason_label`. The fixture demonstrates that stale
evidence — not just scope or run-capability — forces macro replay through review,
and that the record stays valid because it records the trigger and the matching
non-exact outcome rather than replaying silently.

The fixture validates against
`schemas/interaction/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.schema.json`
and shares its seeded record set with the checked support export at
`artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/support_export.json`.
