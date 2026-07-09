# M5 Credential Component Surface Certification (M05-995)

This is the **closing capstone** of the B117 credential-state-row / secret-access-prompt-sheet /
vault-or-keychain-picker / credential-store-capability-row / browser-device-code-handoff-card /
delegated-credential-row / rotation-revoke-event-row / export-safety-banner component lane. Where the
freeze matrix (`m5-credential-component-matrix.schema.json`, M05-988) defines the eight reusable
components, the M05-989..992 primitive lanes narrow each one, the M05-993 consumer lane proves they
are reusable across the claimed settings / request / database / registry / release / remote /
ai-assistant / help / support / export consumers, and the M05-994 accessibility / auto-narrowing
capstone certifies keyboard / screen-reader / CLI / export parity per family, this capstone
**certifies** that the shared credential component truth holds on every claimed M5 credential-bearing
surface — and auto-narrows any surface that cannot sustain it.

- Boundary schema: `schemas/ui/m5-credential-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-credential-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-credential-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-credential-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-credential-component-certification/`
- Implementation: `crates/aureline-provider/src/certify_credential_component_truth_on_every_claimed_m5_credential_bearing_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user signs in, authenticates a registry / provider /
API / DB on, attaches a remote target on, or recovers / audits / exports credential state on — not on
component family or primitive lane. The eight certified surfaces are:

`connector_authorization`, `registry_authentication`, `database_credential_attach`,
`remote_target_attach`, `docs_help`, `support_export`, `credential_audit_export`, and `cli_headless`.

Each surface is scored on six truth axes:

1. `visual` — storage mode, credential class, handle-only-versus-raw reveal posture,
   local-versus-forwarded / delegated identity, expiry / refresh state, and raw-secret-excluded export
   safety are shown on the primary surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `cli_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same credential identity, with the raw-secret handle excluded.
5. `degraded_state` — an unverified store, an expired auth posture, a drifted delegated scope, or a
   blocked reveal policy honestly downgrades a `verified_brokered` / `handle_ready_projection` claim.
6. `credential_boundary_provenance` — storage mode, credential class, reveal posture,
   local-versus-forwarded / delegated identity, expiry / refresh state, and raw-secret-excluded export
   safety stay explicit before any sign-in, authentication, rotation, recovery, or export, never
   inheriting a healthier lane's truth, never letting friendly "connected" / "signed in" wording
   conceal storage mode / forwarded identity / reveal posture, and **the boundary never drops storage
   / class / reveal / identity / expiry / export-safety continuity** between a sign-in, an
   authentication, a rotation, and an export.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a `verified_brokered`
/ `handle_ready_projection` claim while a truth axis is not current — the credential store is
unverified, the auth posture is expired, the delegated scope has drifted, or the reveal policy is
blocked — is over-claiming and is blocked (`red`). A surface that discloses the reduction by narrowing
its credential claim (with a bound reason and a frozen downgrade trigger) is honestly `yellow`. The
always-on `cli_export` axis must always stay certified. **Credential truth never drops continuity**: a
narrowed surface preserves its storage-mode / credential-class / reveal-posture / delegated-identity /
expiry / export-safety continuity rather than dropping it between a sign-in, an authentication, a
rotation, and an export (`credential_truth_preserved` / `preserves_credential_truth_continuity`).

The credential-claim ladder (strongest first) is reused from the M05-994 accessibility capstone:
`verified_brokered` (5) > `handle_ready_projection` (4) > `unverified_store_projection` (3) >
`expired_auth_projection` (2) > `drifted_delegation_projection` (1) > `reveal_blocked_projection` (0).
Certification may only narrow a claim, never strengthen it — so "certified" never implies verified
storage, current auth, or an allowed reveal when the store is unverified, the auth is expired, the
delegation has drifted, or the reveal is policy-blocked.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes, credential
truth preservation, export parity, and claim narrowing. A row is `red` when it is malformed, drops
CLI/export parity, drops credential truth, hides an undisclosed drift, retains a degraded axis behind
a full claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed
and bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (four `green`, four `yellow`, zero
`red`), every one of the eight frozen component families on at least one surface, every axis on every
row, and credential-truth preservation on every surface. Every row cites the one canonical proof
bundle (`artifacts/release/m5-credential-component-proof/support_export.json`) plus the M05-993
consumer and M05-994 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's four auto-narrowing conditions: an unverified credential store
(`database_credential_attach` → `unverified_store_projection`), an expired auth posture
(`remote_target_attach` → `expired_auth_projection`), a drifted delegated scope (`docs_help` →
`drifted_delegation_projection`), and a blocked reveal policy (`credential_audit_export` →
`reveal_blocked_projection`).

## Regenerating the artifacts

The seed builder (`seeded_m5_credential_component_certification_packet`) is the one source of truth for
both the tests and the on-disk export. To regenerate:

```
GEN_CREDENTIAL_CERT_ARTIFACTS=1 cargo test -p aureline-provider --lib \
  certify_credential_component_truth_on_every_claimed_m5_credential_bearing_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the seed
builder. The packet is metadata-only: a raw secret, a revealed handle, a forwarded token, and
credential-bearing material never cross this boundary.
