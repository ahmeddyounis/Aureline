# Run history and evidence panels for M5 automation

This is the reviewer-facing landing page for the **run-history / evidence-panel
object** and its first M5 automation consumers. The live object lives in
[`crates/aureline-runtime/src/run_history/`](../../crates/aureline-runtime/src/run_history/mod.rs);
the cross-tool boundary schema is
[`schemas/automation/run-history.schema.json`](../../schemas/automation/run-history.schema.json);
the checked-in artifacts live under
[`artifacts/m5/automation/run-history/`](../../artifacts/m5/automation/run-history/);
the worked-example fixtures live under
[`fixtures/automation/m5/run-history-evidence/`](../../fixtures/automation/m5/run-history-evidence/);
and the fail-closed gate is
[`tools/ci/m5/run_history_check.py`](../../tools/ci/m5/run_history_check.py).

The frozen run-history boundary contract this lane builds on is
[`docs/automation/run_history_contract.md`](../automation/run_history_contract.md),
whose row, run-record, and safe-summary-export schemas
([`schemas/automation/run_history_row.schema.json`](../../schemas/automation/run_history_row.schema.json),
[`schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json),
and
[`schemas/automation/run_summary_export.schema.json`](../../schemas/automation/run_summary_export.schema.json))
are re-exported here rather than re-invented.

## What the object is

A `RunHistoryEntry` records **one attempted dispatch** as an attributable evidence
row. It carries:

- **Run identity** — `run_id`, `manifest_id`, `manifest_revision_ref`, and an
  optional `manifest_content_address`. All four are opaque, content-addressed, or
  revision references — never raw argv, paths, or secrets.
- **Automation layer** — recorded macro, declarative recipe, managed-only
  template, extension/external automation, or headless-safe run.
- **Schema version** — the integer schema version of the underlying run record.
- **Execution mode** — the surface and dispatch mode (desktop palette /
  keybinding / explicit action, AI assistant, headless CLI explicit / scripted /
  offline replay, queued, managed-only channel, external runner, or imported
  provider event).
- **Result class** — succeeded, partial success, denied at gate, aborted, queued,
  or dry-run only.
- **Artifact links** — opaque, content-addressed references to the run's run log,
  result artifact, evidence bundle, diff, or external artifact.
- **Retention and redaction state** — the retention window (and its expiry when
  windowed), the redaction mode, and the artifact-bundle state (available, not
  produced for a named reason, or purged with only the safe summary remaining).
- **Context** — the execution-context and environment capsule references and the
  trust, policy, and kill-switch observation classes the run recorded.

## Rerun is evidence, not authority

The single most important rule: **history never preserves authority**. The rerun
action a row offers is *derived* from the entry's automation layer, its imported
state, and the `current_policy_blockers` the resolver observed *now* — never from a
cached approval ticket or a preserved environment capsule.

- `resolved_rerun_class` resolves rerun through one of fifteen closed classes: five
  admissible (with no revalidation, or after environment revalidation / fresh
  approval / kill-switch clear / managed-channel resolution) and ten blocked
  (publisher revoked, capability disabled by policy, managed-only template retired,
  recipe revision retired, replay window expired, descriptor revision retired,
  environment capsule drift, macro-recording-only, extension/external runner
  unavailable, imported record).
- `rerun_under_current_policy_admissible_no_revalidation_required` pairs with
  exactly `[no_blocker_present]`; any other class cites at least one
  non-no-blocker entry.
- `resolve_rerun` mints an explicit `RerunResolution` that asserts the rerun
  resolves current policy, reuses no cached approval, reuses no stale environment,
  and re-resolves every secret reference. Yesterday's success is never an
  admissibility argument on its own.

### Forbidden collapses the gate blocks

- An imported provider-event row offering a one-click rerun (it always resolves to
  `rerun_under_current_policy_blocked_imported_record`).
- A recorded macro offering extension/external rerun.
- An open-as-recipe affordance laundering a capability into a recipe its layer does
  not admit.
- A raw secret value in a history row instead of an opaque broker handle.
- An evidence row quoting a rerun action that disagrees with its live entry.

## Exposed to support, incident, AI, and CLI/headless

`to_evidence_row` projects the entry onto a `RunHistoryEvidenceRow` — the canonical
object support packets, incident/runbook follow-up, AI evidence joins, and
CLI/headless inspect surfaces ingest. `export` nests the entry, its evidence row,
and a fresh rerun resolution into a round-trippable
`run_history_evidence_export_record` so a run stays comparable and explainable
after the panel closes. The first-consumers packet's support export carries one
redacted consumer row per entrypoint plus every evidence row, and the CLI/headless
view prints one line per entrypoint.

## First consumers

The `m5_run_history_first_consumers_packet` binds all six first-consumer
entrypoints to a seeded panel, exercising every automation layer and a
representative set of rerun states:

| Entrypoint        | Layer(s)                                   | Demonstrates                                              |
|-------------------|--------------------------------------------|-----------------------------------------------------------|
| Notebook          | declarative recipe + recorded macro        | comparison across earlier runs; the macro-promotion path  |
| Task/test/debug   | headless-safe run                          | a clean rerun under current policy                        |
| Request/API       | declarative recipe                         | rerun admissible only after a fresh approval              |
| Package           | managed-only template                      | rerun admissible only after the managed channel resolves  |
| Incident          | extension/external (imported + external)   | imported rows offer no rerun; external runner unavailable |
| AI assistant      | declarative recipe                         | rerun admissible only after the kill switch clears        |

`RunHistoryFirstConsumersPacket::validate` enforces the freeze mechanically; a
dropped entrypoint, a rerun implying cached approval, an imported row offering
rerun, a macro offering external rerun, a laundered capability, a raw secret, an
inconsistent evidence-row projection, or a violated invariant **blocks stable**.

## How to regenerate and verify

```sh
# Regenerate the checked-in artifacts and worked-example fixtures from the seed.
cargo run -q -p aureline-runtime --example dump_m5_run_history

# Inspect one projection.
cargo run -q -p aureline-runtime --example dump_m5_run_history -- packet
cargo run -q -p aureline-runtime --example dump_m5_run_history -- support-export
cargo run -q -p aureline-runtime --example dump_m5_run_history -- cli-headless
cargo run -q -p aureline-runtime --example dump_m5_run_history -- compact

# The typed contract tests (the artifacts are bit-for-bit derivable from the seed).
cargo test -p aureline-runtime --test m5_run_history
cargo test -p aureline-runtime run_history

# The fail-closed CI gate.
python3 tools/ci/m5/run_history_check.py --repo-root .
```

Adding a new enum value to any frozen vocabulary is additive-minor and bumps the
relevant `_schema_version` const; repurposing an existing value is breaking and
requires a new decision row.
