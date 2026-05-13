# M2 Theme, Density, Motion, and Visual-Diff Alpha

This page is the reviewer entry point for the M2 appearance lane. It binds the
runtime `aureline-ui` appearance APIs, protected fixtures, and visual-diff audit
manifest to one governed appearance-session object.

## Canonical Artifacts

- Runtime contracts:
  - `crates/aureline-ui/src/themes/session.rs`
  - `crates/aureline-ui/src/themes/package.rs`
  - `crates/aureline-ui/src/themes/import_review.rs`
  - `crates/aureline-ui/src/themes/audit.rs`
- Protected fixtures:
  - `fixtures/design/m2_theme_density_motion/manifest.yaml`
  - `fixtures/design/m2_theme_density_motion/imported_theme_review_before_commit.yaml`
  - `fixtures/design/m2_theme_density_motion/power_saver_motion_floor.yaml`
- Visual-diff and accessibility manifest:
  - `artifacts/design/m2_appearance_visual_diff_alpha/manifest.yaml`
- Validator:
  - `ci/check_m2_theme_density_motion.py`

## Runtime Contract

The session API now exposes checkpointed apply and rollback helpers so theme,
density, motion, overlay, and import changes can be committed as one session
revision. A stale checkpoint is refused without mutating the active session.

The theme-package projection loads the first-party package manifest and verifies:

- all four theme classes are declared;
- compact, standard, and comfortable density rows are supported;
- standard, reduced, low-motion, power-saver, and critical-hot-path postures are
  declared;
- per-mode contrast metadata is present.

The import-review projection loads the audited imported-theme mapping report and
keeps translated, fallback, unsupported, and unresolved slots visible before a
commit can claim parity.

## Evidence Contract

The visual-diff manifest ties dark, light, high-contrast dark, high-contrast
light, and compact shell captures to `appearance_session:profile.default:steady:01`.
Safety-critical trust, onboarding/import, and notification changes require both
visual-diff evidence and accessibility audit evidence before widening a claim.

OS-driven theme, contrast, text scale, and reduced-motion transitions cite the
existing transition fixtures. If a surface cannot apply live, the fixture must
declare the reload or confirm path rather than relying on prose.

## Verification

Run:

```sh
python3 ci/check_m2_theme_density_motion.py --repo-root .
cargo test -p aureline-ui themes::
```
