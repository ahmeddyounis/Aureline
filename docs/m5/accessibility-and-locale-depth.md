# M5 accessibility and locale depth (companion doc)

This page is the companion to the M5 accessibility-and-locale qualification
audit. It carries the stable v1 accessibility and localization contract forward
into the M5 depth lanes: every new pane — notebook cells, result-grid rows,
pipeline/log views, profiler timelines, guided tours, docs/help panes,
companion surfaces, query consoles, preview-route panes, glossary panels, and
support packets — must stay a first-class citizen for keyboard users,
assistive-technology users, high-zoom users, IME users, right-to-left locales,
and localized profiles, instead of working only for a sighted mouse user on an
English, left-to-right, single-script happy path. No rich, structured, or
translated content path may silently corrupt text, drop narration, hide a focus
indicator, or narrow a localized row to English-only guidance.

The audit data, the per-surface blocking findings, the per-row coverage
numbers, the locale-anchor index, and the narrowable-marketed-rows list all come
from one mint-from-truth path — the seeded audit in
[`crate::m5_inclusive_depth`](../../crates/aureline-shell/src/m5_inclusive_depth/mod.rs)
— so the live shell accessibility/locale inspector, the CLI/headless inspector,
the support-export wrapper, the cross-surface hardening matrix, the
release-center packets, and the CI gate never disagree on what each M5 surface
certifies across the inclusive scenario rows.

Authoritative artifacts:

- [`/artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md`](../../artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md)
  — the rendered audit generated from the seeded projection.
- [`/fixtures/a11y/m5_ime_bidi_pseudoloc/report.json`](../../fixtures/a11y/m5_ime_bidi_pseudoloc/report.json)
  — the JSON snapshot of the same record consumed by every surface.
- [`/fixtures/a11y/m5_ime_bidi_pseudoloc/support_export.json`](../../fixtures/a11y/m5_ime_bidi_pseudoloc/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/a11y/m5-depth-qualification.schema.json`](../../schemas/a11y/m5-depth-qualification.schema.json)
  — the boundary schema the fixtures conform to.

The CI gate that enforces all of this is
[`tools/ci/m5/inclusive_depth_check.py`](../../tools/ci/m5/inclusive_depth_check.py).

## The nine inclusive scenario rows

The audit certifies exactly the nine inclusive scenario rows every marketed M5
surface must pass, grouped by the inclusive dimension Aureline already claims:

| Row | Dimension | Meaning |
| --- | --------- | ------- |
| `keyboard_reachability` | interaction | The surface is fully reachable and operable by keyboard, with no focus trap. |
| `screen_reader_narration` | interaction | The surface is narrated truthfully by assistive technology (role, name, state). |
| `high_zoom` | interaction | The surface reflows at high zoom without clipping or stranding content. |
| `ime_composition` | text | The surface preserves IME composition (pre-edit) and commits it correctly. |
| `grapheme_correctness` | text | The surface renders combined clusters, emoji, and escaped text exactly. |
| `bidi_direction` | text | The surface isolates bidi / mixed-direction content without leakage. |
| `pseudolocalization` | localization | The surface survives pseudolocalized strings without truncation or breakage. |
| `locale_fallback` | localization | The surface falls back honestly when a locale string is missing, never silently English-only. |
| `translated_help_parity` | localization | Translated help, tour, and glossary copy keeps parity with the feature packet. |

These are the inclusive conditions Aureline already claims — the audit certifies
and hardens them across the new M5 panes; it does not promise locale coverage
beyond the locales the surface can currently qualify. Unsupported locales or
translated-help assets are narrowed explicitly, never inferred.

For every registered M5 surface, each row carries a qualification binding. The
qualification status is one of:

- `qualified` — the row is qualified with captured inclusive evidence and quotes
  the canonical evidence fields (evidence pack, keyboard reachability,
  screen-reader narration, focus visibility, text correctness, evidence
  freshness, plus an IME-composition result on the IME row, a bidi-isolation
  result on the bidi row, a zoom-reflow result on the high-zoom row, and a
  locale-parity result on the localization rows).
- `explicitly_narrowed` — the surface narrows this row but names a
  `narrowing_reason`.
- `not_applicable` — the row does not apply to this surface; a reason is named.
- `locale_omitted` — the row is not surfaced for the claimed locales; a reason
  is named.
- `declared_capture_gap` — an extension- or provider-backed surface declares a
  known capture gap honestly, with a reason, instead of silently shipping an
  un-qualified row.
- `unqualified_local_a11y_path` — the surface drives this row through an ad-hoc
  local accessibility/locale path outside the shared inclusive-conformance
  harness. **Always blocking.**
- `missing_evidence` — a marketed row is claimed with no captured evidence.
  **Always blocking.**

A surface is "high-salience" when its descriptor pins a semantic salience of
`lifecycle_bearing`, `trust_bearing`, or `severity_bearing` — i.e. it conveys
lifecycle, trust, or severity meaning. A high-salience surface MUST keep a
present suspicious-content cue on every qualified row, so no inclusive scenario
can hide that meaning behind colour alone or strip it from narration.

## Captured evidence and text honesty

Every qualified row projects the captured evidence the row requires so the audit
can prove the surface was certified against the real inclusive condition, not
just the happy path:

- An evidence-pack ref, keyboard reachability, screen-reader narration, focus
  visibility, and text correctness are required on **every** qualified row. A
  missing field is a `missing_projection` blocker; a red result
  (`keyboard_unreachable`, `narration_silent`, `narration_misannounced`,
  `focus_indicator_hidden`, `text_corrupted`, `suspicious_content_hidden`) is a
  blocker in its own class. Text correctness audits the raw, rendered, and
  escaped copy paths plus decode-recovery so rich or structured content cannot
  undermine grapheme correctness.
- The IME row (`ime_composition`) additionally requires an IME-composition
  result; a `broken` result is an `ime_composition_broken` blocker, proving
  pre-edit composition survives intact rather than being dropped or committed
  early.
- The bidi row (`bidi_direction`) additionally requires a bidi-isolation result;
  a `leaking` result is a `bidi_leaking` blocker, proving mixed-direction runs
  stay isolated and do not reorder adjacent content.
- The high-zoom row (`high_zoom`) additionally requires a zoom-reflow result; a
  `clipped` result is a `zoom_content_clipped` blocker.
- The localization rows (`pseudolocalization`, `locale_fallback`,
  `translated_help_parity`) additionally require a locale-parity result; a
  `silent_english_fallback` or `mismatched` result is a `locale_parity_lost`
  blocker, so a localized row can never silently narrow to English-only guidance
  or drift out of date with the feature it documents.
- Stale evidence on a marketed row is a `stale_evidence_on_marketed_row`
  blocker, so a marketed row whose drills have aged out narrows instead of
  shipping as implicitly accessible or implicitly localized.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `unqualified_local_a11y_path`, `missing_evidence` — an ad-hoc local
  accessibility/locale path outside the shared inclusive-conformance harness, or
  a marketed row with no evidence.
- `keyboard_unreachable`, `narration_silent`, `narration_misannounced`,
  `focus_indicator_hidden`, `text_corrupted`, `ime_composition_broken`,
  `bidi_leaking`, `zoom_content_clipped`, `locale_parity_lost`,
  `suspicious_content_hidden` — a red captured result.
- `stale_evidence_on_marketed_row` — stale evidence on a marketed row.
- `dimension_drift`, `missing_narrowing_reason`, `missing_projection`,
  `missing_evidence_pack` — a binding whose dimension disagrees with its row, a
  non-qualified row with no reason, or a qualified row missing a required
  captured-evidence field.
- `descriptor_missing_locale_anchor`, `missing_inclusive_note`,
  `missing_claimed_locales`, `surface_not_on_inclusive_harness` — a descriptor
  with no locale/narration anchor, no inclusive note, or no claimed locale, or a
  surface that drives its own accessibility/locale path outside the shared
  inclusive-conformance harness.

## Consuming the audit and narrowing marketed rows

The cross-surface hardening matrix and the release-center packets ingest the
checked-in `report.json` directly when qualifying or narrowing a marketed M5 row
instead of cloning status text. The report's `narrowable_marketed_rows` list
names every marketed surface row whose accessibility or localized evidence is
stale or red, so release tooling can narrow that marketed M5 row on the affected
locales before publication instead of shipping the surface as implicitly
accessible or implicitly localized. In the clean checked-in audit that list is
empty.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- validate
cargo test -p aureline-shell --test m5_inclusive_depth_fixtures
python3 tools/ci/m5/inclusive_depth_check.py
```
