# Filesystem truth review surfaces

This packet freezes four user-visible filesystem-safety surfaces for M5
file-bearing lanes:

- watch-fidelity strips;
- ignore-resolution drawers;
- compare-first external-change reviews; and
- cross-root move or rename reviews.

The goal is to keep local, remote, container, and generated roots honest
without forcing each shell surface to mint its own vocabulary for degraded
watch guarantees, absent rows, save conflict review, or boundary-changing
file operations.

## What the packet covers

The checked-in packet at
`artifacts/state/filesystem_truth_review.json` seeds one scenario for
each required root family:

| Root family | Surface lane | Watch truth | Ignore truth | External-change truth | Cross-root truth |
|---|---|---|---|---|---|
| `local_filesystem` | notebook document | reduced-fidelity watch with rename-pairing caveat | hidden checkpoint drawer | compare-first notebook drift review | local-to-container copy review |
| `remote_agent` | request workspace document | polling fallback | scope-limited remote results | revision-aware remote compare | remote-to-local copy review |
| `container_mount` | preview output artifact | reduced-fidelity container bridge | generated-overlay drawer | rendered-output compare | container copy-out review |
| `generated_managed` | provider local draft | provider-refresh-only | policy-hidden commentary | generated draft compare | generated-lineage detach review |

## Required invariants

1. Non-live watch modes carry at least one affected guarantee and visible
   actions.
2. Ignore drawers keep hidden-file, generated-overlay, policy-hidden, and
   scope-limited absence distinct.
3. External-change reviews keep `compare_to_disk` visible and forbid silent
   overwrite.
4. Cross-root reviews keep `preview_plan` visible before any proceed or
   copy-out path.
5. Fixtures must cover the same four root/surface lanes and lock one
   expected watch mode, ignore class, compare outcome, and boundary
   crossing per lane.

## Repo companions

- Schema: `schemas/state/filesystem_truth_review.schema.json`
- Artifact packet: `artifacts/state/filesystem_truth_review.json`
- Reviewer report: `artifacts/state/filesystem_truth_review.md`
- Fixtures: `fixtures/state/filesystem_truth_review/`
