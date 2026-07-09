# M5 Credential Component Accessibility & Auto-Narrowing (M05-994)

This lane is the **accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone** over the frozen M5 credential component matrix
(`schemas/ui/m5-credential-component-matrix.schema.json`, M05-988). Where the freeze matrix
defines the reusable credential-state row, secret-access-prompt sheet, vault-or-keychain picker,
credential-store-capability row, browser/device-code handoff card, delegated-credential row,
rotation/revoke-event row, and export-safety banner primitives, and the M05-989 through M05-993
implementation / consumer lanes resolve their per-surface truth, this lane certifies — per
component family — that credential claims stay **keyboard-complete, assistive-tech-reachable,
CLI/export-safe, and self-narrowing** across desktop, assistive, headless, and export paths.

- Module: `crates/aureline-provider/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_store_verification_auth_posture_delegated_scope_or_reveal_policy_is_limited_expired_or_policy_blocked_across_claimed_m5_credential_components`
- Schema: `schemas/ui/m5-credential-component-accessibility-parity.schema.json`
- Support export: `artifacts/release/m5-credential-component-accessibility-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-credential-component-accessibility-proof/matrix.csv`
- Report: `artifacts/release/m5-credential-component-accessibility-proof.md`
- Fixtures: `fixtures/ui/m5-credential-component-accessibility-parity/`

## What it guarantees

1. **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
   screen-reader-reachable, and CLI/headless-reachable path into the same canonical credential
   identity, storage mode, handle-only-versus-raw-reveal posture, local-versus-forwarded /
   delegated identity, expiry / lifecycle state, and raw-secret-excluded export boundary the rich
   component shows — never a hover-only chip that strands assistive-tech or headless users.
   Hierarchy-heavy families (the export-safety banner's nested export-surface / excluded-field /
   redaction-posture lineage) additionally bind their tree to a flat list / textual path.
2. **Export parity.** The support / release / evaluation export reconstructs each component's
   meaning from typed tokens and opaque refs without a screenshot, preserving the same canonical
   IDs, storage modes, reveal postures, delegated-identity labels, expiry states, export-safety
   boundaries, and narrowing reasons shown in-product — and never a raw secret.
3. **Honest auto-narrowing.** When store verification is missing, auth posture is expired,
   delegated scope has drifted, or reveal policy is blocked by deployment / profile policy, the
   component's credential claim auto-narrows from `verified_brokered` / `handle_ready_projection`
   to an `unverified_store_projection` / `expired_auth_projection` / `drifted_delegation_projection`
   / `reveal_blocked_projection`, discloses the narrowing with a precise trigger and binding
   dimension, and preserves the canonical identity / storage / delegation / expiry lineage. A
   component with every dimension intact must not carry a spurious narrowing, and an **unverified,
   expired, or reveal-blocked state never masquerades as verified-brokered** — it never silently
   implies verified storage, current auth, or allowed reveal / export behavior.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the credential-settings,
   secret-prompt, vault-picker, device-code-handoff, delegated-identity, status-bar, general
   product UI, headless CLI, and support / release exports so product, docs, and release
   publication stay aligned on credential-boundary downgrade behavior — no friendly "connected" /
   "signed in" wording is allowed to conceal storage mode, forwarded / delegated identity,
   raw-secret reveal posture, or export-safety limits.

## Claim ladder

| Claim | Rank | Meaning |
| --- | --- | --- |
| `verified_brokered` | 5 | Verified store, current auth, in-scope delegation, allowed reveal/export; brokerable and usable now. |
| `handle_ready_projection` | 4 | Self-sufficient handle-only projection (usable through its handle); not itself raw-revealable. |
| `unverified_store_projection` | 3 | Store verification is missing; cannot claim verified storage until verified. |
| `expired_auth_projection` | 2 | Auth posture is expired; cannot present as current; re-authenticate first. |
| `drifted_delegation_projection` | 1 | Delegated scope drifted from what was granted; cannot present as in-scope until reconciled. |
| `reveal_blocked_projection` | 0 | Reveal/export policy blocked by deployment/profile policy; cannot claim allowed reveal/export. |

## Condition → ceiling → trigger

| Condition state | Permitted ceiling | Frozen trigger |
| --- | --- | --- |
| `verified_current` | `verified_brokered` | — (baseline) |
| `store_unverified` | `unverified_store_projection` | `store_capability_unstated` |
| `auth_expired` | `expired_auth_projection` | `lifecycle_state_hidden` |
| `delegated_scope_drifted` | `drifted_delegation_projection` | `delegated_identity_unstated` |
| `reveal_policy_blocked` | `reveal_blocked_projection` | `reveal_posture_unstated` |

`store_unverified`, `auth_expired`, and `reveal_policy_blocked` are the states that would silently
imply verified storage, current auth, or allowed reveal / export: a row modeling any of them must
never let its effective claim assert `verified_brokered`.

## Coverage

Eight rows over eight frozen families (one per family): **4 green / 4 yellow / 0 red**. Every
claim dimension, every condition state, every claim tier, and all nine consumer surfaces are
exercised across the packet.

## Regenerating artifacts

The support export, CSV, report, and fixtures are generated from the single in-code seed and are
byte-checked by the test suite. Regenerate with:

```
GEN_CREDENTIAL_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-provider --lib generate_artifacts
```
