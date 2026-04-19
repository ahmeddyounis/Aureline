# Fixture: BOM, dominant newline, and final-newline state are preserved on save

## Scenario

The user opens four files in turn, edits each minimally, and saves:

1. UTF-8 with BOM, `\r\n` line endings, no final newline.
2. UTF-8 without BOM, `\n` line endings, with a final newline.
3. UTF-8 without BOM, mixed `\n` and `\r\n` (dominant `\n`), with
   a final newline.
4. UTF-16 LE with BOM, `\r\n` line endings, with a final newline.

For each file, the user's edit is one inserted character mid-file
followed by `Cmd-S`. No format-on-save participants run.

## Hooks exercised

- `buffer_open` — fires once per file.
- `text_edit_apply` — fires once per character insert.
- `transaction_apply` — fires once per text edit and once per
  save group.
- `save_pipeline_run` — fires once per save.

## Undo classes emitted

- `text_edit` (the user's edits)
- `save_participant_group` (the save itself, even with zero
  participants)

## Stack elements stressed

- Encoding detection stickiness: detected encoding is part of the
  saved snapshot and survives every save unless the user issues an
  explicit conversion command.
- BOM preservation: file opened with a BOM is saved with the same
  BOM; file opened without a BOM is saved without one.
- Dominant-newline detection and preservation; mixed-newline files
  preserve the dominant mode and surface the mix.
- Final-newline state preservation; adding or removing a final
  newline requires an explicit command.

## Expected observable outcomes

- File 1: saved bytes start with the UTF-8 BOM, all line endings
  remain `\r\n`, and the file still has no final newline. The
  status surface displays "UTF-8 BOM • CRLF • no final newline".
- File 2: saved bytes have no BOM, all line endings remain `\n`,
  and the final newline is preserved. The status surface displays
  "UTF-8 • LF • final newline".
- File 3: saved bytes have no BOM, the dominant `\n` newline mode
  is preserved, and the original `\r\n` lines are preserved
  verbatim (no opportunistic conversion). The status surface
  displays "UTF-8 • LF (mixed) • final newline".
- File 4: saved bytes start with the UTF-16 LE BOM, all line
  endings remain `\r\n`, and the final newline is preserved. The
  status surface displays "UTF-16 LE BOM • CRLF • final newline".
- A save that flips BOM, dominant newline, or final-newline state
  without an explicit conversion command is a contract violation.

## ADR sections motivating this fixture

- Source-fidelity rules — encoding detection at open, BOM
  preservation, newline preservation, final-newline state.
