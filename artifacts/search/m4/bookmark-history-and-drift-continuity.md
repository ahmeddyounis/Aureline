# Bookmark, History, And Drift Continuity Artifact

Stable navigation continuity is represented by `schemas/search/bookmark-history-and-drift-continuity.schema.json` and implemented by `aureline_navigation::bookmark_history_and_drift_continuity`.

The checked packet preserves:

- durable anchors for breadcrumb trails, outline snapshots, navigation marks, history entries, and peek contexts
- the complete drift vocabulary: `bound`, `remapped`, `drifted`, `missing_target`, `scope_unavailable`, `archived`
- stable remap evidence with `used_nearby_fallback = false`
- restore rows that keep unresolved artifacts visible with reasons and recovery choices
- consumer projections for editor, diff, notebook, docs, search, and topology

The baseline fixture is `fixtures/search/m4/bookmark-history-and-drift-continuity/baseline_stable.json`.
