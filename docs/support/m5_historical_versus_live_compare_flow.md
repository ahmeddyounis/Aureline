# M5 Historical-vs-Live Compare Flows: One Vocabulary Across Surfaces

This lane lets a user inspect **how a preserved snapshot differs from the current live object** without
collapsing the two into one ambiguous view. It is the B149 historical-vs-live compare-flow lane over the five
non-live-evidence object classes frozen in the
[historical-reference matrix](./m5-historical-evidence-ops.md) and made machine-readable by the
historical-snapshot-descriptor implement lane. Where the
[archived-snapshot viewer lane](./m5_archived_snapshot_viewer_consumers.md) proves how a single preserved
snapshot is *shown* as non-live, this lane proves how a preserved snapshot is *compared against its live
target*.

- **Module:** `crates/aureline-ui/src/m5_historical_versus_live_compare_flow/`
- **Schema:** [`schemas/program/m5-historical-versus-live-compare-flow.schema.json`](../../schemas/program/m5-historical-versus-live-compare-flow.schema.json)
- **Support export:** `artifacts/support/m5-historical-versus-live-compare/support_export.json` (+ `matrix.csv`, `summary.md`)
- **Fixtures:** `fixtures/recovery/m5-historical-versus-live-compare/`
- **Emitter:** `cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- <subcommand>`

## What it proves

Every compare surface — a support bundle viewer, a retirement snapshot page, a review / incident evidence
reopen flow, and the shell, help / docs, runbook-archive, release-center, companion / export,
program-governance, and CLI / export consumers among them — pairs a preserved snapshot with its current live
object at **one canonical compare vocabulary**: the same historical-side grammar (historical-role word,
snapshot label, capture time, provenance, mutation-blocked posture) plus explicit identity, freshness, and
drift labels.

Three honesty axes mirror the batch acceptance criteria.

1. **Identity / freshness / drift, always labeled.** A seeded historical snapshot compares against a current
   live object with an explicit identity-match state (`same_object_identity`, `approximate_identity`, or
   `identity_unverifiable`), a freshness / drift state (`in_sync_no_drift`, `snapshot_behind_live`,
   `snapshot_diverged_from_live`, or `freshness_unverifiable`), and a never-empty drift summary. The
   historical-role word is a token from the frozen historical-reference role vocabulary, so no surface rewrites
   the grammar. A surface may narrow the live comparison but never reword the historical grammar.
2. **No dead end, no silent failure.** A missing or mismatched live target never dead-ends: the user can still
   inspect the historical packet and read an explicit mismatch note naming *why* the live comparison narrowed
   or failed — `missing_live_target`, `changed_scope`, `changed_branch_or_worktree`, `retired_capability`, or
   `unsupported_skew`.
3. **Never implies apply / sync is safe.** The compare action set is a closed enum with **no apply / sync /
   restore variant** — only `inspect_historical`, `export_comparison`, and (where the live target exists and
   is validated) `open_current_live_object`. The historical side stays mutation blocked while navigation to a
   validated live object and export of the comparison packet remain available. Only an explicit, reviewed
   mutation handoff may name a separate path that takes over any actual mutation.

## Compare outcomes

| Outcome | Identity | Freshness | Open-current-live-object? | Disclosure |
| --- | --- | --- | --- | --- |
| `live_target_paired` | `same_object_identity` | verifiable | yes (validated) | full pairing, no narrowing |
| `approximate_pairing` | `approximate_identity` | verifiable | yes (validated) | approximate-pairing detail note |
| `live_target_missing` | `identity_unverifiable` | `freshness_unverifiable` | no | inspect-historical-packet-only note |
| `policy_blocked_pairing` | `identity_unverifiable` | `freshness_unverifiable` | no | inspect-historical-packet-only note |

A narrowed outcome names exactly one mismatch reason from its allowed set (approximate pairings name a
changed scope, changed branch / worktree, or unsupported skew; missing / policy-blocked pairings name a
missing target or retired capability), so the narrowing is always disclosed rather than failing silently.

## Regenerating the checked-in artifacts

```text
cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- support-export
cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- csv
cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- report
cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- fixture-missing-target-narrowed
cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- fixture-policy-blocked-narrowed
cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- validate
```

The `seed.rs` builders are the only mint-from-truth path; the checked-in JSON must byte-match their output.
