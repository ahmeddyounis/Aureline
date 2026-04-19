# Fixture: bidi IME composition

## Scenario

An author types an Arabic word into an editor using a system IME on a
line that already contains bidi text. The IME emits composition-start,
several composition-update events as candidate segments change, and a
composition-commit event. The caret moves inside the composition range
during typing.

Representative sequence of OS events:

1. Cursor placed at column 14 of a line reading `let label = "";`.
2. IME composition starts between the quotes.
3. Composition string grows: `م` → `مر` → `مرحبا`.
4. Caret moves inside the composition range.
5. User confirms; composition commits as `مرحبا`.

## Hooks exercised

- `ime_composition_update` — fires on composition-start, every
  composition-update, and composition-commit.
- `caret_move` — fires for caret motion within the composition range.
- `selection_change` — MUST NOT fire unless the OS emits a selection
  delta inside composition.

## Stack elements stressed

- Platform-input adapter (winit-class input plus OS IME bridge).
- Overlay-layer composition painting; the composition string is
  drawn on the overlay layer so glyph-atlas state for the
  surrounding line does not invalidate.
- Optional `platform_native` shaper for the composition run if the
  surface's shaping policy declares OS-composed clusters are
  required.

## Expected observable outcomes

- `ime_composition_update` carries the composition string, underline
  segmentation, and caret-inside-composition position on every update.
- The glyph raster cache for the surrounding line is not invalidated
  during composition; only the overlay layer repaints.
- Committing composition invalidates the line-layout cache entry for
  that line exactly once.
- The accessibility tree announces the composition state per the
  bridge's composition protocol.

## ADR sections motivating this fixture

- Text shaping — platform-native shaper seam.
- Renderer stack — overlay-layer separation from glyph raster.
- Protected-hot-path hook list — `ime_composition_update`, `caret_move`.
