# Service Topology And Process Placement

This document is the normative service-topology and process-placement map
for Aureline's protected paths. It turns the architecture document's
interaction-plane and runtime rules into one repo-local contract that
benchmark work, shell-spike work, and CI checks can all cite without
re-inventing boundary language.

Companion artifacts:

- [`/artifacts/architecture/protected_path_dependency_rules.yaml`](../../artifacts/architecture/protected_path_dependency_rules.yaml)
  - compile-time dependency classes, protected package rows, monitored
    hot-path modules, and sentinel patterns that CI enforces.
- [`/artifacts/architecture/process_placement_map.yaml`](../../artifacts/architecture/process_placement_map.yaml)
  - machine-readable plane map, process roles, scheduling classes, and
    placement policy.
- [`/artifacts/perf/protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml)
  - stable protected-path ids that the topology rows below reference.
- [`/tools/check_protected_dependencies.py`](../../tools/check_protected_dependencies.py)
  - direct validator entry point for the protected dependency and
    blocking-I/O-on-hot-path checks.

## Why this exists

The architecture spec is already clear about the non-negotiables:

- protected shell/editor/render paths perform no blocking filesystem,
  network, or process-launch I/O;
- compile-time dependency direction on protected paths is explicit; and
- process placement must keep failures and slow work off the shell.

What this repo lacked was one place that answers three operational
questions at the same time:

1. Which service plane owns a package or module?
2. Which process or worker class may run that work?
3. Which dependency directions and hot-path imports are forbidden enough
   that CI should fail the change?

This document is that join point.

## Plane map

Two different boundaries matter and should not be conflated:

- **Runtime/service interaction**
  - typed RPC, command dispatch, or append-only event flow between
    cooperating services.
- **Compile-time/package dependency**
  - direct crate or module imports that make hidden coupling the default.

The runtime topology is broader than the compile-time topology. The shell
may issue a typed request to search or task execution; that does not mean
the shell package is allowed to compile directly against those services.

| Plane | Owns | Seeded packages or modules today | Primary placement | Protected-path rule |
| --- | --- | --- | --- | --- |
| `shell_ui` | input dispatch, command routing, view models, layout, accessibility entry | `aureline-shell-spike`; `crates/aureline-shell-spike/src/input_path.rs` | desktop shell process, `ui_input` lane | never performs blocking I/O or process launch inline |
| `renderer` | scene composition, damage classification, frame submit seam | `aureline-render`; `crates/aureline-shell-spike/src/render_path.rs` | desktop shell process, `render_submission` lane | stays local and bounded; no blocking waits |
| `text_buffer` | text primitives, selections, undo/redo, visible edit state | `aureline-text`, `aureline-buffer` | desktop shell process for visible edits; background workers for non-visible analysis | visible edit path stays local-first and cancellation-friendly |
| `vfs_watchers` | canonical paths, saves, watch state, external-change truth | `aureline-vfs` | knowledge worker group | watcher, save, and identity work never run inline on the hot path |
| `index_search` | search, graph, workspace knowledge, ranking | none seeded in the production cone yet; `aureline-graph-proto` is off-cone | knowledge worker group | partial/stale answers are surfaced instead of stalling shell input |
| `task_execution` | task/test/debug launch, PTY/Git helper routing, provenance | none seeded yet | execution helper group | launch or attach work is helper-owned, never shell-inline |
| `remote_helper` | typed RPC fabric, remote connectors, helper bridges | `aureline-rpc` | local supervisor plus remote connector | transport helpers stay inspectable and never mutate shell semantics silently |
| `ai_control_plane` | AI routing, policy, entitlements, hosted metadata, org state | none seeded yet | helper process or managed service boundary | failure may degrade assistance, not core editing |
| `updater_release` | update checks, package/install operations, release metadata | none seeded yet | updater helper or installer process | install/update work is never on the input/render path |
| `support_diagnostics` | tracing, metrics, crash capture, support export, benchmark evidence | `aureline-telemetry`; benchmark and support tooling | background worker or support collector | writes are non-blocking and bounded on protected paths |

## Runtime call directions

These are the allowed logical service interactions. They describe who may
issue typed requests or consume events from whom. They are broader than the
compile-time dependency directions enforced in the rule file.

| Source plane | Allowed runtime targets |
| --- | --- |
| `shell_ui` | `renderer`, `text_buffer`, `vfs_watchers`, `index_search`, `task_execution`, `remote_helper`, `ai_control_plane`, `updater_release`, `support_diagnostics` |
| `renderer` | `text_buffer`, `support_diagnostics` |
| `text_buffer` | `renderer`, `support_diagnostics`, `vfs_watchers` |
| `vfs_watchers` | `text_buffer`, `index_search`, `task_execution`, `remote_helper`, `support_diagnostics` |
| `index_search` | `vfs_watchers`, `task_execution`, `remote_helper`, `support_diagnostics` |
| `task_execution` | `vfs_watchers`, `remote_helper`, `support_diagnostics`, `ai_control_plane` |
| `remote_helper` | `task_execution`, `vfs_watchers`, `ai_control_plane`, `support_diagnostics` |
| `ai_control_plane` | `index_search`, `task_execution`, `remote_helper`, `support_diagnostics` |
| `updater_release` | `remote_helper`, `support_diagnostics`, `ai_control_plane` |
| `support_diagnostics` | every plane as a sink for bounded telemetry or export-safe diagnostics |

Forbidden runtime shortcuts:

- `shell_ui` or `renderer` directly opening files, sockets, or subprocesses
  instead of routing through VFS, helper, or supervisor-owned contracts.
- feature-local path canonicalization, file watching, or save behavior
  outside `vfs_watchers`.
- alternate transport or remote semantics that bypass `remote_helper`.
- hidden support, telemetry, or update work that widens privilege or egress
  from the shell path.

## Compile-time dependency directions

The machine-checked package rules are intentionally narrower than the
runtime call map.

| Package class | Allowed compile-time dependency classes |
| --- | --- |
| `shell_ui` | `renderer`, `text_buffer`, `vfs_watchers`, `remote_helper`, `support_diagnostics` |
| `renderer` | `text_buffer`, `support_diagnostics` |
| `text_buffer` | `text_buffer`, `support_diagnostics` |
| `vfs_watchers` | `text_buffer`, `support_diagnostics` |
| `index_search` | `vfs_watchers`, `text_buffer`, `remote_helper`, `support_diagnostics` |
| `task_execution` | `vfs_watchers`, `remote_helper`, `support_diagnostics` |
| `remote_helper` | `support_diagnostics` |
| `ai_control_plane` | `index_search`, `task_execution`, `remote_helper`, `support_diagnostics` |
| `updater_release` | `remote_helper`, `support_diagnostics` |
| `support_diagnostics` | none; this is a leaf for protected packages |
| `off_cone` | any class allowed only for off-cone or disposable packages |

The repo now enforces this at two levels:

- exact package edges still resolve through
  [`/artifacts/governance/package_inventory.yaml`](../../artifacts/governance/package_inventory.yaml);
- plane and package-class direction, plus hot-path sentinels, resolve
  through
  [`/artifacts/architecture/protected_path_dependency_rules.yaml`](../../artifacts/architecture/protected_path_dependency_rules.yaml).

## Process placement policy

| Placement class | Work that may run here | Work that must not run here |
| --- | --- | --- |
| `input_render_inline` | input resolution, caret and selection updates, dirty-rect classification, visible text shaping/layout, frame timing stamps, cancellation of stale work, bounded telemetry emission | filesystem I/O, network I/O, process launch, archive or compression work, workspace scans, symbol indexing, support bundle generation, update checks, sleeps and blocking waits |
| `foreground_worker` | quick-open ranking, visible search refinement, hover/completion requests, save intent handoff, remote capability negotiation, terminal/debug/task progress fan-out | durable support export, full index rebuild, package install/update, provider polling, unbounded retries |
| `background_worker` | watcher normalization, index maintenance, graph ingest, semantic cache refresh, docs indexing, diagnostics aggregation, benchmark post-processing | anything that blocks the shell waiting for completion |
| `helper_process` | PTY host, Git helper, debug adapters, remote connector, updater/install helper, external extension hosts, AI/tool brokers, crash symbolication | owning unsaved editor state or mutating shell-owned view models directly |

## Protected modules

The first CI sentinel covers the two shell-spike modules that are closest
to the input/render path:

- [`/crates/aureline-shell-spike/src/input_path.rs`](../../crates/aureline-shell-spike/src/input_path.rs)
  - may import only the local hook vocabulary; must not pull in
    filesystem, network, process-launch, or blocking-wait APIs.
- [`/crates/aureline-shell-spike/src/render_path.rs`](../../crates/aureline-shell-spike/src/render_path.rs)
  - may import only local input, zone, and hook seams; must not pull in
    blocking I/O or process-launch APIs.

This is intentionally a seed, not whole-repo static analysis. The current
bar is:

- protected packages have explicit dependency classes and forbidden
  directions;
- protected hot-path modules have explicit monitored files and sentinel
  patterns; and
- CI can fail a pull request when one of those declared boundaries is
  crossed without updating the rule set.

## Path linkage

The topology map aligns with the protected-path ledger rows below:

- `path.shell.first_useful_chrome`
- `path.command_palette.open`
- `path.editor.first_useful_edit`
- `path.editor.save`

Those ids remain the stable public names for benchmark and journey work.
This document governs where the corresponding work is allowed to run and
which compile-time edges are acceptable while implementing them.

## Local and CI verification

Run the direct checker:

```bash
python3 tools/check_protected_dependencies.py \
  --repo-root . \
  --report target/contract-validation/protected_dependency_report.json
```

Run the shared CI wrapper:

```bash
./ci/contract_validation.sh --out-dir target/contract-validation
```
