# Representation-labeled copy/export alpha

This document defines the bounded alpha contract for copy and export actions
on risky diff, review, search, and package/install review surfaces.

The canonical implementation lives in
`crates/aureline-content-safety::representation_copy_export`. The protected
fixture corpus lives in
`fixtures/content_safety/representation_copy_export_alpha/`.

## Source Contracts

- `docs/ux/shell_interaction_safety_contract.md`
- `schemas/ux/interaction_safety.schema.json`
- `docs/security/safe_preview_trust_classes.md`
- `docs/git/diff_view_alpha.md`
- `docs/search/search_query_session_contract.md`
- `docs/package/package_action_contract.md`

## Contract

Every protected surface emits one structural packet with:

- surface class: `diff`, `review`, `search`, or `package`;
- shell interaction-safety surface class;
- target boundary ref;
- provenance refs;
- inspect or reveal paths before risky transfer;
- reopen, history, or recovery affordances;
- copy/export actions with explicit representation class; and
- a `copy_export_representation_record` for every action.

Default copy actions must be raw or plain-text safe. Rendered, richer,
context-bearing, or export-packet actions must use an explicit label class
such as `copy_rendered`, `copy_with_context`, or `copy_export_packet`.
Sensitive values, including private paths, registry handles, and support
bundle links, must require preview before reaching the clipboard.

## Protected Surface Rules

| Surface | Required default | Required risky path |
|---|---|---|
| Diff | Plain-text or raw line copy | Raw/escaped reveal plus reopen closed diff |
| Review | Plain-text review anchor copy | Review context export preserves anchor provenance |
| Search | Plain result identity copy | Context copy previews private path material |
| Package/install review | Plain package coordinate copy | Export-packet and metadata-only actions preview sensitive handles |

## Validation

The validator checks structure, not visible text. It proves that:

- all four protected surfaces are present exactly once;
- each surface has one raw/plain safe default;
- every action mints an interaction-safety copy/export record;
- sensitive copy/export actions require preview;
- ambiguous surfaces expose inspect or reveal paths; and
- at least one reconciliation group spans multiple surfaces while preserving
  representation, target boundary, provenance, and recovery affordances.

Run:

```sh
cargo test -p aureline-content-safety --test representation_copy_export_alpha
cargo run -q -p aureline-content-safety --bin representation_copy_export_alpha -- fixtures/content_safety/representation_copy_export_alpha/protected_cross_surface_copy_export.json
```

The protected fixture currently validates diff/review and search/package
reconciliation groups with no remaining mismatches.
