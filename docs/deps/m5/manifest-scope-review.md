# Manifest-scope review: root-versus-member targets and source-trust cues

This document describes the **manifest-scope review** — the review/result object
that makes the *target* of an M5 package mutation explicit before it leaves
review: which manifest or workspace member is being changed, whether a
member-level operation broadens to the workspace root, and which
registry/mirror/auth/freshness/revocation path will resolve it. It is the
user-facing companion to the governed artifact at
`artifacts/deps/m5/manifest-scope-review.json`, the schema at
`schemas/deps/manifest-scope-review.schema.json`, and the typed model in the
`aureline-deps` crate (`manifest_scope_and_source_review`).

Where the
[package-state matrix](./freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.md)
*freezes the vocabulary* and the
[package-state descriptor](./package-state-descriptors.md) *describes one
package's state*, this object answers a monorepo-specific question: **was the
right manifest changed, and can the source it resolves from be trusted right
now?** One row is reused by the desktop workspace, CLI/headless, the review
workspace, AI context, and support/export packets, so root-versus-member truth
stays product-visible rather than hidden inside a per-ecosystem adapter.

## Manifest-scope selectors carry durable identity

Each row carries two [manifest-scope selectors] — the one the user *requested*
and the one the operation *resolved to*. A selector names:

- a **role** — `workspace_root`, `workspace_member`, `standalone_manifest`, or
  `path_or_vcs_target` — the root-versus-member distinction made explicit;
- a frozen **scope class** — the breadth of the operation (`whole_workspace`,
  `selected_manifest`, `workset_slice`, `workspace_member`, `path_or_vcs_target`)
  that the role must permit;
- a **durable manifest id** and a **continuity token** that survive an apply and
  a reopen unchanged;
- for a member, the **parent (root) manifest id**, so the root is always
  reachable from a member view.

A member selector may only resolve to itself or a selected manifest; only a root
selector can anchor a whole-workspace operation. That is how a member-level
change is stopped from claiming a workspace-wide scope.

## Root-versus-member operations never silently broaden

Each row carries a **scope fidelity** that classifies how the resolved scope
relates to the requested target:

- **exact** — nothing beyond the target changes;
- **disclosed shared lockfile** — a member change that necessarily updates the
  shared workspace lockfile; the broadening is disclosed, the member stays the
  target, and the operation is still appliable;
- **confirmed workspace-wide** — a whole-workspace operation that was explicitly
  confirmed;
- **unconfirmed broadening** — the resolved scope is wider than requested and was
  *not* confirmed. This row is **never appliable** (`can_apply` is `false`), so a
  member request can never quietly widen to the wrong manifest.

The `scope_diff()` projection renders the requested-versus-resolved manifest id,
role, scope class, the full affected-manifest set, and whether the change
broadened and was confirmed.

## Requested-versus-resolved identity stays separate

Each row reuses the descriptor's requested and optional resolved identity in
distinct fields. A policy pin is a **requested** constraint; the dependency
relation and resolved ref are **resolved** facts. The two label sets are disjoint
by construction, and the package's requested manifest scope must agree with the
requested selector, so requested-versus-resolved truth holds at both the package
and the scope level.

## Registry-source cues never overclaim trust

Anywhere a mutation or install review invites trust, the **registry-source cue**
says where the package comes from and whether that path can be trusted now:

- the **source class** (public/private/mirror/cache/offline) and, for a private
  registry or enterprise mirror, the redacted **mirror owner**;
- the **auth mode**, **freshness**, and **revocation state**.

A **revoked** credential or an **unsatisfied auth** requirement blocks trust, so
`can_apply` is `false`. A stale or offline source discloses itself but does not
block on its own. The cue's message class is always a specific, frozen source
disclosure and never a generic "package not found" or "install failed".

## The same object feeds every surface

Each row projects into:

- `view()` / `scope_diff()` — the desktop, review, and AI inspect surfaces;
- `source_view()` — the registry/mirror/auth/freshness/revocation cue;
- `export_row()` — the redaction-safe row reused by support/export packets and
  the CLI inspect surface;
- `surface_projection(surface)` — the row rendered for a marketed surface with
  the write authority that surface may carry, pinned from the frozen matrix, so
  only a mutating surface can apply an already-appliable row.

Every row binds to the frozen matrix through `references_matrix_id`, and every
label it surfaces resolves to a frozen state row, so product, CLI, and
support/export paths express the same governed vocabulary mechanically.

[manifest-scope selectors]: ./manifest-scope-review.md
