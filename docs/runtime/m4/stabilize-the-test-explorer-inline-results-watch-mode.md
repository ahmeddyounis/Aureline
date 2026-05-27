# Stabilize the test explorer, inline results, watch-mode truth, and rerun/debug-from-test parity — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
stabilize-the-test-explorer / inline-results / watch-mode / rerun /
debug-from-test truth packet. The cross-tool boundary schema lives at
[`schemas/runtime/stabilize_the_test_explorer_inline_results_watch_mode_truth.schema.json`](../../../schemas/runtime/stabilize_the_test_explorer_inline_results_watch_mode_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/stabilize_the_test_explorer_inline_results_watch_mode/`](../../../crates/aureline-runtime/src/stabilize_the_test_explorer_inline_results_watch_mode/),
and the checked-in stable packet at
[`artifacts/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.json`](../../../artifacts/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.json).

The packet pins one boundary truth that the test-explorer surface,
inline-results surface, watch-mode surface, rerun surface,
debug-from-test surface, AI tool surface, CLI/headless inspector,
evidence export, support export, release proof index, Help/About proof
card, and the conformance dashboard all read. Surfaces MUST NOT mint
local copies, flatten watch-mode posture into "watch on / off", or
paraphrase durable selectors into display labels or transient row
order; they project the packet verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host test-explorer sessions.
- `remote_helper_lane` — SSH / remote-agent test-explorer sessions.
- `container_lane` — container-attached test-explorer sessions.
- `notebook_lane` — notebook-bridge test-explorer sessions (cell-based
  tests).

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `test_explorer_stabilization_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per test-explorer wedge
  (`test_explorer_identity_truth`, `inline_results_truth`,
  `watch_mode_truth`, `rerun_debug_from_test_parity`). All four
  required for any `launch_stable` lane.
- `test_identity_admission` — one row per test identity
  (`suite_identity`, `case_identity`, `template_identity`,
  `invocation_identity`). All four required for any `launch_stable`
  lane.
- `discovery_posture_admission` — one row per discovery posture
  (`partial_discovery_record`, `loaded_versus_known_counts`,
  `case_enumeration_at_runtime`). All three required for any
  `launch_stable` lane.
- `watch_mode_support_admission` — one row per watch-mode support
  class (`live`, `reduced`, `polling`, `unavailable`). All four
  required for any `launch_stable` lane.
- `selector_durability_admission` — one row per durable selector
  class (`durable_id_selector`, `trait_selector`,
  `snapshot_scoped_query_selector`). All three required for any
  `launch_stable` lane.
- `consumer_surface_binding` — one row per consumer surface
  (`test_explorer_surface`, `inline_results_surface`,
  `watch_mode_surface`, `rerun_surface`, `debug_from_test_surface`).
  All five required for any `launch_stable` lane. Each row MUST
  attest the test-identity, watch-mode-support, and durable-selector
  vocabularies it is required to preserve.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into emitted test-explorer envelopes and
  downstream consumer surfaces. Required for every `launch_stable`
  lane and MUST surface a non-empty `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Test-explorer wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `test_explorer_identity_truth` — stable suite/case/template/invocation
  identities with partial-discovery records, loaded-versus-known counts,
  and explicit case-enumeration-at-runtime labeling.
- `inline_results_truth` — inline gutter / editor result rows linked to
  durable case/invocation identity and a mapping-fidelity badge that
  survives export/support packets.
- `watch_mode_truth` — per-target-family watch-mode support classification
  and durable session/attempt lineage so the watch loop does not mutate
  one anonymous status row.
- `rerun_debug_from_test_parity` — rerun, debug-from-test, saved
  selectors, AI tool plans, and exported test packets all operate on
  durable IDs, traits, or snapshot-scoped queries.

A missing wedge auto-narrows the lane below `launch_stable` with a
typed `missing_wedge_admission_coverage` finding.

## Test identities (required per `launch_stable` lane)

Rerun, debug-from-test, saved selectors, AI tool plans, and exported
test packets MUST reference these four durable identities instead of
display labels or transient row order:

| identity token | meaning |
|---|---|
| `suite_identity` | durable suite identity (file/path-rooted or adapter-rooted, stable across reload). |
| `case_identity` | durable case identity (stable across reload; survives rename via adapter-supplied stable id when available). |
| `template_identity` | durable template identity (parameterized test template) distinct from per-invocation identity. |
| `invocation_identity` | durable per-invocation identity (parameterized invocation, theory data row, generated case). |

A missing identity auto-narrows the lane below `launch_stable` with a
typed `missing_test_identity_coverage` finding.

## Discovery postures (required per `launch_stable` lane)

The test explorer MUST keep partial-discovery, loaded-versus-known
counts, and case-enumeration-at-runtime labeling observable instead of
flattening them into a green "everything loaded" row. The three closed
postures are:

| posture token | meaning |
|---|---|
| `partial_discovery_record` | partial discovery records that document which suites/cases are known but not yet loaded. |
| `loaded_versus_known_counts` | explicit loaded-versus-known counts surfaced on the explorer chrome. |
| `case_enumeration_at_runtime` | explicit labeling that a case (parameterized invocation, theory row, generated case) is enumerated at runtime instead of at discovery time. |

A missing posture auto-narrows the lane below `launch_stable` with a
typed `missing_discovery_posture_coverage` finding.

## Watch-mode support classes (required per `launch_stable` lane)

Watch mode MUST distinguish per-target-family support instead of
collapsing the vocabulary down to "watch on / off":

| support token | meaning |
|---|---|
| `live` | runner reports test results live via an incremental watcher channel. |
| `reduced` | runner reports a reduced/coalesced subset live; some fidelity downgrades are surfaced explicitly. |
| `polling` | runner is polled on a debounced cadence; the surface MUST distinguish polling from live. |
| `unavailable` | watch mode is not supported for the target family on this lane and the surface MUST disclose the gap. |

A missing support class auto-narrows the lane below `launch_stable`
with a typed `missing_watch_mode_support_coverage` finding.

## Selector durability classes (required per `launch_stable` lane)

Rerun, debug-from-test, saved selectors, AI tool plans, and exported
test packets all operate on durable selectors instead of display
labels or transient row order. The three closed durable classes are:

| durability token | meaning |
|---|---|
| `durable_id_selector` | selector pinned by durable suite/case/template/invocation id. |
| `trait_selector` | selector pinned by adapter-provided traits (tags, categories, ownership predicates). |
| `snapshot_scoped_query_selector` | selector pinned to a query scoped to a captured discovery snapshot, so the selector stays reproducible across reload. |

A missing durability class auto-narrows the lane below `launch_stable`
with a typed `missing_selector_durability_coverage` finding.

## Consumer surfaces (required per `launch_stable` lane)

Every `launch_stable` lane MUST publish a `consumer_surface_binding`
row for each of:

- `test_explorer_surface` — MUST attest `attests_test_identity_preserved`.
- `inline_results_surface` — MUST attest `attests_test_identity_preserved`.
- `watch_mode_surface` — MUST attest `attests_test_identity_preserved`
  and `attests_watch_mode_support_preserved`.
- `rerun_surface` — MUST attest `attests_test_identity_preserved` and
  `attests_durable_selector_preserved`.
- `debug_from_test_surface` — MUST attest `attests_test_identity_preserved`
  and `attests_durable_selector_preserved`.

A missing surface, missing test-identity attestation, missing
watch-mode-support attestation, or missing durable-selector
attestation auto-narrows the lane below `launch_stable` with a typed
`missing_consumer_surface_coverage` /
`consumer_surface_missing_test_identity_attestation` /
`consumer_surface_missing_watch_mode_support_attestation` /
`consumer_surface_missing_durable_selector_attestation` finding.

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces
(test-explorer envelopes, support packets, evidence exports, inline
result rows, watch-mode session/attempt rows) carry the same lineage
id so a "why this test run?" question always resolves to the same
execution-context object.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `test_explorer_surface`
- `inline_results_surface`
- `watch_mode_surface`
- `rerun_surface`
- `debug_from_test_surface`
- `ai_tool_surface`
- `cli_headless`
- `evidence_export`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the twelve vocabularies
verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_wedge_vocabulary`,
`preserves_test_identity_vocabulary`,
`preserves_discovery_posture_vocabulary`,
`preserves_watch_mode_support_vocabulary`,
`preserves_selector_durability_vocabulary`,
`preserves_consumer_surface_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to
`narrowed_below_stable`. The closed finding vocabulary covers
missing identity, missing lane coverage, missing wedge / test-identity
/ discovery-posture / watch-mode-support / selector-durability /
consumer-surface coverage, missing lineage admission, missing surface
attestations, unbound support / known-limit / downgrade-automation /
evidence bindings, missing or collapsed disclosure refs, raw test
source / scrollback / secret / ambient authority leaks, missing or
drifted consumer projections, and promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/stabilize_the_test_explorer_inline_results_watch_mode/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is
the honesty contract: no lane silently inherits adjacent green
claims, and no surface paraphrases test-explorer truth into free-form
prose.

## Anchors

- `auto_narrow_on_wedge_admission_gap`
- `auto_narrow_on_test_identity_gap`
- `auto_narrow_on_discovery_posture_gap`
- `auto_narrow_on_watch_mode_support_gap`
- `auto_narrow_on_selector_durability_gap`
- `auto_narrow_on_consumer_surface_gap`
- `auto_narrow_on_test_identity_attestation_gap`
- `auto_narrow_on_watch_mode_support_attestation_gap`
- `auto_narrow_on_durable_selector_attestation_gap`
- `auto_narrow_on_lineage_break`
- `auto_block_on_missing_evidence`

## See also

- Reviewer artifact:
  [`artifacts/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md`](../../../artifacts/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md)
- Generator:
  [`tools/regenerate_stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.py`](../../../tools/regenerate_stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.py)
- Companion debug-fidelity packet:
  [`docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md`](./harden-breakpoint-call-stack-variables-watch-evaluate-and.md)
- Companion task-event truth packet:
  [`docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md`](./stabilize-task-discovery-launch-profiles-rerun-last-behavior.md)
- Companion execution-context resolver packet:
  [`docs/runtime/m4/stabilize-execution-context-resolver.md`](./stabilize-execution-context-resolver.md)
