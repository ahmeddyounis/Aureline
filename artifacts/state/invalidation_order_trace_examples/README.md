# Invalidation-order trace examples

Condensed order audits extracted from the reactive-state prototype's
scenario table. These are the short, reviewer-facing counterparts to the
full lifecycle traces under `artifacts/state/invalidation_trace_examples/`.

Each JSON file answers a narrow question:

- what had to happen first;
- what stale or resync notice had to be made explicit before a new
  snapshot was trusted; and
- what drift pattern would violate the contract.

## How to regenerate

From the repo root:

```sh
./tools/reactive_proto.sh --emit-order-audits artifacts/state/invalidation_order_trace_examples
```

The wrapper shares the same reproducibility posture as the full trace
emitter: pinned `SOURCE_DATE_EPOCH`, `TZ=UTC`, and `LC_ALL=C`.

## Coverage contract

- `authority_before_derived_refresh.json`
  — authoritative refresh must precede derived stale and refresh frames.
- `delta_gap_requires_resync.json`
  — detected delta gaps must surface a resync before replacement
  snapshots land.
- `snapshot_required_switch.json`
  — coalesced delivery that escalates to `snapshot_required` must record
  the switch and resync before trusting the replacement snapshot.

If a later scenario adds a new required ordering invariant, this
directory and the emitter must update in the same change.
