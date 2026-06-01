# Traffic-Origin and Exposure Chips, Tunnel, Port, and Publish-Target Explainability — Milestone Note

This is the milestone-level note for the traffic-origin and exposure chips
finalization lane that binds traffic-origin chip sets, exposure chip sets,
tunnel explainability records, port explainability records, and
publish-target explainability records to one stable command-authority contract.
The authoritative contract document is
`docs/commands/m4/finalize_traffic_origin_and_exposure_chips.md`.
The canonical checked-in artifact is
`artifacts/commands/m4/finalize_traffic_origin_and_exposure_chips/support_export.json`.
The schema lives at
`schemas/commands/finalize_traffic_origin_and_exposure_chips.schema.json`.
The fixture corpus lives under
`fixtures/commands/m4/finalize_traffic_origin_and_exposure_chips/`.

The implementation lives at
`crates/aureline-commands/src/finalize_traffic_origin_and_exposure_chips/mod.rs`
and is tested by the unit tests in
`crates/aureline-commands/src/finalize_traffic_origin_and_exposure_chips/tests.rs`.

## Lane invariants

Every claimed stable tunnel-open, port-forward, and publish-target action on
this lane must surface:

- A traffic-origin chip (naming the origin class) visible without a debug toggle
  from all claimed stable surfaces.
- An exposure chip (naming the exposure class) visible without a debug toggle
  from all claimed stable surfaces.
- A per-tunnel explainability record with `drift_forces_reapproval: true`,
  disclosed in chip, preview, and support export.
- A per-port explainability record disclosed in chip and support export.
- A per-publish-target explainability record with a non-empty `approval_scope_ref`
  and `spend_posture_token`, disclosed in chip and support export.
- Cross-surface chip parity across all nine required command surfaces.

Any surface that cannot meet the lane invariants is automatically narrowed below
Stable in product copy, docs/help, and release packets rather than inheriting
the adjacent stable row's posture.
