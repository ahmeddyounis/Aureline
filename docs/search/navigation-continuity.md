# Navigation continuity: breadcrumb, outline, bookmark, history, and peek

This document describes the navigation-continuity layer that keeps breadcrumb,
outline, bookmark, recent-location, back/forward, and peek artifacts bound to
canonical anchors across the M5 editor, diff, notebook, docs, search, and graph
surfaces. Where `aureline-navigation` owns the canonical drift vocabulary,
surface set, and artifact kinds, this layer binds that vocabulary to the M5
search-session and result-identity world so navigation continuity survives
edits, rename, workset narrowing, restore, and cross-surface jumps — and
degrades **visibly** instead of silently relocating to a nearby target.

- Schema: `schemas/search/navigation-continuity.schema.json`
- Packet model: `crates/aureline-search/src/navigation_continuity/mod.rs`
- Desktop consumer: `crates/aureline-shell/src/navigation_continuity/mod.rs`
- Canonical continuity vocabulary: `aureline-navigation` (drift states, surfaces, artifact kinds)
- Fixtures: `fixtures/search/m5/navigation-continuity/`

## Continuity artifacts

Each surface keeps a set of `ContinuityArtifactBinding` rows. Every artifact
separates the **canonical** anchor identity (resolved before any remap rules
run) from the **resolved** target, names its drift state, and — when the drift
state needs review — carries a visible reason and recovery choices:

| Field | Meaning |
| --- | --- |
| `artifact_kind` | `breadcrumb_trail`, `outline_snapshot`, `navigation_mark`, `navigation_history_entry`, or `peek_context`. |
| `surface` | `editor`, `diff`, `notebook`, `docs`, `search`, or `topology` (graph). |
| `canonical_target_ref` | The canonical anchor identity, resolved before remap. |
| `resolved_target_ref` | The current target; present iff bound or remapped, absent when drift needs review. |
| `result_id_ref` | The durable, surface-independent result URN, for result-bearing surfaces. |
| `origin_target_ref` | Origin anchor for history entries, return anchor for peek. |
| `history_role` | `back`, `forward`, or `recent`; present iff the artifact is a history entry. |
| `drift_state` | `bound`, `remapped`, `drifted`, `missing_target`, `scope_unavailable`, or `archived`. |
| `drift_reason` / `recovery_choices` | The user-visible reason and recovery, present iff the drift needs review. |

Because the same artifact objects are reused, the product UI, back/forward
replay, session restore, and support replay all land on **one** anchor identity
rather than reconstructing a near-miss target from rendered chrome text.

## Bind first, remap second — never relocate silently

Bookmarks bind to canonical anchors first and remap second. A remap is only
permitted with **stable evidence** (a stable symbol id, graph node id, or
filesystem identity) and never via a nearest-target fallback:
`used_nearby_fallback` must always be `false`. When an anchor cannot be resolved,
the artifact stays in a visible drift state instead of jumping to a nearby
symbol, line, or document:

- **Drifted.** The anchor changed and cannot be opened without review; it keeps
  a visible reason and recovery choices.
- **Missing target.** The anchor no longer resolves; the bookmark stays visible
  as missing rather than relinking.
- **Scope unavailable.** The target may exist but is outside the active
  workspace, trust, or scope contract; it is disclosed, not hidden.
- **Archived.** The artifact is intentionally retained as a tombstone.

A drifted, missing, scope-unavailable, or archived artifact never carries a
resolved target ref.

## Restore parity

`restore` proves session restore reopens continuity artifacts with their drift
and missing-target reasons preserved instead of dropping them. Each
`RestoredContinuityArtifact` that does not resolve under the current scope keeps
`artifact_preserved = true`, a visible `restore_reason`, and recovery choices, so
a restart never silently drops a bookmark or history entry or relocates it.

## Cross-surface and support reuse

The `surfaces` cover editor, diff, notebook, docs, search, and graph, and the
four consumer projections (`product_ui`, `history_back_forward`,
`session_restore`, `support_replay`) assert
`reuses_same_continuity_objects = true` and `widens_authority = false`. Back and
forward history and recent locations are all distinct, attributable roles that
preserve origin and destination anchors, so a support bundle inspects the exact
continuity artifact that was recorded.

## Guardrails enforced (fail-closed)

- A drifted, missing, scope-unavailable, or archived artifact must keep a visible
  reason and recovery choices and must not carry a resolved target ref.
- A remap must cite stable evidence, resolve to a target different from the
  canonical ref, and never use a nearest-target fallback.
- History entries declare exactly one `back`/`forward`/`recent` role and keep an
  origin anchor distinct from the destination; peek keeps a return anchor.
- Restore must preserve a non-resolving artifact with a visible reason instead of
  dropping it.
- Convenience routing must not widen authority; every artifact asserts
  `authority_not_widened = true`.
- The packet is metadata-only: no raw query text, source bodies, secrets, or
  private rank weights are admitted, and sessions are referenced hash-only.
