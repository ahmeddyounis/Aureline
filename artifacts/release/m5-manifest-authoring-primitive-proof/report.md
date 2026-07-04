# M5 Manifest-Authoring Primitive: Header, Schema/Validator Row, Chips, and Apply-Review Banner

- Packet: `m5-manifest-authoring-primitive:stable:0001`
- Label: `M5 Manifest-Authoring Primitive: Header, Schema/Validator Row, Chips, and Apply-Review Banner`
- Authoring surfaces: 6 / 6
- Source types: authored_file, rendered_artifact, imported_snapshot, provider_overlay, generated_template
- Execution origins: local_workspace, connected_cluster, dry_run_sandbox, imported_replay, provider_console
- Schema sources: bundled_with_app, remote_registry, cluster_discovered, imported_snapshot, provider_overlay, unknown

## Authoring surfaces

- **Desktop manifest editor**
  - Owner: Infrastructure editor guild
  - Scope: Authoring headers over source files with preview/apply entry points
  - Worked cases: 2
    - `authoring:web-deployment:0001` → `web-deployment.yaml` (authored_desired), schema `fresh`, apply available
    - `authoring:web-deployment-rendered:0002` → `web-deployment.rendered.yaml` (rendered), schema `fresh`, apply gated
- **Plan / preview pane**
  - Owner: Plan / dry-run guild
  - Scope: Plan diffs with disclosed schema freshness before apply
  - Worked cases: 1
    - `authoring:plan-preview:0003` → `ingress.yaml` (planned), schema `stale`, apply available
- **Cluster / resource explorer**
  - Owner: Live-resource guild
  - Scope: Read-only live explorer keeping target context and truth class visible
  - Worked cases: 1
    - `authoring:live-explorer:0004` → `web (live)` (live), schema `fresh`, apply gated
- **Apply-review dialog**
  - Owner: Apply-safety guild
  - Scope: Apply-review banner gating mutation on target, validation, and connector health
  - Worked cases: 2
    - `authoring:apply-review:0005` → `statefulset.yaml` (planned), schema `fresh`, apply gated
    - `authoring:web-deployment:0001` → `web-deployment.yaml` (authored_desired), schema `fresh`, apply available
- **Provider-console handoff**
  - Owner: Provider-overlay guild
  - Scope: Provider-console handoff naming overlay source and unversioned schema
  - Worked cases: 1
    - `authoring:provider-console:0006` → `load-balancer (provider overlay)` (provider_overlay), schema `unversioned`, apply gated
- **Support / export replay**
  - Owner: Support / diagnostics guild
  - Scope: Offline replay reconstructing authoring truth from an imported snapshot
  - Worked cases: 1
    - `authoring:support-replay:0007` → `web-deployment (imported snapshot)` (planned), schema `stale`, apply gated
