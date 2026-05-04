# Graph provenance-stamp, confidence-rollup, and source-anchor-drift contract

This document is the cross-tool contract every graph-backed surface
reads when it explains why an object or relation exists, how stale or
inferred it is, and whether the source anchor still resolves. Surfaces
covered: graph overlay, topology map, impact explorer, cited-explainer
overlay, AI context assembler, review pack, support bundle, and the
eventual public query-family surface.

The machine-readable schemas live at:

- [`/schemas/graph/provenance_stamp.schema.json`](../../schemas/graph/provenance_stamp.schema.json)
- [`/schemas/graph/confidence_rollup.schema.json`](../../schemas/graph/confidence_rollup.schema.json)

Companion fixtures live under:

- [`/fixtures/graph/provenance_cases/`](../../fixtures/graph/provenance_cases/)

This contract is normative. It layers on top of the workspace-graph
seed ([`/docs/graph/workspace_graph_seed.md`](workspace_graph_seed.md))
without restating the seed's node-class or edge-class vocabulary; the
seed's inline `provenance_stamp` and `confidence_rollup` slots are a
strict subset of the records defined here. Where this document
disagrees with the seed, the seed wins on identity rules and this
document is updated in the same change. Where a downstream surface
mints its own provenance / freshness / confidence vocabulary, this
document wins and the surface is non-conforming.

## Why freeze this now

A graph surface that renders a heuristic-inferred relation with the
same chip as a parser-confirmed relation has already lied once. A
surface that collapses a moved or deleted source anchor into a generic
"not available" message has lied a second time. A surface that
silently upgrades a low-confidence contributor to high because the
loudest voice was high has lied a third time. Freezing the provenance
stamp, the deterministic confidence rollup, and the typed anchor-drift
vocabulary in machine-readable form makes those three honesty
contracts enforceable rather than aspirational.

The second hazard is divergence across surfaces. Topology maps,
impact explorers, and explainer overlays all read the same nodes and
edges, but each one has historically rendered its own confidence
chip. Pinning the rollup algorithm here — exact floor rules, exact
conflict vocabulary, exact partial-visibility interaction — guarantees
that the same node renders the same label in every surface.

## Scope

- Freeze one `provenance_stamp_record` covering source class,
  extraction-or-inference mode, freshness, anchor validity,
  imported-root status, and supporting-artifact references.
- Freeze one `source_anchor_drift_record` and the typed
  `source_anchor_drift_state` vocabulary covering moved, deleted,
  imported, generated, and unknown-lineage anchors.
- Freeze one `confidence_rollup_record` and the deterministic
  rollup algorithm covering nodes, edges, impact-reason slots, and
  explainer-citation slots, including conflicting-evidence handling
  and partial-workset-visibility interaction.
- Pin the floor-reason vocabulary so a reviewer can replay the
  rollup deterministically from the contributor list.

## Out of scope

- The graph topology query planner, the indexing engine, and the
  final visual graph UI. Those land later; this contract freezes
  only the record shapes and the rollup algorithm.
- Per-language symbol resolvers' internal confidence calibration.
  Producers report `high` / `medium` / `low` / `unknown` on this
  contract; how each resolver chooses a level is a separate decision.
- The cited-explainer overlay's render layout. The overlay reads
  `rolled_up_level`, `floor_reason`, `conflict_state`, and
  `anchor_drift_summary` and renders them; the surface treatment is
  a UX decision separate from this schema.

## 1. Provenance stamp

Every workspace-graph node, edge, impact-reason slot, and
explainer-citation slot resolves to exactly one
`provenance_stamp_record`. The stamp answers four questions in one
record:

| Field                          | Question answered                                                                |
|--------------------------------|----------------------------------------------------------------------------------|
| `source_class`                 | Which producer family emitted the record?                                        |
| `extraction_or_inference_mode` | How did the producer derive the value?                                           |
| `freshness` + `stale_reason`   | When was the value last refreshed against the canonical owner, and why is it not authoritative? |
| `anchor_drift_observations`    | Does the source anchor still resolve, and if not, in which typed drift class?     |

Plus three pointer fields the support / replay / AI surfaces walk
without re-querying the producer:

| Field                       | Purpose                                                                         |
|-----------------------------|---------------------------------------------------------------------------------|
| `imported_root_status`      | Whether (and how) the record was hydrated from an imported root.                |
| `supporting_artifact_refs`  | Opaque refs to mutation journal, lineage, imported bundle, replay capture, codeowners rule, support packet, AI evidence packet. |
| `support_ref`               | Optional opaque pointer to the full forensic support-bundle packet for this record. |

### 1.1 `source_class`

Re-exported from
[`/schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json):
`workspace_filesystem`, `buffer_editor`, `symbol_resolver`,
`docs_pack`, `codeowners_resolver`, `build_toolchain`, `codegen_tool`,
`package_resolver`, `notebook_kernel`, `preview_runtime`,
`connected_provider`, `remote_agent`, `ai_inference`,
`imported_bundle`, `replay_capture`, `policy_projection`,
`manual_annotation`. This contract does not mint new source classes;
adding one is additive-minor on the seed and on this document in the
same change.

### 1.2 `extraction_or_inference_mode`

Names the path the producer took to derive the value. Closed
vocabulary:

| Mode                               | Meaning                                                                                            |
|------------------------------------|----------------------------------------------------------------------------------------------------|
| `observed_direct_extraction`       | Producer observed the value first-hand (parser, symbol resolver, filesystem walker, build run).    |
| `parsed_from_structured_source`    | Extracted from a structured artifact (lockfile, OpenAPI doc, manifest).                            |
| `projected_from_authoritative_index` | Read from a downstream index that re-exports an authoritative producer.                           |
| `replayed_from_capture`            | Rehydrated from a deterministic replay capture.                                                    |
| `hydrated_from_imported_bundle`    | Hydrated from a signed upstream bundle, vendor drop, or docs-pack import.                          |
| `derived_by_heuristic`             | Derived by a non-AI heuristic pass (cross-reference, fuzzy match, transitive walk).                |
| `generated_by_codegen`             | Produced by a deterministic codegen tool whose lineage is available.                               |
| `suggested_by_ai_inference`        | Produced by an AI inference pass; surfaces never silently upgrade.                                 |
| `manually_annotated`               | A human added or amended this record (triage tag, override note, manual review).                   |

This axis is orthogonal to `source_class` and `provenance_class`; the
combination tells the consumer whether the row is a parser-confirmed
fact, a downstream projection, an inference, or a human annotation.

### 1.3 `freshness` and `stale_reason`

Re-exported from the seed (and ultimately from ADR-0005 / ADR-0014).
Frames with non-`authoritative` freshness MUST carry a typed
`stale_reason`.

### 1.4 `imported_root_status`

Closed vocabulary mirroring the seed's `import_kind` plus a
`not_imported` sentinel: `imported_signed_upstream_bundle`,
`imported_vendor_drop`, `imported_replay_capture`,
`imported_migration_import`, `imported_docs_pack`,
`imported_template_expansion`. Records produced by an authoritative
local producer carry `not_imported`.

### 1.5 `supporting_artifact_refs`

Each ref pairs an opaque id with one
`supporting_artifact_class`:
`mutation_journal_entry`,
`generated_artifact_lineage_record`,
`imported_bundle_entry`,
`replay_capture_entry`,
`codeowners_rule`,
`support_bundle_packet`,
`ai_evidence_packet`,
`policy_projection_record`,
`annotation_note`,
`build_toolchain_run_record`,
`package_resolver_lockfile_entry`. Support bundles, AI evidence
packets, and review packs walk this list to assemble a forensic
packet without re-querying upstream producers.

## 2. Source-anchor drift

Every `source_anchor` on a workspace-graph node or edge resolves to
exactly one `anchor_drift_observation`. Two axes:

### 2.1 `anchor_validity_state`

Three-state vocabulary every consumer reads to decide whether to
render a citation row, a drift chip, or a missing-anchor badge:

| State                          | Meaning                                                                                                   |
|--------------------------------|-----------------------------------------------------------------------------------------------------------|
| `anchor_present_resolves`      | Anchor still points at the same object the producer originally observed.                                  |
| `anchor_resolves_with_drift`   | Anchor still resolves but the underlying object has drifted (move, regenerated artifact, imported re-pin). |
| `anchor_unresolvable`          | Anchor no longer resolves; surface MUST render the missing-anchor badge with the typed `drift_state`.     |

### 2.2 `source_anchor_drift_state`

Closed vocabulary covering the drift classes the spec requires:

| State                                | Pairs with                                                                                  |
|--------------------------------------|---------------------------------------------------------------------------------------------|
| `anchor_present_no_drift`            | `anchor_present_resolves` (no chip).                                                        |
| `anchor_moved_resolves_via_alias`    | Source moved but resolves through ADR-0006 alias_set or a forwarding rename. Surfaces render a moved-anchor chip and quote the canonical_filesystem_object the resolver landed on. |
| `anchor_deleted_no_replacement`      | Source deleted, no forwarding alias resolves; pairs with `missing_anchor_node` and `missing_reason ∈ {deleted_filesystem_object, unresolved_symbol, provider_resource_unreachable}`. |
| `anchor_imported_unverified`         | Source anchor lives inside an imported root and has not been cross-validated against a workspace-local producer. Pairs with `provenance_class == imported_external` and `freshness == imported`. |
| `anchor_generated_pending_lineage`   | Source anchor points at a generated artifact whose `generated_artifact_lineage_record` is missing or pending regeneration. Pairs with `drift_state == unknown_lineage` on the lineage record. |
| `anchor_unknown_lineage`             | Anchor lineage cannot be reconstructed (replay-capture missing entry, imported-bundle missing entry, ai_inference without supporting evidence). Surfaces render the unknown-lineage badge. |

This axis is the answer to the spec's "moved source anchor, deleted
anchor, imported anchor, generated anchor, and unknown anchor lineage"
requirement: each appears once in the closed vocabulary, with a
typed pairing rule, so anchor drift remains reviewable instead of
collapsing into a generic "not available" message.

### 2.3 Stand-alone drift records

Surfaces that emit a drift event independent of (or in addition to) a
parent provenance stamp use `source_anchor_drift_record`. The record
carries the same `anchor_drift_observation`, an opaque
`drift_record_id`, and an optional `audit_event_id` from a closed
vocabulary (`graph_anchor_drift_recorded`,
`graph_anchor_resolved_via_alias`, `graph_anchor_deleted`,
`graph_anchor_imported_unverified`,
`graph_anchor_generated_pending_lineage`,
`graph_anchor_unknown_lineage`). Replay captures, support bundles,
and AI evidence packets cite these audit ids verbatim.

## 3. Confidence rollup

Multi-source nodes, edges, impact-reason slots, and
explainer-citation slots resolve to one
`confidence_rollup_record` per surface. The rollup answers one
question: *given multiple contributors, what is the single confidence
label every surface should render?*

### 3.1 Surface classes

`surface_class ∈ {node, edge, impact_reason, explainer_citation}`.
The same algorithm applies across all four classes; the
`surface_class` is recorded so a downstream consumer can dispatch on
it without re-deriving from the parent record.

### 3.2 Contributors

Each contributor is one `source_confidence_contributor` carrying its
`confidence_level`, a `contributor_kind` (`primary_producer`,
`projecting_surface`, `imported_bundle`, `replay_capture`,
`heuristic_inference`, `ai_inference`, `manual_annotation`,
`policy_projection`, `cross_reference_walker`, `build_toolchain`,
`codegen_tool`, `package_resolver`, `connected_provider`,
`remote_agent`, `docs_pack`), an optional `evidence_state` (edges
only), and an advisory `weight_hint`. The rollup algorithm itself
ignores `weight_hint` and applies the closed floor rules; the hint is
renderer-only.

### 3.3 Rollup algorithm

The algorithm is deterministic and pure: same inputs → same
`rolled_up_level` and same `floor_reason`. Steps:

1. **Baseline.** Start with `level = min(contributor.confidence_level)`
   under the order `high > medium > low > unknown` (i.e. lowest level
   wins).
2. **Unknown floor.** If any contributor reports `unknown`, lower
   `level` to at most `low`. Set `floor_reason =
   unknown_contributor_pulls_to_low` if this rule fired.
3. **Low contributor floor.** If any contributor reports `low`,
   `level` is already at or below `low`. Set `floor_reason =
   low_contributor_pulls_to_low` if this rule fired and rule 2 did
   not.
4. **Evidence-state floors.** For edge rollups, walk
   `evidence_states_present`:
   - `missing_anchor` → lower `level` to at most `low`. Set
     `floor_reason = missing_anchor_pulls_to_low`.
   - `stale_relation` → lower `level` to at most `low`. Set
     `floor_reason = stale_relation_pulls_to_low`.
   - `inferred_relation` → cap `level` at `medium`. Set
     `floor_reason = inferred_relation_caps_at_medium`.
5. **Contributor-kind floors.**
   - Any `ai_inference` contributor caps `level` at `medium`. Set
     `floor_reason = ai_inference_caps_at_medium`.
   - Any `imported_bundle` contributor without a corroborating
     local-authoritative contributor caps `level` at `medium`. Set
     `floor_reason = imported_unverified_caps_at_medium`.
6. **Anchor-drift floor.** If
   `anchor_drift_summary.anchor_drift_present == true`, lower `level`
   to at most `low` and set `floor_reason =
   anchor_drift_pulls_to_low` (unless `worst_drift_state` is
   `anchor_imported_unverified` only, in which case the existing
   `imported_unverified_caps_at_medium` floor already applies).
7. **Partial-visibility floors.** Read
   `partial_visibility_interaction.visibility_state`:
   - `partial_visible` → cap `level` at `medium`. Set `floor_reason =
     partial_visibility_caps_at_medium`.
   - `policy_hidden` → cap `level` at `medium`. Set `floor_reason =
     policy_hidden_caps_at_medium`.
   - `missing_in_scope` → lower `level` to at most `low`. Set
     `floor_reason = missing_in_scope_pulls_to_low`.
8. **Most-restrictive floor wins.** When multiple floors apply, the
   one that produced the lowest level is recorded in `floor_reason`.
   When two floors produce the same level, the one earlier in the
   list above wins (deterministic tie-break).
9. **No-floor case.** If no rule fires, `floor_reason =
   no_floor_applied` and `rolled_up_level == min(contributor.confidence_level)`.

### 3.4 Conflict state

Independently of the floors, the rollup records a `conflict_state`:

| State                                            | Triggers when                                                                          |
|--------------------------------------------------|----------------------------------------------------------------------------------------|
| `no_conflict`                                    | Every contributor agrees within one level.                                             |
| `level_split_high_and_low`                       | Contributors disagree by ≥ 2 levels (`high` + `low`).                                  |
| `level_split_with_unknown`                       | At least one contributor reported `unknown`.                                           |
| `evidence_state_split_direct_and_inferred`       | At least one `direct_evidence` and at least one `inferred_relation`.                   |
| `evidence_state_split_direct_and_stale`          | `direct_evidence` and `stale_relation`.                                                |
| `evidence_state_split_direct_and_missing_anchor` | `direct_evidence` and `missing_anchor`.                                                |
| `imported_vs_local_split`                        | `imported_evidence` contributor and a local-authoritative contributor disagree.        |
| `ai_vs_authoritative_split`                      | `ai_inference` contributor and `authoritative_producer` contributor disagree.          |
| `partial_visibility_pulls_down`                  | Contributors agreed but workset visibility pulled the rollup down.                     |

Surfaces render the conflict chip alongside the rolled-up level so
the user can see *why* the level is what it is.

### 3.5 Partial-workset visibility

`partial_visibility_interaction.visibility_state` re-exports the
seed's workset visibility vocabulary (`fully_visible`,
`partial_visible`, `policy_hidden`, `missing_in_scope`). The
visibility floor (rule 7 above) ensures a node visible only through a
partial or policy-hidden scope cannot present `high` confidence
through any surface. `policy_hidden` additionally projects
`hidden_member_count` (never the hidden member ids themselves).

### 3.6 Determinism contract

A reviewer reading `source_confidences`, `evidence_states_present`,
`partial_visibility_interaction`, and `anchor_drift_summary` MUST be
able to recompute the same `rolled_up_level` and `floor_reason`.
Topology, impact, and explainer surfaces share this contract so any
disagreement is a producer bug, not a rendering bug.

## 4. Surface rules

These rules apply to every surface that renders, logs, exports, or
reasons about provenance, confidence, or anchor drift on a workspace
graph.

1. **No surface invents private confidence labels.** Surfaces render
   `rolled_up_level` verbatim, with the `conflict_state` and
   `floor_reason` chips. Local-only labels like "verified", "fresh",
   "trusted", "best guess" are forbidden when a graph record carries
   one of the values defined here.
2. **Anchor drift remains typed and reviewable.** Surfaces render the
   `source_anchor_drift_state` chip (moved / deleted / imported /
   generated / unknown-lineage) instead of a generic "not available"
   message. The drift_note may be rendered alongside; raw paths,
   raw URLs, raw provider URLs never cross.
3. **Imported records project the imported-root status.** When
   `imported_root_status != not_imported`, surfaces render the
   imported-root chip. Surfaces that hide imported content (signed
   upstream bundles users cannot edit) render the imported-root chip
   instead of folding into a local row.
4. **AI-suggested rows never silently upgrade.** A row whose
   `extraction_or_inference_mode == suggested_by_ai_inference` (or
   whose rollup carries an `ai_inference` contributor) caps at
   `medium`. Surfaces render the AI-inference chip alongside the
   level so the user can see why.
5. **Stale or missing-anchor relations carry the right chip.** Edge
   rollups with `evidence_state ∈ {stale_relation, missing_anchor}`
   render the staleness or missing-anchor chip with the typed
   `floor_reason`; surfaces never collapse to a clean
   high-confidence row.
6. **Policy-hidden is visible as hidden.** A
   `policy_hidden` visibility caps the rollup at `medium` and
   surfaces project `hidden_member_count`; the exact hidden list never
   projects.
7. **Support bundles quote ids.** Support bundles, replay captures,
   and AI evidence packets cite `stamp_id`, `rollup_id`, and
   `drift_record_id` verbatim and walk `supporting_artifact_refs` to
   reconstruct the forensic packet.

## 5. Worked examples

Each example references a companion fixture under
[`/fixtures/graph/provenance_cases/`](../../fixtures/graph/provenance_cases/).
The fixtures are human-authored YAML and validate against the
schemas listed at the top of this document.

### 5.1 Direct-code evidence

A symbol resolver emits a `defines_symbol` edge first-hand. Stamp:
`source_class = symbol_resolver`,
`extraction_or_inference_mode = observed_direct_extraction`,
`freshness = authoritative`, `imported_root_status = not_imported`,
one anchor with `anchor_validity_state = anchor_present_resolves` and
`drift_state = anchor_present_no_drift`. Rollup: one contributor at
`high`, `floor_reason = no_floor_applied`, `conflict_state =
no_conflict`, `rolled_up_level = high`.

See
[`direct_code_evidence.yaml`](../../fixtures/graph/provenance_cases/direct_code_evidence.yaml).

### 5.2 Imported schema evidence

A signed upstream bundle hydrated a `references_symbol` edge into an
imported root. Stamp:
`source_class = imported_bundle`,
`extraction_or_inference_mode = hydrated_from_imported_bundle`,
`freshness = imported`, `stale_reason = imported_from_external`,
`imported_root_status = imported_signed_upstream_bundle`, anchor with
`anchor_validity_state = anchor_resolves_with_drift` and `drift_state
= anchor_imported_unverified`. Rollup: one `imported_bundle`
contributor at `medium`, `floor_reason =
imported_unverified_caps_at_medium`, `conflict_state =
no_conflict`, `rolled_up_level = medium`.

See
[`imported_schema_evidence.yaml`](../../fixtures/graph/provenance_cases/imported_schema_evidence.yaml).

### 5.3 Stale build-graph evidence

A `depends_on` edge between two crates was last refreshed by a
build-toolchain run that is no longer authoritative; the upstream
input changed. Stamp:
`source_class = build_toolchain`,
`extraction_or_inference_mode = parsed_from_structured_source`,
`freshness = stale`, `stale_reason = upstream_input_stale`,
`imported_root_status = not_imported`, anchor with
`anchor_validity_state = anchor_resolves_with_drift` and `drift_state
= anchor_moved_resolves_via_alias`. Rollup: two contributors —
`build_toolchain` at `medium` and `package_resolver` at `low`,
`evidence_states_present = [stale_relation]`, `floor_reason =
stale_relation_pulls_to_low`, `conflict_state =
evidence_state_split_direct_and_stale`, `rolled_up_level = low`.

See
[`stale_build_graph_evidence.yaml`](../../fixtures/graph/provenance_cases/stale_build_graph_evidence.yaml).

### 5.4 AI-suggested relation with low confidence

An AI inference pass suggested a `references_symbol` edge between two
helpers; a heuristic cross-reference walker corroborated. Stamp:
`source_class = ai_inference`,
`extraction_or_inference_mode = suggested_by_ai_inference`,
`freshness = authoritative`, `imported_root_status = not_imported`,
anchor with `anchor_validity_state = anchor_present_resolves` and
`drift_state = anchor_present_no_drift`. Rollup: two contributors —
`ai_inference` at `low` and `cross_reference_walker` at `medium`,
`evidence_states_present = [inferred_relation]`. Walking the floors:
`low_contributor_pulls_to_low` fires (rule 3) and produces `low`;
`inferred_relation_caps_at_medium` and `ai_inference_caps_at_medium`
each fire but only cap at `medium` (less restrictive). Most-restrictive
floor wins, so `floor_reason = low_contributor_pulls_to_low` and
`rolled_up_level = low`. Contributors disagree by one level (`low`
and `medium`) so `conflict_state = no_conflict` (the
`ai_vs_authoritative_split` and
`evidence_state_split_direct_and_inferred` triggers do not match this
contributor mix).

See
[`ai_suggested_low_confidence.yaml`](../../fixtures/graph/provenance_cases/ai_suggested_low_confidence.yaml).

### 5.5 Missing-anchor degradation

A `references_symbol` edge points at a symbol that was deleted; the
graph projects a `missing_anchor_node`. Stamp:
`source_class = symbol_resolver`,
`extraction_or_inference_mode = observed_direct_extraction`,
`freshness = stale`, `stale_reason = upstream_input_stale`,
`imported_root_status = not_imported`, anchor with
`anchor_validity_state = anchor_unresolvable` and `drift_state =
anchor_deleted_no_replacement`. Rollup: one `primary_producer`
contributor at `low` (the resolver itself reported `low` because the
target is missing), `evidence_states_present = [missing_anchor]`,
`floor_reason = missing_anchor_pulls_to_low`, `conflict_state =
evidence_state_split_direct_and_missing_anchor` is not used (only
one contributor); `conflict_state = no_conflict`. `rolled_up_level =
low`. The surface MUST render the typed
`anchor_deleted_no_replacement` chip alongside the missing-anchor
badge — never collapse into "not available".

See
[`missing_anchor_degradation.yaml`](../../fixtures/graph/provenance_cases/missing_anchor_degradation.yaml).

## 6. Acceptance

- Any graph-backed surface can answer "why is this here, when was it
  last refreshed, and how was it derived" by quoting one
  `provenance_stamp_record` rather than inventing local-only
  certainty labels.
- Anchor drift is typed (moved / deleted / imported / generated /
  unknown-lineage) and reviewable; surfaces render the typed chip
  instead of a generic "not available" message.
- Confidence rollups are deterministic across topology, impact, and
  explainer surfaces. Identical inputs produce identical
  `rolled_up_level` and `floor_reason`; a divergence is a producer
  bug.
- The fixtures cover direct-code evidence, imported schema evidence,
  stale build-graph evidence, AI-suggested low-confidence relation,
  and missing-anchor degradation, and validate against the schemas.

## 7. Changing this vocabulary

- **Additive-minor.** Adding a new `extraction_or_inference_mode`,
  a new `source_anchor_drift_state`, a new `imported_root_status`, a
  new `supporting_artifact_class`, a new `conflict_state`, a new
  `floor_reason`, a new `surface_class`, or a new `audit_event_id`
  on the drift record lands here, in the schemas, and in the
  fixtures in the same change. The change must cite the motivating
  fixture.
- **Repurposing** an existing value is breaking. It opens a new
  decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The workspace-graph seed wins on identity rules; this document
  and the schemas are updated in the same change when the seed
  changes.

## Linked artifacts

- Workspace-graph seed:
  [`docs/graph/workspace_graph_seed.md`](workspace_graph_seed.md),
  [`schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json).
- Provenance stamp / source-anchor-drift schema:
  [`schemas/graph/provenance_stamp.schema.json`](../../schemas/graph/provenance_stamp.schema.json).
- Confidence-rollup schema:
  [`schemas/graph/confidence_rollup.schema.json`](../../schemas/graph/confidence_rollup.schema.json).
- Provenance-case fixtures:
  [`fixtures/graph/provenance_cases/`](../../fixtures/graph/provenance_cases/).
- ADR (filesystem identity, save pipeline, cache identity):
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- ADR (subscription envelope and invalidation semantics):
  [`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- ADR (search readiness, ranking, result truth):
  [`docs/adr/0014-search-readiness-ranking-result-truth.md`](../adr/0014-search-readiness-ranking-result-truth.md).
- Generated-artifact lineage schema:
  [`schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json).
- Execution-context schema:
  [`schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json).
