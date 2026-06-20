# Benchmark publication ingestion

Every user-facing description of an Aureline benchmark or performance result must
derive from the same canonical publication object. Documentation, in-product
help, the About surface, enterprise evaluation packs, and support exports do not
write their own benchmark prose; they **ingest** a claim publication entry and
render its fields. This page describes the ingestion contract and the register
that enforces it.

## Why ingestion instead of cloned prose

A benchmark claim is only trustworthy while the run behind it is fresh and
comparable. The [shiproom benchmark-freshness ledger](claim-narrowing.md)
recomputes, for every claim publication entry, a freshness and comparability
state and a **narrowed effective claim**: when a corpus revision, hardware class,
lab image, threshold version, or run-metadata field drifts, the claim narrows or
quarantines automatically.

If a docs page, help topic, About box, evaluation pack, or support export carried
its own copy of the wording, that copy would keep asserting the old claim after
the ledger narrowed it. Hand-written benchmark copy in a stable-facing surface is
therefore not allowed once a canonical publication entry exists. Each surface
binds to the entry and renders the entry's current values, so a narrowing in the
ledger downgrades every surface in the same release train.

## The canonical objects

| Object | Location | Identifier |
| --- | --- | --- |
| Claim publication entry | [`shiproom-benchmark-freshness.json`](../../artifacts/benchmarks/shiproom-benchmark-freshness.json) | `entry_id` |
| Protected metric / threshold revision | [`m5-benchmark-governance.json`](../../artifacts/benchmarks/m5-benchmark-governance.json) | `metric_ref` |
| Reproducibility pack | [`public-comparison-pack-register.json`](../../artifacts/benchmarks/public-comparison-pack-register.json) | `pack_ref` |
| Ingestion register | [`publication-ingestion-register.json`](../../artifacts/benchmarks/publication-ingestion-register.json) | `register_id` |

A claim publication entry carries the posture, the narrowed `effective_claim`, the
`freshness_state`, the `shiproom_blocker.downgrade_label`, and the `metric_refs`
the surface renders. The reproducibility pack carries the raw configuration,
environment, and reproduction recipe a reviewer reruns the claim against, and its
`surfaces` list states which surfaces it may publish to.

## The ingestion register

[`publication-ingestion-register.json`](../../artifacts/benchmarks/publication-ingestion-register.json)
binds each consuming surface to the entry it renders. Each binding records:

- `surface` — `docs`, `help`, `about`, `evaluation_pack`, or `support_export`.
- `surface_locator` — where the surface renders the entry.
- `consumes_entry_id` — the claim publication entry in the freshness ledger.
- `repro_pack_ref` — the reproducibility pack whose `governance_pack_ref` equals
  the consumed entry and whose `surfaces` authorize this surface.
- `renders_verbatim_from_entry` — asserts the surface renders the entry's values,
  not hand-written prose.
- `rendered_projection` — the `posture`, `effective_claim`, `freshness_state`,
  `downgrade_label`, and `metric_refs` the surface shows. Every value must equal
  the entry's.
- `disclosed_fields` — the export-safe field names the surface discloses.

## What the gate enforces

`ci/check_publication_ingestion.py` (workflow
`check_publication_ingestion`) fails closed when:

1. **A binding renders a claim that differs from the entry.** The
   `rendered_projection` must equal the ledger entry's posture, effective claim,
   freshness state, downgrade label, and metric refs. This is what makes a
   narrowed or quarantined claim propagate to every surface at once, and what
   stops a surface from ever publishing a stronger claim than the entry supports.
2. **A binding points at an unknown entry**, or its reproducibility pack backs a
   different entry.
3. **A binding renders a surface its reproducibility pack does not publish to.**
   For example, the enterprise-evaluation surface is authorized only for the
   head-to-head entry, because only its reproducibility pack publishes to
   enterprise evaluation.
4. **A binding discloses a field that is not export-safe**, or a field on the
   forbidden denylist. See
   [`support-export-benchmark-fields.md`](../../artifacts/benchmarks/support-export-benchmark-fields.md)
   for the export-safe field set and the redaction rules.
5. **A surface that must render an entry is missing.** Docs, help, and support
   exports must render every entry; the About surface must render every
   claim-bearing entry; an evaluation pack must render every entry its pack
   publishes to enterprise evaluation.
6. **The register's narrowed/quarantined projection disagrees with the ledger.**

The checked-in fixtures in
[`fixtures/benchmarks/docs-ingestion/`](../../fixtures/benchmarks/docs-ingestion/)
and the validator's negative drills prove each rejection path fires.

## Adding or changing a surface

1. Add or update the claim publication entry in the freshness ledger and its
   reproducibility pack first; never start from the surface copy.
2. Add a binding to the ingestion register pointing at the entry, with a
   `rendered_projection` copied from the entry and a `repro_pack_ref` that
   authorizes the surface.
3. Disclose only export-safe fields.
4. Run `python3 ci/check_publication_ingestion.py --repo-root .` until it passes.

Never leave hand-written benchmark copy in a stable-facing surface once its
canonical publication entry exists.
