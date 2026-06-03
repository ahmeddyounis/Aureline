# Docs, Help, About, service-health, and package/dependency safety truth register — proof packet

Reviewer-facing proof packet for the docs/help/About/service-health truth register that, for the M4 stable line, binds each user-facing truth surface to the stable claim manifest entry whose lifecycle label it backs, a packet-freshness SLO, the service-contract state (for service-health rows), the About provenance card (for About rows), the package-safety disclosure (for package-safety rows), and truth rules that gate publication.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Register: [`/artifacts/release/harden_docs_help_about_and_service_health_truth.json`](../harden_docs_help_about_and_service_health_truth.json)
- Companion doc: [`/docs/m4/harden-docs-help-about-and-service-health-truth.md`](../../../docs/m4/harden-docs-help-about-and-service-health-truth.md)
- Validation capture: [`/artifacts/release/captures/harden_docs_help_about_and_service_health_truth_validation_capture.json`](../captures/harden_docs_help_about_and_service_health_truth_validation_capture.json)
- Typed consumer: `aureline_release::harden_docs_help_about_and_service_health_truth`

This register is intended to be wired into the stable proof index through the `proof_index_ref` each row's proof packet carries (`artifacts/release/stable_proof_index.json#proof:*`), so a launch reviewer reaches the docs, Help, About, service-health, and package-safety evidence from the same proof index that grounds the launch-blocking requirements.

## What this register proves

1. **Each surface binds a public claim to a proof packet and truth state.** Every row names its surface kind (`docs`, `help`, `about`, `service_health`, `package_safety`), the stable claim manifest entry it backs (`claim_ref`, `claim_label`), its `proof_packet` and freshness SLO, and the `truth_state` it earned. The register reuses the stable claim level vocabulary rather than minting per-surface labels, so docs, Help/About, shiproom, the release center, and support exports render one label per surface.

2. **Service-health rows expose one stable `service_contract_state` vocabulary.** The register adopts the vocabulary `ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`, `policy_blocked`, and `unavailable` for service-health rows, and reuses it in desktop UI, CLI/headless output, docs packs, cached notices, and support exports so every consumer shares one vocabulary.

3. **About rows carry a provenance summary card with trust-class-grouped destinations.** The `about_card` exposes version, channel, install mode, provenance/build state, a copy-build-info action, and grouped destinations labeled by `DestinationTrustClass` (`official`, `community`, `mirrored`, `self_hosted`, `vendor_owned`) rather than generic external-link chrome.

4. **Package-safety rows carry a disclosure before write.** The `package_safety` disclosure exposes manifest scope, registry/auth source, script or native-build risk, lockfile impact, license/advisory delta, validation tasks, and rollback path for install/update/remove/resolve flows.

5. **A surface may carry the claim's label or narrow below it, but never wider.** The CI gate reads the stable claim manifest named by `claim_manifest_ref` and fails when a row's `claim_label` is not the label that manifest publishes for the entry named by `claim_ref`, when a row names an entry the manifest does not carry, or when a row renders wider than the public claim's canonical label. A surface's effective verdict can never outrun the public claim it backs.

6. **The truth rules narrow surfaces before promotion.** Each packet carries a freshness SLO and a recorded `slo_state`. The CI gate recomputes the freshness state and the waiver-expiry state against the register `as_of` date, failing when a declared state overstates the clock, when a backed row rides a stale packet or an expired waiver, when evidence is incomplete, when a claim manifest mismatch is active, or when an owner sign-off is missing under a Stable claim.

7. **The five surface kinds and the release-blocking set stay covered.** The gate fails if any of `docs`, `help`, `about`, `service_health`, or `package_safety` has no row, if a declared release-blocking surface has no covering row, if a release-blocking row is not declared, or if a `surface_ref` repeats.

8. **The publication verdict is recomputed, not asserted.** The gate recomputes the `hold`/`proceed` decision and the blocking rule/entry sets from the firing truth rules and fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so shiproom and release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-06-02)

The checked-in register holds publication. Of sixteen truth rows across eight public claims, seven publish a label at or above the cutline and nine are narrowed below it:

- **Docs main** — Current, Stable, release-blocking, backed by a fresh proof packet.
- **Help main** — CurrentOnWaiver, Stable, release-blocking, backed by a fresh packet with an active locale-overlay waiver.
- **About main** — Current, Stable, release-blocking, with a complete provenance card showing official, community, and mirrored destinations.
- **Service health API** — Current, Stable, release-blocking, service contract state `ready`.
- **Package safety install** — Current, Stable, release-blocking, with a complete safety disclosure.
- **Service health cache** — CurrentOnWaiver, Stable, service contract state `degraded`, on a waiver for partial regional coverage.
- **Service health local** — CurrentOnWaiver, Stable, service contract state `local_only`, on a waiver for managed-endpoint reachability gap.

Narrowed rows:
- **Service health sync** — NarrowedStale, Beta, service contract state `stale`, proof packet breached its 14-day freshness SLO.
- **Package safety update** — NarrowedUnbacked, Beta, incomplete advisory review for GHSA-2026-0002.
- **Docs module** — NarrowedContractMismatch, Beta, published module doc version does not match the claim manifest.
- **Help local** — NarrowedClaimNarrowed, Beta, inherits the beta ceiling of its backing claim.
- **Service health offline** — NarrowedUnbacked, Beta, service contract state `unavailable`, owner sign-off missing.
- **Package safety remove** — NarrowedStale, Beta, no proof packet captured.
- **Docs changelog** — NarrowedWaiverExpired, Beta, format-review waiver expired on 2026-05-01.
- **Service health policy** — NarrowedUnbacked, Beta, service contract state `policy_blocked`, compliance evidence for blocked regions is incomplete.
- **Service health mismatch** — NarrowedContractMismatch, Beta, service contract state `contract_mismatch`, published contract does not match the claim manifest.

Six of the narrowed rows back claims still published Stable, so they fire six blocking truth rules and hold the `ci/check_docs_help_about_service_health_truth.py` gate. Promotion clears once the gaps close or the backing claims are formally narrowed.

## How to re-verify

```sh
cargo test -p aureline-release --test harden_docs_help_about_and_service_health_truth
```

This runs the typed contract tests that bind the model to the checked-in register.
