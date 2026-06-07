# Forms, Parameter Sources, Validation, And Apply Timing

Stable structured-input surfaces now share one governed record:
`forms_parameter_source_and_staged_apply_record`.

The Rust source of truth is
`crates/aureline-shell/src/forms_parameter_source_and_staged_apply`. The boundary
schema is
`schemas/release/forms-parameter-source-and-staged-apply.schema.json`, and the
fixture corpus is under
`fixtures/forms/m4/forms-parameter-source-and-staged-apply/`.

## Contract

Every packet carries:

- visible form title, affected scope, client scope, and surface class;
- field rows with visible label, current value summary, required/optional state,
  source label, validation state, apply timing, and reset/clear/details actions;
- precedence audits that keep `Default`, `Detected`, `Imported`, `Workspace`,
  `Policy`, `User override`, and `Secret reference` values distinct;
- validation class and state, including pending validation with preserved
  last-known result and stale validation with dependency/target invalidation;
- form-level apply timing: `Immediate`, `Staged`, `Preview-first`, or
  `Policy-locked`;
- staged or preview-first checkpoint, review, dirty-state durability, revert,
  save/resume, target scope, side effects, and exact final action label;
- secret, path, object-reference, and code-backed field truth for storage mode,
  path basis, stable identity, diff preview, preservation expectations, and
  redaction-safe export;
- restricted-client and browser-companion limitation disclosures before final
  submit;
- keyboard, screen-reader, IME, RTL, reduced-motion, and focus-return review
  evidence.

## Corpus

The corpus covers:

| Fixture | Surface | Client | Timing |
| --- | --- | --- | --- |
| `settings_immediate_user_override.json` | settings editor | desktop | immediate |
| `remote_staged_workspace_settings.json` | settings editor | remote workspace | staged |
| `provider_preview_first_publish.json` | publish review | desktop | preview-first |
| `managed_policy_locked_setup.json` | policy review | managed workspace | policy-locked |
| `offline_stale_recovery.json` | recovery review | offline degraded | staged |
| `browser_companion_restricted_scaffold.json` | scaffold wizard | browser companion | preview-first |
| `restricted_client_provider_account.json` | provider account flow | restricted client | preview-first |

## Verification

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_forms_parameter_source_and_staged_apply -- emit-fixtures \
  fixtures/forms/m4/forms-parameter-source-and-staged-apply

cargo test -p aureline-shell --test forms_parameter_source_and_staged_apply_fixtures
```

