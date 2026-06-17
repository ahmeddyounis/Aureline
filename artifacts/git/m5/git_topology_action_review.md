# Topology Action Review Sheets

- Packet: `topology-action-review:0001`
- Sheets: 7

## Sheets

- **widen** → `sparse` (sparse_slice): network false, approval `not_network_bearing`, guard `target_matches_authoritative_root`, scope `active_root_only`
- **deepen** → `shallow` (shallow_history): network true, approval `approval_required`, guard `target_matches_authoritative_root`, scope `active_root_only`
- **initialize** → `submodule` (child_repo): network true, approval `approval_required`, guard `target_matches_authoritative_root`, scope `active_root_only`
- **hydrate** → `partial` (promisor_remote): network true, approval `approval_required`, guard `target_matches_authoritative_root`, scope `active_root_only`
- **hydrate** → `lfs` (pointer_backed_asset): network true, approval `approval_required`, guard `target_matches_authoritative_root`, scope `active_root_only`
- **hydrate** → `partial` (promisor_remote): network true, approval `approval_required`, guard `target_matches_authoritative_root`, scope `explicit_multi_root_preview_required`
- **widen** → `sparse` (sparse_slice): network false, approval `approval_required`, guard `retarget_required_wrong_root`, scope `mutation_denied`
