# M5 advisory-claim downgrade certification: stale-advisory, mirror-lag, unsigned-distribution, and continuity-downgrade rules across managed, self-hosted, and offline profiles

Generated from the seeded packet in
[`crate::m5_advisory_claim_downgrade_certification`](../../crates/aureline-shell/src/m5_advisory_claim_downgrade_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- markdown > \
  artifacts/security/m5-advisory-claim-downgrade-certification.md
```

- Packet id: `m5-advisory-claim-downgrade-certification:stable:0001`
- Source schema ref: `schemas/security/m5-advisory-claim-downgrade-certification.schema.json`
- Certifies matrix packet: `m5-advisory-components:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required dimensions: `advisory_freshness`, `mirror_propagation`, `distribution_signature`, `local_continuity`
- Required profiles: `managed`, `self_hosted`, `offline`
- Required claim surfaces: `release`, `help`, `procurement`, `evaluation`, `support`
- Distinct claim states preserved: `warning_only`, `forced_disable`, `awaiting_user_action`, `mirror_lagged`, `unsigned_unverified`
- Rows certified: 3
- Green (full standing): 1
- Yellow (auto-narrowed): 2
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Profile | Status | Badge | Advisory freshness | Mirror propagation | Distribution signature | Local continuity | Waiver |
| ------- | ------ | ----- | ------------------ | ------------------ | ---------------------- | ---------------- | ------ |
| Managed (centrally governed fleet) | `green` | `advisory_claim_current` | `fresh_advisory_state_certified` | `mirror_current_and_propagated` | `fully_signed_and_verified` | `local_continuity_proven_and_safe` | — |
| Self-hosted (self-mirrored advisory feed) | `yellow` | `advisory_claim_narrowed` | `fresh_advisory_state_certified` | `disclosed_mirror_lag_narrowing` | `disclosed_partial_verification_narrowing` | `local_continuity_proven_and_safe` | — |
| Offline (signed advisory/distribution bundle) | `yellow` | `advisory_claim_narrowed` | `disclosed_stale_notice_narrowing` | `mirror_current_and_propagated` | `fully_signed_and_verified` | `disclosed_reduced_continuity_proof` | `waiver:offline-reduced-continuity-proof:0001` |

## Auto-narrowed rows

- `self_hosted` (`yellow`, states `mirror_lagged|unsigned_unverified`) — The self-hosted profile's advisory mirror lags upstream and only part of the distribution it trusts is re-verified, so the release/help/procurement/evaluation/support claim auto-narrows to disclosed mirror-lagged and unsigned/unverified states with refresh-mirror and re-sign/re-verify as the named restore actions, instead of staying silently green.
- `offline` (`yellow`, states `warning_only|awaiting_user_action`) — The offline profile's advisory notice is stale between bundle imports and its local-continuity proof is reduced to the last signed bundle pending an operator acknowledgement, so the claim auto-narrows to disclosed warning-only and waivered awaiting-user-action states with await-notice-refresh and acknowledge-or-act as the named restore actions, instead of staying silently green.

## Exact claim causes

- `self_hosted` — `mirror_propagation` / `mirror_lag_undisclosed` (disclosed: `true`, restore: `refresh_mirror`) — The profile's advisory mirror lags upstream, so the claim is narrowed to a disclosed mirror-lagged state until the mirror is refreshed.
- `self_hosted` — `distribution_signature` / `unsigned_distribution_undisclosed` (disclosed: `true`, restore: `re_sign_or_reverify`) — Only part of the distribution the profile trusts is verified, so the claim is narrowed to a disclosed unsigned/unverified state until the distribution is fully re-signed or re-verified.
- `offline` — `advisory_freshness` / `stale_notice_state_silent` (disclosed: `true`, restore: `await_notice_refresh`) — The advisory notice state is stale under this profile, so the claim is narrowed to a disclosed warning rather than left silently green until the next notice refresh lands.
- `offline` — `local_continuity` / `local_continuity_hidden` (disclosed: `true`, restore: `acknowledge_or_act`) — The local-continuity proof is reduced pending a user action under this profile, so the claim is narrowed to a disclosed, waivered awaiting-user-action state while local work stays visibly safe.

## Active waivers

- `waiver:offline-reduced-continuity-proof:0001` (`offline`, owner: Offline continuity surface owner, expires `2026-09-30T00:00:00Z`) — On an air-gapped install the local-continuity proof is reduced to the last signed bundle's evidence pending an operator acknowledgement, so the advisory claim is narrowed to a disclosed, waivered awaiting-user-action state while local work stays visibly safe; the full continuity proof is restored on the next bundle import.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- validate
cargo test -p aureline-shell --test m5_advisory_claim_downgrade_certification_fixtures
```
