# Command diagnostics and invocation preview sheets contract

Command diagnostics and invocation preview sheets are protected review surfaces.
They project from the canonical command registry entry and invocation session so
shell surfaces never invent command identity, enablement reasons, preview
posture, or automation labels.

## Source-of-truth rules

1. **One command object.** Both sheets project from the same
   `command_registry_entry_record` used by dispatch, palette, keybinding help,
   and menus.
2. **One preflight result.** The sheets surface the same `preflight` decision
   and `enablement_snapshot` the dispatch path would enforce for the current
   surface context.
3. **Structured reasons.** Disabled and blocked state is represented by typed
   `decision_class` and `disabled_reason_code` values, not sheet-local prose.
4. **Sheet records are exportable.** The shell persists sheet records so
   support and parity audits can inspect structured truth without scraping
   rendered UI strings.

## Canonical packet

Both sheets quote one shared packet:

- `command_review_packet_record` — canonical command identity, typed arguments,
  argument provenance, and preflight posture.

This packet is the join point that keeps command ID, enablement reasons, and
preview posture aligned across palette preview, review sheets, dispatch, and
support export.

## Sheet records

The shell materializes one of these records per review surface:

- `command_diagnostics_sheet_record` — structured explanation of why a command is
  unavailable, including disabled-reason details and repair hooks when present.
- `command_invocation_preview_sheet_record` — structured preview of a pending
  invocation, quoting both the canonical packet and the in-flight
  `command_invocation_session`.

When review sheets are opened, the shell writes the records to:

- `.logs/review_sheets/`

## Fixtures

Fixtures pin representative sheet records in:

- `fixtures/commands/review_sheets/diagnostics/`
- `fixtures/commands/review_sheets/invocation_preview/`

They ensure diagnostics and preview sheets reuse the same command truth
vocabulary (command IDs, disabled-reason codes, and preflight decision classes)
as other command surfaces.

