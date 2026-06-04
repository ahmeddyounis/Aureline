# Bookmark, History, And Drift Continuity

This contract defines the stable navigation packet used by editor, diff, notebook, docs, search, and topology consumers when a breadcrumb, outline node, bookmark, recent location, back/forward entry, or peek return context is reopened after source, scope, trust, branch, or index drift.

The packet is metadata-only. It carries durable anchors, provider/source refs, freshness, partiality, scope identity, drift state, remap evidence, restore reasons, and recovery choices. It does not carry source bodies, raw provider payloads, credentials, or ambient pane state.

## Required Drift States

Every stable packet must preserve the full drift vocabulary:

- `bound`: the target still resolves exactly under the active workspace, trust, and scope contract.
- `remapped`: the target resolved through stable remap evidence.
- `drifted`: the target changed, but opening it would require user review.
- `missing_target`: the target no longer resolves.
- `scope_unavailable`: the target is outside the active workset, trust, docs pack, branch, or remote/index scope.
- `archived`: the artifact is retained as archive or tombstone metadata.

Surfaces must render the same vocabulary and must not collapse these states into a generic unavailable label.

## Remap Rules

Resolution order is fixed:

1. Bind to the canonical anchor when it still resolves.
2. Remap only through stable evidence such as filesystem identity, symbol identity, docs-pack entry identity, graph remap edge, or explicit rename/move history.
3. Preserve the artifact with a drift reason and recovery choices when stable evidence is absent.

Nearest-line, nearest-symbol, path-similarity-only, or nearby docs-section fallback is not a stable remap. Those cases must remain `drifted`, `missing_target`, or `scope_unavailable` until explicit review.

## Restore Rules

Session restore may reopen a navigation artifact only when the target resolves under the current workspace, trust, and scope contract. When exact resolution fails, restore must preserve the artifact, attach the visible reason, and offer bounded recovery choices. Deleting the artifact or pretending a guessed target is authoritative is invalid.

## Consumer Contract

Each packet includes projections for `editor`, `diff`, `notebook`, `docs`, `search`, and `topology`. A projection is stable only when it preserves export-safe IDs, the full drift vocabulary, provider/source refs, restore reasons, and origin/destination/scope attribution.

Support/export packets can reconstruct what the user saw from this packet alone: origin, destination, scope, drift reason, remap evidence, and recovery choices.
