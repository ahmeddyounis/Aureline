# M5 Project-Entry Component Consumer Proof

Release evidence for M05-842, the first-consumer adoption lane over the frozen
M5 project-entry component matrix (M05-836/839/840/841).

The packet certifies that the ten reusable project-entry component families —
start-center quick-action card, recent-work row, workspace-switcher entry,
restore-prompt card, entry-chooser row, entry-review sheet, destination-collision
sheet, post-entry handoff card, admission-checkpoint card, and archetype-readiness
row — are adopted as shared primitives across every claimed M5 entry surface:
Start Center / `Open recent` / command palette, system-open / file-association
intake, protocol / deep-link / browser-mobile handoff, CLI / headless entry, and
support / diagnostics + docs/help.

Every consumer keeps the identical entry-verb / literal-target / resulting-mode /
write-scope-trust-host-auth / restore-or-first-useful-work labels, the identical
`command_id` for a given entry verb (so entry surfaces never fork vocabulary by
client, trigger, or platform handoff origin), and the identical degraded-state
vocabulary. Deep-link and system-open rows preserve literal target and resulting
mode without special-case copy, and every row carries an opaque `entry_object_ref`
plus its canonical command id so support and automation can reconstruct the taken
entry path.

## Files

- `support_export.json` — canonical metadata-only packet (`include_str!`-embedded
  by the Rust module and asserted byte-aligned with the seeded builder).
- `matrix.csv` — one adoption row per line for release / support handoff.
- `report.md` — deterministic Markdown summary.

## Regenerate

```
cargo run -p aureline-shell --example dump_project_entry_component_consumers
```

- **Rust module:**
  `crates/aureline-shell/src/add_shared_start_center_system_open_deep_link_and_cli_headless_project_entry_component_consumers/`
- **Boundary schema:** `schemas/ui/m5-project-entry-component-consumer.schema.json`
- **Contract doc:** `docs/opening-projects/m5_project_entry_component_consumer_contract.md`
