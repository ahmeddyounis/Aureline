# Profile switch review and temporary profile lifecycle fixtures

These JSON fixtures are generated from the Rust corpus in
`crates/aureline-settings/src/stabilize_profile_switch_review_and_temporary_profile_lifecycle/`.

Regenerate with:

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_stabilize_profile_switch_review_and_temporary_profile_lifecycle \
  -- emit-fixtures fixtures/settings/m4/stabilize-profile-switch-review-and-temporary-profile-lifecycle
```

Validate with:

```sh
cargo test -q -p aureline-settings --test profile_switch_review_temporary_lifecycle_fixtures
```
