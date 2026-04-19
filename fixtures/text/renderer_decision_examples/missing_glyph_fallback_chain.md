# Fixture: missing-glyph fallback chain

## Scenario

A file contains a code point that the active editor font does not
cover and for which no user-configured fallback or script-aware group
resolves. The renderer must fall through to the bundled last-resort
subset without rendering `.notdef` boxes.

Representative content:

```
// Runic sample:  ᚠᚢᚦᚨᚱᚲ
// Ancient Egyptian sample:  𓂀𓆼𓏏𓏲
```

## Hooks exercised

- `fallback_glyph_resolution` — fires once per run per stage the
  fallback chain visits.

## Stack elements stressed

- Font-discovery abstraction on each claimed host (CoreText,
  DirectWrite, fontconfig).
- Script-aware preference groups for runic / Egyptian hieroglyphs (or
  the absence thereof on a host that does not ship those fonts).
- Bundled Noto-class subset shipped with the desktop binary.

## Expected observable outcomes

- The Runic run resolves through `fallback_glyph_resolution.stage`
  values reflecting whichever stages it visited, ending at either
  `3 (system_ui)` or `4 (bundled_last_resort)` depending on the host.
- The Egyptian hieroglyph run resolves at stage `4
  (bundled_last_resort)` on a host that does not ship hieroglyph
  coverage.
- Neither run produces `.notdef` boxes in the glyph buffer.
- The support bundle includes the `fallback_glyph_resolution` record
  for each stage visited so the substitution path is inspectable
  offline.

## ADR sections motivating this fixture

- Font discovery and fallback — deterministic chain through stages 1
  to 4.
- Fallback transparency — every run records which stage produced
  each glyph.
