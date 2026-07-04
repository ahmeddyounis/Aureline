# M5 Deployment-Summary Primitive: Deployment Summary Card, Residual-Dependency Rows, and Control/Data-Plane Status Strip

- Packet: `m5-deployment-summary-primitive:stable:0001`
- Label: `M5 Deployment-Summary Primitive: Deployment Summary Card, Residual-Dependency Rows, and Control/Data-Plane Status Strip`
- Deployment surfaces: 6 / 6
- Deployment scopes: shared_managed, dedicated_managed, self_hosted, sovereign, local_only
- Failure consequences: blocks_activation, blocks_updates, blocks_sign_in, degrades_optional_feature, no_user_impact
- Local-safe next steps: continue_local_work, work_offline_cached, retry_control_plane, restore_from_checkpoint, contact_admin

## Deployment surfaces

- **About / deployment summary card**
  - Owner: Deployment-summary guild
  - Scope: About-page deployment summary card naming scope, tenant/region, mirror posture, and last control-plane sync
  - Worked cases: 1
    - `deployment:shared-managed:0001` → scope `shared_managed`, mode `managed`, planes `operational`/`operational`, residual `2`, next `continue_local_work`
- **Admin deployment console**
  - Owner: Deployment-admin guild
  - Scope: Admin deployment console keeping a self-hosted boundary honest about its residual vendor dependency
  - Worked cases: 1
    - `deployment:self-hosted:0002` → scope `self_hosted`, mode `self_hosted`, planes `degraded`/`operational`, residual `2`, next `continue_local_work`
- **Service-health panel**
  - Owner: Service-health guild
  - Scope: Service-health panel keeping control-plane and data-plane health distinct with a local-safe next step
  - Worked cases: 1
    - `deployment:sovereign:0003` → scope `sovereign`, mode `air_gapped`, planes `unavailable`/`operational`, residual `1`, next `work_offline_cached`
- **Diagnostics deployment pane**
  - Owner: Diagnostics guild
  - Scope: Diagnostics deployment pane separating a degraded control plane from a degraded data plane
  - Worked cases: 1
    - `deployment:dedicated-managed:0004` → scope `dedicated_managed`, mode `managed`, planes `degraded`/`degraded`, residual `1`, next `retry_control_plane`
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing deployment scope, planes, and residual dependency from an imported snapshot
  - Worked cases: 1
    - `deployment:support-replay:0005` → scope `shared_managed`, mode `managed`, planes `unknown`/`operational`, residual `1`, next `continue_local_work`
- **Docs deployment reference**
  - Owner: Docs / help guild
  - Scope: Docs deployment reference framing a local-only desktop install with no control plane or residual dependency
  - Worked cases: 1
    - `deployment:local-only:0006` → scope `local_only`, mode `desktop`, planes `unknown`/`operational`, residual `0`, next `continue_local_work`
