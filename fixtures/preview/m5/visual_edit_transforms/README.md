# M5 Visual-Edit Transforms Fixtures

## visual_edit_transforms_round_trip_apply_fallback_and_protected_path.json

A round-trip, fallback, and protected-path drill fixture for the visual-edit
transform packet. The four visual-edit outcomes — exact-round-trip apply,
approximate-round-trip apply, code-first fallback, and inspect-only — are each
carried by at least one row, normalized onto one shared vocabulary.

The packet demonstrates the apply contract: an `exact_round_trip_apply` of a
literal `className` on a React preview that previews the real unified source diff
and takes a `checkpoint_revertible` rollback before apply; and an
`approximate_round_trip_apply` of a design-token value that previews a multi-file
source diff and a `snapshot_revertible` rollback on a `protected_review_required`
path. Both carry a transform manifest routed through the shared
`pipeline:preview-apply-revert` pipeline.

It also demonstrates the no-silent-rewrite contract: a `code_first_fallback` for
a `dynamic_bound_expression` that degrades to a code-first suggestion diff with
the selection preserved; an `inspect_only` row for a generated vendor stylesheet
node with no source span; and a Flutter `code_first_fallback` for a
loop-generated widget on a `protected_blocked` path, where the protected block
forces the degrade. Each fallback carries a precise, non-generic
unsupported-construct card whose reason is quoted verbatim, no transform manifest,
a `no_mutation_no_rollback` rollback class, and no review posture.

Because the React family carries both an `exact_round_trip_apply` row and an
`inspect_only` row, the fixture proves release/support surfaces can distinguish a
preview-only row from an exact-round-trip row on the same framework pack family.

The fixture validates against
`schemas/preview/visual_edit_transforms.schema.json` and is byte-aligned with the
in-crate builder via
`cargo run -p aureline-preview --example dump_m5_visual_edit_transforms`.
