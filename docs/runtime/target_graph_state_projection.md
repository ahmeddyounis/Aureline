# Target-graph state, host-boundary cue, and operator-truth projection contract

This document freezes the **projection contract** for “what target is current, where did it come from, and how trustworthy is it?” when the answer is derived from a **build adapter + target graph** rather than from informal team knowledge.

The intent is auditability: a reviewer should be able to reconstruct **current target**, **truth class**, **confidence**, and **host boundary** from exported packet data alone, and every first-class surface should use the same vocabulary (no per-surface synonyms).

If this document disagrees with the PRD, technical architecture/design sources, or the frozen taxonomies it references, those sources win and this document plus its companion artifacts MUST be updated in the same change.

## Companion artifacts

- [`/docs/tooling/target_graph_and_adapter_contract.md`](../tooling/target_graph_and_adapter_contract.md)
  — canonical build-adapter descriptor, target-graph snapshot, and target-descriptor identity contract.
- [`/schemas/runtime/target_graph_state.schema.json`](../../schemas/runtime/target_graph_state.schema.json)
  — boundary schema for the `target_graph_state_record` this document defines.
- [`/artifacts/runtime/target_graph_badge_projection.yaml`](../../artifacts/runtime/target_graph_badge_projection.yaml)
  — machine-readable badge/chip projection vocabulary and worked mappings.
- [`/fixtures/runtime/target_graph_cases/`](../../fixtures/runtime/target_graph_cases/)
  — reviewer-facing example cases for exact/imported/heuristic/hybrid truth, wrong-target recovery, provider routing, remote boundaries, provider loss fallback, and stale/partial graphs.

The record defined here composes with (and MUST NOT mint alternatives to):

- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](target_discovery_and_install_review_taxonomy.md)
  — `host_boundary_cue_class` and cue-stack ordering rules.
- [`/docs/runtime/origin_target_route_taxonomy.md`](origin_target_route_taxonomy.md)
  — `action_route_class`, `action_exposure_class`, and authority-linkage vocabulary.
- [`/docs/runtime/authority_class_matrix.md`](authority_class_matrix.md)
  — `authority_class` meaning (canonical writer vs. projection boundary).

## Scope

Frozen by this contract:

- The **field set** that defines target-graph “current target” truth:
  intended target, resolved target, truth class, confidence, host boundary cues, route/exposure, lifecycle/availability state, authority class, and explicit belief basis.
- The **projection vocabulary** every surface uses:
  - truth class: `exact` / `imported` / `heuristic` / `hybrid`
  - availability: `current` / `warming` / `stale` / `partial` / `unavailable`
  - confidence chip: `High` / `Medium-high` / `Medium` / `Low`
  - host-boundary cues: frozen `host_boundary_cue_class` tokens (no synonyms)
- Cross-surface rules for **badges**, **inspectors**, **command diagnostics**, **run/debug/task headers**, **provider entry surfaces**, and **support exports**.

Out of scope:

- Implementing a full target-graph discovery engine, a complete adapter runtime, or any build-system-specific target enumeration logic.
- UI layout, typography, or final component visuals (this is a data/projection contract, not a render spec).

## The record: `target_graph_state_record`

The canonical unit is one `target_graph_state_record` per “current target graph state” claim. It is intended to be emitted by the **canonical owner of target-graph truth** (typically `derived_knowledge` / build-intelligence lanes) and quoted everywhere else.

At minimum, the record MUST answer:

1. **What was intended?** (`intended_target`)
2. **What is resolved as current?** (`resolved_target`)
3. **How do we classify the truth?** (`operator_truth_projection_class`)
4. **How confident is this claim?** (`confidence_chip_class`, plus typed reasons)
5. **Which host boundary did the fact cross?** (`host_boundary_cue_class` and `host_boundary_cue_stack`)
6. **What route/exposure did the claim rely on?** (`action_route_class`, `action_exposure_class`)
7. **What availability/lifecycle state is the graph in?** (`target_graph_availability_state`)
8. **Who is the canonical writer?** (`authority_class`)
9. **Why does the system believe this is current?** (`belief_basis`)

### Truth class (operator-truth projection)

Every record MUST carry exactly one `operator_truth_projection_class`:

- `exact` — current target derived from authoritative adapter/graph truth for the declared scope.
- `imported` — current target derived from imported/cached/replayed evidence; must never masquerade as live.
- `heuristic` — current target inferred from heuristics (name/path/regex/log parsing) and must never present as authoritative.
- `hybrid` — a mixed claim (e.g., cached/imported base plus partial live refresh), which MUST preserve which parts are non-exact via `belief_basis` and confidence constraints.

These tokens are deliberately shared with the product’s broader operator-truth posture (exact/imported/heuristic/hybrid) so target truth does not fragment into build-only jargon.

### Availability / lifecycle state

Target-graph availability is frozen as:

- `current` — graph is current for the workset scope.
- `warming` — graph is being resolved/refreshed; surfaces must disclose that refinements may arrive.
- `stale` — graph exists but is outside freshness floor; must render as stale, not as current.
- `partial` — graph is incomplete for the scope (sparse slice, streamed pending completion, policy-limited view, degraded adapter).
- `unavailable` — no graph claim can be made; surfaces must provide a typed denial instead of generic failure copy.

### Host boundary cues

Every record MUST carry:

- exactly one `host_boundary_cue_class` (local facts use `local_host_boundary`);
- optionally a `host_boundary_cue_stack` ordered **outermost-to-innermost** when the fact crossed multiple boundaries.

Surfaces MUST NOT invent alternative labels like “remote-ish” or “cloud” if a frozen cue exists; they project the cue tokens.

### Intended vs resolved target

Records MUST keep `intended_target` distinct from `resolved_target` so wrong-target recovery and retargeting do not collapse into generic “failed” copy.

Typical mismatch scenarios that must remain explicit:

- an intended target resolved through a lineage remap (rename/merge/split);
- an intended target outside the current workset scope (scope-excluded);
- a target that cannot be resolved from a partial graph (target missing due to partial coverage);
- a provider-routed target (inspect-only + external handoff) that changes the action boundary.

### Belief basis (why this is current)

`belief_basis` is a non-empty list of typed entries. It is the export-safe explanation of *why* this record claims a given resolved target, without requiring raw logs.

Each entry MUST:

- name a closed `belief_basis_class` token (no free-form basis types);
- carry a short `summary` sentence safe for support exports and CLI JSON.

When `operator_truth_projection_class != exact`, at least one basis entry MUST describe the downgrade (import, heuristic inference, hybrid merge, etc.).

## Cross-surface projection rules

### Badges and chips

Every surface that shows a condensed target state MUST project the same three chips, in this order:

1. **Target truth chip** — `operator_truth_projection_class` + `target_graph_availability_state`
2. **Confidence chip** — `confidence_chip_class`
3. **Host boundary chip** — `host_boundary_cue_class` (and stack indicator when non-trivial)

The canonical chip label mapping lives in
[`/artifacts/runtime/target_graph_badge_projection.yaml`](../../artifacts/runtime/target_graph_badge_projection.yaml).

### Inspectors

The inspector view MUST render:

- intended vs resolved target side-by-side;
- graph availability (`current/warming/stale/partial/unavailable`) and any partial-coverage disclosures;
- confidence chip + typed confidence reasons;
- host boundary cue stack with outermost-to-innermost ordering;
- route/exposure fields (so “where did this fact run” is auditable);
- belief basis list.

### Command diagnostics (CLI / logs)

Command diagnostics that mention targets MUST:

- include `resolved_target.display_label` (or a stable id when redacted),
- include `operator_truth_projection_class`,
- include `confidence_chip_class`,
- include `host_boundary_cue_class`,
- and include `target_graph_availability_state`.

Diagnostics MUST NOT collapse degraded states into a generic “target unavailable” string when a typed state exists (`stale`, `partial`, `unavailable`, wrong-target correction).

### Run/debug/task headers

Run/debug/task headers MUST:

- name the resolved target (not only the command),
- show the same truth/confidence/boundary chips as above,
- and keep the availability state visible when non-`current`.

### Provider entry surfaces (external handoff / provider routing)

When a target is provider-routed (inspect-only or external handoff), the surface MUST:

- preserve `action_route_class` and `action_exposure_class`,
- keep the host boundary cue visible,
- and keep the truth class explicit (provider routing does not upgrade confidence).

### Support exports

Support bundles and other exported evidence MUST include the `target_graph_state_record` verbatim (schema-valid), plus enough linked records (graph snapshot/descriptor, lineage events, drift events) to reconstruct:

- what the current target was,
- how confidently,
- across which boundary,
- and why the system believed the claim at that time.

## Required example cases

The fixture corpus under [`/fixtures/runtime/target_graph_cases/`](../../fixtures/runtime/target_graph_cases/) MUST include at least:

- exact local target
- imported or inferred target
- wrong-target recovery (explicit correction + reapproval requirement)
- provider-routed target (inspect-only external handoff)
- remote attach target (non-local host boundary)
- local fallback after provider loss (degraded state remains explicit)
- stale and partial target graphs (do not collapse into generic failure)

