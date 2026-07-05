# M5 Deployment/Continuity Surface Certification

- Packet: `m5-deployment-continuity-surface-certification:stable:0001`
- Matrix: `m5-deployment-continuity-component-matrix:stable:0001`
- Surfaces: 9 / 9 (7 narrowed)

## Surface Rows

- **local_only**: claimed=full_truth effective=full_truth auto_narrowed=false
- **managed**: claimed=full_truth effective=local_safe_only auto_narrowed=true
  - Narrowing: Control-plane outage narrows managed continuity to local-safe operation
- **self_hosted**: claimed=full_truth effective=degraded_narrowed auto_narrowed=true
  - Narrowing: Residual vendor dependency prevents a fully independent self-hosted label
- **mirrored**: claimed=full_truth effective=degraded_narrowed auto_narrowed=true
  - Narrowing: Mirror freshness is stale, so mirrored deployment cannot inherit a live label
- **sovereign**: claimed=full_truth effective=degraded_narrowed auto_narrowed=true
  - Narrowing: Sovereign deployment is narrowed until residual dependency review is current
- **air_gapped**: claimed=full_truth effective=local_safe_only auto_narrowed=true
  - Narrowing: Air-gapped deployment uses cached-offline truth and cannot claim live freshness
- **side_by_side**: claimed=full_truth effective=full_truth auto_narrowed=false
- **portable**: claimed=full_truth effective=local_safe_only auto_narrowed=true
  - Narrowing: Portable state root is unavailable, so portable install narrows to reattach-required
- **fleet_rollout**: claimed=full_truth effective=degraded_narrowed auto_narrowed=true
  - Narrowing: Rollout ring is held at canary pending promotion evidence
