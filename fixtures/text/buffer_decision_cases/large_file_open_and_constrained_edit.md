# Fixture: large-file open enters reduced-capability mode and constrains edits

## Scenario

The user opens three files in turn:

1. A 250 MiB log file on a workspace whose
   `large_file_size_threshold` is the default 100 MiB.
2. A 4 MiB minified single-line JavaScript bundle classified by
   workspace policy as "minified pack".
3. A 30 MiB text file on a workspace whose threshold is set to
   25 MiB by policy override.

For each file, the user attempts: a viewport-bounded multicursor
edit (allowed), a whole-file multicursor edit (denied), a viewport
search (allowed), a whole-file format-on-save (denied), and a
single keystroke insertion (allowed).

## Hooks exercised

- `buffer_open` — fires once per file open.
- `large_file_mode_enter` — fires once per file as it crosses
  into large-file mode, recording which switch condition triggered
  the mode (size threshold, classification, or operator override).
- `text_edit_apply` — fires for each permitted edit.
- `transaction_apply` — fires for each permitted transaction.
- `large_file_mode_exit` — fires only if the user explicitly
  downgrades the buffer after the conditions no longer apply.

## Undo classes emitted

- `text_edit` (only for permitted edits; denied edits never reach
  the journal)

## Stack elements stressed

- Switch conditions in evaluation order: size threshold → resource
  pressure → classification → decode posture → operator override.
- Reduced-capability list: viewport-bounded variants permitted;
  whole-file variants denied.
- Mmap-backed paged reader plus paged-rope write overlay; the
  buffer never loads the whole file into RAM.
- Journal per-buffer cap: a transaction whose stored inverse
  exceeds the cap is rejected, surfaced via
  `journal_inverse_rejected`.

## Expected observable outcomes

- File 1: enters large-file mode with `trigger = size_threshold`
  (250 MiB > 100 MiB). Whole-file multicursor edit is denied with
  a recoverable banner explaining the reduced-capability list.
  Viewport search and single-keystroke insert succeed and emit
  `text_edit_apply`.
- File 2: enters large-file mode with `trigger = classification`
  (minified pack rule), even though 4 MiB is well under the size
  threshold. Whole-file format-on-save is denied; range format is
  permitted.
- File 3: enters large-file mode with `trigger = size_threshold`
  (30 MiB > workspace 25 MiB override). Behaviour otherwise
  matches File 1.
- For all three files, the accessibility tree publishes the
  visible viewport. Indexing and background analysis remain off.
- A buffer that loaded the whole file into RAM despite the switch
  condition is a contract violation.
- A buffer that exited large-file mode without explicit user
  downgrade is a contract violation; the buffer must emit a
  checkpoint at the mode transition.

## ADR sections motivating this fixture

- Large-file mode — switch conditions, reduced or denied
  features, backing store.
- Performance and memory assumptions — open latency at and above
  the size threshold, memory high-water mark across mode entry,
  edit, and exit.
