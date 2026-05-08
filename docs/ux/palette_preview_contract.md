# Command palette preview pane and action-footer contract

The command palette preview pane is a protected inspection surface. It is a
projection over the canonical command registry entry and enablement engine; it
does **not** invent palette-local command identity, availability, or argument
truth.

This contract exists so users can answer, from the palette surface itself:

- what command is selected (stable `command_id` + `canonical_verb`);
- whether it can run right now (enablement decision + disabled reason);
- whether preview or approval is required before apply; and
- which inspection/copy actions are truthful for the selected command.

## Source-of-truth rules

1. **One command object.** Preview and footer project from the same
   `command_registry_entry_record` used for dispatch.
2. **One enablement result.** The preview must surface the same enablement and
   preflight decision that dispatch will apply for the current surface context.
3. **Structured reasons.** Disabled or blocked state is represented via typed
   `decision_class` + `disabled_reason_code` rather than palette-local prose.
4. **Copy does not execute.** Copy affordances export inspection tokens only.
   They never mint an execution bypass.

## Runtime payload

The shell materializes a `palette_preview_record` that is used by both the
preview pane and action footer:

- `selection.kind = command` includes `command_id`, `command_revision_ref`,
  `canonical_verb`, descriptor posture (`preview_class`, `approval_posture_class`),
  typed argument slots, and the evaluated enablement snapshot.
- `selection.copy.command_id` is always present for command rows.
- `selection.copy.cli_skeleton` is present only when the descriptor admits the
  `cli` client scope. The skeleton is non-executable and only names the
  canonical verb and typed argument slots.

The shell writes these records to `.logs/palette_previews/` when a copy action is
invoked so support and review flows can inspect structured command truth without
scraping rendered UI strings.

## Fixtures

Preview payload fixtures live in `fixtures/commands/palette_preview_cases/`.
They pin representative preview records for:

- an enabled command with copy-ID and copy-CLI skeleton affordances; and
- a disabled/policy-blocked command where copy-ID remains available.

