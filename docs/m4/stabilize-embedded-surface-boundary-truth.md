# Stabilize embedded surface boundary truth — M4 milestone note

This is the milestone-level note for the embedded-surface boundary truth lane that stabilizes docs/help panes, extension webviews, marketplace/account pages, service dashboards, and auth confirmation surfaces with owner/origin chrome, service-boundary truth, native-approval boundaries, and system-browser auth integrity for the M4 stable line.

The authoritative typed consumer is `aureline_release::stabilize_embedded_surface_boundary_truth`.
The canonical checked-in artifact is `artifacts/release/stabilize_embedded_surface_boundary_truth.json`.
The proof packet lives at `artifacts/release/m4/stabilize_embedded_surface_boundary_truth_proof_packet.md`.
The fixture corpus lives under `fixtures/release/stabilize_embedded_surface_boundary_truth/`.
The validation capture lives at `artifacts/release/captures/stabilize_embedded_surface_boundary_truth_validation_capture.json`.

## Scope

This lane governs the five embedded surface families that must carry owner/origin chrome, boundary state, native-approval fences, and browser-fallback posture before any stable promotion:

| Surface | Kind | Release-blocking | Current state |
|---------|------|------------------|---------------|
| Embedded docs/help main | `embedded_docs_help` | Yes | Current, Stable, LiveVerified |
| Extension webview main | `extension_hosted_surface` | Yes | CurrentOnWaiver, Stable, LiveVerified |
| Marketplace/account main | `embedded_marketplace_or_account` | Yes | Current, Stable, LiveVerified |
| Service dashboard main | `embedded_service_dashboard` | Yes | NarrowedStale, Beta, StaleSnapshot |
| Auth confirmation main | `embedded_auth_confirmation` | Yes | Current, Stable, LiveVerified |
| Embedded docs/help offline | `embedded_docs_help` | No | NarrowedUnbacked, Beta, OfflineSnapshot |
| Extension webview policy blocked | `extension_hosted_surface` | No | NarrowedUnbacked, Beta, PolicyBlocked |
| Marketplace/account stale | `embedded_marketplace_or_account` | No | NarrowedStale, Beta, StaleSnapshot |
| Auth confirmation embedded exception | `embedded_auth_confirmation` | No | CurrentOnWaiver, Stable, LiveVerified |
| Service dashboard policy blocked | `embedded_service_dashboard` | No | NarrowedUnbacked, Beta, PolicyBlocked |
| Embedded docs/help certificate failed | `embedded_docs_help` | No | NarrowedUnbacked, Beta, CertificateFailed |
| Extension webview cross-origin | `extension_hosted_surface` | No | CurrentOnWaiver, Stable, CrossOriginLimited |
| Auth confirmation missing packet | `embedded_auth_confirmation` | No | NarrowedStale, Beta, ExternalOpenOnly |
| Marketplace/account external open only | `embedded_marketplace_or_account` | No | NarrowedUnbacked, Beta, ExternalOpenOnly |
| Service dashboard offline | `embedded_service_dashboard` | No | NarrowedClaimNarrowed, Beta, OfflineSnapshot |

Each surface binds to a stable claim manifest entry whose canonical lifecycle label is a hard ceiling. The register adopts one stable `boundary_state` vocabulary (`live_verified`, `stale_snapshot`, `policy_blocked`, `certificate_failed`, `cross_origin_limited`, `offline_snapshot`, `external_open_only`) shared across all embedded surface families. Docs/help rows carry a `source_truth` snapshot proving source class, version match, and freshness disclosure. All rows carry a `browser_fallback` snapshot proving open-in-browser posture and object-identity preservation, a `native_approval` snapshot proving high-risk approvals remain host-owned, and auth rows carry an `auth_handoff` snapshot proving system-browser default, return-path labeling, exact-target preservation, and exception disclosure.

## Downgrade behavior

Any surface that loses freshness, boundary integrity, native-approval fences, or auth integrity narrows automatically instead of lingering as an unearned stable promise:

- **Claim label narrowed** (`claim_label_narrowed`): the backing public claim is itself below the cutline.
- **Proof packet freshness breached** (`proof_packet_freshness_breached`): the proof packet is older than its freshness SLO.
- **Proof packet missing** (`proof_packet_missing`): no proof packet has been captured.
- **Evidence incomplete** (`evidence_incomplete`): the surface's boundary audit, embedded-boundary alpha snapshot, or browser-fallback alpha packet is incomplete.
- **Waiver expired** (`waiver_expired`): the provisional waiver passed its expiry date.
- **Owner sign-off missing** (`owner_signoff_missing`): the owning team has not signed.
- **Claim manifest mismatch** (`claim_manifest_mismatch`): the surface's published claim does not match the current claim manifest.
- **Owner or origin chrome missing** (`owner_origin_chrome_missing`): the embedded surface hides ownership, origin, or freshness.
- **Native approval boundary leaked** (`native_approval_boundary_leaked`): high-risk approvals, destructive confirmations, trust elevation, update verification, or AI apply review leaked to embedded content.
- **System-browser default missing** (`system_browser_default_missing`): auth confirmation or identity rows do not default to system-browser auth.
- **Browser fallback unavailable** (`browser_fallback_unavailable`): no honest open-in-browser or exact-reopen fallback is offered.

## Verification

```sh
cargo test -p aureline-release --test stabilize_embedded_surface_boundary_truth
```

Run this from the repository root to validate the typed model against the checked-in artifact and fixtures.
