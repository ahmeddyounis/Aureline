# Region, tenant boundary, and key-mode drill packets (beta)

Reviewer-facing drill packets for the region, tenant boundary, and key-mode
beta projection. Every packet is a JSON record of one failure or failover
scenario exercised against the same managed action lane that the beta page
disclosed, and the page row links the packet by id.

The canonical record kind is
`security_region_tenant_key_mode_beta_drill_packet_record`. The schema lives
at
[`/schemas/security/region_tenant_key_mode_beta.schema.json`](../../../../schemas/security/region_tenant_key_mode_beta.schema.json).
The matrix that lists axes, lanes, profiles, and outcomes lives at
[`region_tenant_key_mode_matrix.yaml`](region_tenant_key_mode_matrix.yaml).

## Files

| File | Drill kind | Axis | Profile | Outcome |
| --- | --- | --- | --- | --- |
| `region_pinning_failure_001.json` | `region_pinning_failure` | region | `offline` | `narrowed_awaiting_admin` |
| `region_failover_001.json` | `region_failover` | region | `connected` | `failed_over_to_declared_fallback` |
| `tenant_boundary_drift_001.json` | `tenant_boundary_drift` | tenant | `connected` | `narrowed_then_recovered` |
| `tenant_failover_001.json` | `tenant_failover` | tenant | `offline` | `failed_over_to_declared_fallback` |
| `key_mode_drift_001.json` | `key_mode_drift` | key_mode | `enterprise_managed` | `narrowed_then_recovered` |
| `key_mode_failover_001.json` | `key_mode_failover` | key_mode | `offline` | `failed_over_to_declared_fallback` |

Each packet records the before / after state labels, the before / after
authority tokens on the affected lane, an export-safe explanation, and a hard
`sibling_lanes_unwidened` guarantee.

## How to regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-region-pinning-failure > region_pinning_failure_001.json
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-region-failover > region_failover_001.json
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-tenant-boundary-drift > tenant_boundary_drift_001.json
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-tenant-failover > tenant_failover_001.json
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-key-mode-drift > key_mode_drift_001.json
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-key-mode-failover > key_mode_failover_001.json
```

## Invariants

- A drill packet must not widen authority on a sibling lane. The validator
  rejects packets that set `sibling_lanes_unwidened = false`.
- A drill packet must preserve local editing. The validator rejects packets
  that set `local_editing_preserved = false`.
- A drill packet must exclude raw private / secret material. The validator
  rejects packets that set `raw_private_material_excluded = false`.
- The seeded beta page must include at least one drill packet for each of the
  three axes (`region`, `tenant`, `key_mode`).

See the reviewer-facing landing page at
[`/docs/security/m3/region_tenant_key_mode_beta.md`](../../../../docs/security/m3/region_tenant_key_mode_beta.md).
