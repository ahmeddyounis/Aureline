# Stable shiproom dashboard — packet freshness, fitness functions, qualification stop rules

This document is the reviewer-facing companion for the gated shiproom dashboard:

- [`/artifacts/release/shiproom_dashboard.json`](../../artifacts/release/shiproom_dashboard.json)
- schema: [`/schemas/release/shiproom_dashboard.schema.json`](../../schemas/release/shiproom_dashboard.schema.json)
- proof packet:
  [`/artifacts/release/m4/shiproom_dashboard_proof_packet.md`](../../artifacts/release/m4/shiproom_dashboard_proof_packet.md)

The dashboard is the **canonical truth** for what each shiproom panel renders — claim
truth, qualification, public proof, and maintenance — and whether each panel is **green**
for the release line. The other stable launch-control artifacts answer adjacent questions:
the [stable claim manifest](./stable_claim_manifest.md) decides the single canonical label
each *subject* publishes; the
[stable qualification matrix](./stable_qualification_matrix.md) decides whether each
*qualification row* holds its claimed level; the [stable proof index](./stable_proof_index.md)
decides whether each launch-blocking *requirement* is proven; and the
[maintenance-control packet](./maintenance_control_packet.md) decides whether each
post-release maintenance *lane* is governed. This dashboard is the **consuming layer** over
all of them: for each panel, **is the panel green — backed by a fresh freshness packet, by
fitness functions that all clear their thresholds, by the qualification rows it watches
still holding the cutline, and by an owner sign-off?** Downstream dashboards, docs,
Help/About surfaces, release packets, and support exports MUST ingest this dashboard by
`panel_id` and render its `displayed_label`, `panel_state`, and fitness/freshness posture
rather than minting their own per-panel status wording.

## Panels, sources, freshness packets, fitness functions, claims — one row each

Each `panel` is one `(panel, public claim)` binding. It names:

- the shiproom **panel** it renders — `panel_kind` (`claim_truth`, `qualification`,
  `public_proof`, or `maintenance`), `panel_ref`, `panel_summary`, and whether it is
  `release_blocking`;
- the upstream **source** it ingests — `source_ref` (a canonical claim manifest,
  qualification matrix, proof index, or maintenance packet);
- the **qualification rows** it watches — `qualification_row_refs` (ids into the stable
  qualification matrix);
- the **freshness packet** that proves it is current — `freshness_packet` (id, packet ref,
  the stable-proof-index registration ref, captured-at date, freshness SLO, SLO state, and
  evidence refs);
- the **fitness functions** it must clear — `fitness_functions` (see below);
- the **waiver** (if any) that holds it provisionally — `waiver`;
- the public **claim** it backs — `claim_ref` (a stable-claim-manifest entry) and
  `claim_label`, the canonical lifecycle label that entry publishes.

## The claim ceiling — no per-panel widening

`claim_label` is a **hard ceiling**: a panel may render the public claim at its label or
narrow below it, but its `displayed_label` may never be **wider** (stronger) than the public
claim's canonical label. This is what makes the dashboard *ingest* the claim manifest rather
than restate it — the CI gate reads the stable claim manifest named by `claim_manifest_ref`
and fails when a panel's `claim_label` is not the label the claim manifest publishes for the
entry named by `claim_ref`. The dashboard reuses the stable claim level vocabulary — `lts`,
`stable`, `beta`, `preview`, `withdrawn` — rather than minting per-panel labels.

## The launch cutline

The cutline fixes the boundary between a panel that renders green for a Stable (or LTS)
claim and one narrowed below it:

```
lts > stable   |   beta > preview > withdrawn   (below the cutline)
              cutline
```

A panel renders green only when **all** of the following hold: its freshness packet is
within its SLO, every fitness function clears its threshold, every watched qualification row
holds the cutline, any waiver it relies on is unexpired, an owner has signed off, and the
public claim it backs is itself at or above the cutline. A panel that loses any of those is
structurally required to drop its `displayed_label` **below** the cutline (`beta`,
`preview`, `withdrawn`); it never inherits an adjacent green panel's label.

## Panel states

| `panel_state` | Meaning | Renders the claim's label? |
|---|---|---|
| `green` | Fresh packet, fitness all clear, qualification rows hold, owner-signed | yes |
| `green_on_waiver` | Renders the label only via an active, unexpired waiver | yes |
| `narrowed_unbacked` | Panel capability/evidence incomplete, or owner sign-off absent | no — narrows |
| `narrowed_regressed` | A fitness function failed/unmeasured or a watched qualification row regressed | no — narrows |
| `narrowed_claim_narrowed` | The backing public claim is itself below the cutline | no — inherits ceiling |
| `narrowed_stale` | The freshness packet breached its SLO or is missing | no — narrows |
| `narrowed_waiver_expired` | The waiver the panel relied on expired | no — narrows |

## Fitness functions

A **fitness function** is a measurable architectural-fitness check the panel must clear.
Each carries a `metric`, a `unit`, a `comparator` (`at_least`, `at_most`, or `equals`), a
`threshold`, an optional `warn_threshold` comfort band, a `measured` value (or `null` when
no measurement has been captured), and a `status`. The status is computed, not asserted:

- `unmeasured` when `measured` is `null`;
- `fail` when the measurement does not satisfy the comparator against the threshold;
- `warn` when it satisfies the threshold but not the warn-band comfort boundary;
- `pass` otherwise.

The warn band must be consistent with the comparator: an `at_least` warn boundary may not
sit below the threshold, an `at_most` warn boundary may not sit above it, and an `equals`
function carries no warn boundary. A panel may render green only when **every** fitness
function it carries is measured and clears its threshold (`pass` or `warn`); a `fail`
fitness function makes the panel name `fitness_function_failing` and an `unmeasured` one
makes it name `panel_evidence_incomplete`, narrowing the panel before the dashboard shows
green. The CI gate recomputes each status and fails on any drift.

## Packet-freshness SLO

Each panel's `freshness_packet` carries a `freshness_slo` — a `target_max_age_days`, a
`warn_within_days` threshold, and an `slo_register_ref` — plus a recorded `slo_state`
(`current`, `due_for_refresh`, `breached`, or `missing`). The CI gate recomputes the state
from the packet's `captured_at` against the dashboard `as_of` date and fails when a declared
state is **fresher** than the clock allows, or when a `green`/`green_on_waiver` panel rides a
packet that is `breached` or `missing`. A panel whose freshness packet ages past its SLO
narrows automatically before promotion — the dashboard cannot render green on a stale packet.

## Waiver expiry

A panel in `green_on_waiver` whose `waiver.expires_at` has passed against `as_of` is
rejected; a panel in `narrowed_waiver_expired` whose waiver is still active is also rejected.
A waiver narrows the panel the moment it lapses.

## Qualification-row stop rules

Each panel watches zero or more `qualification_row_refs`. The CI gate reads the stable
qualification matrix named by `qualification_matrix_ref` and fails when a watched row is not
in the matrix, when a watched row's effective level has regressed **below** the cutline but
the panel does not name `qualification_row_regressed` and narrow, or when a panel names
`qualification_row_regressed` while every watched row still holds the cutline. This is the
qualification-row stop rule: a panel cannot keep rendering Stable once a qualification row it
depends on falls below the cutline.

## Coverage

The dashboard must cover all four panel kinds (`claim_truth`, `qualification`,
`public_proof`, `maintenance`); every declared `release_blocking_panel_refs` entry must have
exactly one covering release-blocking row; every release-blocking row must be declared; and
no `panel_ref` may repeat. A shiproom lane cannot quietly drop out of the dashboard.

## Publication gate

`publication` records the proceed/hold verdict for the `shiproom_dashboard_publication`
gate. Each `stop_rule` names a closed stop reason it watches, the labels it applies to
(`lts`, `stable`), a default action, and whether it `blocks_promotion`. The verdict is
`hold` when any blocking rule fires — that is, when a panel whose public claim is still at or
above the cutline carries the rule's trigger reason.
`ci/check_shiproom_dashboard.py --require-proceed` exits non-zero on `hold`, so shiproom and
release tooling can fail promotion directly from this artifact. A panel whose backing claim
is already narrowed below the cutline inherits that ceiling and does **not** hold promotion
on its own — that narrowing is owned upstream by the stable claim manifest.

## Why this is not a spreadsheet

The dashboard is metadata-only: typed states, integer measurements, and opaque refs, never
raw artifacts, logs, signatures, or credentials. The typed Rust consumer
(`aureline_release::shiproom_dashboard`) and the CI gate read the *same* JSON, so the model
and the gate agree without a cargo build in CI. Any surface that needs to show shiproom
posture renders `ShiproomDashboard::support_export_projection` rather than re-deriving status
— there is exactly one place the truth lives.
