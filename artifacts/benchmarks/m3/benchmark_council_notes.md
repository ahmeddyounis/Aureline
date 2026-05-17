# Benchmark council review notes

These notes record the beta review disposition for public benchmark
language. The council accepted methodology disclosure and blocked wider
benchmark claims until the rerun checklist below is complete.

## Summary

| Area | Disposition |
|---|---|
| Methodology, corpus, hardware, thresholds, freshness | Publishable |
| Numeric public claims | Internal only for now |
| Public comparison copy | Rerun required before promotion |

## Canonical machine source

<!-- BEGIN canonical:benchmark_council_review_notes -->
```yaml
schema_version: 1
notes_kind: benchmark_council_review_notes
notes_id: benchmark_council.review.beta_public_benchmark
as_of: "2026-05-17"
packet_ref: artifacts/benchmarks/m3/publication_packet/packet.md
review_decision: keep_methodology_only
review_forum_ref: forum:performance_council
owner_dri: "@ahmeddyounis"

publishable_claims:
  - claim_id: methodology_disclosure
    posture: publishable
    public_text_rule: >
      Public docs and partner packets may say the beta benchmark packet
      publishes methodology, corpus, hardware, threshold, freshness, and
      rerun-trigger metadata.
    evidence_refs:
      - artifacts/benchmarks/m3/publication_packet/packet.md
      - artifacts/benchmarks/m2_fixture_register.yaml
      - artifacts/perf/reference_hardware_manifest.yaml
      - artifacts/bench/protected_metrics.yaml
  - claim_id: known_limits_disclosure
    posture: publishable
    public_text_rule: >
      Public docs and partner packets may say the beta benchmark packet
      is methodology-only and not yet comparable.
    evidence_refs:
      - artifacts/benchmarks/m3/publication_packet/packet.md

internal_only_items:
  - item_id: raw_trace_bodies
    reason: raw traces are not part of the public methodology packet
    allowed_ref_style: stable_internal_ref_or_omission_note
  - item_id: ad_hoc_local_capture_notes
    reason: self-captures are not promoted reference evidence
    allowed_ref_style: stable_internal_ref_or_omission_note
  - item_id: council_thread_discussion
    reason: decision notes are summarized here; discussion remains internal
    allowed_ref_style: stable_internal_ref_or_omission_note

blocked_public_claims:
  - claim_id: public_comparison_claim
    reason: no current full-corpus comparable reference capture exists
    required_to_unblock:
      - full_reference_capture_on_all_listed_hardware_rows
      - public_comparison_review_packet_signed
      - docs_and_partner_copy_gate_refreshed
  - claim_id: numeric_performance_claim
    reason: no publishable numeric result table is admitted by the packet
    required_to_unblock:
      - signed_result_table
      - threshold_rows_promoted_from_provisional
      - benchmark_publication_capture_refreshed
  - claim_id: certified_performance_claim
    reason: beta packet does not certify launch-archetype performance
    required_to_unblock:
      - certified_archetype_scorecards_current
      - benchmark_packet_links_scorecards
      - public_proof_index_capture_refreshed

rerun_before_promotion:
  - rerun_id: full_corpus_reference_capture
    owner_lane: benchmark_lab
    required_refs:
      - artifacts/benchmarks/m2_fixture_register.yaml
      - artifacts/perf/reference_hardware_manifest.yaml
      - artifacts/bench/protected_metrics.yaml
    completion_signal: signed_capture_with_all_governed_hardware_rows
  - rerun_id: threshold_calibration_review
    owner_lane: benchmark_lab
    required_refs:
      - artifacts/bench/protected_metrics.yaml
      - artifacts/bench/fitness_function_catalog.yaml
    completion_signal: thresholds_no_longer_to_be_set_by_benchmark_council
  - rerun_id: public_copy_gate_refresh
    owner_lane: docs_public_truth
    required_refs:
      - docs/release/m3/public_benchmark_beta.md
      - artifacts/benchmarks/m3/publication_packet/partner_packet.md
      - ci/check_m3_public_benchmark_beta.py
    completion_signal: validation_capture_passes_after_packet_promotion

signoff:
  benchmark_owner: approved_methodology_only
  release_owner: approved_narrow_publication
  docs_owner: approved_copy_gate
  support_owner: informed_no_support_claim_change
```
<!-- END canonical:benchmark_council_review_notes -->
