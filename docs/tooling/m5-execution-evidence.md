# Problems, output-channel, and execution-evidence causality contract

This packet freezes one canonical causal model for **Problems rows, output
channels, channel headers, execution-evidence projections, evidence bundles,
stale/superseded labels, and reopen-to-origin semantics** across the claimed M5
tooling surfaces. Problems, output, and execution evidence are one causal system,
not three loosely related panes: a user investigating a failure must be able to
answer **what ran, what produced this message, how certain the parser was, what
run/provider/channel it came from, and how to reopen the originating evidence**
without stitching raw logs together by hand.

It exists so that Problems, output, diagnostics, AI evidence, support export,
review, CLI/headless, and docs surfaces ingest **one** governed projection instead
of inventing a parallel causal vocabulary inside individual panes. Reuse the
canonical task-event envelopes, diagnostic IDs, activity rows, run objects, and
[`evidence_link`](../../schemas/execution/evidence_link.schema.json) edges already
landed earlier; this packet binds them into one causal-claim matrix rather than
forking one more ad hoc Problems/output vocabulary.

If this doc, the
[`m5-execution-evidence.schema.json`](../../schemas/tooling/m5-execution-evidence.schema.json)
boundary, the frozen matrix under
[`/artifacts/tooling/m5-execution-evidence/`](../../artifacts/tooling/m5-execution-evidence/),
and the perturbation corpus under
[`/fixtures/tooling/m5-execution-evidence/`](../../fixtures/tooling/m5-execution-evidence/)
disagree, the machine-readable schema plus the checked-in support export
(`artifacts/tooling/m5-execution-evidence/support_export.json`) win, and this doc
must update in the same change.

## Companion artifacts

- [`/schemas/tooling/m5-execution-evidence.schema.json`](../../schemas/tooling/m5-execution-evidence.schema.json)
  — boundary schema for the `m5_execution_evidence_causality_matrix_packet` and
  every frozen taxonomy.
- [`/artifacts/tooling/m5-execution-evidence/support_export.json`](../../artifacts/tooling/m5-execution-evidence/support_export.json)
  — the canonical causality matrix (the source of truth for every lane).
- [`/artifacts/tooling/m5-execution-evidence/matrix.json`](../../artifacts/tooling/m5-execution-evidence/matrix.json)
  and [`report.md`](../../artifacts/tooling/m5-execution-evidence/report.md) — the
  generated causal-claim matrix and certification report (do not edit by hand;
  regenerate with the validator).
- [`/fixtures/tooling/m5-execution-evidence/`](../../fixtures/tooling/m5-execution-evidence/)
  — the perturbation corpus that pins each narrowing/floor rule.
- `tools/release/execution_evidence_causality.py` — the engine that re-derives the
  effective causal claim per lane and emits/validates the matrix and report.
- `crates/aureline-runtime/src/m5_execution_evidence_causality_matrix/` — the
  in-process Rust truth source. It deserializes the checked-in support export into
  one typed packet, re-derives the same per-lane causal claim, narrowing reasons,
  and floor/overlay/labs ladder as the Python engine, and exposes
  `current_m5_execution_evidence_causality_matrix()` so desktop, CLI/headless, AI
  evidence, support export, review, and docs consumers ingest the governed
  projection without re-parsing raw logs or forking a parallel causal vocabulary.

## The causal chain every lane preserves

Each row of the matrix is a claimed (or Labs) tooling **causality lane**: a problem
record, output channel, execution-evidence projection, or evidence-bundle export
bound to its origin identity. A lane preserves one causal chain when:

- structured and heuristic origins stay distinct, and a heuristic parse keeps a
  mandatory raw-output backlink on both the problem row and the owning channel
  header;
- run, step, provider, channel, build/toolchain, and host/target identity survive
  into every overlay (support, AI, review, and pipeline overlays never flatten the
  original lineage);
- large logs stay stream-first, searchable, and exportable without forcing
  whole-run materialization;
- stale and superseded state remain visible wherever the evidence is rendered;
- imported provider/remote evidence stays a read-only overlay that never claims
  live local authority;
- the canonical evidence stays reopenable to its originating run, channel, or
  artifact, and evidence-bundle exports carry the minimum identity to reopen it
  without the original UI state or a live provider session.

## Frozen taxonomies

| Taxonomy | Values |
|---|---|
| **Surface family** | `problems_panel`, `output_channel`, `execution_evidence_projection`, `evidence_bundle_export` |
| **Origin class** | `local_task`, `local_test`, `local_debug_session`, `notebook_run`, `headless_automation`, `extension_owned_run`, `ai_triggered_run`, `remote_linked_run`, `pipeline_provider_run`, `imported_provider_evidence` |
| **Problem source kind** (Appendix BI.1) | `structured_language_diagnostic`, `normalized_task_event`, `heuristic_output_parse`, `imported_provider_annotation`, `not_applicable` |
| **Output channel class** (Appendix BI.2) | `task_test_debug_output`, `extension_ai_tool_output`, `remote_provider_imported_output`, `evidence_bundle`, `not_applicable` |
| **Confidence tier** | `structured_full`, `heuristic_high`, `heuristic_medium`, `heuristic_low`, `provider_mapped`, `unmapped_requires_review` |
| **Freshness / stale-superseded state** | `live`, `cached_within_window`, `stale_expired`, `superseded_by_newer_run`, `unanchored`, `missing` |
| **Reopen target** | `owning_run`, `output_channel`, `generated_artifact`, `provider_run_page`, `raw_output_backlink`, `editor_anchor`, `none_keyboard_fallback` |
| **Proof currency** | `verified_current`, `cached_within_window`, `imported_current`, `stale_expired`, `missing_proof`, `requires_review` |

Adding an enum value is additive-minor and requires a `schema_version` bump;
repurposing an existing value is breaking and requires a new decision row.

## Stable identity and lifecycle fields

Every lane carries one `identity` block so desktop, CLI, review, AI, support, and
docs share one truth model and reconstruct lineage from refs rather than from
freeform display text: `execution_context_ref` (required), plus `run_ref`,
`step_ref`, `provider_ref`, `channel_ref`, `build_toolchain_ref`,
`host_target_ref`, `task_event_envelope_ref`, `problem_record_id`, and
`evidence_bundle_id`. Remote, pipeline/provider, and imported origins must carry a
`provider_ref` so the read-only overlay can always name its source. No raw stdout/
stderr bytes, command lines, provider log bodies, env bodies, absolute paths,
URLs, or secrets ever cross this boundary.

## Causal-claim ladder

The engine re-derives an effective claim per lane that never reads wider than the
evidence supports:

| Claim | Meaning |
|---|---|
| `causal_chain_certified` | full first-party causal chain preserved, fresh, confidence honest, reopenable |
| `causal_chain_narrowed` | a first-party lane held below certified by a stale/missing/labelled gap, but lineage stays reopenable |
| `evidence_read_only_overlay` | remote/pipeline/imported evidence; attributable and reopenable but never claims live local authority |
| `causal_chain_unreconstructable` | lineage/channel/reopen broken or evidence missing; the lane surfaces a raw-output backlink or keyboard fallback instead of a clean-but-false causal claim |
| `causal_evidence_labs_not_claimed` | Labs/unadvertised; makes no public causal claim and is never widened |

## Auto-narrowing rules

A claimed lane auto-narrows below its headline claim when any causal-chain axis
fails or its verification evidence is stale or missing. **Floor** reasons drop a
lane all the way to `causal_chain_unreconstructable`; the remainder hold a
first-party lane at `causal_chain_narrowed`. An overlay is already the minimal
honest claim, so **any** unresolved reason on an overlay floors it.

| Reason | Effect |
|---|---|
| `run_channel_lineage_flattened` | floor |
| `channel_identity_flattened` | floor |
| `reopen_target_lost` | floor |
| `raw_output_backlink_missing` | floor |
| `export_packet_incomplete` | floor |
| `evidence_missing` | floor |
| `imported_overlay_claims_live` | floor |
| `origin_kind_flattened` | narrow |
| `confidence_unlabeled` | narrow |
| `build_or_host_target_missing` | narrow |
| `stream_not_virtualized` | narrow |
| `superseded_state_not_marked` | narrow |
| `evidence_unanchored` | narrow |
| `evidence_stale` | narrow |
| `verification_proof_stale` | narrow |
| `verification_proof_missing` | narrow |

One primary alert may summarize a failure, but the canonical evidence must stay
reopenable: a floored lane keeps its raw-output backlink or keyboard fallback
rather than hiding provider/run/channel identity, heuristic confidence, or
stale/superseded state behind a cleaner UI.

## Origin-path mapping

Local, remote, notebook, extension-owned, AI-triggered, headless, pipeline/
provider, and imported runs map into the same causal vocabulary without erasing
adapter-specific detail:

- **First-party** (`local_task`, `local_test`, `local_debug_session`,
  `notebook_run`, `headless_automation`, `extension_owned_run`, `ai_triggered_run`)
  can certify a full local causal chain. Extension/AI-tool output keeps actor
  attribution and trust state; notebook and headless runs reuse the same task-event
  and output-channel vocabulary as local tasks.
- **Overlay** (`remote_linked_run`, `pipeline_provider_run`,
  `imported_provider_evidence`) certify only as `evidence_read_only_overlay`. They
  preserve provider/run/step lineage and a provider-run-page reopen path, may be
  cached or visibly stale, and never masquerade as live local truth.
- **Labs** lanes make no public claim and are never widened.

## Consumer projection guarantee

The generated matrix projects each lane's effective claim onto the public consumer
surfaces — `problems_panel`, `output_channel_header`, `editor_decoration`,
`timeline_history`, `review_annotation`, `support_export`, `ai_evidence`,
`cli_headless`, `docs_help`, and `public_proof_packet`. The validator refuses any
projection that renders a claim wider than the lane's effective claim, and requires
imported/overlay lanes to stay marked read-only on every surface. Stable IDs,
stale/superseded semantics, and confidence labels are defined once here and reused
by all of them.

## Regenerating and validating

```
python3 tools/release/execution_evidence_causality.py emit-matrix
python3 tools/release/execution_evidence_causality.py emit-report
python3 tools/release/execution_evidence_causality.py self-test
```

`self-test` schema-checks the source packet, confirms the generated matrix and
report match the checked-in artifacts (no manual drift), and runs the full
perturbation corpus.
