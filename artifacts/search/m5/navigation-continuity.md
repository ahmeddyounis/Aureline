# Review artifact — navigation continuity: breadcrumb, outline, bookmark, history, peek

Packet id: `search.m5.navigation_continuity.v1`

This artifact is the reviewer-facing summary of the navigation-continuity layer
for the M5 editor, diff, notebook, docs, search, and graph surfaces. It is
produced from the seeded packet and is metadata-only.

## What this lane delivers

- Breadcrumb, outline, bookmark, recent-location, back/forward, and peek
  artifacts bound to canonical anchors across all six surfaces. The continuity
  vocabulary (drift states, surfaces, artifact kinds) is reused verbatim from
  `aureline-navigation`, and result-bearing surfaces bind to the durable,
  surface-independent result identity from the M5 search lane.
- Visible drift states (`drifted`, `missing_target`, `scope_unavailable`,
  `archived`) that keep a reason and recovery choices and never carry a resolved
  target — instead of silently relocating to a nearby symbol, line, or document.
- Remaps that bind to canonical anchors first and follow stable evidence only
  (`used_nearby_fallback = false`), so a bookmark follows a renamed symbol or a
  moved graph node through its stable id, never a nearest-target guess.
- A restore projection proving continuity artifacts reopen with their drift and
  missing-target reasons preserved instead of being dropped.
- A desktop continuity projection
  (`crates/aureline-shell/src/navigation_continuity/mod.rs`) that reuses the same
  artifacts, preserves drift states and origin/return anchors, and renders a
  drift as a visible, recoverable cue instead of a silent open.

## Acceptance evidence

| Acceptance criterion | Evidence |
| --- | --- |
| Back/forward, bookmark, outline, and breadcrumb artifacts remain attributable and do not relocate silently when anchors drift | All six surfaces realize all five artifact kinds and all six drift states; back/forward/recent history roles are all realized with origin anchors distinct from destinations; every drifted/missing/archived artifact keeps a visible reason, recovery choices, and no resolved target, and `used_nearby_fallback` is always `false`. |
| Restore can reopen continuity artifacts with visible drift/missing-target reasons instead of dropping them | The restore projection preserves several non-resolving artifacts (drifted, scope-unavailable, missing-target) with `artifact_preserved = true`, a visible `restore_reason`, and recovery choices. |
| Search, docs, graph, notebook, and diff surfaces share one continuity vocabulary and export model | The packet reuses the `aureline-navigation` drift/surface/artifact vocabulary, and the four consumer projections (`product_ui`, `history_back_forward`, `session_restore`, `support_replay`) set `reuses_same_continuity_objects`/`preserves_drift_vocabulary`/`preserves_drift_reasons`/`preserves_origin_destination = true`. |

## Guardrails enforced (fail-closed)

- A drifted/missing/scope-unavailable/archived artifact must keep a visible
  reason and recovery choices and must not carry a resolved target ref.
- A remap must cite stable evidence, resolve away from the canonical ref, and
  never use a nearest-target fallback.
- History entries declare exactly one role and keep an origin anchor distinct
  from the destination; peek keeps a return anchor.
- Restore must preserve a non-resolving artifact with a visible reason.
- Convenience routing must not widen authority; the packet stays metadata-only
  with hash-only session refs.

## Degraded variant

`workset_drift.json` proves that under a further-narrowed active workset a search
bookmark that was previously bound now drifts visibly (strictly more unresolved
artifacts than the canonical packet), while the surface, artifact-kind, and
drift-state vocabulary, the artifact identities, and the reused continuity
objects are preserved. The drifted bookmark survives restore with a visible
reason and is never relocated to a nearby in-scope result.

## Sources

- Contract doc: `docs/search/navigation-continuity.md`
- Schema: `schemas/search/navigation-continuity.schema.json`
- Fixtures: `fixtures/search/m5/navigation-continuity/`
- Model + tests: `crates/aureline-search/src/navigation_continuity/`
- Desktop consumer + tests: `crates/aureline-shell/src/navigation_continuity/`
- Canonical continuity vocabulary: `crates/aureline-navigation/src/bookmark_history_and_drift_continuity/`
