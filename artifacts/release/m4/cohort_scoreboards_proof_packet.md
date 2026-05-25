# Cohort scoreboards — proof packet

Reviewer-facing proof packet for the signoff-loop layer that finalizes the
design-partner, certified-archetype, and stable-cohort scoreboards as one
canonical packet, binds every scoreboard row to a public claim ceiling and a
proof packet, and narrows any row whose packet is stale, whose metric fails,
whose waiver expired, or whose required signoff loop is incomplete before the row
can widen release, docs, Help/About, or support-export language.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Packet: [`/artifacts/release/cohort_scoreboards.json`](../cohort_scoreboards.json)
- Schema: [`/schemas/release/cohort_scoreboards.schema.json`](../../../schemas/release/cohort_scoreboards.schema.json)
- Companion doc: [`/docs/release/cohort_scoreboards.md`](../../../docs/release/cohort_scoreboards.md)
- Validator: `ci/check_cohort_scoreboards.py`
- Validation capture:
  [`/artifacts/release/captures/cohort_scoreboards_validation_capture.json`](../captures/cohort_scoreboards_validation_capture.json)
- Typed consumer:
  `aureline_release::finalize_design_partner_certified_archetype_and_stable_cohort`

This packet is registered under the stable proof index through the
`stable_proof_index_ref` it carries and the `proof_index_ref` each row's packet
carries (`artifacts/release/stable_proof_index.json#proof:*`), so a launch
reviewer reaches the cohort-scoreboard signoff loops from the same proof index
that grounds the launch-blocking requirements, rather than from a side
spreadsheet.

## What this packet proves

1. **Each scoreboard row binds a public claim to a scoreboard packet, metrics, and
   a required signoff loop.** Every row names its lane (`design_partner`,
   `certified_archetype`, `stable_cohort`), the stable claim manifest entry it
   backs (`claim_ref`, `claim_label`), its `scoreboard_packet` and freshness SLO,
   the measurable `metrics` it must clear, and the human `signoff_loop` whose
   required roles must all sign. The packet reuses the stable claim level
   vocabulary rather than minting per-scoreboard labels, so docs, Help/About,
   shiproom, the release center, and support exports render one label per row.

2. **A row signs off only when every gate is clean.** A row may render at or above
   the cutline (`signed_off` or `signed_off_on_waiver`) only when it carries a
   captured within-freshness-SLO packet, every metric clears its threshold, the
   row owner has signed, every required signoff in the loop is complete, any
   waiver it relies on is unexpired, and the public claim it backs is itself at or
   above the cutline. The typed model and the CI gate both enforce this.

3. **The packet ingests the stable claim manifest as a hard ceiling.** The CI gate
   reads the stable claim manifest named by `claim_manifest_ref` and fails when a
   row's `claim_label` is not the label that manifest publishes for the entry named
   by `claim_ref`, when a row names an entry the manifest does not carry, or when a
   row renders wider than the public claim's canonical label. A row's effective
   label can never outrun the public claim it backs.

4. **The packet-freshness, waiver-expiry, metric, and signoff stop rules narrow
   rows before promotion.** Each packet carries a freshness SLO and a recorded
   `slo_state`. The CI gate recomputes the freshness state and the waiver-expiry
   state against the packet `as_of` date, failing when a declared state overstates
   the clock, when a signed-off row rides a stale packet or an expired waiver, when
   a metric falls below threshold, or when a required signoff loop is incomplete
   under a Stable claim.

5. **The three lanes and the release-blocking set stay covered.** The gate fails if
   any of `design_partner`, `certified_archetype`, or `stable_cohort` has no row,
   if a declared release-blocking scoreboard has no covering row, if a
   release-blocking row is not declared, or if a `scoreboard_id` repeats.

6. **The publication verdict is recomputed, not asserted.** The gate recomputes the
   `hold`/`proceed` decision and the blocking rule/scoreboard sets from the firing
   stop rules and fails on any drift. With `--require-proceed` it exits non-zero on
   `hold`, so shiproom and release tooling fail promotion directly from this
   artifact.

## Current snapshot (as of 2026-05-24)

The checked-in packet holds promotion. Of eight scoreboard rows across three
public claims, four sign off and back Stable claims cleanly (the managed-pilot
design-partner row, the Rust workspace self-host and TypeScript web-app certified
archetypes — the latter on an active waiver — and the stable-cohort admission and
health row). Four rows are narrowed below the cutline:

- the **design-partner alpha-evidence** row narrowed to beta because its packet
  breached its freshness SLO;
- the **legacy remote-SSH** certified archetype narrowed to beta because its
  certified-checks metric measured 84 of the 100 required;
- the **extension-author** cohort row narrowed to beta because the waiver it
  relied on expired; and
- the **export-and-offboarding** cohort row inherits the ceiling from a public
  claim already published beta.

Three of those — the breached packet, the failing metric, and the expired waiver —
back claims still published Stable, so they fire three blocking stop rules and
hold the `release.shiproom.cohort_scoreboards` gate. The packet narrows the
optimistic Stable rows automatically instead of letting them ride; promotion
clears once the breached packet is refreshed, the legacy archetype's checks pass,
and the extension-author waiver is renewed (or those public claims are formally
narrowed).

## How to re-verify

```
python3 ci/check_cohort_scoreboards.py --repo-root . --check
cargo test -p aureline-release
```

The first command revalidates the packet, recomputes the freshness/waiver
automations and the metric and signoff checks against `as_of`, runs the negative
drills and fixture cases, and writes the validation capture. The second runs the
typed contract tests that bind the model to the checked-in packet. Add
`--require-proceed` to the gate to turn the recorded `hold` into a non-zero exit
for shiproom use.
