# Scope propagation alpha fixtures

Each fixture pairs an alpha `workset_artifact_record` with the beta
scope-truth a consumer surface projects, then names a single crossing
(remote helper attach, provider overlay link, export archive write,
browser-handoff mint, or support-packet bundle) and asserts the
propagation alpha record preserves the workset / scope labels verbatim.

Run the lane via:

```
cargo test -p aureline-workspace --test scope_propagation_alpha
```

## Cases

| File | Scenario |
| --- | --- |
| `remote_helper_attach_exact.json` | Sparse slice attached through a remote helper; the helper is reachable and the propagation preserves every scope label exactly. |
| `provider_overlay_link_degraded.json` | Selected workset linked to a provider overlay whose cache window expired; the propagation preserves labels and discloses the degraded reason. |
| `export_archive_write_blocked_by_outside_scope.json` | A search hit jumped into a sibling repo not in the active workset and an export was attempted; the crossing is blocked with `blocked_by_outside_scope` and a typed explain note. |
| `browser_handoff_mint_exact.json` | Workspace runtime lane mints a provider follow-up packet for a sparse slice; the propagation preserves scope labels and discloses the active attach state. |
| `support_packet_bundle_policy_limited.json` | Policy-limited managed-cloud view bundled into a support packet; the propagation preserves the underlying workset in lineage and the hidden_member_count without leaking the hidden list. |
