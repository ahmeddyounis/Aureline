# Onboarding Alpha Contract

The first-run onboarding alpha is implemented as an exportable shell projection,
not a product tour. The canonical runtime type is
[`OnboardingAlphaSurfaceRecord`](../../crates/aureline-shell/src/onboarding/mod.rs),
with the machine-readable boundary at
[`schemas/user/onboarding_progress_alpha.schema.json`](../../schemas/user/onboarding_progress_alpha.schema.json).

## Runtime Surface

`aureline_shell --emit-onboarding-alpha <path>` writes the current projection as
JSON. The record is intended for Start Center review, CLI/headless inspection,
support export, and fixture validation.

The projection preserves these first-run guarantees:

- `Open`, `Clone`, `Import`, `Restore`, and `Recent work` remain separate entry
  verbs.
- Local useful work is available under the `individual_local` profile without
  account creation.
- Launch-bundle recommendations expose `Apply`, `Compare`, `Dismiss`, `Open
  minimal`, and `Set up later` as distinct command-backed actions.
- Remembered recommendation choices restore only the preference; they do not
  install packages, widen trust, or suppress later review.
- Onboarding cards, migration hints, keymap bridges, contextual tips, and help
  search carry stable command IDs and keyboard routes.
- Dismissals, completed tasks, deferred setup, protected recovery
  recommendations, and imported-profile history live in portable profile state.
- Help and glossary packs expose source version, install state, locale posture,
  citation availability, and source-language fallback.
- If no learning digest is installed, the surface renders a truthful
  `not_installed_placeholder` while preserving the no-account path.

## Protected Fixtures

The protected fixture rows live under:

- [`fixtures/ux/onboarding_alpha/`](../../fixtures/ux/onboarding_alpha/)
- [`fixtures/ux/onboarding_help_search_alpha/`](../../fixtures/ux/onboarding_help_search_alpha/)

The integration test
[`crates/aureline-shell/tests/onboarding_alpha.rs`](../../crates/aureline-shell/tests/onboarding_alpha.rs)
loads those fixtures and compares them against the generated projection.

## Verification

```sh
cargo test -p aureline-shell --test onboarding_alpha
cargo run -p aureline-shell --bin aureline_shell -- --emit-onboarding-alpha /tmp/aureline-onboarding-alpha.json
```
