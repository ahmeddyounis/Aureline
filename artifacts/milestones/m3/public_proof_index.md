# M3 public-proof artifact index

Canonical machine source for the M3 public-proof lanes. Every beta-bearing
claim family in the M3 claim manifest (`artifacts/release/m3/claim_manifest.json`)
has exactly one row here, naming the canonical packet, the current
machine-readable outputs the family points to, the freshness window before
the packet stops being claim-bearing, and the named rerun triggers that
expire the packet immediately even when the time window remains open.

Downstream M3 packets, dashboards, docs, support exports, and review
checklists consume this index by id; they do not restate proof links or
freshness rules locally.

- Reviewer-facing entrypoint: this file
- Validator: `ci/check_m3_public_proof_index.py`
- Latest validation capture:
  `artifacts/milestones/m3/captures/public_proof_index_validation_capture.json`
- Review-packet template: `artifacts/milestones/m3/review_packet_template.md`
- Publication shelf-life policy: `docs/governance/m3/publication_shelf_life_policy.md`

The canonical truth for downstream tools lives in the
`canonical:public_proof_index` block at the bottom of this file (delimited
by HTML comment sentinels). Hand-edit the YAML in that block, refresh the
prose tables in the same change set, then run the validator. Tooling MUST
parse the block between the sentinel markers; tooling MUST NOT parse the
surrounding prose.

## Storage conventions

Public-proof evidence lives under these governed roots:

- Beta-scope packets and captures: `artifacts/milestones/m3/`
- Compatibility-report outputs and captures: `artifacts/compat/m3/`
- Benchmark / protected-fitness outputs and captures:
  `artifacts/benchmarks/m3/`
- Docs / public-truth outputs and captures: `artifacts/docs/m3/`
- Release / claim manifest outputs and captures: `artifacts/release/m3/`
- Shared build-identity anchor: `artifacts/build/build_identity.json`
- Corpus lineage registry: `fixtures/registry/`
- Corpus freshness report: `artifacts/registry/`

Public-proof evidence for an M3 row MUST be stored under one of these roots.
New roots MUST be declared in the canonical block before any packet writes
into them.

## Claim families covered

| Claim family | Canonical packet | Owner | Freshness window | Stale-propagation profile |
|---|---|---|---|---|
| `boundary_truth` | `artifacts/release/m3/claim_manifest.md` (boundary rows) | @ahmeddyounis | P14D | `docs_truth_stale` |
| `exact_build_identity` | `artifacts/release/m3/claim_manifest.md` (build-identity rows) | @ahmeddyounis | P14D | `claim_narrow_and_hold` |
| `benchmark_publication` | `artifacts/benchmarks/m3/publication_packet/packet.md` | @ahmeddyounis | P14D | `claim_narrow_and_hold` |
| `docs_freshness` | `artifacts/docs/m3/docs_truth_report.md` | @ahmeddyounis | P14D | `docs_truth_stale` |
| `version_skew_truth` | `artifacts/compat/m3/compatibility_report.md` | @ahmeddyounis | P14D | `compatibility_retest_pending` |
| `launch_wedge` | `artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml` | @ahmeddyounis | P14D | `compatibility_retest_pending` |

Every row above is bound to one canonical packet, one current output set,
one freshness window, and one stale-propagation profile in the canonical
block below. The validator fails closed when any row drifts.

## Acceptance evidence

The validator (`ci/check_m3_public_proof_index.py`) enforces:

- the canonical block parses and carries the required header fields;
- every M3 claim family present in
  `artifacts/release/m3/claim_manifest.json#/rows[*].claim_family` is
  bound to exactly one row in this index;
- every row names one canonical packet, at least one current output, one
  proof-class id from
  `artifacts/governance/evidence_freshness_slos.yaml#/proof_classes`,
  one freshness window no wider than that class ceiling, and at least one
  rerun-trigger id from
  `artifacts/governance/evidence_rerun_triggers.yaml#/trigger_rows`;
- every named packet, output, capture, and capture-report ref exists on
  disk under a declared storage root;
- every row names an `owner_dri`, a `signoff_packet_ref`
  (pointing at the review-packet template or a filled instance), and a
  `latest_capture` block when the row is not declared planned-only;
- a derived downgrade matrix is written into the validation capture so
  downstream surfaces can downgrade stale rows automatically without
  re-reading prose.

## How to refresh

1. Edit the canonical YAML block below.
2. Refresh the prose tables in the same change set.
3. Run `python3 ci/check_m3_public_proof_index.py --repo-root .` and
   commit the regenerated capture.
4. If a new claim family lands in `artifacts/release/m3/claim_manifest.json`,
   add a matching row here in the same change set; the validator fails
   closed when a beta-bearing family has no public-proof row.

## Failure drill

To confirm the guardrail is live:

1. Temporarily remove one current-output ref from a row in the canonical
   block.
2. Re-run the validator; it must fail with an actionable error naming
   the missing artifact.
3. Restore the ref and re-run; it must pass.

## Canonical machine source

The block below is the canonical machine truth for this index. The
validator parses YAML between the sentinel markers and ignores the
surrounding prose. Do not split this block, do not embed nested code
fences, and do not delete the sentinel markers.

<!-- BEGIN canonical:public_proof_index -->
```yaml
schema_version: 1
index_id: m3_public_proof_artifact_index
milestone_id: m3
release_channel_scope: beta
as_of: "2026-05-15"
owner: "@ahmeddyounis"
backup_owner: null
backup_waiver: single-maintainer-backup
human_entrypoint_ref: artifacts/milestones/m3/public_proof_index.md
review_packet_template_ref: artifacts/milestones/m3/review_packet_template.md
publication_shelf_life_policy_ref: docs/governance/m3/publication_shelf_life_policy.md
validator_ref: ci/check_m3_public_proof_index.py
validation_capture_ref: artifacts/milestones/m3/captures/public_proof_index_validation_capture.json
claim_manifest_source: artifacts/release/m3/claim_manifest.json
slo_catalog_source: artifacts/governance/evidence_freshness_slos.yaml
rerun_trigger_catalog_source: artifacts/governance/evidence_rerun_triggers.yaml

storage_roots:
  - root_id: m3_milestone_root
    root_ref: artifacts/milestones/m3/
  - root_id: m3_compat_root
    root_ref: artifacts/compat/m3/
  - root_id: m3_benchmarks_root
    root_ref: artifacts/benchmarks/m3/
  - root_id: m3_docs_root
    root_ref: artifacts/docs/m3/
  - root_id: m3_release_root
    root_ref: artifacts/release/m3/
  - root_id: build_identity_root
    root_ref: artifacts/build/
  - root_id: corpus_registry_root
    root_ref: fixtures/registry/
  - root_id: corpus_freshness_root
    root_ref: artifacts/registry/

rows:
  - row_id: m3_public_proof:boundary_truth
    claim_family: boundary_truth
    title: M3 boundary-truth public-proof lane
    owner_dri: "@ahmeddyounis"
    canonical_packet_ref: artifacts/release/m3/claim_manifest.md
    signoff_packet_ref: artifacts/milestones/m3/review_packet_template.md
    proof_class_id: docs_claim_truth_proof
    visibility_class: public
    freshness:
      stale_after: P14D
      freshness_class: warm_cached
      stale_propagation_profile: docs_truth_stale
    current_outputs:
      - artifacts/release/m3/claim_manifest.json
      - artifacts/release/m3/claim_manifest.md
      - artifacts/milestones/m3/claimed_surface_register.json
      - artifacts/milestones/m3/cohort_guardrails.yaml
    supporting_evidence_refs:
      - docs/milestones/m3/beta_admission_matrix.md
      - docs/product/boundary_manifest_strawman.md
    exact_build_identity_ref: artifacts/build/build_identity.json
    rerun_trigger_refs:
      - claim_row_or_channel_binding_changed
      - schema_or_packet_header_contract_changed
      - deployment_topology_or_boundary_changed
      - docs_truth_contract_or_pack_revision_changed
    latest_capture:
      captured_at: "2026-05-15T20:39:50Z"
      command: python3 ci/check_m3_claim_manifest.py --repo-root .
      report_ref: artifacts/release/m3/captures/claim_manifest_validation_capture.json

  - row_id: m3_public_proof:exact_build_identity
    claim_family: exact_build_identity
    title: M3 exact-build identity public-proof lane
    owner_dri: "@ahmeddyounis"
    canonical_packet_ref: artifacts/release/m3/claim_manifest.md
    signoff_packet_ref: artifacts/milestones/m3/review_packet_template.md
    proof_class_id: docs_claim_truth_proof
    visibility_class: public
    freshness:
      stale_after: P14D
      freshness_class: warm_cached
      stale_propagation_profile: claim_narrow_and_hold
    current_outputs:
      - artifacts/release/m3/claim_manifest.json
      - artifacts/release/m3/claim_manifest.md
      - artifacts/build/build_identity.json
    supporting_evidence_refs:
      - docs/build/exact_build_identity_model.md
      - docs/release/build_identity_baseline.md
      - artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md
      - artifacts/milestones/m3/proof_consumption_walkthrough.md
    exact_build_identity_ref: artifacts/build/build_identity.json
    rerun_trigger_refs:
      - exact_build_identity_chain_changed
      - claim_row_or_channel_binding_changed
      - schema_or_packet_header_contract_changed
    latest_capture:
      captured_at: "2026-05-15T20:39:50Z"
      command: python3 ci/check_m3_claim_manifest.py --repo-root .
      report_ref: artifacts/release/m3/captures/claim_manifest_validation_capture.json

  - row_id: m3_public_proof:benchmark_publication
    claim_family: benchmark_publication
    title: M3 protected-fitness benchmark-publication lane
    owner_dri: "@ahmeddyounis"
    canonical_packet_ref: artifacts/benchmarks/m3/publication_packet/packet.md
    signoff_packet_ref: artifacts/milestones/m3/review_packet_template.md
    proof_class_id: benchmark_publication_proof
    visibility_class: public
    freshness:
      stale_after: P14D
      freshness_class: warm_cached
      stale_propagation_profile: claim_narrow_and_hold
    current_outputs:
      - artifacts/benchmarks/m3/publication_packet/packet.md
      - artifacts/benchmarks/m3/publication_packet/partner_packet.md
      - artifacts/benchmarks/m3/publication_packet/captures/public_benchmark_beta_validation_capture.json
      - artifacts/benchmarks/m3/benchmark_council_notes.md
      - artifacts/benchmarks/m3/dashboard_snapshot.json
      - artifacts/benchmarks/m3/protected_fitness_catalog.yaml
      - artifacts/milestones/m3/waiver_register.yaml
      - artifacts/registry/corpus_freshness_report.json
    supporting_evidence_refs:
      - docs/milestones/m3/protected_fitness_catalog.md
      - artifacts/benchmarks/m3/publication_dry_run/packet.md
      - docs/release/m3/public_benchmark_beta.md
      - ci/check_m3_public_benchmark_beta.py
      - fixtures/registry/corpus_registry.yaml
      - docs/release/m3/corpus_lineage_and_public_proof.md
      - artifacts/milestones/m3/proof_consumption_walkthrough.md
    exact_build_identity_ref: artifacts/build/build_identity.json
    rerun_trigger_refs:
      - reference_hardware_image_changed
      - corpus_or_fixture_revision_changed
      - protected_metrics_or_fitness_catalog_changed
      - exact_build_identity_chain_changed
    latest_capture:
      captured_at: "2026-05-17T18:00:00Z"
      command: python3 ci/check_m3_public_benchmark_beta.py --repo-root .
      report_ref: artifacts/benchmarks/m3/publication_packet/captures/public_benchmark_beta_validation_capture.json

  - row_id: m3_public_proof:docs_freshness
    claim_family: docs_freshness
    title: M3 docs / public-truth freshness lane
    owner_dri: "@ahmeddyounis"
    canonical_packet_ref: artifacts/docs/m3/docs_truth_report.md
    signoff_packet_ref: artifacts/milestones/m3/review_packet_template.md
    proof_class_id: docs_claim_truth_proof
    visibility_class: public
    freshness:
      stale_after: P14D
      freshness_class: warm_cached
      stale_propagation_profile: docs_truth_stale
    current_outputs:
      - artifacts/docs/m3/docs_truth_report.md
      - artifacts/docs/m3/public_proof_parity_report.md
      - artifacts/docs/m3/captures/m3_docs_freshness_validation_capture.json
      - artifacts/docs/m3/captures/m3_stale_example_validation_capture.json
      - artifacts/docs/m3/captures/m3_docs_public_proof_parity_capture.json
      - artifacts/release/m3/release_notes_draft.md
    supporting_evidence_refs:
      - artifacts/ci/m3_docs_truth_source_map.yaml
      - tools/ci/m3/docs_freshness_gate.py
      - tools/ci/m3/stale_example_checker.py
      - tools/ci/m3/docs_public_proof_gate/
      - docs/governance/m3/stale_example_policy.md
    exact_build_identity_ref: artifacts/build/build_identity.json
    rerun_trigger_refs:
      - docs_truth_contract_or_pack_revision_changed
      - claim_row_or_channel_binding_changed
      - schema_or_packet_header_contract_changed
      - interface_or_version_skew_window_changed
    latest_capture:
      captured_at: "2026-05-15T20:39:50Z"
      command: bash ci/check_m3_docs_truth.sh
      report_ref: artifacts/docs/m3/captures/m3_docs_public_proof_parity_capture.json

  - row_id: m3_public_proof:version_skew_truth
    claim_family: version_skew_truth
    title: M3 version-skew and compatibility-report lane
    owner_dri: "@ahmeddyounis"
    canonical_packet_ref: artifacts/compat/m3/compatibility_report.md
    signoff_packet_ref: artifacts/milestones/m3/review_packet_template.md
    proof_class_id: compatibility_report_proof
    visibility_class: public
    freshness:
      stale_after: P14D
      freshness_class: warm_cached
      stale_propagation_profile: compatibility_retest_pending
    current_outputs:
      - artifacts/compat/m3/compatibility_report.json
      - artifacts/compat/m3/compatibility_report.md
      - artifacts/compat/m3/skew_window_matrix.yaml
      - artifacts/registry/corpus_freshness_report.json
    supporting_evidence_refs:
      - artifacts/compat/version_skew_register.yaml
      - docs/release/compatibility_report_template.md
      - fixtures/registry/corpus_registry.yaml
    exact_build_identity_ref: artifacts/build/build_identity.json
    rerun_trigger_refs:
      - interface_or_version_skew_window_changed
      - schema_or_packet_header_contract_changed
      - claim_row_or_channel_binding_changed
      - support_window_or_release_family_changed
    latest_capture:
      captured_at: "2026-05-15T20:39:50Z"
      command: python3 ci/check_m3_compatibility_report.py --repo-root .
      report_ref: artifacts/compat/m3/captures/compatibility_report_validation_capture.json

  - row_id: m3_public_proof:launch_wedge
    claim_family: launch_wedge
    title: M3 certified-archetype launch-wedge publication lane
    owner_dri: "@ahmeddyounis"
    canonical_packet_ref: artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml
    signoff_packet_ref: artifacts/milestones/m3/review_packet_template.md
    proof_class_id: compatibility_report_proof
    visibility_class: public
    freshness:
      stale_after: P14D
      freshness_class: warm_cached
      stale_propagation_profile: compatibility_retest_pending
    current_outputs:
      - artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml
      - artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json
      - artifacts/milestones/m3/cohorts/scorecard_index.yaml
      - artifacts/registry/corpus_freshness_report.json
    supporting_evidence_refs:
      - docs/release/certified_archetype_report_template.md
      - artifacts/compat/reference_workspace_rows.yaml
      - fixtures/registry/corpus_registry.yaml
    exact_build_identity_ref: artifacts/build/build_identity.json
    rerun_trigger_refs:
      - reference_hardware_image_changed
      - corpus_or_fixture_revision_changed
      - claim_row_or_channel_binding_changed
      - support_window_or_release_family_changed
    latest_capture:
      captured_at: "2026-05-15T20:39:50Z"
      command: python3 ci/check_cohort_archetype_scorecards.py --repo-root .
      report_ref: artifacts/milestones/m3/captures/cohort_archetype_scorecard_validation_capture.json

required_claim_families:
  - boundary_truth
  - exact_build_identity
  - benchmark_publication
  - docs_freshness
  - version_skew_truth
  - launch_wedge
```
<!-- END canonical:public_proof_index -->
