# High-authority entry interstitial fixtures

This directory is the seed corpus for the **high-authority / cross-boundary
entry interstitial** contract — the typed review shown before any deep-link,
protocol-handler, auth callback return, collaboration join, remote target open,
managed resume, or OS notification reopen that is broader than a plain local
open.

- Boundary schema:
  [`/schemas/workspace/entry_interstitial.schema.json`](../../../../schemas/workspace/entry_interstitial.schema.json)
- Builder / decision logic:
  [`crates/aureline-shell/src/entry_interstitials/`](../../../../crates/aureline-shell/src/entry_interstitials/)
- Beta contract: [`docs/workspace/m3/high_authority_entry_beta.md`](../../../../docs/workspace/m3/high_authority_entry_beta.md)

Every fixture is the **canonical serialized output** of
`materialize_entry_interstitial` (or, for the fast-path case,
`evaluate_entry_interstitial`). The unit test
`every_fixture_case_matches_materialized_record` rebuilds each request and
asserts the materialized record matches the fixture exactly, so a fixture can
never drift from the code. Regenerate with:

```
cargo test -p aureline-shell -- --ignored regenerate_high_authority_entry_fixtures
```

## Index

| Fixture | Record kind | What it proves |
| --- | --- | --- |
| `protocol_deep_link_managed_open.json` | `entry_interstitial_record` | Browser deep link resuming a managed workspace: tenant + policy + authority boundaries disclosed; confirm bound to the canonical `restore_from_checkpoint` command. |
| `auth_callback_return.json` | `entry_interstitial_record` | System-browser sign-in return: authority-widening boundary, auth-scope effect, confirm bound to the canonical auth-return command. |
| `collaboration_join.json` | `entry_interstitial_record` | Joining a partner-org pair session: tenant + remote + authority boundaries; identity review required. |
| `remote_target_open_unreachable.json` | `entry_interstitial_record` | Remote open whose target is unreachable: truthful placeholder, bounded reconnect/retry fallbacks, confirm disabled — never an empty shell. |
| `managed_resume.json` | `entry_interstitial_record` | Admin-surface managed resume: policy + tenant boundaries; exact target; confirm enabled. |
| `notification_reopen_exact.json` | `entry_interstitial_record` | OS notification reopening an exact managed review thread: resolves the exact object, never a generic home surface. |
| `plain_local_open_fast_path.json` | `entry_plain_local_open_record` | Negative / guardrail case: an OS file-association open of an exact local file crosses no boundary, so no interstitial is shown. |
| `support_packet_managed_resume.json` | `entry_interstitial_support_packet_record` | Metadata-safe support-export projection of the managed-resume record: typed classes, opaque identity, canonical command, and invariant echoes, with no raw URLs/paths/credentials. |

## Invariants every interstitial fixture carries

- `silent_execution_forbidden: true` — no high-authority entry runs without
  confirm/reject.
- `authority_not_widened: true` and `actions[confirm].command_id ==
  canonical_command_ref` — the OS-origin path runs exactly the in-product
  command and cannot widen authority.
- `reopens_generic_home: false` — reopen resolves the exact object or an
  announced placeholder.
- When `target_scope.truth_state` is not `exact_available`, a
  `target_placeholder` with bounded `fallback_actions` preserves the original
  intent instead of opening an empty shell.
