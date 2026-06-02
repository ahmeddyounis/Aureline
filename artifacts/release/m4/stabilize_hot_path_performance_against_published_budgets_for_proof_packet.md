# Hot-path performance budget register — proof packet

Reviewer-facing proof packet for the hot-path performance budget register that, for the M4 stable line, binds startup, restore, quick open, typing, scrolling, search, and Git status to published p50/p95 budgets, benchmark-lab traces, corpus metadata, and waiver hooks, narrows any budget whose measured numbers regress or whose proof packet ages out, and protects the public claim so docs, Help/About, and support exports ingest one label per path.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Register: [`/artifacts/release/stabilize_hot_path_performance_against_published_budgets_for.json`](../stabilize_hot_path_performance_against_published_budgets_for.json)
- Companion doc: [`/docs/m4/stabilize-hot-path-performance-against-published-budgets-for.md`](../../../docs/m4/stabilize-hot-path-performance-against-published-budgets-for.md)
- Validation capture: [`/artifacts/release/captures/stabilize_hot_path_performance_against_published_budgets_for_validation_capture.json`](../captures/stabilize_hot_path_performance_against_published_budgets_for_validation_capture.json)
- Typed consumer: `aureline_release::stabilize_hot_path_performance_against_published_budgets_for`

This register is intended to be wired into the stable proof index through the `proof_index_ref` each row's proof packet carries (`artifacts/release/stable_proof_index.json#proof:*`), so a launch reviewer reaches the startup, restore, quick-open, typing, scrolling, search, and Git-status evidence from the same proof index that grounds the launch-blocking requirements.

## What this register proves

1. **Each hot path binds a public claim to a benchmark budget and proof packet.** Every row names its hot path kind (`startup`, `restore`, `quick_open`, `typing`, `scrolling`, `search`, `git_status`), the stable claim manifest entry it backs (`claim_ref`, `claim_label`), its `proof_packet` and freshness SLO, and the `hot_path_budget` that protects the published p50/p95 numbers with corpus metadata and a benchmark-lab trace. The register reuses the stable claim level vocabulary rather than minting per-path labels, so docs, Help/About, shiproom, the release center, and support exports render one label per path.

2. **A budget meets its published p50/p95 only when every gate is clean.** A budget may render at or above the cutline (`meets_budget` or `on_waiver`) only when it carries a captured within-freshness-SLO proof packet, the measured p50/p95 is within the published budget (or an active waiver covers an intentionally tightened threshold), the corpus metadata and benchmark-lab trace are present, the path owner has signed, and the public claim it backs is itself at or above the cutline. The typed model and the CI gate both enforce this.

3. **The register ingests the stable claim manifest as a hard ceiling.** The CI gate reads the stable claim manifest named by `claim_manifest_ref` and fails when a row's `claim_label` is not the label that manifest publishes for the entry named by `claim_ref`, when a row names an entry the manifest does not carry, or when a row renders wider than the public claim's canonical label. A path's effective verdict can never outrun the public claim it backs.

4. **The budget-regression, packet-freshness, and waiver stop rules narrow budgets before promotion.** Each packet carries a freshness SLO and a recorded `slo_state`. The CI gate recomputes the freshness state and the waiver-expiry state against the register `as_of` date, failing when a declared state overstates the clock, when a backed row rides a stale packet or an expired waiver, when measured p50/p95 exceeds the published budget without a tightened-budget waiver, or when an owner sign-off is missing under a Stable claim.

5. **The seven hot path kinds and the release-blocking set stay covered.** The gate fails if any of `startup`, `restore`, `quick_open`, `typing`, `scrolling`, `search`, or `git_status` has no row, if a declared release-blocking surface has no covering row, if a release-blocking row is not declared, or if a `surface_ref` repeats.

6. **The promotion verdict is recomputed, not asserted.** The gate recomputes the `hold`/`proceed` decision and the blocking rule/entry sets from the firing stop rules and fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so shiproom and release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-06-02)

The checked-in register holds promotion. Of seven hot-path budget rows across five public claims, three meet their budget and back Stable claims cleanly (typing, search, and quick open — the last on an active tightened-budget waiver). Four rows are narrowed below the cutline:

- the **startup** budget narrowed to beta because its measured p95 (2400 ms) regressed beyond the published ceiling (2000 ms);
- the **restore** budget inherits the ceiling from a public claim already published beta;
- the **scrolling** budget narrowed to beta because its proof packet breached its 30-day freshness SLO; and
- the **git_status** budget inherits the ceiling from a public claim already published beta.

Two of those — the startup budget regression and the scrolling stale packet — back claims still published Stable, so they fire two blocking stop rules and hold the `release.shiproom.hot_path_performance_budgets` gate. The register narrows the optimistic Stable paths automatically instead of letting them ride; promotion clears once the startup p95 re-meets its budget and the scrolling trace is re-run (or those public claims are formally narrowed).

## How to re-verify

```
cargo test -p aureline-release
```

This runs the typed contract tests that bind the model to the checked-in register.
