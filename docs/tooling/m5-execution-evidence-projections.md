# Execution-evidence projections: preserved run/step/provider/artifact lineage

This packet freezes the canonical truth for the **individual projected overlay**: a
**coverage overlay**, **flaky-test history** strip, **perf-regression note**,
**notebook-output verdict**, **pipeline annotation**, or **review-side marker**
re-rendered somewhere other than the run that produced it. Each projection binds its
overlay to the original **run/step/provider/artifact lineage**, the
**revision-remap quality** that maps origin anchors onto the current
revision/cursor, the evidence **freshness/stale/superseded state**, the
**confidence tier**, and the **reopen-to-origin target** — so old evidence shown on
a fresh surface can never quietly read as current truth.

It is the per-overlay companion to the
[`m5-execution-evidence`](./m5-execution-evidence.md) **lane matrix** and the
[`m5-problem-records`](./m5-problem-records.md) **Problems row**. Where the lane
matrix freezes one row per Problems/output/execution-evidence *surface family* and
the Problems packet freezes one row per *finding*, this packet freezes one row per
*projected overlay*. All three speak one vocabulary — origin class, confidence tier,
freshness state, reopen target, and proof currency are reused, not re-invented — so
coverage, flaky, perf, notebook, pipeline, review, CLI/headless, AI evidence, and
support export ingest one model instead of a private overlay truth model. Reuse the
canonical run/step/provider refs, generated-artifact ids, output channels, and
evidence packets already landed earlier; this packet binds them onto one
inspectable, reopenable overlay.

If this doc, the
[`m5-execution-evidence-projections.schema.json`](../../schemas/tooling/m5-execution-evidence-projections.schema.json)
boundary, the frozen set under
[`/artifacts/tooling/m5-execution-evidence-projections/`](../../artifacts/tooling/m5-execution-evidence-projections/),
and the perturbation corpus under
[`/fixtures/tooling/m5-execution-evidence-projections/`](../../fixtures/tooling/m5-execution-evidence-projections/)
disagree, the machine-readable schema plus the checked-in support export
(`artifacts/tooling/m5-execution-evidence-projections/support_export.json`) win, and
this doc must update in the same change.

## Companion artifacts

- [`/schemas/tooling/m5-execution-evidence-projections.schema.json`](../../schemas/tooling/m5-execution-evidence-projections.schema.json)
  — boundary schema for the `m5_execution_evidence_projection_set_packet` and every
  frozen taxonomy.
- [`/artifacts/tooling/m5-execution-evidence-projections/support_export.json`](../../artifacts/tooling/m5-execution-evidence-projections/support_export.json)
  — the canonical projection set (the source of truth for every overlay).
- [`/artifacts/tooling/m5-execution-evidence-projections/report.md`](../../artifacts/tooling/m5-execution-evidence-projections/report.md)
  — the generated certification report (do not edit by hand; regenerate with the
  Rust dump example).
- [`/fixtures/tooling/m5-execution-evidence-projections/`](../../fixtures/tooling/m5-execution-evidence-projections/)
  — the perturbation corpus that pins each narrowing/floor rule.
- `tools/release/execution_evidence_projections.py` — re-derives the effective
  claim and ordered narrowing reasons per projection and validates the set and
  corpus.
- `crates/aureline-runtime/src/m5_execution_evidence_projection_overlays/` — the
  in-process Rust truth source. It deserializes the checked-in support export into
  one typed packet, re-derives the same effective claim, floor/overlay/labs ladder,
  and ordered narrowing reasons as the Python engine, and exposes
  `current_m5_execution_evidence_projection_set()` so desktop, CLI/headless, AI
  evidence, support export, review, notebook, and pipeline consumers ingest the
  governed projection without re-parsing raw logs or forking a parallel truth model.

## The causal chain a projection preserves

A projection re-renders evidence away from its origin run. To stay honest it must,
on every surface it renders, be able to answer **which run, which step, which
provider, which artifact** produced it; whether the overlay is **on the current
revision** and how well its anchors **remapped**; whether the evidence is **fresh,
stale, or superseded**; and **how to reopen** the originating run, channel,
artifact, or packet. The engine re-derives — rather than trusts — an effective claim
from these invariants:

- **Origin lineage** (`integrity.preserves_origin_run_step`,
  `preserves_provider_artifact`, `lineage_visible_on_demand`): the origin
  run/step and provider/artifact identity survive into the overlay and can be
  revealed on demand on every rendering surface.
- **Revision remap** (`revision_remap.quality`, `anchored_to_current_revision`,
  `cursor_remap_applied`, `remap_quality_labeled`): an overlay anchored
  `exact_current_revision` reads differently from one `shifted_tracked`,
  `approximate_remap`, `stale_unmapped`, or `not_anchored`, and the quality is
  always labelled. A `stale_unmapped` anchor that still claims the current revision
  narrows.
- **Freshness** (`declared_freshness_state`, `integrity.freshness_state_labeled`,
  `superseded_state_marked`): stale and superseded states stay visibly classified;
  missing evidence floors.
- **Confidence** (`declared_confidence_tier`, `integrity.confidence_label_visible`,
  `raw_output_backlink_present`): the tier is visible, and a heuristic projection
  keeps a raw-output backlink.
- **Reopen** (`declared_reopen_target`): every projection can reopen its origin;
  a projection that loses its reopen path keeps a `raw_output_backlink` or
  `none_keyboard_fallback`.
- **Surface honesty** (`renderings[*].rendered_claim`): a rendering surface may
  never render a claim wider than the projection's effective claim.

## The effective-claim ladder

| Effective claim | Meaning |
| --- | --- |
| `projection_certified` | Full first-party lineage preserved, fresh, remap exact/tracked, reopenable. |
| `projection_narrowed` | A first-party projection held below certified by a stale/remap/labelled gap, but lineage stays reopenable. |
| `projection_read_only_overlay` | Remote/pipeline/imported evidence: attributable and reopenable but never claims live local authority. |
| `projection_unreconstructable` | Lineage/remap/reopen broken: surfaces a raw-output backlink or keyboard fallback instead of a clean-but-false overlay. |
| `projection_labs_not_claimed` | Labs/unadvertised: makes no public claim and is never widened. |

**Floor** reasons (`origin_run_step_flattened`, `provider_artifact_flattened`,
`lineage_not_visible`, `raw_output_backlink_missing`, `reopen_target_lost`,
`surface_overclaims`, `imported_overlay_claims_live`, `evidence_missing`) break the
"stay reopenable / never flatten lineage / never masquerade as live" contract
outright and drop the projection to `projection_unreconstructable`. The remaining
reasons hold a first-party projection at `projection_narrowed` (still reopenable). An
overlay is already the minimal honest claim, so any non-floor gap drops it below the
read-only overlay too. Labs projections never accrue narrowing.

## Regeneration

```bash
# Rust: regenerate the support export and report (identical bytes each run).
cargo run -p aureline-runtime --example dump_m5_execution_evidence_projections > \
  artifacts/tooling/m5-execution-evidence-projections/support_export.json
cargo run -p aureline-runtime --example dump_m5_execution_evidence_projections summary > \
  artifacts/tooling/m5-execution-evidence-projections/report.md

# Python: regenerate the perturbation corpus and validate end-to-end.
python3 tools/release/execution_evidence_projections.py emit-corpus
python3 tools/release/execution_evidence_projections.py self-test
```
