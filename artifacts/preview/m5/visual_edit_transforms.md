# M5 Visual-Edit Transforms

- Packet: `m5-visual-edit-transforms:stable:0001`
- Label: `M5 Visual-Edit Transforms`
- Edits: 5 (2 round-trip applies, 3 fallbacks)
- Outcomes: 4 / 4
- Round-trip-vs-preview-only on a shared family: true

## Edits

- **edit:react:attr:0001** (exact_round_trip_apply) [react]
  - Exact round-trip edit of a literal className; the real source diff is previewed and a checkpoint is taken before apply
  - outcome=exact_round_trip_apply round_trip=exact_source_round_trip construct=static_attribute diff=real_source_unified_diff rollback=checkpoint_revertible protected=unprotected family=react
  - Manifest: `manifest:react:attr:0001` pipeline=`pipeline:preview-apply-revert` inverse=true
- **edit:react:token:0001** (approximate_round_trip_apply) [react]
  - Approximate round-trip edit of a design-token value across files; the multi-file diff is previewed and a snapshot backs the revert
  - outcome=approximate_round_trip_apply round_trip=approximate_source_round_trip construct=static_style_token diff=real_source_multi_file_diff rollback=snapshot_revertible protected=protected_review_required family=react
  - Manifest: `manifest:react:token:0001` pipeline=`pipeline:preview-apply-revert` inverse=false
- **edit:react:dynamic:0001** (code_first_fallback) [react]
  - A dynamically bound style cannot round-trip; the edit degrades to a code-first suggestion with the selection preserved
  - outcome=code_first_fallback round_trip=source_only_fallback construct=dynamic_bound_expression diff=code_first_suggestion_diff rollback=no_mutation_no_rollback protected=unprotected family=react
  - Fallback: reason=dynamic_binding — This style is bound to a runtime expression; the visual edit degrades to a code-first source suggestion rather than guess the binding
- **edit:react:generated:0001** (inspect_only) [react]
  - A generated vendor node has no source span; the surface stays inspect-only and never auto-upgrades to a write
  - outcome=inspect_only round_trip=inspect_only_no_write construct=external_or_generated_artifact diff=no_diff_inspect_only rollback=no_mutation_no_rollback protected=unprotected family=react
  - Fallback: reason=generated_or_external_artifact — This node comes from a compiled vendor stylesheet with no hand-authored span; the surface stays inspect-only and never writes back
- **edit:flutter:protected:0001** (code_first_fallback) [flutter]
  - A loop-generated widget on a blocked protected path cannot apply; the edit degrades to a code-first suggestion for owner review
  - outcome=code_first_fallback round_trip=source_only_fallback construct=conditional_or_loop_generated diff=code_first_suggestion_diff rollback=no_mutation_no_rollback protected=protected_blocked family=flutter
  - Fallback: reason=protected_path_blocked — This widget is generated inside a loop and its file is a blocked protected path; the edit degrades to a code-first suggestion the owner can review
