# Large-file decision-case fixtures

These fixtures anchor the large-file-mode switch conditions
frozen in [ADR 0003](../../../docs/adr/0003-buffer-undo-large-file.md)
to concrete inputs the
[`aureline-largefile-proto`](../../../crates/aureline-largefile-proto/)
prototype runs against. Each fixture targets exactly one of the
five switch conditions in the ADR's evaluation order
(size threshold → resource pressure → classification →
decode posture → operator override) so the prototype's harness
can show that every trigger fires deterministically against a
reviewable input.

The bytes are intentionally tiny. The trigger is what matters;
the production build replaces the prototype's tightened
thresholds with workspace-policy values (default `100 MiB` for
size, etc.) without changing the trigger vocabulary.

| Fixture | Switch trigger exercised | How it trips |
|---|---|---|
| [`clean_small_text.txt`](./clean_small_text.txt) | none (control) | Tiny clean text; expected to land in NORMAL mode under default policy. |
| [`above_threshold_text.txt`](./above_threshold_text.txt) | `size_threshold` | Larger than the dedicated scenario's tight `large_file_size_threshold`. |
| [`null_byte_blob.bin`](./null_byte_blob.bin) | `classification` (binary) | Sniff sees a NUL byte; classified as binary. |
| [`minified_long_line.js`](./minified_long_line.js) | `classification` (minified) | Sniff sees one line longer than the scenario's `minified_line_length`. |
| [`pack_suffix_clean.min.js`](./pack_suffix_clean.min.js) | `classification` (pack rule) | Path suffix `.min.js` matches the policy's pack-suffix table even though the sniff is clean. |
| [`decode_recovery_target.txt`](./decode_recovery_target.txt) | `decode_posture` | Sniff is clean; the scenario sets `decode_recovery_chose_large_file = true` to model the user choosing "open in large-file mode" from the decode-recovery surface. |
| [`operator_override_target.txt`](./operator_override_target.txt) | `operator_override` | Sniff is clean; the scenario sets `operator_override = true` to model the user explicitly opening the file in large-file mode. |

The harness embeds these files via `include_bytes!` so it runs
without depending on the workspace path layout at execution
time. The committed copies under this directory are the
authoritative source — regenerate the metrics seed
(`artifacts/bench/large_file_proto_metrics.json`) whenever a
fixture changes.

## What stays out

- **Multi-GiB inputs.** The prototype's switch conditions fire
  via tightened thresholds, not via real 100+ MiB files, so the
  repo stays small. Naming a construction recipe is fine; ship
  the recipe in the corresponding scenario, not as a checked-in
  blob.
- **Binary content beyond a few KiB.** `null_byte_blob.bin` is
  a small binary so the sniff trips reliably; production blobs
  do not belong here.
- **Bytes that depend on host locale.** Fixtures are
  byte-stable; the harness writes them verbatim and reads them
  back through the paged reader. No `\r\n` vs `\n` translation,
  no transcoding.
