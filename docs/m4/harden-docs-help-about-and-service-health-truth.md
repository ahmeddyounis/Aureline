# Harden docs, Help, About, service-health, and package/dependency safety truth — M4 milestone note

This is the milestone-level note for the docs/help/About/service-health and package/dependency safety truth lane that hardens every user-facing truth surface against the claim manifest, version-match, and freshness audits for the M4 stable line.

The authoritative typed consumer is `aureline_release::harden_docs_help_about_and_service_health_truth`.
The canonical checked-in artifact is `artifacts/release/harden_docs_help_about_and_service_health_truth.json`.
The proof packet lives at `artifacts/release/m4/harden_docs_help_about_and_service_health_truth_proof_packet.md`.
The fixture corpus lives under `fixtures/release/harden_docs_help_about_and_service_health_truth/`.
The validation capture lives at `artifacts/release/captures/harden_docs_help_about_and_service_health_truth_validation_capture.json`.

## Scope

This lane governs the five user-facing truth surfaces that must stay aligned with the exact build being promoted:

| Surface | Kind | Release-blocking | Current state |
|---------|------|------------------|---------------|
| Docs main | `docs` | Yes | Current, Stable |
| Help main | `help` | Yes | CurrentOnWaiver, Stable |
| About main | `about` | Yes | Current, Stable |
| Service health API | `service_health` | Yes | Current, Stable, Ready |
| Package safety install | `package_safety` | Yes | Current, Stable |
| Service health cache | `service_health` | No | CurrentOnWaiver, Stable, Degraded |
| Service health local | `service_health` | No | CurrentOnWaiver, Stable, LocalOnly |
| Service health sync | `service_health` | Yes | NarrowedStale, Beta, Stale |
| Package safety update | `package_safety` | No | NarrowedUnbacked, Beta |
| Docs module | `docs` | No | NarrowedContractMismatch, Beta |
| Help local | `help` | No | NarrowedClaimNarrowed, Beta |
| Service health offline | `service_health` | No | NarrowedUnbacked, Beta, Unavailable |
| Package safety remove | `package_safety` | No | NarrowedStale, Beta |
| Docs changelog | `docs` | No | NarrowedWaiverExpired, Beta |
| Service health policy | `service_health` | No | NarrowedUnbacked, Beta, PolicyBlocked |
| Service health mismatch | `service_health` | No | NarrowedContractMismatch, Beta, ContractMismatch |

Each surface binds to a stable claim manifest entry whose canonical lifecycle label is a hard ceiling. The register adopts one stable `service_contract_state` vocabulary (`ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`, `policy_blocked`, `unavailable`) shared across docs, Help, About, and service-health publication surfaces. About rows carry a provenance summary card with version, channel, install mode, provenance state, copy-build-info action, and destinations grouped by trust class. Package-safety rows carry a disclosure with manifest scope, registry/auth source, script/native-build risk, lockfile impact, license/advisory delta, validation tasks, and rollback path.

## Downgrade behavior

Any surface that loses freshness, certification, or proof narrows automatically instead of lingering as an unearned stable promise:

- **Claim label narrowed** (`claim_label_narrowed`): the backing public claim is itself below the cutline.
- **Proof packet freshness breached** (`proof_packet_freshness_breached`): the proof packet is older than its freshness SLO.
- **Proof packet missing** (`proof_packet_missing`): no proof packet has been captured.
- **Evidence incomplete** (`evidence_incomplete`): the surface's docs-maintenance packet, stale-example findings, or service-health descriptor is incomplete.
- **Waiver expired** (`waiver_expired`): the provisional waiver passed its expiry date.
- **Owner sign-off missing** (`owner_signoff_missing`): the owning team has not signed.
- **Claim manifest mismatch** (`claim_manifest_mismatch`): the surface's published claim does not match the current claim manifest.

## Verification

```sh
cargo test -p aureline-release --test harden_docs_help_about_and_service_health_truth
```

Run this from the repository root to validate the typed model against the checked-in artifact and fixtures.
