# Live appearance change & evidence linkage

A live OS appearance change — the user flips dark mode, raises contrast, enables
forced-colors, changes the accent, bumps the display scale, or turns on
reduce-motion while Aureline is running — is governed runtime behavior, not a
happy-path static screenshot. A claimed M5 appearance row is only truly
qualified if it survives that live change without corrupting layout, hiding
state meaning, or producing evidence nobody can attribute back to the build,
theme package, and appearance session that produced it.

This lane closes the gap between appearance qualification and runtime reality.
Every live OS appearance change is projected as one row in a single report
object that the release/evidence center, the support/export wrapper, the
extension-inspection surface, and sync/import all consume instead of restating
appearance behavior by hand.

## Where the truth lives

| Artifact | Path |
| -------- | ---- |
| Typed source of truth | [`crates/aureline-shell/src/live_appearance_evidence/mod.rs`](../../crates/aureline-shell/src/live_appearance_evidence/mod.rs) |
| Boundary schema | [`schemas/ux/m5-live-appearance-evidence.schema.json`](../../schemas/ux/m5-live-appearance-evidence.schema.json) |
| Report fixture | [`fixtures/ux/m5/os-appearance-contrast-accent/report.json`](../../fixtures/ux/m5/os-appearance-contrast-accent/report.json) |
| Support-export fixture | [`fixtures/ux/m5/os-appearance-contrast-accent/support_export.json`](../../fixtures/ux/m5/os-appearance-contrast-accent/support_export.json) |
| Published report | [`artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md`](../../artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md) |
| CI gate | [`tools/ci/m5/live_appearance_evidence_check.py`](../../tools/ci/m5/live_appearance_evidence_check.py) |
| Headless inspector | `aureline_shell_m5_live_appearance_evidence` |

The records are inspectable, serde-serializable truth packets. They carry no raw
screenshots, raw pixel data, raw paths, or raw user content — only opaque capture
refs, closed vocabulary, counts, and short labels. The closed appearance
vocabulary is re-exported by reference from the already-frozen appearance-session
and appearance-parity contracts; this lane mints no parallel appearance values,
only the live-change-specific `os_appearance_signal`, `evidence_capture_kind`,
and `golden_match_state`.

## What every row proves

Each row binds one live OS appearance change, on one platform, to one
attributable exact-build evidence capture:

- **Apply posture, disclosed up front.** A change either `applies_live` through
  the appearance-session model, or carries an explicit `requires_surface_reload`
  / `requires_app_restart` posture, or honestly discloses
  `platform_signal_unavailable`. A reload/restart requirement is never discovered
  after a broken live update.
- **Exact-build evidence linkage.** Every screenshot and golden capture carries a
  `capture_attribution` naming the build identity, release channel, theme package
  and revision, appearance session, checkpoint, platform, and OS signal that
  produced it. A release reviewer can always prove which build, package, and
  session generated a capture. The build identity a live runtime stamps comes
  from `aureline_build_info::exact_build_identity_ref`; the checked-in fixtures
  use a frozen representative value so they stay reproducible.
- **Golden attribution.** A qualified row's golden capture must match a baseline
  (`matched` or `diff_within_tolerance`); a `mismatch` or `no_baseline` capture
  cannot back a claim.
- **Cue preservation.** Trust, severity, lifecycle, and focus cues, plus state
  semantics and layout, are captured across the live change. A high-salience
  surface may never fall back to "not applicable" or hide its cue under a live
  transition.
- **No single-platform or static-only wins.** A marketed appearance axis must be
  proven on at least two platforms and with a live-transition capture — never on
  one platform alone, and never with only a static happy-path screenshot.

## The OS appearance signals

| Anchor | OS signal | Axis |
| ------ | --------- | ---- |
| System theme flip {#system-theme-flip} | `system_theme_flip` | `follow_system` |
| Increase contrast {#contrast-increased} | `contrast_increased` | `contrast` |
| Forced colors enabled {#forced-colors-enabled} | `forced_colors_enabled` | `contrast` |
| Accent color changed {#accent-color-changed} | `accent_color_changed` | `accent` |
| Text / display scale increased {#text-scale-increased} | `text_scale_increased` | `text_scale` |
| Reduce-motion enabled {#reduced-motion-enabled} | `reduced_motion_enabled` | `reduced_motion` |

### Honest platform omission {#forced-colors-portable-omitted}

A portable build that does not register the OS forced-colors handler narrows the
row to `platform_omitted` with a disclosed `narrowing_reason` and a
`platform_signal_unavailable` posture, instead of faking a live forced-colors
capture. An honest omission is accepted; it does not count toward cross-platform
or surface coverage and does not block the report.

## Surfaces a live change may not corrupt

Live changes are exercised across the surfaces whose state meaning matters most:
notebook cell chrome, data result grids, profiler and trace panels, preview-route
badges, docs/browser panes, and companion surfaces. Every one of these required
surface families must be exercised by at least one qualified row.

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- validate
cargo test -p aureline-shell --test m5_live_appearance_evidence_fixtures
python3 tools/ci/m5/live_appearance_evidence_check.py --repo-root .
```

Regenerate the fixtures and the published report from the seed:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- report > \
  fixtures/ux/m5/os-appearance-contrast-accent/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- support-export > \
  fixtures/ux/m5/os-appearance-contrast-accent/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- compact > \
  fixtures/ux/m5/os-appearance-contrast-accent/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- markdown > \
  artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md
```
