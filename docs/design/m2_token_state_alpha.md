# Token/State Alpha Registry

The alpha shell uses one registry for component states, badge families,
notice families, and cross-surface token inheritance:

- [`artifacts/design/state_badge_families_alpha.yaml`](../../artifacts/design/state_badge_families_alpha.yaml)
- [`fixtures/design/m2_state_semantics/manifest.yaml`](../../fixtures/design/m2_state_semantics/manifest.yaml)
- [`crates/aureline-ui/src/tokens/state_semantics.rs`](../../crates/aureline-ui/src/tokens/state_semantics.rs)

The registry extends the earlier token and component-state seeds by joining
state meaning to renderable badge/notice families. Shell chrome, editor rows,
search/command surfaces, docs/help, package/marketplace rows, trust prompts,
and support exports consume the same family classes instead of local warning
chips or one-off badge labels.

## Controlled States

The controlled component-state classes are:

`empty`, `loading`, `pending`, `degraded`, `blocked`, `error`, `completed`,
`focus_visible`, `selection`, `active_target`, `trust_restricted`,
`policy_locked`, `readiness_ready`, and `readiness_partial`.

The registry keeps load-bearing distinctions explicit:

| Distinction | Contract |
| --- | --- |
| Loading vs pending | Loading is preparation before content is ready; pending follows a submitted user action. |
| Degraded vs blocked | Degraded preserves named capability; blocked names the source that prevents action. |
| Selection vs active target vs focus | Selection is chosen-set membership; active target drives route/detail context; focus is keyboard or assistive-tech position. |
| Trust restricted vs policy locked | Trust narrowing and policy locks remain separate axes with shield/lock cues. |
| Completed vs transient success | Completed is durable and reviewable; toast-only success is not enough for meaningful work. |

Every state row carries semantic token refs plus non-color cues. Severity,
trust, policy, readiness, selected, focused, active-target, degraded, and
support-bearing states all require text or structural cues; hue-only signaling
is invalid.

## Badge And Notice Families

Badge families are closed for the alpha wedge:

`lifecycle`, `route`, `support_class`, `readiness`, `policy`, `trust`,
`docs_help`, `package_marketplace`, `support_export`, and `theme_package`.

Notice families are:

`info`, `warning`, `degraded`, `blocked`, `restricted`, and `success`.

Every badge family has an honesty fallback token and text-plus-shape fallback.
Support-export compatibility is explicit, so a package row, docs/help row, or
trust prompt can be exported without rewriting badge language.

## Runtime Consumer

`aureline-ui` loads the registry at compile time and exposes projection APIs:

- `alpha_state_semantics_registry()`
- `StateSemanticsRegistry::state_treatment(...)`
- `StateSemanticsRegistry::badge_treatment(...)`
- `StateSemanticsRegistry::notice_treatment(...)`

`ComponentStateRegistry::load(...)` now validates the alpha registry before
returning component chrome styles. If the registry drifts or becomes
structurally invalid, the shared UI state path fails early.

## Fixture Coverage

The protected fixture manifest covers:

- command palette loading vs pending distinction;
- shell/editor degraded, blocked, selected, active-target, trust, and policy
  separation;
- docs/help badge projection;
- package/marketplace support, policy, lifecycle, and theme-package badges;
- trust prompt notice families;
- embedded extension inheritance-gap disclosure.

Run the validator with:

```sh
python3 ci/check_m2_state_semantics.py --repo-root .
```

Run the Rust consumer tests with:

```sh
cargo test -p aureline-ui
```
