# Command reference beta contract (companion doc)

This page is the companion to the M3 command-reference / help-surface
beta. The detail panel, the in-product docs/help index, the CLI/headless
help renderer, the onboarding tip cards, and the support export all
read one canonical record per stable or beta command instead of
maintaining handwritten copies. Surfaces consume the same projection
minted by the seeded catalog in
[`crate::command_reference`](../../../crates/aureline-shell/src/command_reference/mod.rs).

Authoritative artifacts:

- [`/artifacts/ux/m3/command_reference_parity_report.md`](../../../artifacts/ux/m3/command_reference_parity_report.md)
  -- the rendered parity report generated from the seeded catalog.
- [`/fixtures/ux/m3/command_reference_and_discoverability/catalog.json`](../../../fixtures/ux/m3/command_reference_and_discoverability/catalog.json)
  -- the JSON snapshot of the catalog every surface consumes.
- [`/schemas/commands/command_reference_entry.schema.json`](../../../schemas/commands/command_reference_entry.schema.json)
  -- the boundary schema the fixture conforms to.

## What the catalog promises

For every claimed stable or beta command, the catalog publishes one
[`CommandReferenceEntry`](../../../crates/aureline-shell/src/command_reference/mod.rs)
that exposes:

| Field | What it carries |
| ----- | --------------- |
| `command_id`, `command_revision_ref`, `canonical_verb`, `primary_label_ref`, `title`, `summary` | Stable identity and the user-facing label/summary every surface quotes. |
| `lifecycle_state`, `origin_class`, `risk_class`, `preview_class`, `idempotency_class`, `supports_dry_run` | Lifecycle badges, risk class, preview / dry-run posture, and idempotency hints. |
| `aliases` | Every alias the descriptor owns, with `alias_kind`, `lifecycle_state`, `introduced_version`, `retirement_version`, `replacement_command_id`, `replacement_note_ref`, and `import_impact_class`. |
| `deprecation` | Top-level deprecation truth: `state`, `deprecated_in_version`, `retires_in_version`, `replacement_command_id`, `import_impact_class`, and the canonical migration note ref. |
| `argument_schema` | Typed argument slots with `is_required`, `default_provenance_when_omitted`, and `policy_pinned_when_trust_state_is`. |
| `availability` | Trust gate, policy gate, dependency presence, supported surfaces, and the current structured disabled reason codes / explanation refs the palette, CLI, and help surfaces show. |
| `keybindings` | Default, overriding, shadowed, conflict, and unassigned chord facts per `platform_variant` (`macos`, `windows`, `linux`, `all`). |
| `automation` | Headless / recipe / macro / AI eligibility and the canonical automation labels. |
| `search_index` | Tokens keyed by `human_label`, `command_id`, `canonical_verb`, `alias_id`, and `key_sequence` so search by label, id, alias, or chord resolves to the same entry. |
| `discoverability_links`, `docs_help_anchor_ref`, `migration_notes_refs` | Cross-surface anchors back to docs/help, onboarding, palette, and migration notes. |

## What the validator rejects

The validator
([`validate_command_reference_catalog`](../../../crates/aureline-shell/src/command_reference/validation.rs))
rejects:

1. an empty catalog;
2. a duplicated `command_id`;
3. an entry whose `record_kind`, `schema_version`, or
   `shared_contract_ref` does not match the pinned contract;
4. a missing title, summary, primary label ref, canonical verb,
   command revision ref, or docs/help anchor;
5. an entry with an empty `search_index`, a missing `human_label`
   token, or a missing `command_id` token, so palette / docs / CLI
   search cannot regress to label-only resolution;
6. an entry with no supported surfaces;
7. a deprecated entry that does not name a replacement command id;
   and
8. a deprecated alias that does not declare a retirement version and
   replacement command id.

## How surfaces consume the catalog

- **In-product command detail panel.** The shell consumes the
  [`CommandReferenceEntry`](../../../crates/aureline-shell/src/command_reference/mod.rs)
  record so the keyboard-reachable detail surface shows the same
  identity, lifecycle, alias/deprecation truth, argument schema,
  availability, keybindings, and automation facts the parity report
  publishes.
- **Command palette deep row.** The palette resolves a row to its
  reference entry through the same `command_id`, so quoting the
  detail surface is one stable join. Search by label, id, alias, or
  literal key sequence uses the same
  [`search`](../../../crates/aureline-shell/src/command_reference/search.rs)
  index.
- **CLI / headless help.** The `aureline_shell_command_reference`
  binary is the only mint-from-truth path for the JSON fixture and
  markdown report. The same record drives the CLI help output so
  `cmd:*.help` and the docs/help index stay in agreement.
- **Docs/help, onboarding, and migration.** Each entry quotes the
  canonical `docs_help_anchor_ref` and the `migration_notes_refs`,
  so docs/help and onboarding link back to the same record the
  detail panel renders.
- **Support export.** Support flows pivot from a case to the
  canonical reference entry by stable `command_id`, so disabled
  reason codes, alias lifecycle, and deprecation truth quoted in a
  support transcript stay aligned with what the user saw.

## Accessibility expectations

The detail surface is keyboard reachable end-to-end:

- The reference entry is addressed by stable `command_id`, not by
  hover-only chip or pointer-only menu, so screen readers and
  keyboard-only navigation never depend on pointer affordances.
- Identity, lifecycle badge, risk class, preview class, and current
  disabled reason codes are first-class fields on the entry, so the
  in-product surface can narrate them without requiring secondary
  fetches against external docs.
- Every keybinding fact carries an explicit `binding_state`
  (`default`, `overriding_user_binding`, `shadowed_by_user_binding`,
  `conflict`, or `unassigned`) so users can inspect why a chord is or
  is not active without trial-and-run.

## Regenerating the catalog and report

The catalog and every fixture under it are regenerated through the
headless minter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_command_reference -- catalog \
  > fixtures/ux/m3/command_reference_and_discoverability/catalog.json
cargo run -q -p aureline-shell --bin aureline_shell_command_reference -- report-md \
  > artifacts/ux/m3/command_reference_parity_report.md
```

After regeneration, the fixture-protected integration test asserts
the JSON catalog and the markdown report match the seeded projection
bit-for-bit:

```sh
cargo test -p aureline-shell --test command_reference_fixtures
```
