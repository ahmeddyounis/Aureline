# Pattern contract template

This document is the reusable contract template for the required shared
UX patterns indexed in
[`/docs/ux/pattern_inventory.md`](./pattern_inventory.md). Every pattern
in that inventory MUST publish one contract that conforms to the
sections below; surface-local prose alone is not sufficient evidence
that a protected pattern is honored.

The pattern contract is the *cross-surface* contract: it freezes the
shared journey, state vocabulary, and forbidden shortcuts. It does not
replace the per-component
[`component_contract_template.md`](./component_contract_template.md);
component contracts encode the slot-level packet, while a pattern
contract encodes how multiple surfaces and components compose for one
protected user journey.

## Why this template exists

Without a shared contract template:

- a pattern's behavior is recorded only as prose inside
  [`Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md),
  with no reusable obligation that a new surface can cite;
- governing schemas, fixtures, and evidence rows are spread across
  unrelated families with no single ledger that proves the pattern is
  honored end-to-end;
- accessibility hooks for cross-surface journeys land per surface and
  drift apart;
- waivers and freeze exceptions for surface-local exceptions are buried
  in implementation comments instead of named refs.

The contract template closes that gap by giving every pattern a
reviewable shape.

## Contract rules

- A pattern contract MUST cite spec, schema, fixture, and artifact refs
  by stable path. Prose summaries are non-conforming.
- A pattern contract MUST list participating surfaces by id from
  [`/artifacts/ux/surface_traceability_matrix.yaml`](../../artifacts/ux/surface_traceability_matrix.yaml)
  and reuse the canonical surface vocabulary; new surfaces must be added
  there before they appear in a pattern contract.
- A pattern contract MUST publish a state vocabulary with explicit
  fields and enums. Local synonyms are allowed only when the
  cross-surface vocabulary is preserved verbatim.
- A pattern contract MUST name accessibility hooks (announcement
  channel, focus return, keyboard path, reduced-motion posture). A
  pattern that relies on color, motion, or proximity alone is
  non-conforming.
- A pattern contract MUST list forbidden shortcuts. Reviewers use these
  to challenge surface-local exceptions.
- A pattern contract MUST cite at least one evidence packet family per
  acceptance claim, by stable id from
  [`artifacts/governance/evidence_id_conventions.md`](../../artifacts/governance/evidence_id_conventions.md)
  where applicable.
- A pattern contract MUST declare its waiver state through
  [`review_gate_manifest.yaml`](../../artifacts/ux/review_gate_manifest.yaml)
  refs so design-complete gating can check the pattern's review status.
- A surface that wishes to opt out of a pattern MUST raise a freeze
  exception or waiver against the corresponding row in
  [`/artifacts/ux/pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml);
  silent local exceptions are non-conforming.

## Required sections

Every pattern contract uses the section order below. Section names are
fixed so reviewers can scan a contract without re-learning the layout.

### 1. Identity and scope

- `pattern_id` — stable id matching the row in the inventory and
  crosswalk.
- `pattern_title` — human-readable name.
- `summary` — one paragraph describing the protected user journey.
- `pattern_family_class` — one of `recovery`, `write_review`,
  `continuity`, `governed_share`, `mutation_lineage`, `deferred_intent`,
  `state_provenance`, `support_intake`. Free-form classes are not
  allowed.
- `stability_label` — one of `frozen`, `provisional`, `seeded`. Match
  the narrowest honest status across participating surfaces.
- `source_anchor_refs` — UX spec section refs, ADR refs, or PRD ids
  that fix the pattern. The default citation is
  [`Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  section 22.x; additional anchors are listed alongside.
- `review_gate_refs` — `gate_manifest_ref`, `gate_ids[]`,
  `evidence_pack_id`, `waiver_refs[]`, `waiver_state`. See the rules in
  [`component_contract_template.md`](./component_contract_template.md#review-gate-refs)
  — the same fields apply here.

### 2. Protected user journey

- `journey_summary` — what the user is trying to do, written from the
  user's point of view.
- `entry_signals[]` — events, commands, statuses, or alerts that pull
  the user into the pattern.
- `exit_signals[]` — events that release the user from the pattern
  (resolution, cancellation, escalation).
- `success_definition` — what counts as a successful traversal.
- `failure_classes[]` — named failure or degraded outcomes the pattern
  must keep inspectable rather than collapsing into a generic error.

### 3. Participating surfaces

- `surface_refs[]` — surface ids from
  [`surface_traceability_matrix.yaml`](../../artifacts/ux/surface_traceability_matrix.yaml).
- `surface_role[]` — for each surface, one of `entry`, `review`,
  `apply`, `recovery`, `disclosure`, `escalation`. A surface may take
  multiple roles; each role MUST be listed explicitly.
- `composition_notes` — how the surfaces hand off to each other.
- `non_participating_surfaces[]` — explicit list of surfaces that may
  *touch* the journey but are not part of the contract; this prevents
  silent scope creep into adjacent rows.

### 4. State vocabulary

- `state_field_refs[]` — schema refs that own the canonical fields and
  enums.
- `required_state_classes[]` — enums the pattern relies on.
- `local_synonym_rules[]` — when a participating surface uses a local
  label (for example, `awaiting_input` instead of `Needs_review`),
  declare the canonical class it preserves and the disclosure slot the
  local label appears on.
- `degraded_state_rules[]` — how the pattern remains inspectable in
  partial, stale, restricted, or policy-blocked states.

### 5. Governing schemas and contracts

- `governing_doc_refs[]` — docs that define the pattern's behavior.
- `governing_schema_refs[]` — schema files that encode the state
  vocabulary, packet shape, or transport envelope.
- `governing_artifact_refs[]` — yaml artifacts that pin the matrix,
  classes, or registry rows.
- `governing_fixture_refs[]` — fixture directories that supply worked
  examples.
- `companion_pattern_refs[]` — other entries in
  [`pattern_inventory.md`](./pattern_inventory.md) that this pattern
  composes with (for example, refactor preview composes with mutation
  journal).

### 6. Evidence ids

- `evidence_ids[]` — stable evidence ids from
  [`artifacts/governance/evidence_id_conventions.md`](../../artifacts/governance/evidence_id_conventions.md)
  or the relevant family register.
- `evidence_packet_family_refs[]` — packet families that prove the
  pattern is honored on the participating surfaces.
- `evidence_freshness_refs[]` — ceilings or rerun triggers that apply.
- `provisional_evidence_notes[]` — for any evidence id that is seeded,
  reserved, or stitched today, name the gap and the rerun trigger that
  would close it.

### 7. Accessibility hooks

- `announcement_rules[]` — what changes announce, on which channel
  (`live_region_polite`, `live_region_assertive`, `status_bar`,
  `notification`), and at which interruption tier from
  [`artifacts/ux/interruptibility_escalation_seed.yaml`](../../artifacts/ux/interruptibility_escalation_seed.yaml).
- `keyboard_path_refs[]` — task ids from
  [`fixtures/accessibility/task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml)
  and rows from
  [`artifacts/accessibility/shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml)
  that prove keyboard reachability across the pattern's surfaces.
- `focus_return_rule` — where focus returns when the pattern resolves
  or dismisses.
- `assistive_technology_refs[]` — packets or task corpus rows that
  prove screen readers experience the state changes.
- `reduced_motion_posture` — what the pattern looks like under reduced
  motion, forced colors, and high contrast; the pattern MUST remain
  meaningful in all three.
- `quiet_hours_posture_ref` — row from
  [`artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  when the pattern can drive notifications.

### 8. Forbidden shortcuts

- `forbidden_shortcut_rules[]` — explicit list of behaviors that
  silently weaken the pattern (for example, "auto-running mutating
  repairs without preview", "calling a compensation `undo`",
  "auto-extending recording without renewed consent").
- `enforcement_refs[]` — review gates, fitness functions, schema
  validations, or fixture corpus checks that catch the shortcut.

### 9. Crosswalk obligations

- `crosswalk_row_ref` — the row in
  [`/artifacts/ux/pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml)
  this contract maps to.
- `freeze_exception_policy` — under what conditions a participating
  surface may raise a freeze exception against this pattern, and what
  the exception MUST disclose. Exceptions are visible refs, not silent
  carve-outs.
- `waiver_policy` — when a long-lived waiver is permitted, who owns
  it, and how the next review surfaces it.
- `dependency_pattern_refs[]` — other patterns that this contract
  depends on for compliance (for example, support intake depends on
  mutation journal for repair history).

### 10. Open questions and gaps

- `provisional_section_refs[]` — sections that are still seeded or
  awaiting evidence; each gap MUST name the closing trigger.
- `next_review_signals[]` — events that should re-open the contract
  (for example, schema bump, new participating surface, change to
  underlying ADR).

## Minimal review questions

Before a pattern contract is accepted, reviewers should be able to
answer:

- which protected user journey the pattern guarantees;
- which surfaces participate, and in what role each one plays;
- which state vocabulary, schemas, and fixtures back the contract;
- which evidence packets prove the pattern is honored across all
  participating surfaces;
- how assistive technology experiences the journey;
- which forbidden shortcuts reviewers should challenge first; and
- which row in
  [`pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml)
  the contract maps to, and what its current waiver state is.

## Companion references

- [`docs/ux/pattern_inventory.md`](./pattern_inventory.md) — frozen
  index of required patterns.
- [`/artifacts/ux/pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml)
  — surface-to-pattern crosswalk.
- [`docs/ux/component_contract_template.md`](./component_contract_template.md)
  — per-component packet template (slot-level contract).
- [`docs/ux/surface_traceability.md`](./surface_traceability.md) and
  [`/artifacts/ux/surface_traceability_matrix.yaml`](../../artifacts/ux/surface_traceability_matrix.yaml)
  — surface coverage matrix.
- [`docs/governance/verification_packet_template.md`](../governance/verification_packet_template.md)
  — shared evidence-packet header.
- [`artifacts/ux/review_gate_manifest.yaml`](../../artifacts/ux/review_gate_manifest.yaml)
  — design-complete review gates.
