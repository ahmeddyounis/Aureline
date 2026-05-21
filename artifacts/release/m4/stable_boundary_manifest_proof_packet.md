# Stable boundary manifest — proof packet

Reviewer-facing proof packet for the gated stable boundary manifest across the
local-OSS, self-hosted, managed, and air-gapped value lines.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Manifest: [`/artifacts/release/stable_boundary_manifest.json`](../stable_boundary_manifest.json)
- Schema: [`/schemas/release/stable_boundary_manifest.schema.json`](../../../schemas/release/stable_boundary_manifest.schema.json)
- Companion doc: [`/docs/release/stable_boundary_manifest.md`](../../../docs/release/stable_boundary_manifest.md)
- Validator: `ci/check_stable_boundary_manifest.py`
- Validation capture:
  [`/artifacts/release/captures/stable_boundary_manifest_validation_capture.json`](../captures/stable_boundary_manifest_validation_capture.json)
- Typed consumer: `aureline_release::stable_boundary_manifest`

## What this packet proves

1. **Each subject publishes one lifecycle label per value line, never wider than
   its canonical label.** Every subject the stable line publishes carries one
   boundary row per value line, binding the line to the label it publishes
   (`published_label`), the subject's canonical ceiling (`manifest_label`), the
   line-capability ref it depends on, its proof packet, and its owner sign-off.
   The manifest reuses the stable claim level vocabulary rather than minting
   per-line labels, so docs, Help/About, the release center, and support exports
   render one label per cell.

2. **The manifest ingests the stable claim manifest as a hard ceiling.** The CI
   gate reads the stable claim manifest named by `claim_manifest_ref` and fails
   when a row's `manifest_label` is not the label that manifest publishes for the
   entry, when a row names an entry the manifest does not carry, or when a line
   publishes wider than the subject's canonical label. A value-line boundary can
   never outrun the canonical label it ingests.

3. **The packet-freshness SLO automation narrows stale lines before
   publication.** Each row's proof packet carries a freshness SLO
   (`target_max_age_days`, `warn_within_days`) and a recorded `slo_state`. The CI
   gate recomputes the state from `captured_at` against the manifest `as_of` date
   and fails when a declared state overstates freshness or when a published line
   rides a `breached`/`missing` packet. The air-gapped export/offboarding row is
   the worked example: its packet aged past the SLO, so the automation narrows it
   further to preview.

## Proof-index registration

Each row's proof packet registers under one row of the stable proof index
([`/artifacts/milestones/m3/public_proof_index.md`](../../milestones/m3/public_proof_index.md))
via its `boundary_packet.proof_index_ref`, so this lane's proof is anchored to the
public-proof artifact index rather than to ad hoc notes.

## Boundary matrix at this revision

Per value line, the lifecycle label each subject publishes (canonical ceiling in
parentheses):

| Subject (ceiling) | local-oss | self-hosted | managed | air-gapped |
|---|---|---|---|---|
| Provider-aware language intelligence (stable) | stable | stable | stable | **beta** (capability absent, evidence incomplete) |
| Repair and rollback safety (stable) | stable | stable (due for refresh) | stable (waiver) | **beta** (waiver expired) |
| Export and offboarding support (beta) | beta (manifest ceiling) | beta (manifest ceiling) | beta (manifest ceiling) | **preview** (packet breached) |

Per-line rollup (Stable / total): local-oss 2/3, self-hosted 2/3, managed 2/3,
air-gapped 0/3.

## Current posture

At this revision six of twelve `(subject, value line)` cells publish a Stable
label and six are narrowed below the cutline. The air-gapped line narrows every
subject: provider-aware language intelligence for an absent capability and
incomplete offline qualification, repair/rollback for an expired waiver, and
export/offboarding for a breached proof packet on top of an already-narrowed
subject. Two of those air-gapped lines (whose subjects are canonically Stable)
fire three blocking boundary rules, so stable boundary publication **holds**. The
export/offboarding rows narrow only because their subject's canonical label is
already below the cutline; they inherit that ceiling without holding boundary
publication. That is the honest posture: the repository is pre-implementation and
the air-gapped line has the least qualification.

## Accessibility of this lane

The manifest and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables (no color-only encoding), and the
machine source carries the same truth so Help/About, the release center, support
exports, docs, and shiproom dashboards ingest one record per value line rather
than restating status text.

## How to refresh

1. Land the upstream stable claim manifest entry, line qualification evidence,
   refreshed proof packets, and waivers first; point each row's
   `manifest_entry_ref`, `manifest_label`, `boundary_packet`, and
   `line_capability_ref` at the canonical records.
2. Set each row's `boundary_state`, `slo_state`, `active_narrowing_reasons`, and
   `published_label` to the honest posture.
3. Recompute the `publication` block, the per-line rollups, and `summary`, then
   run `python3 ci/check_stable_boundary_manifest.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower value-line boundary than planned, narrow the line
   label and update the manifest — do not paper over the gap with prose.
