# Build-target discovery, adapter-confidence labels, and target-graph snapshots — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
build-target hardening truth packet. The cross-tool boundary schema
lives at
[`schemas/runtime/harden_build_target_discovery_adapter_confidence_labels_and_truth.schema.json`](../../../schemas/runtime/harden_build_target_discovery_adapter_confidence_labels_and_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/harden_build_target_discovery_adapter_confidence_labels_and/`](../../../crates/aureline-runtime/src/harden_build_target_discovery_adapter_confidence_labels_and/),
and the checked-in stable packet at
[`artifacts/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.json`](../../../artifacts/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.json).

The packet pins one boundary truth that the run surface, the test
surface, the debug surface, the CLI/headless inspector, the support
export bundle, the Help/About proof card, and the conformance
dashboard all read. Surfaces MUST NOT mint local copies, paraphrase
adapter-confidence labels, collapse discovery source / freshness into
a single "ok / unknown" bit, or silently widen target identity on
rerun, replay, or restore.

## Lanes (closed vocabulary)

- `run_lane` — run targets exposed by the runtime to the run surface
  and rerun flows.
- `test_lane` — test targets exposed by the runtime to the test
  explorer, inline results, and rerun flows.
- `debug_lane` — debug targets exposed by the runtime to the debug
  surface, adapter negotiation, and attach/launch flows.
- `target_graph_snapshot_lane` — archived / restored / imported
  target-graph snapshots that downstream surfaces consume after the
  live discovery surface is gone.

Adding or removing a lane is a vocabulary change that requires bumping
the schema and updating the Rust contract, the artifact, the fixture
corpus, and this document together.

## Row classes (closed vocabulary)

- `build_target_hardening_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per wedge
  (`build_target_discovery_truth`,
  `adapter_confidence_label_truth`,
  `target_graph_snapshot_truth`,
  `cross_surface_target_parity_truth`). All four required for any
  `launch_stable` lane. The `cross_surface_target_parity_truth` row
  MUST set `cross_surface_target_parity_attested: true`.
- `discovery_source_admission` — one row per discovery source class
  (`native_protocol`, `structured_adapter`, `heuristic_parser`,
  `imported_metadata`, `user_declared`, `resolver_unavailable`). All
  six required for any `launch_stable` lane.
- `discovery_freshness_admission` — one row per freshness class
  (`fresh_probe`, `recent_within_session`, `imported_authoritative`,
  `stale_imported`, `unknown`). All five required for any
  `launch_stable` lane.
- `adapter_confidence_label_admission` — one row per adapter-confidence
  label (`adapter_authoritative_match`, `adapter_probed_consistent`,
  `adapter_probed_divergent`, `adapter_inferred_from_session`,
  `adapter_unreachable`). All five required for any `launch_stable`
  lane.
- `target_graph_snapshot_admission` — one row per target-graph snapshot
  class (`live_snapshot`, `session_cached_snapshot`,
  `imported_snapshot`, `archived_snapshot`, `snapshot_unavailable`).
  All five required for any `launch_stable` lane.
- `consumer_surface_binding` — one row per consumer surface
  (`run_surface`, `test_surface`, `debug_surface`,
  `cli_headless_inspect`, `support_export`, `help_about`,
  `conformance_dashboard`). All seven required for any `launch_stable`
  lane.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into emitted target-graph snapshots,
  support packets, and evidence exports. Required for every
  `launch_stable` lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a `disclosure_ref`.
`support_unbound` never qualifies for stable promotion.

## Wedges (required per `launch_stable` lane)

| wedge token | what it admits |
|---|---|
| `build_target_discovery_truth` | Build-target discovery exposes discovery source class, freshness, and authority on every certified row; no surface may infer a single "ok / unknown" bit. |
| `adapter_confidence_label_truth` | Adapter-confidence labels are preserved verbatim across run, test, debug, CLI/headless, support export, and Help/About surfaces. |
| `target_graph_snapshot_truth` | Target-graph snapshots are reproducible, lineage-bound, and honest about live vs. cached vs. imported vs. archived state. |
| `cross_surface_target_parity_truth` | Run, test, and debug surfaces see one shared target identity and target-graph; no surface may fork a local copy. The row MUST attest `cross_surface_target_parity_attested: true`. |

## Discovery sources (required per `launch_stable` lane)

| source token | what it admits |
|---|---|
| `native_protocol` | Target was minted by a native runtime / language host / DAP. |
| `structured_adapter` | Target came from a typed adapter parsing a structured manifest. |
| `heuristic_parser` | Target was inferred by a heuristic / regex / fallback parser. |
| `imported_metadata` | Target was lifted from imported CI / external metadata. |
| `user_declared` | Target was declared by the user (override, saved profile). |
| `resolver_unavailable` | The discovery layer was unavailable; protected dispatch refused. |

## Discovery freshness (required per `launch_stable` lane)

| freshness token | meaning |
|---|---|
| `fresh_probe` | Target was probed in the current resolver session and matched. |
| `recent_within_session` | Target was probed earlier in this session and is still trusted. |
| `imported_authoritative` | Target was imported from an authoritative external source. |
| `stale_imported` | Target was imported but the resolver observed drift / staleness. |
| `unknown` | Freshness cannot be determined; treat as unsafe for protected dispatch. |

## Adapter-confidence labels (required per `launch_stable` lane)

| label token | meaning |
|---|---|
| `adapter_authoritative_match` | Adapter reported an authoritative match. |
| `adapter_probed_consistent` | Adapter probe was consistent. |
| `adapter_probed_divergent` | Adapter probe diverged. |
| `adapter_inferred_from_session` | Adapter inferred the target from session context only. |
| `adapter_unreachable` | Adapter was unreachable; the label MUST be carried verbatim. |

These five labels MUST stay distinct in the run, test, and debug
surfaces, in the CLI/headless inspector, in the support export bundle,
in the Help/About proof card, and in the conformance dashboard.

## Target-graph snapshot classes (required per `launch_stable` lane)

| snapshot token | meaning |
|---|---|
| `live_snapshot` | Snapshot was actively resolved this session against live discovery. |
| `session_cached_snapshot` | Snapshot was cached earlier in this session; resolver still trusts it. |
| `imported_snapshot` | Snapshot was imported from an authoritative external source. |
| `archived_snapshot` | Snapshot was read from archive / history. |
| `snapshot_unavailable` | Snapshot could not be produced; downstream dispatch refused. |

## Consumer surfaces (required per `launch_stable` lane)

| surface token | reads the packet via |
|---|---|
| `run_surface` | Run target picker, run cards. |
| `test_surface` | Test explorer, inline results. |
| `debug_surface` | Debug session panel, adapter chips. |
| `cli_headless_inspect` | `aureline runtime inspect` and headless flows. |
| `support_export` | Support export bundle. |
| `help_about` | Help / About proof card. |
| `conformance_dashboard` | Conformance dashboard. |

## Required consumer projections

The packet REQUIRES a consumer projection for each surface above,
preserving the lane, row-class, support-class, wedge, discovery-source,
discovery-freshness, adapter-confidence label, target-graph snapshot,
consumer-surface, known-limit, downgrade-automation, and evidence-class
vocabularies verbatim. Projections MUST also confirm
`supports_json_export`, `raw_private_material_excluded`, and
`ambient_authority_excluded`.

## Validation invariants

- A row claiming `launch_stable` while leaving its known limit,
  downgrade automation, evidence class, or support class unbound is
  refused.
- A row narrowed below `launch_stable`, declaring a non-`none_declared`
  known limit, or binding a non-`none` downgrade automation MUST
  surface a `disclosure_ref`.
- Only `wedge_admission` rows may bind a wedge; only
  `discovery_source_admission` rows may bind a source; only
  `discovery_freshness_admission` rows may bind a freshness class;
  only `adapter_confidence_label_admission` rows may bind a label;
  only `target_graph_snapshot_admission` rows may bind a snapshot
  class; only `consumer_surface_binding` rows may bind a consumer
  surface.
- `lineage_admission` rows MUST bind a non-empty
  `execution_context_id_binding`.
- `wedge_admission` rows binding `cross_surface_target_parity_truth`
  MUST attest `cross_surface_target_parity_attested: true`.
- Raw discovery payloads, raw adapter handshake bodies, raw command
  lines, raw process environment bytes, raw secrets, and ambient
  credentials never cross the boundary.

## Fixture corpus

The fixture corpus at
[`fixtures/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and`](../../../fixtures/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and)
contains the baseline stable case plus nine narrowing / blocking
cases covering unbound evidence, missing discovery-source coverage,
missing adapter-confidence label coverage, missing target-graph
snapshot coverage, cross-surface target parity without attestation,
lineage_admission missing execution_context_id, narrowed row missing
disclosure ref, projection collapsing the adapter-confidence label
vocabulary, and raw source material crossing the boundary. Regenerate
via `python3 tools/regenerate_harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.py`.
