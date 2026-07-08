# Implement merge decision rows and generated-artifact notices

Status: Implemented (M05-967, batch B114)

This contract narrows the `merge_decision_row` and `generated_artifact_notice`
components frozen in
[`m5-structured-artifact-review-component-matrix`](freeze_the_m5_structured_artifact_review_component_matrix.md)
(M05-964) into implemented, export-safe review controls. It keeps structured
merge decisions and generated-artifact boundaries explicit *before* a user
mutates or resolves, so a generated, lockfile, manifest, or policy-owned conflict
never masquerades as an ordinary line merge, and the flow states clearly when
regenerate-first or manual resolution is safer than a direct write-back.

- Boundary schema: [`schemas/ui/m5-merge-decision-generated-notice-controls.schema.json`](../../../schemas/ui/m5-merge-decision-generated-notice-controls.schema.json)
- Producer: `aureline_review::current_merge_generated_controls_export`
- Release proof: [`artifacts/release/m5-merge-decision-generated-notice-controls-proof/`](../../../artifacts/release/m5-merge-decision-generated-notice-controls-proof/)
- Protected fixtures: [`fixtures/ui/m5-merge-decision-generated-notice-controls/`](../../../fixtures/ui/m5-merge-decision-generated-notice-controls/)

## What the components carry

Every `MergeDecisionRow` reuses the frozen `M5ArtifactComponent` tag and answers,
from the row alone:

- **Object identity** (`object_path`, required) — the structured object or
  key-path in conflict.
- **Conflict class** (`conflict_class`: `ordinary_line_merge` /
  `generated_artifact_conflict` / `lockfile_conflict` / `manifest_conflict` /
  `policy_owned_conflict`) — a generated, lockfile, manifest, or policy-owned
  conflict is its own explicit class, never folded into an ordinary line merge.
- **Conflict kind** (`conflict_kind`, required) — the human-readable conflict
  description, for example "both modified".
- **Base / Current / Incoming / Result semantics** (`base_summary`,
  `current_summary`, `incoming_summary`, `result_summary`) — kept as four
  distinct slots, each required and non-empty.
- **Preserve-unknown-fields note** (`preserve_unknown_fields_note`), required for
  every non-ordinary class so structured merges never silently drop fields.
- **Resolution guidance** (`available_guidance`, `recommended_guidance`:
  `manual` / `regenerate_from_source` / `accept_current` / `accept_incoming` /
  `accept_both`) and a **write-back safety note** (`write_back_safety_note`,
  required).
- A **raw-context jump action** (`raw_context_action`, required) and the reused
  `schema_fidelity` and `rollback_posture`.

Every `GeneratedArtifactNotice` names the generated-from boundary:

- **Generated-from relation** (`generated_from_relation`) and **source-of-truth
  pointer** (`source_of_truth_ref`), both required — never hidden behind generic
  file chrome.
- **Generation state** (`generation_state`: `up_to_date` / `stale` / `diverged` /
  `generation_unknown`) and a **last-generated version or time**
  (`last_generated_label`, required).
- **Divergence note** (`divergence_note`), required when the artifact has diverged
  from its source.
- **Actions** (`available_actions`: `regenerate` / `open_source` /
  `compare_against_source` / `view_lineage` / `dismiss`).
- **Write-back restriction** (`write_back_restriction`: `compare_only` /
  `regenerate_only` / `write_back_allowed`) and a required restriction note.

## Derived honesty (the delta this lane enforces)

Resolution safety is *derived* from the conflict class by
`resolve_merge_decision_disclosure`:

- `ordinary_line_merge` is the only class where a direct write-back by picking a
  side is safe; all others require an explicit preserve-unknown-fields note.
- `generated_artifact_conflict` and `lockfile_conflict` make regenerate-first
  safer, so the row must offer `regenerate_from_source`
  (`regenerate_first_guidance_missing`).
- `manifest_conflict` and `policy_owned_conflict` make manual reconciliation
  safer, so the row must offer `manual` (`manual_resolution_guidance_missing`).
- A non-ordinary conflict whose `recommended_guidance` is a direct side-accept
  (`accept_current` / `accept_incoming` / `accept_both`) is masquerading as an
  ordinary line merge (`ordinary_merge_misrepresented`).

Generated-artifact disclosure is derived from the generation state by
`resolve_generated_notice_disclosure`:

- a `stale` or `diverged` artifact must offer a `regenerate` action
  (`regenerate_action_missing`), and
- a `diverged` artifact must carry a divergence note (`divergence_note_missing`).

Every notice must offer an `open_source` action
(`open_source_action_missing`) so the source of truth is always reachable, and
its `write_back_restriction` must agree with its reused `rollback_posture`
(`write_back_restriction_inconsistent`) so a compare-only or regenerate-only
artifact is never silently promoted to a writable posture.

A `generated_artifact_conflict` merge row must be accompanied by a
generated-artifact notice for the same `artifact_ref`
(`generated_conflict_notice_missing`), so the regenerate-first path is always
visible where a generated artifact is being resolved.

## Coverage and invariants

- The merge rows must cover the `ordinary_line_merge`,
  `generated_artifact_conflict`, and `policy_owned_conflict` classes
  (`merge_conflict_class_coverage_missing`).
- The generated notices must cover the `up_to_date`, `stale`, and `diverged`
  states (`generated_artifact_state_coverage_missing`).
- The trust-review and consumer-projection blocks assert that Base / Current /
  Incoming / Result stay distinct, special conflicts are never ordinary line
  merges, unknown fields are preserved, regenerate-first or manual is stated when
  safer, generated-from relations stay explicit, stale/diverged state is
  disclosed, raw context is always reachable, and compare-only artifacts are
  never silently writable.

Raw artifact bodies, raw diffs, credentials, and live provider responses never
cross this boundary; the export is metadata-only and screened by an
export-material heuristic (`raw_boundary_material_in_export`).
