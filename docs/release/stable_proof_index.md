# Stable proof index — requirements, packets, waivers, and public claims

This document is the reviewer-facing companion for the gated stable proof index:

- [`/artifacts/release/stable_proof_index.json`](../../artifacts/release/stable_proof_index.json)
- schema: [`/schemas/release/stable_proof_index.schema.json`](../../schemas/release/stable_proof_index.schema.json)
- proof packet:
  [`/artifacts/release/m4/stable_proof_index_proof_packet.md`](../../artifacts/release/m4/stable_proof_index_proof_packet.md)

The index is the **canonical truth** for the link between each launch-blocking
**requirement**, the proof **packet** that proves it, the **waiver** (if any) that
holds it provisionally, and the public **claim** whose lifecycle label that proof
backs. The other stable launch-control artifacts answer adjacent questions — the
[stable claim manifest](./stable_claim_manifest.md) decides the single canonical
label each *subject* publishes, and the
[stable boundary manifest](./stable_boundary_manifest.md) decides what each
*deployment value line* can carry. This index answers the question the launch
shiproom actually asks: **is each launch-blocking requirement proven, and which
public claim does that proof back?** Downstream dashboards, docs, Help/About
surfaces, release packets, and support exports MUST ingest this index by `proof_id`
and render its `proven_label` rather than minting their own per-requirement
maturity wording.

The launch-blocking requirement set is grounded in the PRD launch-blocking
requirement index (PRD Appendix P). Each requirement that gates the v1.0 launch is
declared in `launch_blocking_requirement_refs` and carries exactly one proof row.

## Requirements, packets, waivers, claims — one row each

Each `row` is one `(requirement, public claim)` binding. It names:

- the launch-blocking **requirement** it proves — `requirement_ref`,
  `requirement_class`, `requirement_summary`, and whether it is `launch_blocking`;
- the proof **packet** that proves it — `proof_packet` (id, packet ref, the
  proof-index registration ref, captured-at date, freshness SLO, SLO state, and
  evidence refs);
- the **waiver** (if any) that holds it provisionally — `waiver`;
- the public **claim** it backs — `claim_ref` (a stable-claim-manifest entry) and
  `claim_label`, the canonical lifecycle label that entry publishes.

A single public claim is typically decomposed into several launch-blocking
requirements; the index records one row per requirement, so a requirement-level gap
narrows even when the aggregate public claim is still optimistically published
Stable.

## The claim ceiling — no per-requirement widening

`claim_label` is a **hard ceiling**: a proof row may back the public claim at its
label or narrow below it, but its `proven_label` may never be **wider** (stronger)
than the public claim's canonical label. This is what makes the proof index
*ingest* the claim manifest rather than restate it — the CI gate reads the stable
claim manifest named by `claim_manifest_ref` and fails when a row's `claim_label`
is not the label the claim manifest publishes for the entry named by `claim_ref`.
The index reuses the stable claim level vocabulary — `lts`, `stable`, `beta`,
`preview`, `withdrawn` — rather than minting per-requirement labels.

## The launch cutline

The cutline fixes the boundary between a requirement whose proof backs a Stable (or
LTS) claim and one narrowed below it:

```
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

A requirement backs a label at or above the cutline only when its proof packet is
within its freshness SLO, any waiver it relies on is unexpired, its
requirement-level evidence is complete, an owner has signed off, and the public
claim it backs is itself at or above the cutline. A requirement that loses any of
those drops to a label below the cutline and never backs a label wider than the
public claim's canonical label.

## Proof rows and states

Every row carries a `requirement_ref` + `requirement_class`, a `claim_ref` +
`claim_label` (the ceiling), a `proof_packet` (with its freshness SLO), an optional
`waiver`, an `owner_signoff`, the `index_state` it earned, its
`active_gap_reasons`, and the `proven_label` it backs.

The `index_state` is the verdict for that requirement:

- `proven` — a captured, within-SLO proof packet backs the public claim at its full
  label, owner-signed.
- `proven_on_waiver` — backs the claim's label only because an active, unexpired
  waiver covers a recorded proof gap.
- `unproven_unbacked` — the requirement evidence is missing or incomplete, or owner
  sign-off is absent; the requirement is not proven and the label must narrow.
- `unproven_claim_narrowed` — the public claim it backs is itself below the cutline,
  so the proof inherits that ceiling and narrows.
- `unproven_stale` — the proof packet breached its freshness SLO (or is missing);
  the requirement is not proven and the label must narrow.
- `unproven_waiver_expired` — the requirement relied on a waiver that has expired;
  the label must narrow.

A narrowing row MUST drop below the cutline and name at least one active gap reason.
A proven row MUST back the public claim's canonical label cleanly — within-SLO
captured packet, owner sign-off, **no** active gap reason.

## Packet-freshness SLO

Each row's `proof_packet` carries a `freshness_slo`:

- `target_max_age_days` — the SLO: the packet may be at most this many days old.
- `warn_within_days` — when the days remaining before the target drop to this or
  below, the packet is `due_for_refresh` (a warning, not yet a breach).
- `slo_register_ref` — the ref into this register that defines the target.

The `slo_state` is the freshness verdict: `current`, `due_for_refresh`,
`breached`, or `missing`. The CI gate performs the **packet-freshness SLO
automation** the typed model cannot: against the index `as_of` date it recomputes
each packet's state from `captured_at` and `freshness_slo`, and fails when a
declared state is fresher than the clock allows, or when a proven row rides a packet
whose recomputed state is `breached` or `missing`. A Stable proof cannot quietly
outlive its packet.

## Gap reasons and the publication automation

The closed reason vocabulary (mirrored in the schema and the typed model) is:

- `claim_label_narrowed`
- `requirement_capability_absent`
- `requirement_evidence_incomplete`
- `proof_packet_freshness_breached`
- `proof_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`

Each `proof_rule` names one reason as its `trigger_reason`, the public-claim labels
it watches (`applies_to_labels`), a `default_action`, and whether it
`blocks_publication`. A rule **fires** when any watched row carries its trigger
reason. Every gap reason has a rule watching for it, so a gap reason can never fire
without a corresponding publication gate.

The `claim_label_narrowed` rule is intentionally **non-blocking**: a row narrowed
only because the public claim it backs is already below the cutline is expected
inheritance, not a proof-index defect — the stable claim manifest already holds that
claim upstream. The remaining reasons describe a requirement that *could* back a
Stable claim (its public claim is canonically Stable) but is not proven, so they
block publication.

## Launch-blocking requirement coverage

`launch_blocking_requirement_refs` is the closed set of requirement refs the index
must prove. The CI gate fails closed when a declared launch-blocking requirement has
no covering `launch_blocking: true` row, when a launch-blocking row's
`requirement_ref` is not declared, or when a requirement ref repeats — so a
launch-blocking requirement can never quietly drop out of the index.

## Publication verdict

The `publication` block records the verdict for the stable proof index. It is
`hold` when any blocking proof rule fires and `proceed` otherwise. The
`blocking_rule_ids` and `blocking_proof_ids` enumerate the firing rules and the rows
that triggered them (only rows whose public claim is at or above the cutline count).
The gate recomputes all three and the summary, and fails on any drift.

At this revision two launch-blocking requirements that back public claims still
published Stable are themselves unproven — provider completion quality rides a proof
packet past its freshness SLO, and rollback state integrity lost its provisional
waiver — so two blocking proof rules fire and stable proof-index publication is
held. That is the honest posture for a pre-implementation repository: the aggregate
public claims are optimistic, and the proof index narrows the requirement-level
truth beneath them.

## CI gate

Run:

```sh
python3 ci/check_stable_proof_index.py --repo-root .
```

The gate fails when a closed vocabulary or the cutline drifts; when a row that is
narrowed does not drop below the cutline or fails to name a reason; when a proven
row carries an active gap reason, rides a stale or uncaptured packet, or lacks owner
sign-off; when a row backs a label wider than its public claim's ceiling; when a
row's `claim_label` disagrees with the stable claim manifest; when a launch-blocking
requirement is uncovered or a requirement ref repeats; when a packet's declared SLO
state overstates its freshness against `as_of`; when a row holds on an expired
waiver; when the publication verdict, blocking sets, or summary counts drift; or
when a referenced artifact does not exist. It also runs negative drills and the
checked-in fixture cases under
[`/fixtures/release/stable_proof_index/`](../../fixtures/release/stable_proof_index/),
and writes a validation capture to
[`/artifacts/release/captures/stable_proof_index_validation_capture.json`](../../artifacts/release/captures/stable_proof_index_validation_capture.json).

Shiproom and release tooling can fail publication directly from this artifact:

```sh
python3 ci/check_stable_proof_index.py --repo-root . --require-proceed
```

This exits non-zero (code 2) whenever the recomputed publication verdict is `hold`,
distinct from an invalid-artifact failure (code 1).

The typed Rust consumer
(`aureline_release::stable_proof_index::current_stable_proof_index`) reads the same
index and runs the same structural cross-check, and exposes a redaction-safe
`support_export_projection()` for Help/About and support surfaces, so
`cargo test -p aureline-release` enforces these invariants without a cargo build in
CI.

## Packet-freshness SLO {#packet-freshness-slo}

The freshness SLO register for this index pins one target per proof packet:
`target_max_age_days` of 30 with a `warn_within_days` window of 7. A packet older
than the target is `breached`; a packet whose remaining headroom is within the warn
window is `due_for_refresh`; a packet with no capture is `missing`. The CI gate
recomputes the state from `captured_at` against the index `as_of` date, so a packet
cannot claim a freshness it has not earned.

## Update rules

1. Land the upstream stable claim manifest entry, the requirement-level
   qualification evidence, refreshed proof packets, and waivers first; point each
   row's `claim_ref`, `claim_label`, `proof_packet`, and `waiver` at the canonical
   records.
2. Set each row's `index_state`, `active_gap_reasons`, `slo_state`, and
   `proven_label` to the honest posture. A requirement whose packet breached its
   freshness SLO or is missing, whose waiver expired, whose evidence is incomplete,
   or whose owner has not signed narrows below the cutline; a requirement whose
   public claim is already below the cutline narrows by inheritance.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_stable_proof_index.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower requirement than planned, narrow the proven label
   and update the index — do not paper over the gap with prose.
