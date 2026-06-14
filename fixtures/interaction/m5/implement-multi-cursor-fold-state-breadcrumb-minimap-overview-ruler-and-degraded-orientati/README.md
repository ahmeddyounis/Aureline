# M5 orientation-aid fixtures

## stale_proof_forces_degraded.json

A drill fixture for the orientation-aid packet. It keeps the full seeded record
set — including the clean fully-active baseline (editor-core multi-cursor count)
and one record for each non-default disclosure (notebook fold-state → preserved
count summary, preview breadcrumb → cross-surface identity alignment, data/API
minimap → reduced-detail-disclosed under a constrained viewport, docs
overview-ruler → motion-reduced-disclosed under reduced motion, review-diff
minimap → degraded-disclosed for a large artifact, browser-runtime overlay fold →
unavailable-disclosed under a limited capability profile, provider-linked
companion breadcrumb → imported identity alignment that never reads as local) —
and adds one extra drill record.

The drill record (`orientation:editor-core:stale-proof:0001`) is an editor-core
multi-cursor aid with every caret rendered live, no high cardinality, no shared
identity, and no viewport, motion, artifact, or profile constraint. On its own it
would qualify for a flat `aid_fully_active`. But its orientation proof has aged
outside the freshness window (`proof_currency` is `stale_expired`), so the
`stale_or_missing_orientation_proof` trigger fires, the required floor rises to
`degraded_disclosed`, and the record correctly resolves to a disclosed degraded
state with a precise `degrade_reason_label` (and the matching
`orientation_aids_degraded_honest` posture). The fixture demonstrates that stale
evidence — not just high cardinality, a shared identity, a constrained viewport,
reduced motion, a large artifact, or a limited profile — forces an aid off the
flat fully-active lane and drops its prior markers rather than showing them as if
current, and that the record stays valid because it records the trigger and the
matching disclosure rather than silently removing the aid.

The fixture validates against
`schemas/interaction/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.schema.json`
and shares its seeded record set with the checked support export at
`artifacts/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati/support_export.json`.

Regenerate with
`cargo run -p aureline-shell --example dump_orientation_aids fixture`.
