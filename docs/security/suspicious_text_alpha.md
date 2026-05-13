# Suspicious text alpha source-surface contract

This note documents the first runtime-backed suspicious-text path for source
surfaces. The implementation lives in
`crates/aureline-content-safety/src/suspicious_text/` and consumes the shared
detector rather than creating per-surface detection logic.

## Covered surfaces

The alpha packet projects one detector run across:

- editor ranges
- diff hunks
- search result rows
- review anchors

Each surface receives the same suspicious-content class set:
`bidi_control`, `invisible_formatting`, and `mixed_script_confusable`.
Warnings carry byte offsets, character offsets, line and column indexes, the
surface subject ref, and a continuity ref that joins the same detector finding
across every surface.

## Copy, export, and review continuity

When suspicious text is present, every surface exposes:

- `copy_raw` for exact source bytes
- `copy_escaped` as the safe inspection path
- `export_sanitized_snapshot` with escaped source text and attached warning ids

The packet does not normalize or strip source text during projection. Review
reopen paths keep the warning refs attached to the exact editor snippet, diff
hunk, search row, or review anchor.

## Inspectable consumer

The CLI consumer prints the same packet that UI surfaces will consume:

```sh
cargo run -p aureline-content-safety --bin suspicious_text_alpha -- case:suspicious-text:stdin < input.txt
```

The command reads UTF-8 text from stdin and writes a
`suspicious_text_surface_packet` JSON document with editor, diff, search, and
review projections.

## Protected fixtures

Fixtures live under:

```text
fixtures/content_safety/suspicious_text_alpha/
```

They cover Python and TS/JS launch-wedge snippets with bidi controls,
invisible formatting, and mixed-script identifiers. The fixture test asserts
surface coverage, warning-class parity, raw/escaped reveal actions, safe copy
and export choices, warning continuity refs, and no silent normalization.
