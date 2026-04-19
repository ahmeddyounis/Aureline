# Fixture: decode failure preserves raw bytes and routes through recovery

## Scenario

The user opens a file whose bytes do not decode cleanly under the
detected encoding (for example, UTF-8 detection on a Shift-JIS or
mixed-encoding file). The buffer enters decode-recovery state and
preserves the original on-disk bytes verbatim. The user picks
"override encoding to Shift-JIS" from the recovery surface.

Representative content (raw byte fragment, intentionally malformed
under UTF-8):

```
0xE3 0x81 0x82 0x82 0xA0 0x82 0xC8 0x82 0xB3 0x82 0xA2
```

## Hooks exercised

- `buffer_open` — fires once for the open attempt.
- `decode_recovery_open` — fires once when the buffer enters
  decode-recovery state.
- `decode_recovery_resolve` — fires once when the user picks a
  resolution (override encoding, treat as binary, open in
  large-file mode).
- `transaction_apply` — fires once for the resulting
  `decode_recovery_change` transaction.

## Undo classes emitted

- `decode_recovery_change`

## Stack elements stressed

- Encoding detection at open: BOM → declared metadata → heuristic
  → user override.
- Raw-bytes preservation invariant: the original bytes survive the
  failed decode and are referenced by `original_bytes_handle`.
- Editing gated until recovery resolves; no edit transaction can
  commit while the buffer is in decode-recovery state.

## Expected observable outcomes

- The buffer surfaces a recoverable banner with three resolutions
  (override encoding, treat as binary, open in large-file mode).
- The original on-disk bytes are stored under
  `original_bytes_handle`; no flow rewrites or discards them
  during recovery.
- After the user picks "override to Shift-JIS", the buffer commits
  one `decode_recovery_change` transaction with `resolution =
  user_override_encoding` and `chosen_encoding = Shift-JIS`.
- The recovered text is now editable. A subsequent save preserves
  the chosen encoding (sticky encoding rule).
- A recovery path that loses the original bytes triggers the
  `original_bytes_lost` reopen trigger and is rejected by the
  journal.

## ADR sections motivating this fixture

- Source-fidelity rules — decode failure and recovery handoff.
- Undo-class taxonomy — `decode_recovery_change` row, including
  the `original_bytes_lost` reopen trigger.
