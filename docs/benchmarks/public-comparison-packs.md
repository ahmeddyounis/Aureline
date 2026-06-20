# Public-comparison and reproducibility pack policy — normative

This document is the **normative** policy for reproducibility packs behind public,
procurement, and enterprise-facing M5 benchmark comparisons. It makes every such
comparison reproducible enough to survive independent review: each claim binds to
a pack that carries the raw configuration, exact commands, corpus and revision
refs, reference-hardware and lab-image identity, environment notes, caveats,
raw-run-metadata refs, and reproduction recipe a reviewer reruns or audits
against.

If this document disagrees with the machine-readable schema or register, the
schema and register win and this document is updated in the same change.

Companion artifacts:

- [`/schemas/benchmarks/public-comparison-pack.schema.json`](../../schemas/benchmarks/public-comparison-pack.schema.json)
  — boundary schema for a reproducibility pack and the pack register.
- [`/artifacts/benchmarks/public-comparison-pack-register.json`](../../artifacts/benchmarks/public-comparison-pack-register.json)
  — canonical register binding every governance publication pack to a
  reproducibility pack (the truth source consumers cite).
- [`/artifacts/benchmarks/sample-public-pack/`](../../artifacts/benchmarks/sample-public-pack/)
  — worked standalone sample pack and rerun recipe.
- [`/fixtures/benchmarks/public-comparison/`](../../fixtures/benchmarks/public-comparison/)
  — fixtures proving each fail-closed path.
- [`/ci/check_public_comparison_pack.py`](../../ci/check_public_comparison_pack.py)
  — the validator that enforces this policy.

This policy sits **alongside** the
[public benchmark comparison rules](./public_comparison_rules.md), which govern
*what posture* a comparison may publish, and **underneath** the
[benchmark-governance matrix](./m5-benchmark-governance.md), which holds each
protected metric's current claim state and each publication pack's required
disclosure fields. This document governs the *reproducibility artifact*: the
checked-in pack that lets an independent reviewer rerun or audit the comparison
later.

## 1. Why a reproducibility pack exists

The governance matrix records *which* fields a publication pack must disclose. It
left implicit the thing a reviewer actually needs to **rerun** a comparison: the
exact commands, the build identity, the corpus and lab-image revisions, the
power/thermal posture, the retained raw-run metadata, and a reproduction recipe.
Without that, a public win rests on memory and slides. The reproducibility pack
closes that gap: every public, procurement, or enterprise comparison is one
self-describing, checked-in pack, and the register keeps one in force for every
publication pack in the matrix.

## 2. The reproducibility pack

Each pack is a single self-describing object validated by its boundary schema. It
carries **no** raw run logs, raw provider payloads, raw machine labels, secret
material, or credential bodies — only stable ids, publishable command lines and
config knobs, and reviewable sentences. The governed fields are:

- **`governance_pack_ref`** — the publication pack in the
  [governance matrix](../../artifacts/benchmarks/m5-benchmark-governance.json)
  this pack backs. Its posture, metric set, and reference-hardware and lab-image
  identity must agree with the governance pack.
- **`posture`** — `methodology_only`, `aureline_only_claim`,
  `public_head_to_head_comparison`, or `quarantined_not_comparable`. The first
  and last are not claim-bearing.
- **`surfaces`** — the surfaces this pack authorizes. Only a claim-bearing pack
  may authorize a `public_comparison`, `procurement_packet`, or
  `enterprise_evaluation` surface.
- **`raw_configuration`** — the exact `command_lines`, material `config_knobs`,
  and `build_identity_refs` that produced the result.
- **`corpus_binding`** — the corpus refs and the corpus-manifest,
  protected-metrics, and fitness-catalog revisions exercised.
- **`environment`** — the reference-hardware profile, display class, lab-image
  revision, environment ref, power/thermal posture, calibration-drift note, and
  free-text environment notes.
- **`comparison`** — for a head-to-head only: the competitor ref and version,
  plugin posture, and task-parity note.
- **`caveats`**, **`raw_run_metadata_refs`**, **`raw_run_metadata_retained`**,
  **`reproduction`** — the exclusions, the stable pointers to retained run
  metadata, and the steps plus rerun-recipe ref a reviewer reruns against.
- **`disclosed_fields`** — must be a superset of the governance pack's
  `required_disclosure_fields`; an incomplete pack is not claim-bearing by
  implication.
- **`freshness`** — the refresh rule, capture date, SLO, and hard expiry.

## 3. Fail-closed rules

The validator rejects a pack that:

- binds to a publication pack absent from the matrix
  (`governance_pack_unresolved`);
- disagrees with its governance pack on posture, metric set, or hardware/lab-image
  identity (`posture_mismatch`, `metric_refs_mismatch`,
  `hardware_identity_mismatch`);
- omits a required disclosure field (`undisclosed_required_field`);
- is non-claim-bearing yet authorizes a public, procurement, or enterprise surface
  (`non_claim_surface_scope`);
- is claim-bearing yet ships **without raw configuration**
  (`missing_raw_configuration`), **without environment metadata**
  (`missing_environment_metadata`), without retained raw-run metadata
  (`missing_raw_run_metadata`), without a reproduction recipe
  (`missing_reproduction_recipe`), or — for a head-to-head — without competitor
  and task-parity disclosure (`missing_comparison_disclosure`); or
- is claim-bearing and past its freshness window
  (`expired_freshness_blocks_claim`).

The register-wide gate additionally requires **exactly one in-force pack per
governance publication pack** (`publication_pack_missing_reproducibility_pack`,
`multiple_in_force_packs`), so no claim ships without a reproducibility pack and
none is ambiguous.

> **Guardrail.** A public benchmark claim never ships without raw configuration
> and environment metadata. The `missing_raw_configuration` and
> `missing_environment_metadata` rules enforce this directly.

## 4. Consuming the packs

Docs, help, release/shiproom, and procurement surfaces **cite pack ids** from the
register rather than rephrasing benchmark context by hand. The validator projects
the in-force packs — their posture, authorized surfaces, freshness, and
claim-bearing flag — for those consumers; a narrowed or expired pack surfaces as
such instead of as its ceiling. This keeps a single benchmark truth source behind
every public-facing comparison.
