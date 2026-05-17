# Beta public benchmark publication packet

This packet is the beta publication rehearsal for public benchmark proof.
It is deliberately methodology-only: the packet publishes corpus,
hardware, threshold, comparability, freshness, and known-limit metadata,
but it does not publish numeric wins or comparable public performance
claims.

Reviewer-facing entrypoints:

- Public benchmark release doc: `docs/release/m3/public_benchmark_beta.md`
- Benchmark council notes: `artifacts/benchmarks/m3/benchmark_council_notes.md`
- Partner packet projection:
  `artifacts/benchmarks/m3/publication_packet/partner_packet.md`
- Validator:
  `ci/check_m3_public_benchmark_beta.py`
- Latest validation capture:
  `artifacts/benchmarks/m3/publication_packet/captures/public_benchmark_beta_validation_capture.json`

## Publication decision

| Lane | Result | Reason |
|---|---|---|
| Benchmark methodology | Publishable | Corpus pins, reference hardware rows, threshold sources, freshness, and rerun triggers are checked in. |
| Public comparison | Blocked | No full-corpus reference capture is current and comparable across the protected beta rows. |
| Partner packet | Methodology-only | Partners can inspect the packet, caveats, and rerun checklist without receiving wider performance wording. |

## Canonical machine source

The block below is the canonical machine truth for the beta packet. The
validator parses this YAML between the sentinel markers and ignores the
surrounding prose.

<!-- BEGIN canonical:public_benchmark_beta_packet -->
```yaml
schema_version: 1
packet_kind: m3_public_benchmark_beta_packet
packet_family: benchmark_publication_pack
packet_id: benchmark_publication.beta.publication_packet
evidence_id: evidence.benchmark_publication.beta.publication_packet
title: Beta public benchmark publication packet
milestone_id: m3
release_channel_scope: beta
as_of: "2026-05-17"
claim_family: benchmark_publication
result_status: methodology_only
visibility_class: public
publication_decision: keep_methodology_only

ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  benchmark_owner: lane:benchmark_lab
  release_owner: lane:release_evidence
  docs_owner: lane:docs_public_truth
  backup_owner: null
  backup_waiver: single-maintainer-backup

coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
    - QE-CORPUS-001
  claim_row_refs:
    - m3_claim_row:canonical.benchmark.publication_truth
  public_proof_index_row_ref: m3_public_proof:benchmark_publication
  covered_lanes:
    - benchmark_publication
    - public_proof
    - docs_known_limits_support

freshness:
  captured_at: "2026-05-17T18:00:00Z"
  stale_after: P14D
  freshness_class: warm_cached
  proof_class_id: benchmark_publication_proof
  stale_propagation_profile: claim_narrow_and_hold
  source_revision: commit:b7ee32adb5eb
  trigger_revision: benchmark_publication.beta.packet@rev1
  freshness_state: current_for_methodology_only

environment:
  channel_context: beta
  deployment_context:
    - desktop_self_managed
  exact_build_identity_ref: artifacts/build/build_identity.json
  workspace_version: "0.0.0"
  environment_summary: >
    Beta benchmark publication rehearsal using the checked-in build
    identity, protected fitness catalog, fixture register, and reference
    hardware rows. The packet is not a comparable public result set.

corpus_pins:
  - fixture_register_row_ref: fixture_register:external_alpha.ts_web_app_reference
    corpus_refs:
      - corpus.reference.ts_web_app_archetype_seed
      - corpus.archetype.ts_web_app_seed
    fixture_packet_ref: fixtures/reference_workspaces/m2/ts_web_app_workflows.yaml
    privacy_decision: admit_public
    repeatability_note: synthetic_descriptor_no_private_bytes
  - fixture_register_row_ref: fixture_register:external_alpha.python_service_data_reference
    corpus_refs:
      - corpus.reference.python_data_app_archetype_seed
      - corpus.archetype.python_data_app_seed
    fixture_packet_ref: fixtures/reference_workspaces/m2/python_service_data_workflows.yaml
    privacy_decision: admit_public
    repeatability_note: synthetic_descriptor_no_private_bytes
  - fixture_register_row_ref: m3_reference_workspace:jvm_service
    corpus_refs:
      - corpus.reference.java_kotlin_service_archetype_seed
      - corpus.archetype.java_kotlin_service_seed
      - corpus.workflow.first_useful_edit_java_kotlin_service
    fixture_packet_ref: fixtures/reference_workspaces/m3/jvm_service/workspace.yaml
    privacy_decision: admit_public
    repeatability_note: synthetic_descriptor_no_private_bytes
  - fixture_register_row_ref: m3_reference_workspace:rust_workspace
    corpus_refs:
      - corpus.reference.small_rust_self_host_slice
      - corpus.workflow.first_useful_edit_rust_self_host
    fixture_packet_ref: fixtures/reference_workspaces/m3/rust_workspace/workspace.yaml
    privacy_decision: admit_public
    repeatability_note: live_repo_slice_records_commit_and_resolved_files
  - fixture_register_row_ref: m3_reference_workspace:go_service
    corpus_refs:
      - corpus.reference.go_service_archetype_seed
      - corpus.archetype.go_service_seed
      - corpus.workflow.first_useful_edit_go_service
    fixture_packet_ref: fixtures/reference_workspaces/m3/go_service/workspace.yaml
    privacy_decision: admit_public
    repeatability_note: synthetic_descriptor_no_private_bytes
  - fixture_register_row_ref: m3_reference_workspace:cpp_native
    corpus_refs:
      - corpus.reference.c_cpp_native_archetype_seed
      - corpus.archetype.c_cpp_native_seed
      - corpus.workflow.first_useful_edit_c_cpp_native
    fixture_packet_ref: fixtures/reference_workspaces/m3/cpp_native/workspace.yaml
    privacy_decision: admit_public
    repeatability_note: synthetic_descriptor_no_private_bytes

reference_hardware_pins:
  manifest_ref: artifacts/perf/reference_hardware_manifest.yaml
  hardware_definition_refs:
    - hardware_definition.ref.macos15.arm64.apple_silicon_14in
    - hardware_definition.ref.windows11.x86_64.thinkpad_t14_gen5
    - hardware_definition.ref.ubuntu24_04.x86_64.framework13
  display_class_refs:
    - display_class.internal_14in_retina_3024x1964_sdr60
    - display_class.internal_14in_1920x1200_sdr60
    - display_class.internal_13_5in_2256x1504_sdr60
  lab_image_manifest_ref: artifacts/perf/lab_image_manifest.yaml
  power_posture_refs:
    - power_posture.ac_balanced
  hardware_freshness_state: current_for_methodology_only

threshold_pins:
  protected_fitness_catalog_ref: artifacts/benchmarks/m3/protected_fitness_catalog.yaml
  dashboard_snapshot_ref: artifacts/benchmarks/m3/dashboard_snapshot.json
  protected_metrics_ref: artifacts/bench/protected_metrics.yaml
  canonical_fitness_catalog_ref: artifacts/bench/fitness_function_catalog.yaml
  threshold_owner_dri: "@ahmeddyounis"
  threshold_decision_forum_ref: forum:performance_council
  threshold_owning_lane: benchmark_lab
  threshold_review_state: provisional_until_reference_capture
  threshold_rows:
    - ff.warm_start_to_first_paint
    - ff.input_to_paint
    - ff.buffer_operations
    - ff.benchmark_lab_health

comparability:
  run_context_class: smoke_subset
  comparability_class: not_yet_comparable
  freshness_state: warm_cached
  comparable_baseline_ref: none
  drift_fields:
    - no_full_corpus_reference_capture
    - threshold_rows_still_provisional
  comparability_note: >
    Readers may compare packet shape, corpus identity, hardware identity,
    threshold sources, and rerun triggers. They may not compare product
    speed, competitor parity, or release-grade performance until a
    current full-corpus reference capture lands on the listed hardware
    rows.

publication_postures:
  - posture_id: methodology_disclosure
    admitted: true
    reason: >
      Corpus, hardware, threshold, freshness, rerun-trigger, and
      known-limit metadata are checked in and public.
  - posture_id: known_limits_disclosure
    admitted: true
    reason: >
      The packet discloses why public comparison and wider benchmark
      copy remain blocked.
  - posture_id: public_head_to_head_comparison
    admitted: false
    reason: >
      No current comparable full-corpus reference capture exists for the
      protected beta rows.
  - posture_id: replacement_or_certified_performance_claim
    admitted: false
    reason: >
      Threshold numerics and reference captures are not yet promoted for
      claim-bearing public copy.
  - posture_id: numeric_performance_win
    admitted: false
    reason: >
      The beta packet carries no numeric result table that can support a
      public win statement.

governed_public_surfaces:
  - surface_ref: docs/release/m3/public_benchmark_beta.md
    surface_kind: release_doc
    required_tokens:
      - benchmark_publication
      - methodology-only
      - not_yet_comparable
      - artifacts/benchmarks/m3/publication_packet/packet.md
      - artifacts/benchmarks/m3/benchmark_council_notes.md
    blocked_phrases:
      - faster than
      - outperforms
      - beats
      - benchmark win
      - certified performance
  - surface_ref: artifacts/benchmarks/m3/publication_packet/partner_packet.md
    surface_kind: partner_packet
    required_tokens:
      - benchmark_publication
      - methodology-only
      - not_yet_comparable
      - artifacts/benchmarks/m3/publication_packet/packet.md
      - artifacts/benchmarks/m3/benchmark_council_notes.md
    blocked_phrases:
      - faster than
      - outperforms
      - beats
      - benchmark win
      - certified performance

artifact_links:
  exact_build_identity_refs:
    - artifacts/build/build_identity.json
  prior_dry_run_refs:
    - artifacts/benchmarks/m3/publication_dry_run/packet.md
    - artifacts/benchmarks/m3/publication_dry_run/captures/publication_dry_run_validation_capture.json
  fixture_register_ref: artifacts/benchmarks/m2_fixture_register.yaml
  source_anchor_refs:
    - artifacts/milestones/m3/public_proof_index.md
    - artifacts/compat/m3/reference_workspace_register.yaml
    - docs/governance/m3/publication_shelf_life_policy.md
    - docs/benchmarks/benchmark_publication_pack_template.md
    - artifacts/benchmarks/m3/benchmark_council_notes.md
  known_limit_refs:
    - known_limit:benchmark_publication.methodology_only
    - known_limit:benchmark_publication.not_yet_comparable
    - known_limit:benchmark_publication.thresholds_provisional

rerun_trigger_refs:
  - reference_hardware_image_changed
  - corpus_or_fixture_revision_changed
  - protected_metrics_or_fitness_catalog_changed
  - exact_build_identity_chain_changed
  - schema_or_packet_header_contract_changed
  - claim_row_or_channel_binding_changed

latest_capture:
  captured_at: "2026-05-17T18:00:00Z"
  command: python3 ci/check_m3_public_benchmark_beta.py --repo-root .
  report_ref: artifacts/benchmarks/m3/publication_packet/captures/public_benchmark_beta_validation_capture.json
```
<!-- END canonical:public_benchmark_beta_packet -->
