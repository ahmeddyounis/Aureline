# Stable claim manifest — proof packet

Reviewer-facing proof packet for the gated stable claim manifest, its canonical
lifecycle labels, and the packet-freshness SLO automation.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Manifest: [`/artifacts/release/stable_claim_manifest.json`](../stable_claim_manifest.json)
- Schema: [`/schemas/release/stable_claim_manifest.schema.json`](../../../schemas/release/stable_claim_manifest.schema.json)
- Companion doc: [`/docs/release/stable_claim_manifest.md`](../../../docs/release/stable_claim_manifest.md)
- Validator: `ci/check_stable_claim_manifest.py`
- Validation capture:
  [`/artifacts/release/captures/stable_claim_manifest_validation_capture.json`](../captures/stable_claim_manifest_validation_capture.json)
- Typed consumer: `aureline_release::stable_claim_manifest`

## What this packet proves

1. **Each subject publishes exactly one canonical lifecycle label.** Every
   subject the stable line publishes has one manifest entry binding it to the
   label it is put forward as (`claimed_label`), the label it effectively
   publishes after narrowing (`published_label`), the backing stable claim,
   qualification rows, and support-class entry it depends on, its proof packet,
   and its owner sign-off. The manifest reuses the stable claim matrix's level
   vocabulary as the lifecycle-label vocabulary rather than re-minting labels, so
   docs, Help/About, shiproom dashboards, and support exports render one label
   instead of cloning their own.

2. **The manifest ingests the three upstream M4 records.** The CI gate reads the
   stable claim matrix, the stable qualification matrix, and the v1.0
   support-class ledger named by `claim_matrix_ref`, `qualification_matrix_ref`,
   and `support_class_ledger_ref`, and fails when an entry publishes a label while
   any backing row is narrowed, or narrows on a backing that still holds, or names
   a backing the neighbouring artifact does not carry. The published label can
   never outrun the stable line it ingests.

3. **The packet-freshness SLO automation narrows stale labels before
   publication.** Each entry's proof packet carries a freshness SLO
   (`target_max_age_days`, `warn_within_days`) and a recorded `slo_state`. The CI
   gate recomputes the state from `captured_at` against the manifest `as_of` date
   and fails when a declared state overstates freshness or when a published label
   rides a `breached`/`missing` packet. The `localization_readiness` entry is the
   worked example: its packet aged past the SLO, so the automation narrows its
   label and fires a blocking publication rule.

## Proof-index registration

Each entry's proof packet registers under one row of the stable proof index
([`/artifacts/milestones/m3/public_proof_index.md`](../../milestones/m3/public_proof_index.md))
via its `proof_packet.proof_index_ref`, so this lane's proof is anchored to the
public-proof artifact index rather than to ad hoc notes. As the v1.0 proof index
is assembled it ingests these manifest entries by `entry_id`.

| Manifest entry | Claimed → published | SLO state | Proof-index row |
|---|---|---|---|
| `provider_aware_language_intelligence` | stable → stable | current | `m3_public_proof:launch_wedge` |
| `repair_and_rollback_safety` | stable → stable (waiver) | due_for_refresh | `m3_public_proof:exact_build_identity` |
| `export_and_offboarding_support` | stable → beta | current | `m3_public_proof:boundary_truth` |
| `localization_readiness` | stable → preview | breached | `m3_public_proof:docs_freshness` |
| `regulated_environment_assurance` | stable → beta | current | `m3_public_proof:version_skew_truth` |

## Current posture

At this revision two subjects publish a Stable label (one outright, one
provisionally under an active waiver whose packet is due for refresh) and three
are narrowed below the cutline: export/offboarding for a narrowed backing claim,
narrowed qualification lane, and thinned support class; localization for a proof
packet past its freshness SLO on top of a narrowed backing; and regulated
assurance for an expired waiver on top of a narrowed backing. Their reasons fire
five blocking publication rules, so stable claim publication **holds**. That is
the honest posture: the repository is pre-implementation and most subjects have
not yet earned a published Stable label.

## Accessibility of this lane

The manifest and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables (no color-only encoding), and the
machine source carries the same truth so Help/About, the release center, support
exports, docs, and shiproom dashboards ingest one record rather than restating
status text.

## How to refresh

1. Land refreshed proof packets, qualification evidence, and waivers first; point
   each entry's `proof_packet`, `backing_claim_ref`, `qualification_row_refs`, and
   `support_class_ref` at the canonical records.
2. Set each entry's `manifest_state`, `slo_state`, `active_narrowing_reasons`, and
   `published_label` to the honest posture. An entry whose backing narrowed, whose
   packet breached its freshness SLO, or whose waiver expired narrows below the
   cutline.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_stable_claim_manifest.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower stable label than planned, narrow the label and
   update the manifest — do not paper over the gap with prose.
