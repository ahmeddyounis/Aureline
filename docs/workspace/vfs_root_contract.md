# VFS root and URI contract

This document defines the minimal VFS root abstraction and URI model used by
Aureline to normalize local filesystem files, generated documents, and virtual
documents behind one identity layer.

The normative vocabulary for identity layers, save coordination, watcher
posture, and root capability envelopes is frozen in:

- `docs/adr/0006-vfs-save-cache-identity.md`
- `docs/filesystem/filesystem_identity_vocabulary.md`

This document is an implementation-facing contract for:

- which URI shapes the VFS uses for **presentation**, **logical**, and
  **canonical** addressing; and
- what a VFS **root** must provide to resolve a document into the five-layer
  identity model without forcing consumers (editor, explorer, search, docs/help)
  to special-case by raw paths.

## Core rule: URIs, not paths

Every document is addressed by URIs:

- `presentation_path.uri` — the URI the user opened, preserved verbatim;
- `logical_workspace_identity.logical_uri` — the stable workspace-relative URI
  other subsystems talk about;
- `canonical_filesystem_object.canonical_uri` — the resolved canonical target
  the next mutation would affect.

No consumer may treat a platform path string as the canonical identity for a
document.

## URI shapes

The VFS recognizes the following URI families for first-generation (local +
generated + virtual) document classes.

### Local filesystem (`file://`)

Local files are represented using `file://` URIs for both presentation and
canonical identity.

Example:

- presentation: `file:///Users/example/project/src/main.rs`
- canonical: `file:///Users/example/project/src/main.rs`

### Workspace logical (`aureline-ws://`)

Workspace logical URIs identify the workspace object the product is tracking,
independent of aliasing or presentation-path changes.

Shape:

`aureline-ws://{workspace_id}/{root_id}/{logical_path}`

Example:

- `aureline-ws://ws-aureline-primary/root-1/src/main.rs`

### Virtual documents (`virtual://`)

Virtual documents are not ordinary host filesystem files (provider-backed
views, docs/help panes, transient inspector projections). Their canonical and
presentation URIs are `virtual://` URIs so no consumer needs to pretend they
are `file://` objects.

Shape:

`virtual://{workspace_id}/{root_id}/{document_id}`

Example:

- `virtual://ws-aureline-primary/root-virtual/docs/help/intro`

### Generated documents (`generated://`)

Generated documents are produced by a generator but still participate in the
same identity model.

Shape:

`generated://{workspace_id}/{root_id}/{document_id}`

Example:

- `generated://ws-aureline-primary/root-virtual/build/compile_commands.json`

## Root abstraction

A root is the unit of:

- capability disclosure (read/write posture, save modes, watcher source);
- canonical identity resolution (presentation → canonical, alias disclosure);
- generation-token observation for compare-before-write safety.

Every open is resolved by exactly one root, producing the identity record
(`presentation_path`, `logical_workspace_identity`, `canonical_filesystem_object`,
`alias_set`) and, where applicable, a save-target token.

## Implementation touchpoints

- Root adapters and root contract: `crates/aureline-vfs/src/roots/`
- URI model: `crates/aureline-vfs/src/uri_model.rs`
- Example consumer wiring (editor buffers): `crates/aureline-shell/src/bootstrap/native_shell.rs`

## Known limitations (current implementation slice)

- Local filesystem identity tokens and generation tokens are derived from host
  metadata with best-effort platform coverage; roots must continue to label
  degraded guarantees explicitly where a stronger token is unavailable.
- Virtual/generated documents default to inspect-only posture; mutation and
  promotion workflows (save-as, materialization, regenerate) are separate
  surfaces.

