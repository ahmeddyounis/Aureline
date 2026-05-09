# Large-file mode

Large-file mode exists to protect editor responsiveness and source-fidelity by
avoiding the normal piece-tree buffer path for oversized or hostile files.
Instead, the file opens in a constrained viewer backed by a bounded-memory
paged reader.

## When large-file mode activates

At open time, the classifier evaluates triggers in a stable order and takes the
first match:

1. **Size threshold**: the on-disk file size exceeds the configured threshold.
2. **Resource pressure**: opening the file would exceed the modeled soft memory
   budget.
3. **Classification**: the file appears binary, unusually minified/dense, or
   matches a configured pack suffix rule.
4. **Decode posture**: a decode recovery flow chose to open the file in
   large-file mode.
5. **Operator override**: the user explicitly chose to open in large-file mode.

The resulting decision carries a human-readable reason and a bounded sniff
summary so the UI can explain what happened without inventing its own local
truth.

## What the constrained viewer supports

Large-file mode is a distinct surface that is intentionally narrower than the
normal editor:

- Scroll, select, and copy
- Plain text search backed by streaming page reads

Large-file mode is read-only by default. Decorations, analysis, and semantic
features are reduced or disabled so the file can be inspected without dragging
the protected editor hot path out of budget.

## Explicit override (`Open anyway`)

When the user chooses `Open anyway`, the file is opened into the normal
piece-tree buffer path even if the classifier would activate large-file mode.
The normal document retains an explicit override record so downstream surfaces
can keep the boundary visible (for example: tab badges, status rows, or review
copy that explains which capabilities may be unavailable).

## Shell dogfood flow

To exercise large-file mode end-to-end in the desktop shell without needing a
100MiB+ fixture, lower the threshold via an environment override and open a
fixture file:

- Set `AURELINE_LARGE_FILE_THRESHOLD_BYTES=500`
- Open `fixtures/text/large/above_threshold_text.txt` in the shell (the file
  should open into the constrained large-file viewer buffer)
- Use `Ctrl/Cmd+Shift+O` to run the `Open anyway` override and re-open the file
  via the normal buffer path (the tab strip retains an explicit override token)

## Implementation notes

- Classification and viewer state live in `crates/aureline-editor/src/large_file/`.
- The constrained viewer uses a bounded-memory LRU page cache and does not load
  the whole file into RAM.
- Tests and fixtures live under `fixtures/editor/large_file_cases/`.
