# M3 benchmark publication dry-run packet

This packet exercises the external benchmark-publication path before the
broader beta train widens claims. It is a methodology + freshness dry run:
the publishable packet header, corpus pins, reference-hardware pins,
threshold owners, and freshness/rerun-trigger metadata are all
inspectable, but no public head-to-head comparison or certified-archetype
wording is admitted by this dry run.

Reviewer-facing entrypoints:

- Public-proof index row: `m3_public_proof:benchmark_publication`
  (`artifacts/milestones/m3/public_proof_index.md`)
- Publication shelf-life policy:
  `docs/governance/m3/publication_shelf_life_policy.md`
- Protected-fitness catalog (M3):
  `artifacts/benchmarks/m3/protected_fitness_catalog.yaml`
- Dashboard snapshot:
  `artifacts/benchmarks/m3/dashboard_snapshot.json`
- Canonical fixture register:
  `artifacts/benchmarks/m2_fixture_register.yaml`
- Reference hardware manifest:
  `artifacts/perf/reference_hardware_manifest.yaml`
- Validator: `ci/check_m3_publication_rehearsal.py`
- Latest validation capture:
  `artifacts/benchmarks/m3/publication_dry_run/captures/publication_dry_run_validation_capture.json`

## Decision

| Lane | Result | Reason |
|---|---|---|
| `benchmark` | `keep_methodology_only` | Corpus, hardware, and threshold metadata are pinned and inspectable; no materialized full-corpus comparable run exists, so the dry run carries the methodology and known-limits set rather than a public comparison. |
| `public_proof` | `keep_methodology_only` | The publishable packet header parses, freshness windows match the proof-class ceiling, and rerun triggers are wired; the protected-fitness dashboard tiles remain `evidence_stale` until warm-start, input-to-paint, rollback, and benchmark-lab self-audit captures land. |
| `docs_known_limits_support` | `narrow_claim_before_publish` | Partner, docs, and support readers can follow the packet via `proof_consumption_walkthrough.md`; raw artifact export remains narrowed to internal review until the captures are real bytes. |

## Acceptance evidence

The dry run is acceptable as a controlled beta rehearsal only while these
statements remain true:

- The publishable header parses against
  `schemas/governance/evidence_packet_header.schema.json`.
- The corpus, fixture, and reference-hardware ids in the canonical block
  resolve to checked-in artifacts.
- The freshness window MUST NOT exceed the `benchmark_publication_proof`
  ceiling in `artifacts/governance/evidence_freshness_slos.yaml`.
- Every rerun trigger named here exists in
  `artifacts/governance/evidence_rerun_triggers.yaml`.
- The packet is tied to the same `artifacts/build/build_identity.json`
  the M3 claim manifest cites.
- The packet carries an explicit known-limits set so no public
  head-to-head, certified-archetype, or replacement-grade wording is
  smuggled in.

## Known limits and exclusions

This rehearsal attaches the rehearsal-specific limits:

- `known_limit:m3.benchmark_publication.dry_run_methodology_only`
- `known_limit:m3.benchmark_publication.no_public_head_to_head_comparison`
- `known_limit:m3.benchmark_publication.dashboard_tiles_evidence_stale`

No marketed comparison claim is admitted by this dry run.

## Refresh trigger

Refresh the packet when any of these change:

- `artifacts/bench/fitness_function_catalog.yaml`
- `artifacts/benchmarks/m3/protected_fitness_catalog.yaml`
- `artifacts/benchmarks/m3/dashboard_snapshot.json`
- `artifacts/perf/reference_hardware_manifest.yaml`
- `artifacts/benchmarks/m2_fixture_register.yaml`
- `artifacts/build/build_identity.json`
- `artifacts/release/m3/claim_manifest.json`

## Failure drill

To confirm the guardrail is live:

1. Temporarily rename one current-output ref in the canonical block
   (e.g. the dashboard snapshot) or set `freshness.stale_after` wider
   than the proof-class ceiling.
2. Re-run `python3 ci/check_m3_publication_rehearsal.py --repo-root .`;
   the validator MUST fail with an actionable error.
3. Restore the value and re-run; it MUST pass.

## Canonical machine source

The block below is the canonical machine truth for this dry-run packet.
The validator parses YAML between the sentinel markers and ignores the
surrounding prose.

<!-- BEGIN canonical:benchmark_publication_dry_run -->
```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: benchmark_publication_pack
packet_id: m3_benchmark_publication.dry_run.first
evidence_id: evidence.m3.benchmark_publication.dry_run.first
title: M3 benchmark publication dry run — methodology-only
milestone_id: m3
release_channel_scope: beta
as_of: "2026-05-15"
packet_state: methodology_only

ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup

coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
  claim_row_refs:
    - m3_claim_row:canonical.benchmark.publication_truth
  covered_lanes:
    - m3_public_proof
    - benchmark_publication
  public_proof_index_row_ref: m3_public_proof:benchmark_publication

result_status: methodology_only
visibility_class: public

freshness:
  captured_at: "2026-05-15T21:35:59Z"
  stale_after: P14D
  freshness_class: warm_cached
  proof_class_id: benchmark_publication_proof
  stale_propagation_profile: claim_narrow_and_hold
  source_revision: commit:b7ee32adb5eb
  trigger_revision: m3_benchmark_publication.dry_run@rev1
  comparability_note: >
    No public head-to-head comparison admitted. Methodology and
    freshness inputs pinned; protected-fitness dashboard tiles render
    evidence_stale until warm-start, input-to-paint, rollback, and
    benchmark-lab self-audit captures land on the reference hardware
    matrix.

environment:
  channel_context: beta
  deployment_context:
    - desktop_self_managed
  environment_summary: >
    M3 beta dry run for the external benchmark publication path. Captured
    against the seed build identity, the reference-hardware manifest,
    the canonical fixture register, and the M3 protected-fitness catalog
    on the date above.

corpus_pins:
  - fixture_register_row_ref: fixture_register:external_alpha.ts_web_app_reference
    corpus_refs:
      - corpus.reference.ts_web_app_archetype_seed
      - corpus.archetype.ts_web_app_seed
    fixture_packet_ref: fixtures/reference_workspaces/m2/ts_web_app_workflows.yaml
  - fixture_register_row_ref: fixture_register:external_alpha.python_service_data_reference
    corpus_refs:
      - corpus.reference.python_data_app_archetype_seed
      - corpus.archetype.python_data_app_seed
    fixture_packet_ref: fixtures/reference_workspaces/m2/python_service_data_workflows.yaml

reference_hardware_pins:
  manifest_ref: artifacts/perf/reference_hardware_manifest.yaml
  display_class_refs:
    - display_class.internal_14in_retina_3024x1964_sdr60
  lab_image_manifest_ref: artifacts/perf/lab_image_manifest.yaml

threshold_pins:
  protected_fitness_catalog_ref: artifacts/benchmarks/m3/protected_fitness_catalog.yaml
  dashboard_snapshot_ref: artifacts/benchmarks/m3/dashboard_snapshot.json
  protected_metrics_ref: artifacts/bench/protected_metrics.yaml
  canonical_fitness_catalog_ref: artifacts/bench/fitness_function_catalog.yaml
  threshold_owner_dri: "@ahmeddyounis"
  threshold_decision_forum_ref: forum:performance_council
  threshold_owning_lane: benchmark_lab

artifact_links:
  exact_build_identity_refs:
    - artifacts/build/build_identity.json
  fixture_register_ref: artifacts/benchmarks/m2_fixture_register.yaml
  source_anchor_refs:
    - artifacts/milestones/m3/public_proof_index.md
    - docs/governance/m3/publication_shelf_life_policy.md
    - artifacts/benchmarks/m3/publication_dry_run/packet.md
  waiver_refs:
    - waiver:single_maintainer_backup
  known_limit_refs:
    - known_limit:m3.benchmark_publication.dry_run_methodology_only
    - known_limit:m3.benchmark_publication.no_public_head_to_head_comparison
    - known_limit:m3.benchmark_publication.dashboard_tiles_evidence_stale
  migration_packet_refs: []

rerun_trigger_refs:
  - reference_hardware_image_changed
  - corpus_or_fixture_revision_changed
  - protected_metrics_or_fitness_catalog_changed
  - exact_build_identity_chain_changed
  - schema_or_packet_header_contract_changed
  - claim_row_or_channel_binding_changed

publication_postures:
  - posture_id: public_head_to_head_comparison
    admitted: false
    reason: >
      The packet is methodology-only; no comparable competitor or
      replacement-grade wording is published.
  - posture_id: certified_archetype_marketing
    admitted: false
    reason: >
      Archetype scorecards downgrade to limited/experimental in the
      claim manifest; this dry run cites them by id and does not
      restate certified wording.
  - posture_id: methodology_disclosure
    admitted: true
    reason: >
      Corpus pins, fixture register rows, reference-hardware ids,
      threshold owners, and freshness/rerun-trigger metadata are
      checked-in and can be quoted publicly.
  - posture_id: known_limits_disclosure
    admitted: true
    reason: >
      The rehearsal-specific known limits are checked in and quotable.

consuming_surfaces:
  - artifacts/milestones/m3/proof_consumption_walkthrough.md
  - artifacts/milestones/m3/public_proof_index.md
  - docs/governance/m3/publication_shelf_life_policy.md

latest_capture:
  captured_at: "2026-05-15T21:35:59Z"
  command: python3 ci/check_m3_publication_rehearsal.py --repo-root .
  report_ref: artifacts/benchmarks/m3/publication_dry_run/captures/publication_dry_run_validation_capture.json
```
<!-- END canonical:benchmark_publication_dry_run -->
