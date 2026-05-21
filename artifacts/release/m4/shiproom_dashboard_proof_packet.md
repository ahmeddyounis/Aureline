# Shiproom dashboard — proof packet

Reviewer-facing proof packet for the gated stable shiproom dashboard that wires each
shiproom panel to its upstream source, a packet-freshness SLO, measurable fitness
functions, and the qualification-row stop rules that hold promotion.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Dashboard: [`/artifacts/release/shiproom_dashboard.json`](../shiproom_dashboard.json)
- Schema: [`/schemas/release/shiproom_dashboard.schema.json`](../../../schemas/release/shiproom_dashboard.schema.json)
- Companion doc: [`/docs/release/shiproom_dashboard.md`](../../../docs/release/shiproom_dashboard.md)
- Validator: `ci/check_shiproom_dashboard.py`
- Validation capture:
  [`/artifacts/release/captures/shiproom_dashboard_validation_capture.json`](../captures/shiproom_dashboard_validation_capture.json)
- Typed consumer: `aureline_release::shiproom_dashboard`

## What this dashboard proves

1. **Each shiproom panel binds a source, a freshness packet, and fitness functions to a
   public claim.** Every panel binds one panel (`panel_kind`, `panel_ref`) to the upstream
   source it ingests (`source_ref`), the qualification rows it watches
   (`qualification_row_refs`), the freshness packet that proves it is current
   (`freshness_packet`), the fitness functions it must clear (`fitness_functions`), the
   waiver that holds it provisionally (`waiver`), and the public claim whose lifecycle label
   it backs (`claim_ref`, `claim_label`). The dashboard reuses the stable claim level
   vocabulary rather than minting per-panel labels, so docs, Help/About, the release center,
   and support exports render one label per panel.

2. **The dashboard ingests the stable claim manifest as a hard ceiling.** The CI gate reads
   the stable claim manifest named by `claim_manifest_ref` and fails when a panel's
   `claim_label` is not the label that manifest publishes for the entry named by
   `claim_ref`, when a panel names an entry the manifest does not carry, or when a panel is
   rendered wider than the public claim's canonical label. A panel's displayed label can
   never outrun the public claim it backs.

3. **The fitness functions are measured, not asserted.** Each fitness function carries a
   comparator, a threshold, an optional warn band, and a measured value; the CI gate
   recomputes the pass/warn/fail/unmeasured status and fails on any drift. A panel may render
   green only when every fitness function it carries is measured and clears its threshold; a
   failing or unmeasured function narrows the panel before the dashboard shows green.

4. **The packet-freshness, waiver-expiry, and qualification-row stop rules narrow panels
   before promotion.** Each panel's freshness packet carries a freshness SLO and a recorded
   `slo_state`; each watched qualification row resolves against the stable qualification
   matrix. The CI gate recomputes the freshness state and the waiver-expiry state against the
   dashboard `as_of` date, and reads the qualification matrix named by
   `qualification_matrix_ref`, failing when a declared state overstates the clock, when a
   green panel rides a stale packet or an expired waiver, or when a watched qualification row
   has regressed below the cutline but the panel keeps rendering Stable.

5. **The four panel kinds and the release-blocking panel set stay covered.** The gate fails
   if any of `claim_truth`, `qualification`, `public_proof`, or `maintenance` has no panel,
   if a declared release-blocking panel has no covering row, if a release-blocking row is not
   declared, or if a `panel_ref` repeats.

6. **The publication verdict is recomputed, not asserted.** The gate recomputes the
   `hold`/`proceed` decision and the blocking rule/panel sets from the firing stop rules and
   fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so shiproom and
   release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-05-21)

The checked-in dashboard holds promotion. Of eleven panels across five public claims, three
panels render green and back Stable claims cleanly (the provider routing claim-truth panel,
the desktop/accessibility qualification panel, and the provider public-proof panel — the
last on an active waiver). Eight panels are narrowed below the cutline:

- the **repair/rollback maintenance freshness** panel narrowed to beta because its freshness
  packet breached its SLO;
- the **state/schema qualification** panel narrowed to beta because its watched qualification
  row regressed below the cutline;
- the **provider completion-quality** panel narrowed to beta because its fitness function
  measured below its threshold;
- the **remote-helper skew-window qualification** panel narrowed to beta because its
  provisional waiver expired;
- the **export/offboarding maintenance**, **localization public-proof**, and
  **regulated qualification** panels inherit ceilings from public claims already narrowed
  upstream (beta, preview, beta); and
- the advisory (non-release-blocking) **regulated drill-freshness** panel is unbacked for an
  unmeasured fitness function and a missing owner sign-off under a beta claim.

Four of those — the breached freshness packet, the regressed qualification row, the failing
fitness function, and the expired waiver — back claims still published Stable, so they fire
four blocking stop rules and hold the `shiproom_dashboard_publication` gate. The dashboard
narrows the optimistic Stable shiproom panels automatically instead of letting them ride;
promotion clears once the freshness packet is refreshed, the qualification row is
re-qualified, the fitness metric is remediated, and the waiver is renewed (or those public
claims are formally narrowed).

## How to re-verify

```
python3 ci/check_shiproom_dashboard.py --repo-root . --check
cargo test -p aureline-release
```

The first command revalidates the dashboard, recomputes the freshness/waiver automations and
the qualification-row stop rules against `as_of`, runs the negative drills and fixture cases,
and writes the validation capture. The second runs the typed contract tests that bind the
model to the checked-in dashboard, the frozen capture, and the negative fixtures. Add
`--require-proceed` to the gate to turn the recorded `hold` into a non-zero exit for shiproom
use.
