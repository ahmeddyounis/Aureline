# Notebook and Data-Rich Surface Qualification

Notebook and data-rich surfaces do not inherit Stable from editor, task,
runtime, debug, or release-center packets. The release packet at
`artifacts/release/m4/notebook-and-data-rich-surface-qualification.json` is the
source of truth for rendered labels and support posture.

Stable coverage is limited to:

- notebook document/runtime/output truth,
- notebook review/export with cell-aware diff and honest fallbacks,
- variable-explorer snapshots.

Preview coverage remains explicit for data tables, result grids, chart
summaries, experiment handoff cards, and API/database response viewers. Those
surfaces preserve row or artifact scope, freshness, lineage, and redaction mode,
but they are not marketed as Stable database, API-client, charting, or
experiment-platform depth.

Verification:

```sh
cargo test -p aureline-release --test notebook_and_data_rich_surface_qualification
```
