# Stable claim manifest, canonical lifecycle labels, and packet-freshness SLO automation

This document is the reviewer-facing companion for the gated stable claim
manifest:

- [`/artifacts/release/stable_claim_manifest.json`](../../artifacts/release/stable_claim_manifest.json)
- schema: [`/schemas/release/stable_claim_manifest.schema.json`](../../schemas/release/stable_claim_manifest.schema.json)
- proof packet:
  [`/artifacts/release/m4/stable_claim_manifest_proof_packet.md`](../../artifacts/release/m4/stable_claim_manifest_proof_packet.md)

The manifest is the **canonical truth** for the lifecycle label each subject
publishes on the stable line. It is the publication layer that binds the three
upstream M4 records together:

- the stable claim matrix
  ([`/docs/release/stable_claim_matrix.md`](./stable_claim_matrix.md)) — which
  subjects may publish as *Stable*;
- the stable qualification matrix
  ([`/docs/release/stable_qualification_matrix.md`](./stable_qualification_matrix.md))
  — the per-lane qualification rows that ground those claims; and
- the v1.0 support-class ledger
  ([`/docs/release/support_class_ledger.md`](./support_class_ledger.md)) — the
  support class each subject publishes.

For each published subject the manifest assigns **exactly one canonical lifecycle
label**, names the backing claim row, qualification rows, and support-class entry
that label depends on, and attaches a **packet-freshness SLO** to its proof
packet. Downstream dashboards, docs, Help/About surfaces, release packets, and
support exports MUST ingest this manifest by `entry_id` and render its
`published_label` rather than minting their own maturity wording.

## No re-minted lifecycle labels

The manifest does not invent a new label vocabulary. The lifecycle label is the
stable claim matrix's level vocabulary — `lts`, `stable`, `beta`, `preview`,
`withdrawn` — reused verbatim (`lifecycle_labels`). The point of this artifact is
that the label is decided **once**, here, so two surfaces can never disagree about
whether a subject is Stable. An entry's `published_label` is the canonical label
after narrowing; it may never be **wider** (stronger) than its `claimed_label`.

## The launch cutline

The cutline fixes the boundary between a published Stable (or LTS) label and a
label narrowed below it:

```
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

A subject publishes a label at or above the cutline only when its backing stable
claim still holds, every backing qualification lane holds, its support class
still holds, its proof packet is within its freshness SLO, and an owner has
signed off. A subject that loses any of those drops to a label below the cutline
and never inherits an adjacent subject's published label.

## Manifest entries and states

Every entry carries a `claimed_label`, a `backing_claim_ref` into the stable
claim matrix, `qualification_row_refs` into the qualification matrix, a
`support_class_ref` into the support-class ledger, a `proof_packet` (with its
freshness SLO), an optional `waiver`, an `owner_signoff`, the `manifest_state` it
earned, its `active_narrowing_reasons`, and the `published_label` it publishes.

The `manifest_state` is the verdict for that entry:

- `published` — full, current, owner-signed backing; publishes the claimed label.
- `provisional_on_waiver` — publishes the claimed label only because an active,
  unexpired waiver covers a recorded gap.
- `narrowed_unqualified` — a backing claim, qualification lane, or support class
  is missing or narrowed; the label must narrow.
- `narrowed_stale` — the proof packet breached its freshness SLO (or is missing);
  the label must narrow.
- `narrowed_waiver_expired` — the entry relied on a waiver that has expired; the
  label must narrow.

A narrowing entry MUST drop below the cutline and name at least one active
narrowing reason. A published entry MUST publish its claim cleanly — within-SLO
captured packet, owner sign-off, **no** active narrowing reason.

## Packet-freshness SLO

Each entry's `proof_packet` carries a `freshness_slo`:

- `target_max_age_days` — the SLO: the packet may be at most this many days old.
- `warn_within_days` — when the days remaining before the target drop to this or
  below, the packet is `due_for_refresh` (a warning, not yet a breach).
- `slo_register_ref` — the ref into this register that defines the target.

The `slo_state` is the freshness verdict:

- `current` — captured well within the SLO.
- `due_for_refresh` — within the SLO but inside the warn window; still
  claim-bearing, refresh is due soon.
- `breached` — age exceeds the target; the packet is stale and the label must
  narrow.
- `missing` — no packet captured.

The CI gate performs the **packet-freshness SLO automation** the typed model
cannot: against the manifest `as_of` date it recomputes each packet's state from
`captured_at` and `freshness_slo`, and fails when a declared state is fresher than
the clock allows, or when a published label rides a packet whose recomputed state
is `breached` or `missing`. A Stable label cannot quietly outlive its proof.

## Narrowing reasons and the publication automation

The closed reason vocabulary (mirrored in the schema and the typed model) is:

- `backing_claim_narrowed`
- `qualification_incomplete`
- `support_class_thinned`
- `proof_packet_freshness_breached`
- `proof_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`

Each `publication_rule` names one reason as its `trigger_reason`, the labels it
watches (`applies_to_labels`), a `default_action`, and whether it
`blocks_publication`. A rule **fires** when any claimed entry in its watch set
carries its trigger reason. Every narrowing reason has a rule watching for it, so
a narrowing reason can never fire without a corresponding publication gate.

The cross-artifact automation is what makes the manifest *ingest* the stable line
rather than restate it. The CI gate reads the three named upstream artifacts and,
for each entry, fails when:

- the entry still publishes a label while its backing stable claim, a backing
  qualification lane, or its support class is narrowed below the cutline (or
  thinned),
- the entry's backing is narrowed but it does not carry the matching reason, or
- the entry names a narrowing reason while its backing still holds, or names a
  backing the neighbouring artifact does not carry.

## Publication verdict

The `publication` block records the verdict for the stable claim line. It is
`hold` when any blocking publication rule fires and `proceed` otherwise. The
`blocking_rule_ids` and `blocking_entry_ids` enumerate the firing rules and the
entries that triggered them. The gate recomputes all three and fails on any
drift, so the verdict can never overstate readiness.

At this revision three subjects put forward as Stable are narrowed below the
cutline — for a narrowed backing stable claim, narrowed qualification lanes, a
thinned support class, a proof packet past its freshness SLO, and an expired
waiver — so five blocking publication rules fire and stable claim publication is
held. That is the honest posture: the repository is pre-implementation and most
subjects have not yet earned a published Stable label.

## CI gate

Run:

```sh
python3 ci/check_stable_claim_manifest.py --repo-root .
```

The gate fails when a closed vocabulary or the cutline drifts; when an entry that
is narrowed does not drop below the cutline or fails to name a reason; when a
published entry carries an active narrowing reason, rides a stale or uncaptured
packet, or lacks owner sign-off; when a packet's declared SLO state overstates its
freshness against `as_of`; when an entry's posture disagrees with a backing claim,
qualification lane, or support-class entry; when a provisional label rides an
expired waiver; when the publication verdict or blocking sets disagree with the
firing rules; when the summary counts drift; or when a referenced artifact does
not exist. It also runs negative drills and the checked-in fixture cases under
[`/fixtures/release/stable_claim_manifest/`](../../fixtures/release/stable_claim_manifest/),
and writes a validation capture to
[`/artifacts/release/captures/stable_claim_manifest_validation_capture.json`](../../artifacts/release/captures/stable_claim_manifest_validation_capture.json).

Shiproom and release tooling can fail publication directly from this artifact:

```sh
python3 ci/check_stable_claim_manifest.py --repo-root . --require-proceed
```

This exits non-zero (code 2) whenever the recomputed publication verdict is
`hold`, distinct from an invalid-artifact failure (code 1).

The typed Rust consumer
(`aureline_release::stable_claim_manifest::current_stable_claim_manifest`) reads
the same manifest and runs the same structural cross-check, and exposes a
redaction-safe `support_export_projection()` for Help/About and support surfaces,
so `cargo test -p aureline-release` enforces these invariants without a cargo
build in CI.

## Update rules

1. Land qualification evidence, refreshed proof packets, and waivers first; point
   each entry's `proof_packet`, `backing_claim_ref`, `qualification_row_refs`, and
   `support_class_ref` at the canonical records.
2. Set each entry's `manifest_state`, `active_narrowing_reasons`, `slo_state`, and
   `published_label` to the honest posture. An entry whose backing narrowed, whose
   packet breached its freshness SLO, or whose waiver expired narrows below the
   cutline.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_stable_claim_manifest.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower stable label than planned, narrow the label and
   update the manifest — do not paper over the gap with prose.
