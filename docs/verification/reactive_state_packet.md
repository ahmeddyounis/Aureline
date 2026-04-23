# Reactive-state verification seed

This packet freezes the reviewer-facing truth model for reactive-state
parity, stale or partial labeling, replay/import posture, backpressure
or debounce disclosure, invalidation-order audits, and cross-surface
query-family identifiers.

If this packet, the parity manifest, the query-family examples, the
invalidation-order audits, and
[`ADR 0005`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
disagree, the ADR and the machine-readable artifacts win; this document
must be updated in the same change.

Companion artifacts:

- [`/fixtures/state/snapshot_delta_parity_manifest.yaml`](../../fixtures/state/snapshot_delta_parity_manifest.yaml)
  — seed parity corpus proving one derived diagnostics summary reaches
  the same materialized view through snapshot and delta replay.
- [`/artifacts/state/query_family_examples/`](../../artifacts/state/query_family_examples/)
  — reviewer-friendly example identifiers and cross-surface bindings for
  workspace truth, graph/replay/import, and provider overlay families.
- [`/artifacts/state/invalidation_order_trace_examples/`](../../artifacts/state/invalidation_order_trace_examples/)
  — condensed order audits extracted from the reactive-state scenario
  table.
- [`/fixtures/state/envelope_examples/`](../../fixtures/state/envelope_examples/)
  — boundary-schema-valid single-frame examples pinned to the frozen
  subscription envelope.
- [`/artifacts/state/invalidation_trace_examples/`](../../artifacts/state/invalidation_trace_examples/)
  — full scenario traces emitted by the prototype harness.
- [`/docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
  — canonical envelope, lifecycle, freshness, completeness, stale
  reason, and view-class contract.
- [`/docs/ux/live_update_review_contract.md`](../ux/live_update_review_contract.md)
  — shared UI copy and review posture for live, buffered, stale, and
  snapshot-review surfaces.
- [`/docs/runtime/resource_governor_contract.md`](../runtime/resource_governor_contract.md)
  — coalescing, queue-collapse, and overloaded/partial disclosure rules.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — `ARCH-STATE-012`, `FIT-STATE-001`, Section 12.3.1, Section 27.22,
  and Appendix DB.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — the reactive-state matrix, watch/debounce disclosure posture, and
  resource-governor queue-collapse rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — typed subscription/backpressure capture and visible coalescing or
  stale-state requirements.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  — controlled `warming`, `cached`, `partial`, `stale`, `rebuilding`,
  and unavailable wording.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.reactive_state.shared_truth_seed
evidence_id: evidence.state.reactive_state_shared_truth_seed
title: Reactive-state verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - ARCH-STATE-012
    - FIT-STATE-001
  claim_row_refs: []
  covered_lanes:
    - governance_packets
    - support_export
    - benchmark_packets
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: commit:working_tree
  trigger_revision: reactive_state_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen reactive-state envelope, the parity corpus,
    the query-family examples, and the invalidation-order audits. No
    product-wide rollout or performance claim is made yet.
artifact_links:
  supporting_evidence_ids:
    - evidence.state.snapshot_delta_parity_corpus
    - evidence.state.query_family_examples
    - evidence.state.invalidation_order_audits
    - evidence.state.subscription_envelope_examples
    - evidence.state.subscription_trace_examples
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/state/snapshot_delta_parity_manifest.yaml
    - fixtures/state/envelope_examples/
  archetype_refs: []
  source_anchor_refs:
    - docs/adr/0005-subscription-envelope-and-invalidation-semantics.md
    - docs/ux/live_update_review_contract.md
    - docs/runtime/resource_governor_contract.md
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes one shared verification story for reactive
state: what parity means, which labels are allowed when truth narrows,
how order drift is detected, and which query-family ids support export,
benchmark packets, and UI copy all cite.

It does not claim that every product surface has migrated to the shared
store. It claims only that the core evidence shape now exists and that
later shell/editor/search/graph/review/CLI surfaces can join one packet
instead of inventing local truth vocabularies.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:reactive_state.snapshot_delta_parity` | `ARCH-STATE-012`, `FIT-STATE-001` | `seed_only` | `internal` | `evidence.state.snapshot_delta_parity_corpus` | Freezes one reviewer-facing parity corpus for a derived/materialized view. |
| `packet_row:reactive_state.truth_labels` | `ARCH-STATE-012` | `seed_only` | `internal` | `evidence.state.subscription_envelope_examples`, `evidence.state.subscription_trace_examples` | Makes stale, partial, replayed, imported, and failed-refresh states explicit enough for UI copy and support export. |
| `packet_row:reactive_state.backpressure_and_debounce` | `ARCH-STATE-012`, `FIT-STATE-001` | `seed_only` | `internal` | `evidence.state.invalidation_order_audits`, `evidence.state.subscription_trace_examples` | Coalescing and snapshot-required transitions are now reviewable rather than anecdotal. |
| `packet_row:reactive_state.invalidation_order` | `FIT-STATE-001` | `seed_only` | `internal` | `evidence.state.invalidation_order_audits` | Order drift can be detected from artifacts without screenshot archaeology. |
| `packet_row:reactive_state.query_family_identifiers` | `ARCH-STATE-012` | `seed_only` | `internal` | `evidence.state.query_family_examples` | Cross-surface ids are controlled across shell, graph, review, CLI, support export, and benchmark tags. |

## What this seed freezes

- One parity corpus proving that the same `language.diagnostics`
  materialized view reaches identical summaries through snapshot replay
  and delta replay.
- One closed packet-level labeling table for `stale`, `partial`,
  `replayed`, `imported`, and `failed refresh` conditions.
- One reviewer-facing projection from wire-level `backpressure_mode`
  plus coalesce/switch notes to visible labels.
- One condensed invalidation-order audit format pinned to the existing
  reactive-state scenario table.
- One cross-surface query-family example set so support export,
  benchmark tagging, and UI chrome reuse the same ids and do not fork by
  feature.

## Packet field set

Use these packet-level fields whenever a verification artifact, support
bundle, or benchmark packet needs to explain reactive-state truth:

| Packet field | Meaning | Source |
|---|---|---|
| `query_family` | Canonical subscription/view family identifier | envelope `query_family` |
| `scope_ref` | Typed scope that the view or audit speaks for | envelope `scope_ref` |
| `snapshot_epoch` / `delta_seq` | Current lineage position | envelope `snapshot_epoch` / `delta_seq` |
| `freshness` | Exact freshness posture | envelope `freshness` |
| `completeness` | Exact completeness posture | envelope `completeness` |
| `backpressure_mode` | Wire-level continuity posture | envelope `backpressure_mode` |
| `view_class` | Persistence and replay posture | envelope `view_class` |
| `stale_reason` | Why continuity or authority narrowed | `invalidation.stale_reason` |
| `terminal_reason` | Why refresh failed terminally, if applicable | envelope `terminal_reason` |
| `producer_refs` | Provenance and derivation lineage | envelope `producer_refs` |
| `order_audit_ref` | Optional audit file proving required ordering | `artifacts/state/invalidation_order_trace_examples/*.json` |
| `parity_case_ref` | Optional parity case proving snapshot/delta equivalence | `fixtures/state/snapshot_delta_parity_manifest.yaml` |

Rules:

1. Packet fields are a projection, not a second envelope schema.
2. `query_family`, `scope_ref`, `snapshot_epoch`, and `delta_seq` are
   the minimum join keys for UI copy, support export, and benchmark
   tagging.
3. A later packet may add optional fields, but it may not redefine the
   meaning of the fields above.

## State-label projections

| Packet condition | Required source signal | Required UI/support meaning | Recovery expectation | Suggested benchmark/support tag |
|---|---|---|---|---|
| `stale_view` | `frame_class = resync_required` or `freshness = stale` | Exact truth was lost; mutating affordances requiring current state stay downgraded | Await replacement snapshot on a new `snapshot_epoch` | `state.stale.<stale_reason>` |
| `partial_data` | `completeness = partial` | Some in-scope data is intentionally missing or still loading | Keep omissions explicit; widen only via later delta/snapshot | `state.partial` |
| `replay_review` | `freshness = replayed` and `stale_reason = replayed_from_bundle` | Reviewer is looking at replayed history, not live authority | Keep the replay separate from the live epoch; do not advance live truth | `state.replayed` |
| `imported_parallel` | `freshness = imported` and `stale_reason = imported_from_external` | Imported or mirrored lineage is visible, but never canonical | Keep imported data read-only and provenance-labeled | `state.imported` |
| `failed_refresh` | `frame_class = terminal`, `terminal_reason = unavailable`, plus a stale reason | Refresh ended in an unavailable state; last-known-good may remain visible but not current | Preserve last-known-safe view, require repair/reconnect before exact operations resume | `state.refresh_failed` |

When a live-review surface projects these states through
[`live_update_review_contract.md`](../ux/live_update_review_contract.md),
the envelope remains the source of truth:

- `stale_view` projects to delivery state `stale`;
- `replay_review` projects to delivery state `snapshot` plus
  review-control state `snapshot_review`;
- `partial_data` keeps `partial` visible instead of collapsing to blank
  or spinner-only UI.

## Backpressure and debounce labels

This packet does not mint a second wire field for debounce. It projects
visible labels from `backpressure_mode` plus explicit coalesce or switch
notes:

| Packet label | Required source | Meaning |
|---|---|---|
| `realtime_exact` | `backpressure_mode = realtime` and no coalesce note | Delta parity is still intact; ordinary exact-state affordances remain legal. |
| `coalesced_visible` | `backpressure_mode = coalesced` or `subscription_backpressure_coalesce` note present | Producer collapsed bursty updates; the collapse count and reason stay reviewable. |
| `snapshot_required_resync` | `backpressure_mode = snapshot_required` or `subscription_snapshot_required_switch` note present | Delta continuity is no longer sufficient; the next trustworthy state is a fresh snapshot. |

Rules:

1. Producer-side debounce or queue collapse must be disclosed by note or
   audit. Silent coalescing is non-conforming.
2. `snapshot_required_resync` blocks exact derived actions until the
   replacement snapshot lands.
3. A visible `coalesced_visible` state is not itself a stale state, but
   it is a sign that reviewers and benchmarks must keep backlog and
   collapse count visible.

## Query-family examples

The example directory is the reviewer-facing counterpart to the envelope
field:

| Example file | Main families | Why it exists |
|---|---|---|
| [`workspace_truth_projection.yaml`](../../artifacts/state/query_family_examples/workspace_truth_projection.yaml) | `vfs.file_identity`, `language.diagnostics` | Pins authoritative-vs-derived workspace truth across shell/editor/CLI surfaces. |
| [`graph_projection_variants.yaml`](../../artifacts/state/query_family_examples/graph_projection_variants.yaml) | `graph.neighborhood` | Pins live/cached, replayed, and imported graph variants without forking the family id. |
| [`provider_overlay_projection.yaml`](../../artifacts/state/query_family_examples/provider_overlay_projection.yaml) | `provider.ci_checks` | Pins provider-overlay truth, stale terminal behavior, and companion-safe export tags. |

The rule is stable: a surface may add local copy or layout, but it may
not rename the `query_family` when it enters status bars, CLI mirrors,
benchmark packets, or support bundles.

## Invalidation-order expectations

The condensed order audits freeze three order contracts:

| Audit file | Required order | Drift this catches |
|---|---|---|
| [`authority_before_derived_refresh.json`](../../artifacts/state/invalidation_order_trace_examples/authority_before_derived_refresh.json) | authoritative refresh → derived stale notice → derived refresh | derived panes or CLI mirrors refreshing before authority truth does |
| [`delta_gap_requires_resync.json`](../../artifacts/state/invalidation_order_trace_examples/delta_gap_requires_resync.json) | gap note → resync_required → replacement snapshot | silent delta loss and silent epoch replacement |
| [`snapshot_required_switch.json`](../../artifacts/state/invalidation_order_trace_examples/snapshot_required_switch.json) | coalesce note → snapshot-required switch → resync_required → replacement snapshot | hidden backpressure drift and silent collapse into a new snapshot |

These are the compact companions to the fuller scenario traces under
`artifacts/state/invalidation_trace_examples/`.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.state.snapshot_delta_parity_corpus` | `verification_corpus` | Proves one derived/materialized view reaches the same summary under snapshot and delta replay | current with packet revision 1 | [`fixtures/state/snapshot_delta_parity_manifest.yaml`](../../fixtures/state/snapshot_delta_parity_manifest.yaml) |
| `evidence.state.query_family_examples` | `verification_corpus` | Freezes reviewer-facing query-family ids and cross-surface bindings | current with packet revision 1 | [`artifacts/state/query_family_examples/`](../../artifacts/state/query_family_examples/) |
| `evidence.state.invalidation_order_audits` | `verification_corpus` | Provides compact artifact-level ordering proofs | current with packet revision 1 | [`artifacts/state/invalidation_order_trace_examples/`](../../artifacts/state/invalidation_order_trace_examples/) |
| `evidence.state.subscription_envelope_examples` | `verification_corpus` | Pins single-frame envelope shapes at the schema boundary | current with packet revision 1 | [`fixtures/state/envelope_examples/`](../../fixtures/state/envelope_examples/) |
| `evidence.state.subscription_trace_examples` | `verification_corpus` | Provides full per-scenario lifecycle traces | current with packet revision 1 | [`artifacts/state/invalidation_trace_examples/`](../../artifacts/state/invalidation_trace_examples/) |

## Verification method

- **Verification classes used:** design review, fixture corpus freeze,
  scenario replay design, unit-test contract
- **Procedure summary:** freeze the packet-level labels, define the
  parity corpus, extract condensed order audits from the existing
  scenario table, and pair them with query-family example bindings so
  copy/export/benchmark consumers join one set of ids
- **Automation refs:** `cargo test -p aureline-reactive-state --locked`;
  `./tools/reactive_proto.sh --emit-order-audits artifacts/state/invalidation_order_trace_examples`

## Known gaps and waivers

- **Waiver refs:** `none`
- **Known-limit refs:** `none`
- **Migration-packet refs:** `none`
- **Explicit gaps:** this packet does not claim product-wide rollout of
  the reactive store or parity on every surface
- **Explicit gaps:** the query-family examples are reviewer-facing
  examples, not yet a generated method manifest
- **Explicit gaps:** the condensed order audits focus on the three seed
  failure classes most useful to review; the full scenario traces remain
  the broader corpus

## Reviewer signoff

- **Reviewer / forum:** `not_yet_reviewed`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:** `packet_row:reactive_state.snapshot_delta_parity`,
  `packet_row:reactive_state.truth_labels`,
  `packet_row:reactive_state.backpressure_and_debounce`,
  `packet_row:reactive_state.invalidation_order`,
  `packet_row:reactive_state.query_family_identifiers`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `reactive_state_contract_or_scenario_change`
- **Expected freshness window:** `P30D`
- **Next packet family to update with the same evidence ids:** release
  evidence or signoff packet
