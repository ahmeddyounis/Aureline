# Symbolication packet fixtures

This corpus carries the checked-in M5 symbolication contract packet and
four representative report rows:

- `exact_local_report.json` - exact-build local symbolication for a crash stack.
- `approximate_mirrored_report.json` - approximate mapping after a visible mirrored lookup.
- `symbol_only_report.json` - exact-build symbol-only mapping with no source lines.
- `unresolved_mismatch_report.json` - rejected mismatched candidate that narrows to unresolved.
- `packet.json` - the full packet consumed by `aureline-debug`.

The packet mirrors:

- `schemas/debug/symbolication_contract.schema.json`
- `docs/debug/symbolication.md`
- `docs/execution/debug_truth_contract.md`

The fixtures remain metadata-only. They never carry raw dump bytes,
raw symbol bytes, raw source bodies, or raw URLs.
