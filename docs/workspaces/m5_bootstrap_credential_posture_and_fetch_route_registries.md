# M5 bootstrap credential-posture and fetch-route registries

This lane is the credential-boundary + mirror/trust-route implement lane over the frozen
[M5 repository-bootstrap matrix](./m5_repository_bootstrap_contract.md). It turns the *credential-posture*
grammar (how a bootstrap authenticates, which trust roots or mirrors it depends on, and how it references
secrets) and the *fetch-route* grammar (public upstream fetch, approved mirror fetch, air-gap bundle import, and
managed snapshot resume) into registry resolvers that produce export-safe, honest projections, so the
acquisition, git, trust, diagnostics, docs, CLI, and support surfaces resolve one canonical credential and route
truth instead of a per-entry, hand-copied reconstruction. The credential posture and the fetch route are
separated in runtime and serialized state: the auth-source reference, proxy / mirror route, host-key / TLS-pin
state, delegated-token policy, handle-only secret reference, and mirror / signer provenance live on the
credential posture, while the route endpoint class, signer / digest continuity, mirror provenance, recovery
language, and trust proof live on the fetch route, and public / mirrored / air-gapped / resumed routes stay
distinct so signer continuity is never silently dropped across an offline or mirrored fetch.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_bootstrap_credential_posture_and_fetch_route_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/workspaces/m5-bootstrap-credential-posture-and-fetch-route-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/workspaces/m5-checkout-plan.schema.json`](../../schemas/workspaces/m5-checkout-plan.schema.json)
  (credential posture shown before network or disk mutation) and
  [`schemas/workspaces/m5-bootstrap-evidence.schema.json`](../../schemas/workspaces/m5-bootstrap-evidence.schema.json)
  (signer / mirror provenance and digest continuity) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-bootstrap-credential-posture-and-fetch-route-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/workspaces/m5-bootstrap-credential-posture-and-fetch-route-registries/`
  (`air_gap_bundle_beta_narrowed.json`, `managed_snapshot_preview_narrowed.json`).

## Two registries

1. **Credential posture** (`resolve_credential_posture_entry`) — publishes one stable credential-posture object
   per acquisition path: the auth-source kind and canonical auth mode, the auth-source reference, the proxy /
   mirror route, the host-key / TLS-pin state, the delegated-token policy, the handle-only secret reference, and
   the mirror / signer provenance. A clean entry names a canonical registry token, a classified auth source, and
   a repository-bootstrap role, covers the canonical / accessible / audit resolution forms, publishes a complete
   object, keeps any referenced secret handle-only, and discloses the host trust state. Otherwise it degrades
   honestly — a posture that embedded raw secret material in a portable manifest (or hid its host-key / TLS-pin
   state behind generic connected-state copy) degrades to
   `credential_posture_embeds_raw_secret_or_hides_host_trust`.
2. **Fetch route** (`resolve_fetch_route_entry`) — keeps the fetch route safe. A clean entry names a classified
   fetch-route class and provides the complete route-endpoint / signer-continuity / digest-continuity /
   mirror-provenance / recovery-language / trust-proof route object; a route that would lose signer or mirror
   provenance across an offline or mirrored fetch, hides its trust proof, or asserts a recovery it cannot explain
   degrades to `fetch_route_breaks_signer_continuity_or_hides_trust_proof`.

## Per-entry credential-posture reference

The auth source carries its canonical auth mode, and the resolver publishes the full posture object, so the
registry — never a hand-copied per-entry assumption — is the single source of truth.
`credential_posture_object_is_complete` rejects an object missing any field,
`credential_posture_stays_handle_only` rejects an embedded raw secret or a hidden host trust state, and
`fetch_route_stays_signer_continuous` rejects a route that has dropped signer continuity.

| auth source | auth mode | auth-source ref | proxy / mirror route | host-key / TLS-pin state | delegated-token policy | mirror / signer provenance |
| --- | --- | --- | --- | --- | --- | --- |
| anonymous public | anonymous_public_auth | `auth-source.acme/anonymous` | `route.acme/public-upstream` | `host-key.tofu-recorded.v3` | `delegated-token.not-required` | `signer-provenance.acme.v3` |
| delegated token | delegated_token_auth | `auth-source.acme/delegated-app` | `route.acme/direct` | `tls-pin.pinned.v3` | `delegated-token.short-lived.v3` | `signer-provenance.acme.v3` |
| stored handle credential | stored_handle_credential_auth | `auth-source.acme/stored-handle` | `route.acme/mirror-eu` | `tls-pin.pinned.v3` | `delegated-token.not-required` | `mirror-provenance.acme.v3` |

An embedded raw secret degrades to `credential_posture_embeds_raw_secret_or_hides_host_trust`, an incomplete
object degrades to `credential_posture_object_incomplete`, and a dropped signer continuity degrades to
`fetch_route_breaks_signer_continuity_or_hides_trust_proof`, so an embedded secret, an incomplete object, or a
broken signer continuity can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Acquisition flows expose auth source, mirror/route class, and trust proof without leaking raw credentials.**
  Clean posture entries cover the canonical anonymous-public / delegated-token / stored-handle /
  host-key-or-TLS-pinned / air-gap-offline auth sources and the first shell / entry / diagnostics / admin /
  support surfaces, an object-incomplete example degrades, and no clean posture entry published an incomplete
  object.
- **Portable manifests and export packets do not embed secrets, tokens, or hidden host trust state.** A
  secret-embed example and an unbound example degrade, a clean handle-only posture entry is present, and no clean
  entry embedded a raw secret or is unbound.
- **Mirror and signer continuity remain visible and reconstructable for public, mirrored, and offline
  acquisition paths.** Clean fetch-route entries cover the public / approved-mirror / air-gap-bundle /
  managed-snapshot routes with full resolution-form coverage while providing the complete route object, and a
  route that drops signer continuity degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- support-export
cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- csv
cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- report
cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- credential-posture-table
cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- fixture-air-gap-bundle-beta-narrowed
cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- fixture-managed-snapshot-preview-narrowed
```
