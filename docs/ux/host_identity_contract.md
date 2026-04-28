# Host Identity, Path Truth, and Boundary-Change Presentation Contract

This contract freezes the presentation object that keeps host,
locality, route, and path truth visible across Aureline shell,
execution, preview, AI evidence, notebook, history, and export
surfaces. It exists so a remote host, container, managed workspace,
browser bridge, or service-plane action cannot look like ordinary local
desktop work merely because a surface has limited space.

Machine-readable companions:

- [`/schemas/contexts/host_identity_chip.schema.json`](../../schemas/contexts/host_identity_chip.schema.json)
  defines the `host_identity_chip_record` consumed by live chrome,
  execution headers, boundary-change banners, history rows, and export
  receipts.
- [`/fixtures/contexts/host_identity_cases/`](../../fixtures/contexts/host_identity_cases/)
  contains worked local, remote reconnect, managed failover,
  devcontainer switch, browser bridge, and service-plane route cases.

This contract composes with, and does not replace:

- [`/docs/ux/title_context_bar_contract.md`](./title_context_bar_contract.md)
  for the canonical shell identity tuple and title/context bar
  projection.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  for execution target, workset, trust, policy, and provenance fields.
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  for action origin, target, route, exposure, route-change, and
  authority-linkage vocabulary.
- [`/docs/verification/target_and_host_boundary_packet.md`](../verification/target_and_host_boundary_packet.md)
  and [`/artifacts/remote/host_boundary_matrix.yaml`](../../artifacts/remote/host_boundary_matrix.yaml)
  for host-boundary cue stacks and wrong-target reapproval posture.
- [`/docs/fs/path_truth_packet.md`](../fs/path_truth_packet.md)
  for path-truth chip semantics, alias disclosure, and save-target
  review vocabulary.
- [`/docs/execution/terminal_truth_contract.md`](../execution/terminal_truth_contract.md),
  [`/docs/execution/run_and_attempt_contract.md`](../execution/run_and_attempt_contract.md),
  [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md),
  and [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  for surface-specific runtime truth.

If this document disagrees with those upstream sources, the upstream
source wins and this document, schema, and fixtures must update in the
same change.

## Scope

This contract covers one cross-surface record:
`host_identity_chip_record`.

The record answers five questions:

1. Which logical workspace object, run, kernel, preview, evidence
   packet, or export receipt is being described?
2. Which actual host class and physical host/path basis owned the work?
3. Which route, tenant or organization scope, and boundary cue stack
   applied when the work happened?
4. Did the host, route, tenant, or path change, and how is the
   transition preserved in live history and export receipts?
5. Which copy-path, run-here, reveal, reconnect, and details actions
   can be offered without hiding host truth?

Out of scope: remote transport implementation, managed workspace
orchestration, route brokering, final icon artwork, and renderer layout.

## Host Classes

Aureline uses one controlled host taxonomy for every host identity chip.
Each class carries both the logical object and the physical execution
or storage truth.

| Host class | Meaning | Required display fields | Required consequence |
| --- | --- | --- | --- |
| `local_desktop` | Current desktop process, local helper, or same-device local runtime. | Logical object label, local desktop label, path basis, route class, and policy/trust state when not ordinary trusted local editing. | Local continuity may be assumed only while the record stays local and current. |
| `remote_host` | SSH, remote agent, tunnel, or attached remote environment. | Logical object label, remote target label/ref, remote path basis, route class, reachability, and prior host when reconnecting or retargeted. | Disconnect, reconnect, stale-target, and wrong-target states remain visible. |
| `container_devcontainer` | Local container, remote container, or declared devcontainer that owns filesystem or execution. | Logical object label, container/devcontainer label, image or profile ref, mounted path basis, and host-boundary cue stack. | Container identity and workspace mapping remain inspectable. |
| `managed_workspace` | Provisioned workspace, prebuild/runtime capsule, or managed remote environment. | Logical object label, managed workspace label/ref, persistence or lifecycle state, path basis, tenant/org scope, and expiry/suspend posture. | Persistence, suspend, failover, and local-safe continuation are explicit. |
| `browser_runtime_bridge` | Browser-attached runtime, system-browser handoff, preview-side helper, or browser companion. | Logical object label, browser/runtime target label, origin/route class, termination point, path or URI basis, and live/snapshot state. | Browser-only, cross-origin, and handoff limitations remain visible. |
| `service_plane` | Hosted control-plane, provider-owned operation, queue, audit, quota, publish, or policy route. | Logical object label, service/tenant label, operation path or object handle, route class, locality/residency note, and local-safe alternative when one exists. | Service degradation cannot imply the local desktop is degraded unless it is. |

`unknown_requires_review` is allowed only as a denied or inspect-only
state. A surface that cannot distinguish local from remote, browser, or
service-plane work must render the unknown state and block mutating
actions until repaired.

## Required Field Groups

Every `host_identity_chip_record` carries these field groups:

| Field group | Purpose |
| --- | --- |
| `logical_object` | Names the workspace/root/workset/run/debug session/kernel/AI evidence/preview/export object the user recognizes. |
| `host_identity` | Names the actual host class, stable host ref, safe display label, host state, lifecycle state, and boundary cue stack. |
| `path_truth` | Separates the presentation path or URI from the physical host/path basis, path handle, redaction posture, and path-truth record ref. |
| `route_truth` | Quotes action origin, target, route, exposure, route-change reason, authority linkage, tenant/org scope, and execution-context refs. |
| `lineage` | Preserves prior host/path/route when reconnect, failover, operator switch, or route/tenant/path change occurs. |
| `boundary_change_banner` | Required whenever `lineage.boundary_change_kind` is not `none`; contains user-impact copy and action bindings. |
| `actions` | Declares copy-path, run-here, reveal, details, reconnect, and export actions with labels that preserve host truth. |
| `surface_projections` | Proves title/context bar, terminal, task/debug, notebook, AI, preview, history, and export surfaces quote the same record. |
| `accessibility` | Text label, accessible name, icon token, and non-color cue guarantee. |

Raw hostnames, IP addresses, ports, account handles, tokens, secret
values, command lines, callback bodies, and unrestricted absolute paths
do not cross this boundary. Live UI may reveal a user-owned path through
a resolver action, but the record itself carries safe display labels and
opaque path handles.

## Propagation Rules

The host identity vocabulary follows the user across every host-aware
surface. A surface may condense the chip, but it must not replace the
record with private labels.

| Surface | Required projection |
| --- | --- |
| `title_context_bar` | Logical workspace label, host class, host state when non-local or degraded, route state, and boundary-change summary. |
| `terminal_header` | Host class, target label, current working path basis, live/transcript state, and last boundary change. |
| `task_launcher` | Target host, working directory/path basis, authority linkage, and run-here confirmation label before launch. |
| `debug_launcher` | Debug target host, adapter/runtime host, path mapping basis, and changed-host warning when reusing a configuration. |
| `notebook_kernel` | Kernel host, notebook logical object, kernel path basis, stale/imported output state, and reattach posture. |
| `ai_evidence` | Host and path where evidence was gathered or a tool ran, route/tenant scope, redaction posture, and replay limits. |
| `preview_strip` | Preview runtime host, termination point, source path basis, live/snapshot state, and copy/open route disclosure. |
| `history_row` | Prior/current host lineage, route-change reason, time, actor class, and export ref. |
| `export_receipt` | Same canonical tokens as live UI plus redacted labels, host lineage, boundary changes, and action/export refs. |

Rules:

1. Every projection lists the canonical field paths it renders.
   Surface-local host fields are non-conforming.
2. A projection may abbreviate a label, but the accessible name must
   include host class and state when non-local, degraded, changed, or
   unknown.
3. Terminal, task, debug, notebook, preview, and AI surfaces may not
   infer "local" from a missing remote field. Missing host truth is
   `unknown_requires_review`.
4. Exports quote the same canonical tokens seen live. They may redact
   labels and paths, but they may not collapse `managed_workspace`,
   `browser_runtime_bridge`, or `service_plane` into generic "remote".

## Boundary-Change Banners

A boundary-change banner is required when a host, route, tenant, or path
transition affects where work runs, where output was produced, or how a
path should be interpreted.

| Change kind | Trigger | Banner must show |
| --- | --- | --- |
| `reconnect` | A live session reconnects after transport loss. | Prior host, current host, identity-match state, cancelled or preserved work, and whether local-safe editing continues. |
| `failover` | Managed or service-hosted work moves to another region, instance, tenant lane, or host witness. | Prior host/route, new host/route, reason, persistence guarantees, and affected terminals/kernels/ports. |
| `operator_host_switch` | User or operator changes current execution host, container, devcontainer, or managed workspace. | Old host/path, new host/path, route/authority delta, and which defaults are or are not promoted. |
| `route_changed` | Route class changes without a logical-object change. | Old route, new route, authority linkage, exposure delta, and copy/open/share implications. |
| `tenant_changed` | Service-plane or managed workspace tenant/org scope changes. | Prior scope, new scope, identity/policy owner, affected stored evidence, and repair/escalation path. |
| `path_changed` | Physical path basis or canonical object changes under the same logical object. | Presentation path, prior physical path basis, new physical path basis, path-truth record ref, and safe reveal/copy actions. |

Rules:

1. Boundary changes append to lineage; they never overwrite prior host
   identity in place.
2. A changed host/path banner must be preserved in history and export
   receipts even after the live surface becomes healthy again.
3. Reconnect and failover banners must state whether prior commands,
   terminal input, notebook cells, debug sessions, ports, and AI tool
   calls were preserved, cancelled, replay-blocked, or require rerun.
4. Route, tenant, and path changes that broaden authority or exposure
   require an authority linkage or typed denial reason before action.

## Copy Path and Run Here

The chip owns action labels for path and execution affordances that
would otherwise be ambiguous.

`copy_path` must name the path basis it copies:

- `Copy local path`;
- `Copy remote path`;
- `Copy container path`;
- `Copy managed workspace path`;
- `Copy browser/runtime URI handle`;
- `Copy service-plane object handle`;
- `Copy redacted path receipt`.

`run_here` must name where execution will occur:

- `Run here on local desktop`;
- `Run here on remote host`;
- `Run here in devcontainer`;
- `Run here in managed workspace`;
- `Run through browser bridge`;
- `Run via service-plane route`.

Rules:

1. `copy_path` and `run_here` actions cite the same
   `host_identity.host_ref` and `path_truth.path_handle_ref` shown on
   the chip.
2. When the current host differs from the logical object's original
   host, both actions require confirmation that names old and new host
   labels.
3. A disabled action still renders a host-truth label and typed reason.
   "Run here" disabled by service-plane routing is not hidden.
4. Copying a path from a terminal transcript, preview strip, notebook
   output, or AI evidence packet must copy from this record or from the
   referenced path-truth record, never by scraping visible text.

## Accessibility and Redaction

Host cues do not rely on color alone. Every chip, banner, and projection
must expose:

- a text label containing the host class;
- an accessible name containing host class, host state, and changed
  boundary when applicable;
- an icon token or shape cue that is supplemental to text;
- the same information in export-safe metadata;
- a details action reachable by keyboard and screen reader.

Redaction changes labels, not truth. A record may replace a path label
with `redacted path` or a host label with `operator-restricted host`,
but it still carries the host class, route class, lineage kind, and
opaque refs required for reconstruction by authorized tooling.

## Fixture Coverage

The fixture corpus covers the conformance-critical cases:

- local desktop work with ordinary copy-path and run-here actions;
- remote host reconnect that preserves local-safe editing while remote
  execution is narrowed;
- managed workspace failover that keeps prior host lineage in history
  and export receipts;
- operator-driven switch from local desktop to devcontainer;
- browser/runtime bridge where preview output has a different origin
  and path basis than the workspace source;
- service-plane route/tenant change that keeps local work available
  while service writes are blocked or rerouted.

Adding a new host class, boundary-change kind, surface projection, or
action kind is additive-minor when the schema, contract, and fixtures
land together. Repurposing an existing token is breaking and requires a
new architecture decision row.
