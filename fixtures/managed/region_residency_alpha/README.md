# Region, Residency, Tenant, and Key-Mode Alpha Fixtures

Worked fixture for
[`/docs/managed/region_residency_alpha.md`](../../../docs/managed/region_residency_alpha.md)
and the shell projection in
[`/crates/aureline-shell/src/managed_truth/`](../../../crates/aureline-shell/src/managed_truth/).

The fixture uses opaque refs for tenants, regions, keys, provider
descriptors, operating-mode cards, and boundary manifests. Raw tenant
names, raw cloud regions, raw provider URLs, raw provider payloads,
raw key material, and certificate bodies do not appear.

| Fixture | Coverage |
|---|---|
| `claimed_managed_provider_boundary.yaml` | Enterprise SaaS managed row with visible region/tenant/storage/key truth, provider-linked row with provider-default residency and no sovereignty overclaim, and managed-workspace row where the control plane is unavailable while the data plane remains locally safe. |
