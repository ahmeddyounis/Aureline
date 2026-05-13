# Proof Packet: Alpha Appearance Visual-Diff

## Scope

This packet closes the M2 appearance lane for launch-wedge theme, density,
motion, imported-theme review, and visual-diff evidence.

## Canonical Evidence

- `artifacts/design/m2_appearance_visual_diff_alpha/manifest.yaml`
- `fixtures/design/m2_theme_density_motion/manifest.yaml`
- `docs/design/m2_theme_density_motion_alpha.md`
- `ci/check_m2_theme_density_motion.py`
- `artifacts/milestones/m2/captures/appearance_visual_diff_alpha_validation_capture.json`

## Runtime Consumers

- `crates/aureline-ui/src/themes/session.rs` exposes checkpointed apply/revert.
- `crates/aureline-ui/src/themes/package.rs` loads the first-party package manifest.
- `crates/aureline-ui/src/themes/import_review.rs` projects imported-theme mapping gaps.
- `crates/aureline-ui/src/themes/audit.rs` loads the visual-diff and accessibility manifest.
- `crates/aureline-shell/src/bootstrap/native_shell.rs` consumes the active appearance session for shell theme, density, and motion rendering.

## Acceptance Coverage

- Dark, light, high-contrast dark, and high-contrast light visual-diff rows are attributable to `appearance_session:profile.default:steady:01`.
- Compact, standard, and comfortable density rows preserve information architecture, focus visibility, and state conveyance.
- Reduced-motion and power-saver rows suppress non-essential AI, terminal, list, and decorative shell motion while preserving non-motion attention cues.
- Imported-theme review keeps translated, fallback, unsupported, and unresolved slots visible before commit and blocks full parity while unresolved rows remain.
- Trust, onboarding/import, and notification token changes require visual-diff and accessibility evidence before claim widening.

## Verification

```sh
python3 ci/check_m2_theme_density_motion.py --repo-root .
cargo test -p aureline-ui themes::
```
