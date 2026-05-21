# Stable boundary manifest across the local-OSS, self-hosted, managed, and air-gapped value lines

This document is the reviewer-facing companion for the gated stable boundary
manifest:

- [`/artifacts/release/stable_boundary_manifest.json`](../../artifacts/release/stable_boundary_manifest.json)
- schema: [`/schemas/release/stable_boundary_manifest.schema.json`](../../schemas/release/stable_boundary_manifest.schema.json)
- proof packet:
  [`/artifacts/release/m4/stable_boundary_manifest_proof_packet.md`](../../artifacts/release/m4/stable_boundary_manifest_proof_packet.md)

The manifest is the **canonical truth** for the lifecycle label each subject
publishes **per deployment value line**. The
[stable claim manifest](./stable_claim_manifest.md) decides the single canonical
label a subject is put forward as; this boundary manifest decides whether each of
the four value lines can actually carry the subject at that label, and how it
narrows when it cannot. Downstream dashboards, docs, Help/About surfaces, release
packets, and support exports MUST ingest this manifest by `boundary_id` and render
its `published_label` per line rather than minting their own per-deployment
maturity wording.

## The four value lines

The `value_lines` block is the closed vocabulary, in canonical order:

### local-oss line

**Local OSS** — the account-free open-source desktop product with no hosted
dependency. A subject is carried here when it works against local sidecars,
bring-your-own provider keys, or on-disk state without any vendor service.

### self-hosted line

**Self-hosted** — a customer-run control plane, identity, registry mirror, and
relay. A subject is carried here when its managed dependencies have a documented
self-hosted form the customer can stand up.

### managed line

**Managed** — vendor-hosted convenience services (managed gateway, sync,
marketplace hosting, collaboration brokering). A subject is carried here when its
hosted dependencies are reachable and qualified.

### air-gapped line

**Air-gapped** — an offline / sovereign deployment with no outbound connectivity,
fed only by mirrored updates, models, and policy bundles. A subject is carried
here only when it has a fully offline, mirror-fed form that is qualified for the
air-gapped line.

## The manifest ceiling — no per-line widening

Each row names the [stable claim manifest](./stable_claim_manifest.md) entry it
maps to (`manifest_entry_ref`) and the canonical lifecycle label that entry
publishes (`manifest_label`). That label is a **hard ceiling**: a value line may
match it or narrow below it, but its `published_label` may never be **wider**
(stronger) than the subject's canonical label. This is what makes the boundary
manifest *ingest* the claim manifest rather than restate it — the CI gate reads
the claim manifest and fails when a row's `manifest_label` is not the label the
claim manifest publishes for that entry. The manifest reuses the stable claim
matrix's level vocabulary — `lts`, `stable`, `beta`, `preview`, `withdrawn` —
rather than minting per-line labels.

The manifest is a full **subject × value-line matrix**: every subject carries
exactly one row per value line, with the same ceiling label everywhere it
appears.

## The launch cutline

The cutline fixes the boundary between a value line that publishes Stable (or LTS)
and one narrowed below it:

```
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

A value line publishes a label at or above the cutline only when the subject's
canonical manifest label is at or above the cutline, the line carries every
capability the subject needs, the line's qualification evidence holds, its proof
packet is within its freshness SLO, and an owner has signed off. A value line that
loses any of those drops to a label below the cutline and never publishes wider
than the subject's canonical manifest label.

## Boundary rows and states

Every row carries a `value_line`, a `manifest_entry_ref` + `manifest_label`
(the ceiling), a `line_capability_ref`, a `boundary_packet` (with its freshness
SLO), an optional `waiver`, an `owner_signoff`, the `boundary_state` it earned,
its `active_narrowing_reasons`, and the `published_label` it publishes.

The `boundary_state` is the verdict for that cell:

- `available` — the line carries the subject at its full canonical label.
- `available_on_waiver` — carries the canonical label only because an active,
  unexpired waiver covers a recorded line-specific gap.
- `narrowed_unsupported` — the line lacks a capability the subject needs or its
  line-specific qualification is incomplete; the label must narrow.
- `narrowed_by_manifest` — the subject's canonical manifest label is itself below
  the cutline, so every line inherits that ceiling and narrows.
- `narrowed_stale` — the line's proof packet breached its freshness SLO (or is
  missing); the label must narrow.
- `narrowed_waiver_expired` — the line relied on a waiver that has expired; the
  label must narrow.

A narrowing row MUST drop below the cutline and name at least one active narrowing
reason. A held row MUST publish the subject's canonical label cleanly — within-SLO
captured packet, owner sign-off, **no** active narrowing reason.

## Packet-freshness SLO

Each row's `boundary_packet` carries a `freshness_slo`:

- `target_max_age_days` — the SLO: the packet may be at most this many days old.
- `warn_within_days` — when the days remaining before the target drop to this or
  below, the packet is `due_for_refresh` (a warning, not yet a breach).
- `slo_register_ref` — the ref into this register that defines the target.

The `slo_state` is the freshness verdict: `current`, `due_for_refresh`,
`breached`, or `missing`. The CI gate performs the **packet-freshness SLO
automation** the typed model cannot: against the manifest `as_of` date it
recomputes each packet's state from `captured_at` and `freshness_slo`, and fails
when a declared state is fresher than the clock allows, or when a published line
rides a packet whose recomputed state is `breached` or `missing`. A Stable line
label cannot quietly outlive its proof.

## Narrowing reasons and the publication automation

The closed reason vocabulary (mirrored in the schema and the typed model) is:

- `manifest_label_narrowed`
- `line_capability_absent`
- `line_evidence_incomplete`
- `boundary_packet_freshness_breached`
- `boundary_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`

Each `boundary_rule` names one reason as its `trigger_reason`, the subject labels
it watches (`applies_to_labels`), a `default_action`, and whether it
`blocks_publication`. A rule **fires** when any watched row carries its trigger
reason. Every narrowing reason has a rule watching for it, so a narrowing reason
can never fire without a corresponding publication gate.

The `manifest_label_narrowed` rule is intentionally **non-blocking**: a row
narrowed only because its subject's canonical label is already below the cutline
is expected inheritance, not a boundary defect — the stable claim manifest already
holds that subject upstream. The remaining reasons describe a value line that
*could* be Stable (its subject is canonically Stable) but is not, so they block
boundary publication.

## Publication verdict

The `publication` block records the verdict for the stable boundary line. It is
`hold` when any blocking boundary rule fires and `proceed` otherwise. The
`blocking_rule_ids` and `blocking_boundary_ids` enumerate the firing rules and the
rows that triggered them (only rows whose subject ceiling is at or above the
cutline count). The gate recomputes all three, the per-value-line rollups, and the
summary, and fails on any drift.

At this revision two value lines whose subjects are canonically Stable are
narrowed for fixable boundary reasons — the air-gapped provider-aware language
intelligence line lacks outbound provider routing and an offline qualification,
and the air-gapped repair/rollback line's waiver expired — so three blocking
boundary rules fire and stable boundary publication is held. The air-gapped line
narrows every subject at this revision; that is the honest posture for a
pre-implementation repository.

## CI gate

Run:

```sh
python3 ci/check_stable_boundary_manifest.py --repo-root .
```

The gate fails when a closed vocabulary or the cutline drifts; when a row that is
narrowed does not drop below the cutline or fails to name a reason; when a
published row carries an active narrowing reason, rides a stale or uncaptured
packet, or lacks owner sign-off; when a row publishes wider than its subject's
ceiling; when a row's `manifest_label` disagrees with the stable claim manifest;
when a subject does not cover all four value lines; when a packet's declared SLO
state overstates its freshness against `as_of`; when a line holds on an expired
waiver; when the publication verdict, blocking sets, per-line rollups, or summary
counts drift; or when a referenced artifact does not exist. It also runs negative
drills and the checked-in fixture cases under
[`/fixtures/release/stable_boundary_manifest/`](../../fixtures/release/stable_boundary_manifest/),
and writes a validation capture to
[`/artifacts/release/captures/stable_boundary_manifest_validation_capture.json`](../../artifacts/release/captures/stable_boundary_manifest_validation_capture.json).

Shiproom and release tooling can fail publication directly from this artifact:

```sh
python3 ci/check_stable_boundary_manifest.py --repo-root . --require-proceed
```

This exits non-zero (code 2) whenever the recomputed publication verdict is
`hold`, distinct from an invalid-artifact failure (code 1).

The typed Rust consumer
(`aureline_release::stable_boundary_manifest::current_stable_boundary_manifest`)
reads the same manifest and runs the same structural cross-check, and exposes a
redaction-safe `support_export_projection()` (with per-value-line rollups) for
Help/About and support surfaces, so `cargo test -p aureline-release` enforces
these invariants without a cargo build in CI.

## Update rules

1. Land the upstream stable claim manifest entry, line qualification evidence,
   refreshed proof packets, and waivers first; point each row's
   `manifest_entry_ref`, `manifest_label`, `boundary_packet`, and
   `line_capability_ref` at the canonical records.
2. Set each row's `boundary_state`, `active_narrowing_reasons`, `slo_state`, and
   `published_label` to the honest posture. A line that lacks a capability, whose
   packet breached its freshness SLO, or whose waiver expired narrows below the
   cutline; a subject whose canonical label is already below the cutline narrows
   every line.
3. Recompute the `publication` block, the per-line rollups, and `summary`, then
   run `python3 ci/check_stable_boundary_manifest.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower value-line boundary than planned, narrow the line
   label and update the manifest — do not paper over the gap with prose.
