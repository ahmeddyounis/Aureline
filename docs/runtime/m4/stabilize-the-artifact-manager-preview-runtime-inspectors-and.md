# Artifact manager, preview/runtime inspectors, and evidence export — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
artifact-manager / preview-runtime-inspector / evidence-export truth
packet. The cross-tool boundary schema lives at
[`schemas/runtime/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth.schema.json`](../../../schemas/runtime/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/stabilize_the_artifact_manager_preview_runtime_inspectors_and/`](../../../crates/aureline-runtime/src/stabilize_the_artifact_manager_preview_runtime_inspectors_and/),
and the checked-in stable packet at
[`artifacts/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json`](../../../artifacts/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json).

The packet pins one boundary truth that the artifact manager, the
preview/runtime inspectors, the evidence-export sheet, the CLI/headless
inspector, the support export bundle, the Help/About proof card, and
the conformance dashboard all read. Surfaces MUST NOT mint local
copies, paraphrase slice freshness, collapse replay/chronology into a
single `recorded`/`not_recorded` bit, mistake exported copies for live
runtime truth, or treat logs / traces / test artifacts as pane-local
blobs.

## Lanes (closed vocabulary)

- `artifact_manager_lane` — chronology, replay packets, capture-versus-
  no-capture state, timeline bookmarks, crash-viewer compare cards, and
  redaction/export sheets owned by the artifact manager.
- `preview_runtime_inspector_lane` — preview / runtime inspectors that
  render logs, metrics, traces, and test-artifact slices into product
  panes.
- `signal_slice_lane` — typed slice objects (`logs_slice`,
  `metrics_slice`, `traces_slice`, `test_artifact_slice`) with source
  identity, target scope, freshness, time window, sample / truncation
  posture, and linked incident-timeline refs.
- `evidence_export_lane` — evidence-export sheets that review included
  channels, problems, artifacts, mapping refs, and redaction profile
  before share/open actions.

Adding or removing a lane is a vocabulary change that requires bumping
the schema and updating the Rust contract, the artifact, the fixture
corpus, and this document together.

## Row classes (closed vocabulary)

- `evidence_export_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per wedge
  (`artifact_chronology_replay_truth`,
  `signal_slice_identity_truth`,
  `evidence_export_review_truth`,
  `cross_surface_evidence_lineage_truth`). All four required for any
  `launch_stable` lane. The `cross_surface_evidence_lineage_truth` row
  MUST set `cross_surface_evidence_lineage_attested: true`.
- `signal_slice_kind_admission` — one row per signal-slice kind
  (`logs_slice`, `metrics_slice`, `traces_slice`,
  `test_artifact_slice`). All four required for any `launch_stable`
  lane.
- `slice_freshness_admission` — one row per slice-freshness state
  (`live_stream`, `buffered_replay`, `cached_snapshot`,
  `imported_evidence`, `truncated_view`, `exported_copy`). All six
  required for any `launch_stable` lane.
- `replay_chronology_admission` — one row per replay-chronology state
  (`recorded`, `not_recorded`, `unsupported`,
  `restart_with_recording_available`, `partially_recorded`). All five
  required for any `launch_stable` lane.
- `retention_class_admission` — one row per retention class
  (`session_only_retention`, `session_plus_window_retention`,
  `policy_bounded_retention`, `archived_retention`,
  `imported_external_retention`). All five required for any
  `launch_stable` lane.
- `consumer_surface_binding` — one row per consumer surface
  (`artifact_manager_surface`, `preview_runtime_inspector_surface`,
  `evidence_export_sheet_surface`, `cli_headless_inspect`,
  `support_export`, `help_about`, `conformance_dashboard`). All seven
  required for any `launch_stable` lane.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into emitted artifacts, signal slices,
  evidence exports, and support packets. Required for every
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
| `artifact_chronology_replay_truth` | Artifact-manager chronology / replay packets, capture-vs-no-capture state, timeline bookmarks, and crash-viewer compare cards expose lineage rather than behaving like screenshot galleries. |
| `signal_slice_identity_truth` | Logs, metrics, traces, and test artifacts are promoted into typed slice objects with source identity, target scope, freshness, time window, sample/truncation posture, and linked incident-timeline refs. |
| `evidence_export_review_truth` | Evidence-export sheets review included channels, problems, artifacts, mapping refs, and redaction profile before share/open and preserve the originating run/channel/artifact reopen path. |
| `cross_surface_evidence_lineage_truth` | Artifact manager, preview/runtime inspectors, and evidence-export sheet see one shared artifact / slice identity and lineage; no surface may fork a local copy. The row MUST attest `cross_surface_evidence_lineage_attested: true`. |

## Signal-slice kinds (required per `launch_stable` lane)

| kind token | what it admits |
|---|---|
| `logs_slice` | A typed logs slice with backend source, time window, freshness, truncation, correlation IDs, redaction/retention class, and embedded-vs-by-reference export posture. |
| `metrics_slice` | A typed metrics slice with backend source, sample posture, freshness, and retention class. |
| `traces_slice` | A typed traces slice with backend source, retention class, freshness, mapping-quality state, and incident-timeline refs. |
| `test_artifact_slice` | A typed test-artifact slice with run/test/artifact identity, retention class, freshness, and reopen path. |

## Slice freshness (required per `launch_stable` lane)

| freshness token | meaning |
|---|---|
| `live_stream` | Slice is actively streamed truth from the running execution. |
| `buffered_replay` | Slice is replayed from the chronology / replay buffer. |
| `cached_snapshot` | Slice is a session-cached snapshot the resolver still trusts. |
| `imported_evidence` | Slice was imported from an authoritative external source. |
| `truncated_view` | Slice is a truncated / sample view of the full evidence; downstream dispatch must show the truncation banner. |
| `exported_copy` | Slice is an exported / shared copy that may not match live runtime truth; users must never confuse it with `live_stream`. |

## Replay-chronology states (required per `launch_stable` lane)

| state token | meaning |
|---|---|
| `recorded` | Chronology / replay is being recorded for the run. |
| `not_recorded` | Chronology / replay is intentionally not being recorded; reopen actions are disclosed accordingly. |
| `unsupported` | The runtime / adapter does not support chronology recording. |
| `restart_with_recording_available` | Recording is available on a restart; the row discloses the restart path. |
| `partially_recorded` | Chronology is partial; downstream dispatch must show the partial-recording badge. |

## Retention classes (required per `launch_stable` lane)

| retention token | meaning |
|---|---|
| `session_only_retention` | Slice / artifact lives only for the current session. |
| `session_plus_window_retention` | Slice / artifact survives the session for the disclosed window. |
| `policy_bounded_retention` | Retention is bounded by an explicit policy. |
| `archived_retention` | Slice / artifact is archived for long-term reopen. |
| `imported_external_retention` | Slice / artifact retention is governed by the imported external source. |

## Consumer surfaces (required per `launch_stable` lane)

| surface token | reads the packet via |
|---|---|
| `artifact_manager_surface` | Artifact manager, chronology / replay panes, crash-viewer compare cards. |
| `preview_runtime_inspector_surface` | Preview / runtime inspectors, logs / metrics / traces panes. |
| `evidence_export_sheet_surface` | Evidence-export sheet, redaction profile review. |
| `cli_headless_inspect` | `aureline runtime inspect` and headless flows. |
| `support_export` | Support export bundle. |
| `help_about` | Help / About proof card. |
| `conformance_dashboard` | Conformance dashboard. |

## Required consumer projections

The packet REQUIRES a consumer projection for each surface above,
preserving the lane, row-class, support-class, wedge, signal-slice
kind, slice-freshness, replay-chronology state, retention-class,
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
  `signal_slice_kind_admission` rows may bind a slice kind; only
  `slice_freshness_admission` rows may bind a freshness class; only
  `replay_chronology_admission` rows may bind a chronology state; only
  `retention_class_admission` rows may bind a retention class; only
  `consumer_surface_binding` rows may bind a consumer surface.
- `lineage_admission` rows MUST bind a non-empty
  `execution_context_id_binding`.
- `wedge_admission` rows binding `cross_surface_evidence_lineage_truth`
  MUST attest `cross_surface_evidence_lineage_attested: true`.
- Raw log bodies, raw trace payloads, raw test-artifact bytes, raw
  command lines, raw process environment bytes, raw secrets, and
  ambient credentials never cross the boundary; evidence is referenced
  through stable IDs, export-safe manifests, and truthful reopen
  packets.

## Fixture corpus

The fixture corpus at
[`fixtures/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and`](../../../fixtures/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and)
contains the baseline stable case plus nine narrowing / blocking cases
covering unbound evidence, missing signal-slice-kind coverage, missing
slice-freshness coverage, missing replay-chronology coverage,
cross-surface evidence lineage without attestation, lineage_admission
missing execution_context_id, narrowed row missing disclosure ref,
projection collapsing the slice-freshness vocabulary, and raw source
material crossing the boundary. Regenerate via `python3 tools/regenerate_stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.py`.
