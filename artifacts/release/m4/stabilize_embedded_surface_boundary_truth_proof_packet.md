# Embedded-surface boundary truth register — proof packet

Reviewer-facing proof packet for the embedded-surface boundary truth register that, for the M4 stable line, binds each embedded surface family to the stable claim manifest entry whose lifecycle label it backs, a packet-freshness SLO, a boundary state, owner/origin chrome snapshots, native-approval boundary snapshots, browser-fallback posture snapshots, auth-handoff snapshots (for auth rows), and truth rules that gate publication.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Register: [`/artifacts/release/stabilize_embedded_surface_boundary_truth.json`](../stabilize_embedded_surface_boundary_truth.json)
- Schema: [`/schemas/release/stabilize_embedded_surface_boundary_truth.schema.json`](../../schemas/release/stabilize_embedded_surface_boundary_truth.schema.json)
- Companion doc: [`/docs/m4/stabilize-embedded-surface-boundary-truth.md`](../../../docs/m4/stabilize-embedded-surface-boundary-truth.md)
- Validation capture: [`/artifacts/release/captures/stabilize_embedded_surface_boundary_truth_validation_capture.json`](../captures/stabilize_embedded_surface_boundary_truth_validation_capture.json)
- Typed consumer: `aureline_release::stabilize_embedded_surface_boundary_truth`

This register is intended to be wired into the stable proof index through the `proof_index_ref` each row's proof packet carries (`artifacts/release/stable_proof_index.json#proof:*`), so a launch reviewer reaches the embedded-surface boundary evidence from the same proof index that grounds the launch-blocking requirements.

## What this register proves

1. **Each embedded surface family binds a public claim to a proof packet, boundary state, and truth state.** Every row names its surface kind (`embedded_docs_help`, `extension_hosted_surface`, `embedded_marketplace_or_account`, `embedded_service_dashboard`, `embedded_auth_confirmation`), the stable claim manifest entry it backs (`claim_ref`, `claim_label`), its `proof_packet` and freshness SLO, the `boundary_state` it earned, and the `truth_state` it earned. The register reuses the stable claim level vocabulary rather than minting per-surface labels, so docs, Help/About, shiproom, the release center, and support exports render one label per surface.

2. **Docs/help rows carry a source-truth snapshot.** The `source_truth` snapshot exposes source class, version match state, freshness class, and snapshot age label so users can tell whether docs are authoritative live, warm cached, degraded cached, stale, or unverified.

3. **All rows carry a browser-fallback snapshot.** The `browser_fallback` snapshot exposes fallback posture class (system-browser-first, device-code fallback, external-open blocked by policy, external-open unavailable offline), fallback target class, whether open-in-browser is available, and whether the fallback preserves exact object identity.

4. **All rows carry a native-approval snapshot.** The `native_approval` snapshot proves that high-risk approval sheets, destructive confirmations, workspace trust elevation, update verification, and AI apply review remain product-owned and are never delegated to embedded content or webview-local UI.

5. **Auth rows carry an auth-handoff snapshot.** The `auth_handoff` snapshot proves system-browser auth default, return-path labeling, exact-target preservation across handoff, and whether any embedded exception is visibly disclosed with lower-trust cues.

6. **A surface may carry the claim's label or narrow below it, but never wider.** The CI gate reads the stable claim manifest named by `claim_manifest_ref` and fails when a row's `claim_label` is not the label that manifest publishes for the entry named by `claim_ref`, when a row names an entry the manifest does not carry, or when a row renders wider than the public claim's canonical label. A surface's effective verdict can never outrun the public claim it backs.

7. **The truth rules narrow surfaces before promotion.** Each packet carries a freshness SLO and a recorded `slo_state`. The CI gate recomputes the freshness state and the waiver-expiry state against the register `as_of` date, failing when a declared state overstates the clock, when a backed row rides a stale packet or an expired waiver, when evidence is incomplete, when a claim manifest mismatch is active, when owner/origin chrome is missing, when native approval has leaked, when system-browser auth default is missing, when browser fallback is unavailable, or when an owner sign-off is missing under a Stable claim.

8. **The five surface families and the release-blocking set stay covered.** The gate fails if any of `embedded_docs_help`, `extension_hosted_surface`, `embedded_marketplace_or_account`, `embedded_service_dashboard`, or `embedded_auth_confirmation` has no row, if a declared release-blocking surface has no covering row, if a release-blocking row is not declared, or if a `surface_ref` repeats.

9. **The publication verdict is recomputed, not asserted.** The gate recomputes the `hold`/`proceed` decision and the blocking rule/entry sets from the firing truth rules and fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so shiproom and release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-06-02)

The checked-in register holds publication. Of fifteen truth rows across five public claims, six publish a label at or above the cutline and nine are narrowed below it:

- **Embedded docs/help main** — Current, Stable, LiveVerified, release-blocking, backed by a fresh proof packet, with complete source truth and owner sign-off.
- **Extension webview main** — CurrentOnWaiver, Stable, LiveVerified, release-blocking, backed by a fresh packet with an active cross-origin review waiver.
- **Marketplace/account main** — Current, Stable, LiveVerified, release-blocking, backed by a fresh proof packet, with complete provider identity and owner sign-off.
- **Auth confirmation main** — Current, Stable, LiveVerified, release-blocking, system-browser default proven, return-path labeled, exact target preserved, owner-signed.
- **Auth confirmation embedded exception** — CurrentOnWaiver, Stable, LiveVerified, device-code fallback with disclosed exception and lower-trust cues.
- **Extension webview cross-origin** — CurrentOnWaiver, Stable, CrossOriginLimited, cross-origin limitation intentional and disclosed with browser fallback.

Narrowed rows:
- **Service dashboard main** — NarrowedStale, Beta, StaleSnapshot, proof packet breached its 14-day freshness SLO.
- **Marketplace/account stale** — NarrowedStale, Beta, StaleSnapshot, proof packet breached its 14-day freshness SLO.
- **Auth confirmation missing packet** — NarrowedStale, Beta, ExternalOpenOnly, no proof packet captured, but system-browser default and return-path labeling remain enforced.
- **Extension webview policy blocked** — NarrowedUnbacked, Beta, PolicyBlocked, compliance evidence for the blocked publisher is incomplete.
- **Service dashboard policy blocked** — NarrowedUnbacked, Beta, PolicyBlocked, owner sign-off pending for compliance review.
- **Embedded docs/help certificate failed** — NarrowedUnbacked, Beta, CertificateFailed, origin verification incomplete due to certificate failure, but owner/origin chrome is still disclosed.
- **Embedded docs/help offline** — NarrowedUnbacked, Beta, OfflineSnapshot, browser fallback unavailable while offline, but local inspect/export recovery is provided.
- **Marketplace/account external open only** — NarrowedUnbacked, Beta, ExternalOpenOnly, embedded body withheld pending full provider verification; only external open is offered.
- **Service dashboard offline** — NarrowedClaimNarrowed, Beta, OfflineSnapshot, inherits the beta ceiling of its backing claim.

Seven of the narrowed rows back claims still published Stable, so they fire seven blocking truth rules and hold the `ci/check_embedded_surface_boundary_truth.py` gate. Promotion clears once the gaps close or the backing claims are formally narrowed.

## How to re-verify

```sh
cargo test -p aureline-release --test stabilize_embedded_surface_boundary_truth
```

This runs the typed contract tests that bind the model to the checked-in register.
