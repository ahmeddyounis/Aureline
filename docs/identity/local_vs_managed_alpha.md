# Local Versus Managed Identity Alpha

This page is the reviewer entry point for the account-free local, self-hosted
organization, and managed convenience identity-mode baseline.

## Runtime Contract

The canonical packet is
[`IdentityModeBaselinePacket`](../../crates/aureline-auth/src/identity_modes/mod.rs).
Each [`IdentityModeBaselineRow`](../../crates/aureline-auth/src/identity_modes/mod.rs)
records:

- identity mode and account-boundary class;
- auth mode and provisioning class;
- account-free local-core continuity;
- policy-source class, freshness, bundle refs, and local inspection posture;
- entitlement state, offline behavior, grace or snapshot refs, and recovery
  action;
- the current deployment boundary the active profile actually provides.

The row validator rejects any packet that makes local core require account
creation, omits required local-core capability ids, hides policy-source or
entitlement detail on self-hosted and managed rows, or claims a broader
managed boundary than the active deployment provides.

The crate also exposes the shared `RegionMode`, `ResidencyMode`, and
`KeyMode` vocabulary consumed by provider-linked and managed truth rows.
Those modes include explicit `unknown` values so downstream registries
can flag missing locality or key truth rather than silently treating it
as allowed provider-default behavior.

The packet cites existing governance and identity artifacts instead of copying
their full schemas:

- [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md)
- [`docs/identity/offline_entitlement_and_policy_seed.md`](offline_entitlement_and_policy_seed.md)
- [`artifacts/governance/boundary_manifest_alpha.yaml`](../../artifacts/governance/boundary_manifest_alpha.yaml)
- [`artifacts/governance/entitlement_snapshot_alpha.yaml`](../../artifacts/governance/entitlement_snapshot_alpha.yaml)
- [`artifacts/governance/schema_registry_alpha.yaml`](../../artifacts/governance/schema_registry_alpha.yaml)
- [`artifacts/governance/record_class_registry_alpha.yaml`](../../artifacts/governance/record_class_registry_alpha.yaml)

## Shell Consumer

The first shell consumer is
[`TerminalPaneSnapshot`](../../crates/aureline-shell/src/terminal_pane/mod.rs).
It attaches [`IdentityModeSurfaceRow`](../../crates/aureline-auth/src/identity_modes/mod.rs)
records from an identity-mode baseline packet and renders them next to the
existing auth, credential-state, and claimed-identity rows.

The shell does not infer a generic signed-in state from these rows. It quotes
the packet projection so local-core availability, policy source, entitlement
state, offline behavior, and deployment boundary match support/export evidence.

## Protected Fixtures

Fixtures live under
[`fixtures/auth/identity_mode_alpha`](../../fixtures/auth/identity_mode_alpha/):

- `baseline_all_modes.json` proves the three identity modes can render
  together while account-free local core remains available.
- `managed_grace_pauses_new_managed.json` proves stale managed policy and
  grace entitlement pause new managed actions with visible recovery while
  local work continues.

All fixtures use refs and aliases only. Raw credentials, raw tenant names, raw
user email addresses, hosted-console payloads, signed policy bundle bodies, and
signed entitlement snapshot bodies are excluded.

## Verification

```sh
cargo test -p aureline-auth identity_modes --no-fail-fast
cargo test -p aureline-auth --test identity_mode_alpha_cases --no-fail-fast
cargo test -p aureline-shell terminal_pane::tests::snapshot_surfaces_identity_mode_policy_and_entitlement_inspectors --no-fail-fast
```
