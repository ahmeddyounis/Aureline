# Save-plan examples

Machine-emitted save-plan records for every scenario in
`crates/aureline-vfs/src/harness.rs::SCENARIOS`. One `<label>.json` file per
scenario, plus an `aggregate.json` that carries the full harness report
(schema version, corpus id, aggregate counters, every scenario inlined).

These are the reviewable counterpart to the human-authored fixtures under
`fixtures/fs/save_truth_cases/`. A reader comparing both sides sees:

- **Fixtures** — the scenario's inputs and expected outcome, in a shape a
  human can diff.
- **Artifacts** — the byte-stable save plan the prototype actually produced,
  covering layers 1–5 of the filesystem-identity model plus the save
  manifest, watcher frames, reviewer notes, and hook-counter snapshot.

## How to regenerate

From the repo root:

```
./tools/vfs_proto.sh --emit-scenarios artifacts/fs/save_plan_examples
```

Flags honour the reproducibility posture used by the other prototype
wrappers:

- `SOURCE_DATE_EPOCH` defaults to the repo's latest commit time.
- `TZ=UTC` and `LC_ALL=C` are pinned so no locale-dependent formatting
  leaks into the output.

Re-running on a different host MUST produce identical bytes. If it doesn't,
the harness has a nondeterminism bug — file an issue and do not merge the
diff.

## Coverage contract

Every scenario in the `SCENARIOS` table MUST emit a file here, and every
file here MUST correspond to a scenario in that table. The harness tests
assert scenario-label uniqueness; the wrapper script writes one file per
label.

See `prototypes/vfs/README.md` for the known holes, carry-forward items,
and how these artifacts feed the later benchmark lab.
