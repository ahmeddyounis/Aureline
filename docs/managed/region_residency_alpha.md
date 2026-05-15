# Managed and Provider-Linked Region, Residency, Tenant, and Key Truth Alpha

This document defines the bounded alpha projection used by shell,
support, and review surfaces to show where claimed managed or
provider-linked work runs, where data or copies live, which tenant
boundary applies, and how keys are handled.

The runtime implementation lives in
[`/crates/aureline-shell/src/managed_truth/`](../../crates/aureline-shell/src/managed_truth/)
and the protected fixture lives in
[`/fixtures/managed/region_residency_alpha/`](../../fixtures/managed/region_residency_alpha/).

## Purpose

Managed and hosted claims are credible only when users and support
staff can inspect the active boundary without opening procurement
documents or guessing from a generic cloud badge. The alpha row answers
four questions for each claimed row:

- where processing runs;
- where data, managed copies, provider copies, or local-safe artifacts live;
- which tenant, provider account, or project boundary applies;
- which key mode and key-state posture applies.

It also carries separate control-plane and data-plane states. A control
plane outage may pause identity refresh, policy distribution, region
assignment, or new managed attach, while local edit/save/search/Git and
already-local artifacts remain available. The row must say that
directly instead of using whole-product failure copy.

## Consumed Contracts

The shell row is a projection, not a new source of truth. Each row cites
the upstream records it consumes:

- the managed boundary/offboarding manifest in
  [`/artifacts/governance/boundary_manifest_alpha.yaml`](../../artifacts/governance/boundary_manifest_alpha.yaml);
- the identity-mode baseline in
  [`/docs/identity/local_vs_managed_alpha.md`](../identity/local_vs_managed_alpha.md);
- the service operating-mode and region/key-state contract in
  [`/docs/service/operating_mode_and_capacity_contract.md`](../service/operating_mode_and_capacity_contract.md);
- the connected-provider registry in
  [`/docs/providers/connected_provider_alpha.md`](../providers/connected_provider_alpha.md).

Provider-linked rows must use provider-default residency or an explicit
unknown/review posture unless a provider contract actually proves a
stronger boundary. A provider-linked row must not imply customer-pinned,
self-hosted, sovereign, or regulated operation merely because the
desktop runs in such a profile.

Connected-provider descriptors carry `region_mode`, `residency_mode`,
and `key_mode` from the identity-mode vocabulary. Shell provider rows
project those fields directly; omitted provider region, residency, or
key truth remains `unknown` and triggers review instead of silently
rendering as allowed provider-default truth.

## Row Shape

`ManagedTruthRow` carries:

- `region_residency`: region scope, opaque region ref, residency scope,
  locality residency disclosure class, and a redaction-safe summary;
- `tenant`: tenant or provider account/project scope, opaque ref, and summary;
- `storage_copy`: processing location, storage location, copy posture,
  retention class, and summary;
- `key`: key mode, key state, opaque key ref, affected action families,
  and fail posture;
- `planes`: control-plane state, data-plane state, affected action
  families, fail posture, and last observed timestamps;
- `local_continuity`: retained local-safe capabilities and any blocked
  managed/provider-only capabilities;
- `sovereignty`: actual boundary class plus residual dependency refs;
- `display_copy`: invariants that keep whole-product failure, stronger
  sovereignty, silent fail-open, and plaintext-secret fallback claims false.

## Required Invariants

- Claimed managed rows must disclose non-`not_applicable` region,
  tenant, storage/copy, and key-mode truth.
- Provider-linked rows must cite the connected-provider registry and
  must not reuse managed-tenant residency vocabulary or sovereign
  boundary classes.
- Any key-state or plane impairment must list the minimum affected
  action families and a fail posture.
- Plane impairment must distinguish control plane from data plane when
  either is impaired.
- Every row must keep local-safe continuation visible.
- Support export is metadata-only and excludes raw tenant names, raw
  cloud regions, raw provider URLs, provider payloads, key bytes,
  certificate bodies, and secret material.

## First Consumer

`SupportSeedSurface::managed_truth_preview` adds a metadata-only
support-preview item with artifact kind
`managed_truth_export_packet`. The preview lets support and review
flows inspect the same row the shell display consumes, without scraping
UI text or inlining unsafe payloads.

## Verification

Run the shell fixture test:

```sh
cargo test -p aureline-shell --test managed_region_residency_alpha --no-fail-fast
```
