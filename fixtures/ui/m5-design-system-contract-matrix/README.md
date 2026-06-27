# M5 design-system contract matrix drill fixtures

These fixtures are valid, export-safe contract-matrix packets that exercise the
missing-object, stale-proof, and waiver drills the canonical matrix keeps green.
Each one keeps the full governed-object inventory, the controlled-vocabulary set,
the conformance-review, consumer-projection, and release-posture invariants — the
difference is which claimed surface develops a gap, on which object, and whether
it auto-narrows, blocks, or is waived. They are minted from the same seed builder
as the canonical support export by `aureline_design_system_m5_contract_matrix`.

## missing_object.json

The shell-chrome surface additionally requires a `diff-viewer` component contract
the inventory does not publish: a claimed surface that lacks a mapped contract
object. The gap is `unmapped_object`, so the surface status is `uncovered` (red),
the gate is `blocked`, the effective claim is `held`, and the packet-level
release gate blocks Stable promotion. The gap is named, not hidden. Demonstrates
the row a release/public-truth run fails on for an unmapped contract object.

## stale_proof_retest_pending.json

The shell-chrome component-contract object's proof has fallen out of its freshness
SLO. The gap is `stale_proof`, so the surface status is `retest_pending` (yellow)
and the gate is `auto_narrowed` to `beta`. The stale object is named in the
dashboard. Stale proof narrows but never blocks — the surface keeps shipping at
the reduced claim. Demonstrates auto-narrowing on stale design-system proof.

## waived_narrowed.json

The shell-chrome surface's unmapped `diff-viewer` gap is accepted under an active,
disclosed waiver (scope, owner, expiry, and the `preview` claim it accepts). The
waived gap no longer blocks promotion — the surface ships `auto_narrowed` to
`preview` — while its true status stays `uncovered` (red) and the gap is named as
waived. The dashboard names the active waiver. Demonstrates a disclosed waiver
that ships a narrowed claim without hiding the gap.
