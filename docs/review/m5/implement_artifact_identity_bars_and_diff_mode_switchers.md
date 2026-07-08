# Implement artifact identity bars and diff-mode switchers

Status: Implemented (M05-965, batch B114)

This contract narrows the `artifact_identity_bar` and `diff_mode_switcher`
components frozen in
[`m5-structured-artifact-review-component-matrix`](freeze_the_m5_structured_artifact_review_component_matrix.md)
(M05-964) into implemented, export-safe review controls. It makes artifact class
and review mode obvious *before* a reviewer trusts a structured diff: the
identity bar names what the artifact is and where its canonical source of truth
lives, and the diff-mode switcher enumerates which review lenses exist, which is
active, and why any unavailable lens cannot be used.

- Boundary schema: [`schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json`](../../../schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json)
- Producer: `aureline_review::current_artifact_review_controls_export`
- Release proof: [`artifacts/release/m5-artifact-identity-diff-mode-controls-proof/`](../../../artifacts/release/m5-artifact-identity-diff-mode-controls-proof/)
- Protected fixtures: [`fixtures/ui/m5-artifact-identity-diff-mode-controls/`](../../../fixtures/ui/m5-artifact-identity-diff-mode-controls/)

## What the components carry

Every `ArtifactIdentityBar` reuses the frozen `M5ArtifactComponent` tag and
answers, from the bar alone:

- **Artifact class** (`artifact_class_label`) and **canonical source of truth**
  (`canonical_source_disclosure`, required and non-empty — canonical source is
  never buried in a distant panel).
- **Origin** (`origin_class`: `authored_in_repo` / `generated_from_source` /
  `imported_external` / `policy_owned`) — the generated / imported / policy-owned
  identity axis.
- **Parser/schema state** (`parser_schema_state`), reused directly from the
  frozen `M5ArtifactFidelityState` vocabulary.
- **Writable-target truth** (`claims_writable_target`) and **rollback posture**
  (reused `M5ArtifactComponentRollbackPosture`).

Every `DiffModeSwitcher` enumerates the review lenses as `DiffModeOption`s and
carries the active lens plus the compare-only-versus-write-back safety.

## Derived honesty (the delta this lane enforces)

Writable-target status is *derived*, never asserted directly, by
`resolve_artifact_identity_disclosure(origin, parser_schema_state)`:

- Only an artifact **authored in the repo** whose parser/schema state is
  `structured_faithful` or `structured_partial` is a writable target. A
  generated, imported, policy-owned, schema-unrecognized, untrusted, or redacted
  artifact can never masquerade as a plain editable file
  (`writable_target_misrepresented`, both directions).
- A **generated** artifact must name its generated-from relation
  (`generated_from_relation_missing`).
- A **non-authored** artifact must point at the canonical source of truth living
  elsewhere (`source_of_truth_pointer_missing`).
- A **narrowed** parser/schema state must keep an explicit raw/export-safe
  fallback note (`raw_fallback_note_missing`).
- Rollback posture must agree with the writable-target claim: only a writable
  target may carry `write_back_attributable` (`rollback_posture_inconsistent`).

The diff-mode switcher enforces:

- A raw/export-safe fallback lens is always available
  (`raw_fallback_lens_missing`) so a narrowed render or schema is never flattened
  without an escape hatch.
- The active lens is present and available (`active_lens_unavailable`).
- Every unavailable lens carries a reason (`lens_unavailability_reason_missing`)
  so a reviewer can always tell *why* a lens is off.

## Pairing (canonical-source and compare-only truth stay together)

Identity bars and diff-mode switchers are paired by `artifact_ref`: the set of
artifact references shown as identity bars must equal the set shown as
diff-mode switchers (`artifact_pairing_incomplete`). This keeps canonical-source
and compare-only truth beside the lens picker rather than in a distant panel.

## Regenerating the proof

```
GEN_ARTIFACT_IDENTITY_DIFF_MODE_ARTIFACTS=1 \
  cargo test -p aureline-review \
  implement_artifact_identity_bars_and_diff_mode_switchers_with_artifact_class_canonical_source_parser_schema_and_compare_only_truth::tests::generate_artifacts
```
