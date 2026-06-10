# Add notebook result comparison, baseline selection, and confounder visibility

## Overview

This document describes the M5 notebook result comparison, baseline selection,
and confounder visibility surface that keeps experiment lineage, review, and
reproducibility claims honest about result differences, baseline provenance,
and environmental confounders.

## Design principles

- **Result comparison is cell-aware by default** whenever stable cell IDs are
  present. When cell-aware comparison is impossible, the mode degrades
  explicitly to `raw_json_fallback` with truthful labels.
- **Baseline selection is explicit** about source (`last_successful_run`,
  `pinned_experiment`, `tagged_commit`, `manual_upload`, `workspace_snapshot`)
  and state (`selected`, `stale`, `unavailable`, `ambiguous`, `explicit_none`)
  so reviewers know what is being compared and why.
- **Confounder visibility is mandatory when results differ**. When the
  comparison outcome is `different`, at least one confounder record must be
  referenced so users do not silently attribute changes to code alone.
- **Confounders are classified** by type (`environment_drift`,
  `dependency_change`, `dataset_change`, `hardware_difference`,
  `kernel_restart`, `runtime_parameter_change`, `unspecified`) and visibility
  (`visible`, `suppressed`, `unknown`, `not_applicable`) so the review surface
  can show truthful explanations.
- **Raw JSON fallback remains available** as an explicit degraded state when
  semantic comparison cannot proceed safely.

## Comparison modes

| Mode | Typical use | Downgrade behavior |
|---|---|---|
| **Cell aware** | Per-cell source and output comparison | Default when stable IDs exist |
| **Output aware** | Output-only comparison | Used when source is unchanged |
| **Summary only** | High-level equivalence check | Used for large notebooks |
| **Raw JSON fallback** | Structural JSON diff | Explicit degraded state |

## Comparison outcomes

| Outcome | Meaning | Confounder requirement |
|---|---|---|
| **Equivalent** | Baseline and current match | None |
| **Different** | Baseline and current differ | At least one confounder ref required |
| **Baseline missing** | Baseline run is missing or unavailable | Baseline selection state explains why |
| **Current missing** | Current run is missing or not yet executed | None |
| **Incomparable** | Comparison mode or scope prevents valid comparison | None |

## Comparison scopes

| Scope | Typical use |
|---|---|
| **Full notebook** | Compare all cells |
| **Selected cells** | Compare a user-selected subset |
| **Active cell** | Compare only the currently active cell |

## Baseline sources

| Source | Meaning |
|---|---|
| **Last successful run** | Most recent successful execution |
| **Pinned experiment** | Explicitly pinned experiment run |
| **Tagged commit** | Baseline tied to a version-control tag |
| **Manual upload** | User-uploaded baseline artifact |
| **Workspace snapshot** | Baseline from a workspace snapshot |

## Baseline selection states

| State | Meaning | Baseline run ref required |
|---|---|---|
| **Selected** | Baseline is active and valid | Yes |
| **Stale** | Baseline is outdated but still present | Yes |
| **Unavailable** | Baseline cannot be resolved | No |
| **Ambiguous** | Multiple baselines match criteria | Optional |
| **Explicit none** | User chose no baseline | No |

## Confounder classes

| Class | Typical evidence |
|---|---|
| **Environment drift** | Environment fingerprint diff |
| **Dependency change** | Package or kernel dependency diff |
| **Dataset change** | Dataset card or source diff |
| **Hardware difference** | CPU/GPU/memory topology diff |
| **Kernel restart** | Runtime restart between runs |
| **Runtime parameter change** | Execution parameter diff |
| **Unspecified** | Confounder detected but not classified |

## Confounder visibility classes

| Class | Meaning |
|---|---|
| **Visible** | Confounder is shown to the reviewer |
| **Suppressed** | Confounder is hidden by policy or redaction |
| **Unknown** | Visibility cannot be determined |
| **Not applicable** | No confounder applies to this comparison |

## Records

### `NotebookResultComparison`

Carries a cell-aware comparison between a baseline run and a current run,
with mode, scope, outcome, and confounder refs so reviewers never silently
assume code is the only variable.

### `NotebookBaselineSelection`

Names how the baseline was chosen, its source, state, and pinning actor so
the comparison boundary is explicit.

### `NotebookConfounderVisibility`

Surfaces potential confounders that may explain a result difference
independent of source changes, with visibility classes so users know why
results may differ.

### `NotebookResultComparisonPacket`

Checked-in artifact that downstream docs, help, support, and CI surfaces
ingest instead of cloning status text.

## Schema

The boundary schema lives at:
`/schemas/notebook/add_notebook_result_comparison_baseline_selection_and_confounder_visibility.schema.json`

## Fixtures

Worked fixtures live at:
`/fixtures/notebook/m5/add_notebook_result_comparison_baseline_selection_and_confounder_visibility/`

## Integration

The crate `aureline-notebook` exposes these records and validators. Downstream
review, collaboration, experiment lineage, and export surfaces consume the
checked-in packet and closed vocabularies rather than redefining them.
