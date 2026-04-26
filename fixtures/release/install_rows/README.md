# Install-row fixtures

These fixtures exercise
[`schemas/release/install_row.schema.json`](../../../schemas/release/install_row.schema.json)
for install-profile cards and side-by-side import sheets.

- `portable_stable_profile_card.json` shows portable mode with all
  machine-global integrations suppressed and the portable state root
  colocated with the portable bundle.
- `side_by_side_preview_to_stable_import_sheet.json` shows first-run
  import from a preview install into a stable install, including compare,
  skip, checkpoint, rollback, file-association, and shared-state
  collision disclosures.
