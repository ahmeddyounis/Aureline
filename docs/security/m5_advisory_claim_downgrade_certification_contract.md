# M5 advisory-claim downgrade certification contract

Task: **M05-770** — Add stale-advisory, mirror-lag, unsigned-distribution, and
continuity-downgrade rules that auto-narrow release / help / procurement /
evaluation / support advisory claims across M5 managed, self-hosted, and offline
profiles.

This lane makes advisory truth participate in claim governance. If notice
freshness, mirror propagation, signature state, or continuity proof falls behind,
Aureline **narrows what it claims** in the release, help/about, procurement,
evaluation, and support surfaces instead of silently preserving stronger trust
language.

The certification is a **capstone** over the frozen M5 advisory-component matrix
(`freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`).
It mints no parallel advisory vocabulary: the advisory-component families each
profile evaluates and the downgrade triggers each cause names are re-exported by
reference from that matrix, and the evaluated-family set is pulled straight from
the matrix's seeded packet, so the certification cannot audit a family the matrix
does not freeze.

## What is certified

Each claimed **deployment profile** carries one certification row:

- `managed` — a centrally governed fleet where policy administers the advisory
  feed and the trusted distribution; the reference profile every advisory claim
  is measured against.
- `self_hosted` — a self-hosted install mirroring the advisory feed and
  distribution itself, where mirror lag and partial re-verification are the
  standing exposure risks.
- `offline` — an air-gapped install consuming a signed advisory/distribution
  bundle, where notice staleness and a reduced local-continuity proof are the
  standing exposure risks.

Every profile row certifies four downgrade **dimensions** together:

| Dimension | Full standing | Disclosed narrowing (yellow) | Silent overclaim / loss (red) |
| --------- | ------------- | ---------------------------- | ----------------------------- |
| `advisory_freshness` | `fresh_advisory_state_certified` | `disclosed_stale_notice_narrowing` | `advisory_state_stale_and_overclaimed` |
| `mirror_propagation` | `mirror_current_and_propagated` | `disclosed_mirror_lag_narrowing` | `mirror_lagged_claim_overclaimed` |
| `distribution_signature` | `fully_signed_and_verified` | `disclosed_partial_verification_narrowing` | `unsigned_or_unverified_distribution` |
| `local_continuity` | `local_continuity_proven_and_safe` | `disclosed_reduced_continuity_proof` (requires waiver) | `continuity_proof_missing_or_unsafe` |

## Derived, never asserted

The per-row green/yellow/red status is **recomputed** from the four dimension
postures, so the auto-narrowing is the single source of truth:

- **red** if any dimension goes silent and overclaims, local continuity is lost,
  or the profile fails to evaluate every claimed advisory family or project its
  downgrade state into every claimed claim surface;
- **yellow** if the profile discloses a stale notice, a lagging mirror, a
  partially verified distribution, or a reduced local-continuity proof;
- **green** otherwise.

## Distinct downgrade reasons

Managed, self-hosted, and offline profiles each preserve their **own distinct
downgrade reason** instead of collapsing into one generic "degraded" wording.
The five distinct claim states kept apart are `warning_only`, `forced_disable`,
`awaiting_user_action`, `mirror_lagged`, and `unsigned_unverified`. Every claim
cause names both the exact frozen downgrade trigger that fired and the restore
action that would restore the claim (`refresh_mirror`, `re_sign_or_reverify`,
`acknowledge_or_act`, `await_notice_refresh`, `restore_continuity_proof`).

## Claim-surface projection

Every profile row projects its downgrade state — and the controlled badge
(`advisory_claim_current` / `advisory_claim_narrowed` / `advisory_claim_blocked`)
— into all five claim surfaces so a narrowed claim never stays green on one
surface while narrowed on another: `release`, `help`, `procurement`,
`evaluation`, and `support`.

## Records and shape

- **Certification packet** — the full set of per-profile rows with derived
  status, aggregate green/yellow/red counts, active waivers, the exact claim
  causes, and the blocking findings the lane refuses to ship with.
- **Certification dashboard** — a light projection the release / help /
  procurement / evaluation / support automation reads to auto-narrow a claimed
  advisory claim and paint the controlled badge.
- **Support export** — the packet plus dashboard wrapped with the stable case
  ids (packet id, matrix packet ref, build identity, each profile, each active
  waiver id) a support reviewer pivots on.

The shape is fixed by the boundary schema
`schemas/security/m5-advisory-claim-downgrade-certification.schema.json`. The
records carry only stable ids, closed vocabulary, counts, refs, and short labels
— never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or
credentials.

## Source of truth and regeneration

The Rust builder and validator in
`crates/aureline-shell/src/m5_advisory_claim_downgrade_certification/` are the
authoritative gate. The headless emitter
`aureline_shell_m5_advisory_claim_downgrade_certification` is the only
mint-from-truth path for the published artifacts and the protected fixtures:

```sh
BIN=aureline_shell_m5_advisory_claim_downgrade_certification

# Published proof
cargo run -q -p aureline-shell --bin $BIN -- packet   > artifacts/release/m5-advisory-claim-downgrade-certification-proof/packet.json
cargo run -q -p aureline-shell --bin $BIN -- dashboard > artifacts/release/m5-advisory-claim-downgrade-certification-proof/dashboard.json
cargo run -q -p aureline-shell --bin $BIN -- support-export > artifacts/release/m5-advisory-claim-downgrade-certification-proof/support_export.json
cargo run -q -p aureline-shell --bin $BIN -- csv      > artifacts/release/m5-advisory-claim-downgrade-certification-proof/matrix.csv
cargo run -q -p aureline-shell --bin $BIN -- markdown > artifacts/security/m5-advisory-claim-downgrade-certification.md

# Protected fixtures
cargo run -q -p aureline-shell --bin $BIN -- packet         > fixtures/security/m5-advisory-claim-downgrade-certification/packet.json
cargo run -q -p aureline-shell --bin $BIN -- dashboard      > fixtures/security/m5-advisory-claim-downgrade-certification/dashboard.json
cargo run -q -p aureline-shell --bin $BIN -- support-export > fixtures/security/m5-advisory-claim-downgrade-certification/support_export.json
cargo run -q -p aureline-shell --bin $BIN -- compact        > fixtures/security/m5-advisory-claim-downgrade-certification/compact.txt
```

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- validate
cargo test -p aureline-shell --lib m5_advisory_claim_downgrade_certification
cargo test -p aureline-shell --test m5_advisory_claim_downgrade_certification_fixtures
```

## Companion artifacts

- Markdown report: `artifacts/security/m5-advisory-claim-downgrade-certification.md`
- Published packet: `artifacts/release/m5-advisory-claim-downgrade-certification-proof/packet.json`
- Published dashboard: `artifacts/release/m5-advisory-claim-downgrade-certification-proof/dashboard.json`
- Published support export: `artifacts/release/m5-advisory-claim-downgrade-certification-proof/support_export.json`
- Published CSV: `artifacts/release/m5-advisory-claim-downgrade-certification-proof/matrix.csv`
- Protected fixtures: `fixtures/security/m5-advisory-claim-downgrade-certification/packet.json`
- Boundary schema: `schemas/security/m5-advisory-claim-downgrade-certification.schema.json`
- Certifies the frozen matrix: `schemas/security/m5-advisory-component-matrix.schema.json`
