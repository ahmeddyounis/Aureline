# Canonical filesystem identity lineage — contract

This document describes the canonical filesystem identity lineage record:
the workspace's governed, export-safe projection that finalizes canonical
filesystem identity, the alias inspector, save-target review, and
wrong-target prevention into one record per posture.

The record is the single artifact every consuming surface (workspace
status, save-target review sheet, alias inspector, support export,
Help/About, headless CLI) ingests instead of cloning status text.

## Inputs

The projection ingests two live VFS records verbatim:

1. **`aureline_vfs::IdentityRecord`** — layers 1–4 of the filesystem
   identity model: presentation path, logical workspace identity,
   canonical filesystem object (canonical URI, normalization form,
   strongest and fallback identity tokens), and alias set with
   resolution chains.
2. **`aureline_vfs::SaveTargetToken`** — layer 5: capability flags,
   atomic-write mode, compare-before-write generation token pinned at
   open, permission snapshot, and the review-required gates the save
   pipeline enforces.

For determinism and replay, the projection also accepts a serializable
`CanonicalIdentityObservation` (a one-shot mirror of the
`SaveTargetToken`). Fixtures and the headless emitter use this form.

## What the record proves

The four claims the stable line is anchored on, specialized to canonical
filesystem identity:

- **Source fidelity.** Presentation URI is preserved verbatim; canonical
  URI carries its normalization form and strongest identity token so the
  identity can be re-derived without re-walking the filesystem.
- **Canonical-path truth.** The next save's write target is the
  resolved canonical URI, never the presentation URI; the alias inspector
  lists every alias of that canonical object.
- **Restore is no-rerun.** Wrong-target writes are structurally guarded
  by a pinned compare-before-write generation token plus a save-target
  review that names every blocker (read-only, policy-constrained,
  review-required, untrusted, divergent-unknown-alias) before any byte
  moves. A destructive action (the next save) is always reachable, so the
  compare-before-write inspection hook must be available before it.
- **Lineage / export honesty.** The record carries no raw source bytes
  (`raw_payload_excluded = true`) and the shared
  `FilesystemIdentityReferenceSet` is consistent across editor, Git,
  restore, and mutation flows.

## Narrow reasons

When a claim cannot be proven on the captured posture the record auto-
narrows below Stable with a named reason. Protective postures (read-only
roots, policy-constrained roots, review-required gates, divergent-unknown
aliases the save-target review correctly blocks) stay Stable — the
contract working as designed is a pass, not a gap.

| Narrow reason                          | Fires when                                                                            |
| -------------------------------------- | ------------------------------------------------------------------------------------- |
| `canonical_target_unresolved`          | The canonical URI is empty or `"unknown"`                                             |
| `presentation_alias_missing`           | Presentation differs from canonical but no alias entry explains the redirect          |
| `divergent_unknown_alias_unguarded`    | A divergent-unknown class open lacks the matching save-target review blocker          |
| `compare_before_write_not_pinned`      | The compare-before-write generation token has no value or no observed-at timestamp    |
| `save_target_misaddressed`             | `writes_to_canonical_uri` disagrees with the canonical URI on the identity record     |
| `untrusted_workspace_save_unguarded`   | Trust is not `trusted` but the save-target review lacks the untrusted_workspace blocker |
| `identity_reference_inconsistent`      | Editor / Git / restore / mutation refs disagree with the shared identity ref          |
| `destructive_action_no_compare_hook`   | The compare-before-write inspection hook is unavailable                               |
| `lineage_export_unsafe`                | The workspace or root ref is empty (would break support export)                       |

## Inspection hooks

A canonical-identity posture must always let the user inspect aliases,
review the save target, compare before write, export the record, and
repair without clearing local state. The default hook set has all five
hooks available; fixtures may model a degraded subset (e.g. an unavailable
compare hook) to prove the corresponding narrow reason.

| Hook class             | Action id                                          | Purpose                                              |
| ---------------------- | -------------------------------------------------- | ---------------------------------------------------- |
| `alias_inspect`        | `canonical_identity.show_alias_details`            | Open the alias inspector with resolution chains      |
| `save_target_review`   | `canonical_identity.review_save_target`            | Open the save-target review surface                  |
| `compare_before_write` | `canonical_identity.compare_before_write`          | Re-read the canonical generation token + diff        |
| `export`               | `canonical_identity.export_record`                 | Export the record for support without raw bytes      |
| `repair`               | `canonical_identity.re_resolve`                    | Re-resolve the presentation path against the VFS     |

## Consumer surfaces

The same projection is consumed by:

- The workspace canonical-identity status surface (path-truth chip and
  inspector).
- The headless CLI emitter
  (`crates/aureline-workspace/src/bin/aureline_canonical_identity_lineage.rs`).
- Help/About and support export (via `canonical_identity_lineage_lines`).
- The replay gate
  (`crates/aureline-workspace/tests/canonical_identity_lineage_replay.rs`),
  which re-projects every fixture and asserts equality.

The shared human-readable projection (`canonical_identity_lineage_lines`)
is the canonical text every surface quotes — none of them re-render their
own status text from the underlying fields.

## Schema and stability

The boundary schema is
[`schemas/workspace/canonical_identity_lineage.schema.json`](../../../schemas/workspace/canonical_identity_lineage.schema.json).
The record's `canonical_identity_lineage_schema_version` is currently `1`
and is owned by the workspace crate; any change to the projection must
update the schema, fixtures, and replay gate together.
