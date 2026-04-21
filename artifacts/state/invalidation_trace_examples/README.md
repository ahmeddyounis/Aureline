# Invalidation trace examples

Machine-emitted invalidation traces for every scenario in
`crates/aureline-reactive-state/src/harness.rs::SCENARIOS`. One
`<label>.json` file per scenario, plus an `aggregate.json` that carries
the full harness report (schema version, corpus id, aggregate counters,
every scenario inlined).

Each per-scenario record answers the ADR 0005 acceptance question: **why
is this projection fresh, stale, partial, or recomputing?** The trace is
an ordered list of subscription-lifecycle events (`subscribe`,
`frame_emit`, and supporting `note` rows) with the exact
`SubscriptionEnvelope` the producer wrote at each step and the
consumer's observation at apply-time (frame class, freshness,
completeness, snapshot_epoch / delta_seq, stale / terminal reason, and
reviewer notes when the lifecycle validator flagged something).

These are the reviewable counterpart to the human-authored single-frame
fixtures under `fixtures/state/envelope_examples/`. A reader comparing
both sides sees:

- **Fixtures** — one representative envelope per scenario, in a shape a
  human can diff and a JSON Schema can validate.
- **Traces** — the byte-stable scenario-by-scenario lifecycle the
  prototype produced, covering every hook fire, every envelope, every
  consumer observation, and the aggregate counter snapshot.

## How to regenerate

From the repo root:

```
./tools/reactive_proto.sh --emit-scenarios artifacts/state/invalidation_trace_examples
```

Flags honour the reproducibility posture used by the other prototype
wrappers:

- `SOURCE_DATE_EPOCH` defaults to the repo's latest commit time.
- `TZ=UTC` and `LC_ALL=C` are pinned so no locale-dependent formatting
  leaks into the output.

Re-running on a different host MUST produce identical bytes. If it
doesn't, the harness has a nondeterminism bug — file an issue and do
not merge the diff.

## Coverage contract

Every scenario in the `SCENARIOS` table MUST emit a file here, and every
file here MUST correspond to a scenario in that table. The harness
tests assert scenario-label uniqueness (`every_scenario_label_is_unique`)
and the wrapper script writes one file per label.

Cross-scenario invariants the harness asserts and every trace records:

- An authoritative producer's fresh snapshot precedes the derived view's
  refresh on the same scope (`authority_precedes_derived_in_refresh_ordering_scenario`).
- A derived producer never claims `freshness = authoritative`
  (`derived_producers_never_claim_authoritative`).
- A replayed frame never advances the live `snapshot_epoch`
  (`replay_frames_never_advance_live_epoch`).

See `prototypes/reactive_state/README.md` for the known holes,
carry-forward items, and how these artifacts feed the later benchmark
lab, support-export lane, and production subscription fabric.
