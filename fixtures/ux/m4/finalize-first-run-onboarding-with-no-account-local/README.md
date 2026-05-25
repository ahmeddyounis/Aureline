# First-run onboarding stable fixtures

Each `*.json` here is a pinned `first_run_onboarding_record` (schema:
`schemas/ux/finalize-first-run-onboarding-with-no-account-local.schema.json`),
minted bit-for-bit from the in-code corpus in
`crates/aureline-shell/src/first_run_onboarding/corpus.rs` through the live
`FirstRunOnboardingRecord::build` honesty gate, so these records are a genuine
projection of the shell's first-run code rather than a parallel model.

These are **generated, not hand-edited**. Regenerate with:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_first_run_onboarding -- emit-fixtures \
  fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local
```

The replay + invariant gate is
`crates/aureline-shell/tests/first_run_onboarding_fixtures.rs`; it fails if any
fixture drifts from the corpus or violates a first-run invariant (no-account
local entry, setup-later posture, repair-safe recovery, durable/accessible
truth, or the account-free landing). The contract narrative is
`docs/ux/m4/finalize-first-run-onboarding-with-no-account-local.md` and the
release-evidence packet is
`artifacts/ux/m4/finalize-first-run-onboarding-with-no-account-local.md`. The
upstream first-run no-account contract is
`docs/ux/no_account_local_entry_contract.md`.

The seven records cover the claimed stable matrix:

| Fixture | Scenario | Health | Landing | Deferred / repair cues |
| --- | --- | --- | --- | --- |
| `clean_first_run.json` | Clean first run | healthy | empty_editor | 0 / 0 |
| `setup_deferred_local_only.json` | Setup deferred, local only | healthy | local_workspace | 4 / 0 |
| `setup_completed_with_import.json` | Setup completed with import | healthy | local_workspace | 1 / 0 |
| `degraded_settings_store.json` | Degraded settings store | degraded | readme | 0 / 3 |
| `needs_repair_partial_migration.json` | Needs repair: partial migration | needs_repair | local_workspace | 0 / 2 |
| `missing_locale_pack.json` | Missing locale pack | degraded | readme | 0 / 1 |
| `newer_profile_incompatible.json` | Newer, incompatible profile | needs_repair | sample_project | 0 / 1 |

Every repair cue carries an export-safe chain of custody — a `doctor.finding.*`
finding code, a `repair_transaction:<family>.<reason>` id, an opaque
`checkpoint:*` ref, and the `metadata_safe_default` redaction class — so support,
docs, and shiproom packets reference the same truth.
