# Protected benchmark corpus

This directory is the home of the **corpus manifest** — the single
machine-readable register of every fixture the benchmark lab, the
journey harness, the boundary-truth validators, the compatibility
scoreboards, and the support-export lanes read when they need to
name "which corpus revision, against which protected journey, on
which platform class, under which retention and visibility class".

- [`corpus_manifest.yaml`](./corpus_manifest.yaml) — the register.
  Tooling reads this file; consumers resolve fixtures by stable id
  and never guess paths.
- [`/docs/benchmarks/fixture_classes.md`](../../docs/benchmarks/fixture_classes.md)
  — the normative companion that defines the corpus-class, protected-
  journey, size-class, visibility / retention / license, host-
  platform, toolchain, archetype, support, and evidence-consumer
  vocabularies the manifest resolves against.

## What the corpus covers today

| Corpus class                 | Count | Representative entries                                                                                                   |
|------------------------------|------:|--------------------------------------------------------------------------------------------------------------------------|
| `microbenchmark_scenario`    |     2 | shaping smoke corpus; interaction-safety cases                                                                           |
| `workflow_scenario`          |     3 | warm-start-to-first-paint; first-useful-edit on a Rust self-host slice; plain-open unknown-archetype                     |
| `archetype_seed`             |     3 | TS web-app; Python data-app; unrecognised misc folder                                                                    |
| `large_file_trigger`         |     8 | one fixture per ADR-0003 trigger (six checked-in) plus a control and an oversize synthetic recipe                        |
| `recovery_or_restore_scenario` |   3 | compatible restore after crash; recent-work missing target; managed-workspace reauth                                     |
| `reference_workspace`        |     6 | micro local folder; Rust self-host slice; TS web-app; Python data-app; unrecognised misc folder; partially-ready restore |
| `boundary_truth_case`        |     3 | filesystem-identity cases; mutation-lineage cases; entry-restore cases                                                   |

See [`corpus_manifest.yaml`](./corpus_manifest.yaml) for the full
register with per-fixture metadata.

## How to read a fixture entry

Every entry carries:

- `id` — stable id, resolved verbatim by downstream consumers.
- `corpus_class` — one value from the closed corpus-class set.
- `path` **or** `resolution_mode` — `concrete_file` (default) points
  at checked-in bytes; `live_repo_slice` resolves a pattern list
  against the live repository at run time; `recipe_only` names a
  generator and a deterministic seed (the only admissible mode for
  `size_class: oversize`).
- `source_revision` — where the bytes came from, the provenance
  class, and the manifest revision the fixture first appeared in.
- `size_class`, `visibility_class`, `retention_class`,
  `license_status` — retention / visibility / license posture.
- `toolchain_assumption`, `host_platform_class` — build-time
  assumptions the benchmark report records alongside metrics.
- `protected_journeys[]` — the protected-path buckets the fixture
  exercises (aligned with
  [`/docs/benchmarks/spike_metric_names.md`](../../docs/benchmarks/spike_metric_names.md)).
- `support_classes[]` and `archetype_tags[]` — the archetype support
  tier and placeholder tag the fixture exercises.
- `evidence_consumer_channels[]` — which benchmark-lab lane, UX
  evidence packet, support-bundle summary, release-evidence claim
  manifest, or boundary-truth validator reads the fixture.
- `trigger_exercised` / `task_success_corpus_ref` /
  `degraded_notes[]` — optional slots that attach additional
  machine-readable context.

## Adding a fixture

Per the change policy in
[`/docs/benchmarks/fixture_classes.md`](../../docs/benchmarks/fixture_classes.md)
§12:

- **Additive-minor** changes land in this directory and in the
  manifest in one change. Cite the motivating scenario or evidence
  family on the fixture entry.
- **Material changes** (cross corpus class; re-target across lanes;
  promote to `certified_archetype_match`; switch `resolution_mode`)
  require a benchmark-council decision per the charter §3.
- **Never ship** `requires_extra_privacy_review_before_ci` or
  `visibility_class: restricted` fixtures from this directory;
  those require an approved segregation bundle elsewhere.

## Regeneration rules

- Fixtures with `resolution_mode: concrete_file` are their own
  authoritative bytes; regeneration means editing the fixture.
- Fixtures with `resolution_mode: live_repo_slice` are resolved by
  the benchmark harness at run time; the report records the
  repository commit and the resolved file list.
- Fixtures with `resolution_mode: recipe_only` are produced by the
  named generator at run time. The recipe id is the stable identity;
  the emitted bytes are derivable.

The corpus manifest carries a `schema_version` and a
`manifest_revision`. Any change that alters a vocabulary token,
renames a fixture id, or changes a `resolution_mode` bumps the
manifest revision and cites the decision row.
