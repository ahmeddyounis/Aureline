# Filesystem identity corpus

This directory is the fixture corpus for canonical path truth, alias-set
disclosure, case-only rename behavior, and save-coordination review.

The corpus is split into five parts:

- `identity_corpus_manifest.yaml`
  Stable case ids and the join table across save-truth fixtures,
  alias/symlink supplements, difficult save-target-review cases,
  rename-matrix rows, and reviewer-facing artifacts.
- `save_truth_cases/`
  Input-heavy save scenarios consumed by the VFS prototype.
- `alias_and_symlink_cases/`
  Smaller, reviewer-facing identity cases that focus on alias
  convergence, boundary disclosure, and degraded remote-root behavior.
- `difficult_save_review_cases/`
  The hardest filesystem fixtures the path-truth chip and the
  alias-inspector view must stay honest across (case-only rename
  with collision, symlink target shift mid-session, managed overlay,
  whole-file rewrite fallback, archive inner alias, bind-mount
  canonical drift). Each fixture pairs with a chip artifact in
  `artifacts/fs/path_truth_examples/` and an inspector-view
  artifact in `artifacts/fs/alias_inspector_examples/`.
- `case_only_rename_matrix.yaml`
  Explicit claimed, degraded, and unsupported rows for case-only rename
  behavior across root profiles.
- `conflict_save_cases/`
  Reviewer-facing conflict-aware save cases that freeze staging, fidelity
  validation, compare-before-write mismatch/uncertainty handling, review choices,
  and checkpoint/journal expectations for recovery-safe conflict resolution.

Rules:

1. Consumers join on `case_id` or `row_id`, never by guessing filenames.
2. Exact, degraded, and unsupported states stay explicit; a row may not
   collapse them into a generic "save failed" or "rename failed" label.
3. If the VFS prototype exports a scenario, that JSON should carry the
   `corpus_case_id` declared in `identity_corpus_manifest.yaml`.
