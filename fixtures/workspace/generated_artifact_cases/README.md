# Generated-artifact lineage cases

These fixtures cover the workspace generated-artifact lineage truth that
explorer and search surfaces consume to label generated rows distinctly from
canonical sources.

Each case captures:

- a small list of workspace-relative paths the catalog runs against, and
- the lineage record the workspace MUST emit per path (or `expect_match=false`
  when the path is *not* generated and the detector MUST return no hint).

The cases drive the integration test
`crates/aureline-workspace/tests/generated_artifact_cases.rs`. Run that test
to confirm the catalog stays aligned with the documented behavior.

| Case | Purpose |
| --- | --- |
| `lockfile_lineage.json` | Cargo lockfiles point back at their manifest sibling, including in nested workspace directories. |
| `build_output_no_canonical_sibling.json` | Build outputs and vendored snapshots are detected without a single canonical edit target. |
| `source_sibling_lineage.json` | Generated source siblings (`*.gen.rs`, `*.generated.ts`) point back at the matching hand-authored source. |
| `no_match_keeps_truth_explicit.json` | Failure drill: ordinary files and look-alike paths produce no hint rather than a guess. |

The reviewer-facing entry point for this surface is
`docs/workspace/generated_artifact_lineage.md`.
