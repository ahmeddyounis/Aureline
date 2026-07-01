# M5 resume breadcrumbs: partial-truth and resume lineage across degraded or restored M5 journeys

Generated from the seeded packet in
[`crate::m5_resume_breadcrumbs`](../../crates/aureline-shell/src/m5_resume_breadcrumbs/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_resume_breadcrumbs -- markdown > \
  artifacts/lifecycle/m5-resume-breadcrumbs.md
```

- Packet id: `m5-resume-breadcrumbs:stable:0001`
- Source schema ref: `schemas/lifecycle/m5-resume-breadcrumbs.schema.json`
- Certifies matrix packet: `m5-lifecycle-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required breadcrumb dimensions: `provenance_labeling`, `lineage_breadcrumb`, `not_resumed_disclosure`, `capture_parity`
- Object families certified: 13
- Green (full breadcrumbs): 9
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Object family | Status | Provenance | Lineage | Not resumed | Capture | Headless | Waiver |
| ------------- | ------ | ---------- | ------- | ----------- | ------- | -------- | ------ |
| Workspace / window session | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Installed extension | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Remote / tunnel session | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Collaboration session | `yellow` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `disclosed_grouped_not_resumed_summary` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | `waiver:collaboration-grouped-not-resumed:0001` |
| AI assistant action | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Update / rollback | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Notebook runtime | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Request / API run | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Preview / live-server session | `yellow` | `provenance_class_labeled_on_every_surface` | `disclosed_partial_lineage_breadcrumb` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Pipeline / task run | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Data / database session | `green` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |
| Profiler / trace capture | `yellow` | `provenance_class_labeled_on_every_surface` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `disclosed_partial_capture` | `true` | — |
| Companion / paired-device session | `yellow` | `disclosed_coarse_provenance_grouping` | `source_actor_boundary_checkpoint_preserved` | `not_resumed_actions_explicit` | `breadcrumbs_captured_in_export_and_screenshot` | `true` | — |

## Auto-narrowed rows

- `collaboration_session` (`yellow`) — When a collaboration session reconnects, the journey discloses a grouped, waivered summary of the actions it intentionally did not rerun or reauthorize — the pending control-transfer requests and outbound presence broadcasts are named as one withheld category rather than each individually — while still disclosing that actions were withheld and offering the reconnect affordance, so the collaboration breadcrumb is narrowed and disclosed rather than leaving the not-resumed set silently absent.
- `preview_session` (`yellow`) — On the compact preview status strip the preview session shows a disclosed partial lineage breadcrumb — the host/boundary facet is dropped while the source class, rebuilding subsystem, and checkpoint lineage are still named — so the preview breadcrumb is narrowed and disclosed rather than collapsing into generic recovered wording.
- `profiler_capture` (`yellow`) — The profiler capture exports a disclosed reduced subset of its breadcrumb detail — intermediate lineage steps are collapsed in the compact trace export while the provenance header and terminal breadcrumb are still captured — so the captured breadcrumb truth is narrowed and disclosed rather than absent from the export.
- `companion_session` (`yellow`) — On the small companion / paired-device surface the session presents a disclosed coarse provenance grouping — restored context and cached evidence are grouped under one disclosed recovered-context header while live truth and the restart-required placeholder stay distinct — so the companion breadcrumb is narrowed and disclosed rather than leaving the provenance ambiguous.

## Exact breadcrumb causes

- `collaboration_session` — `upstream_dependency_narrowed` (disclosed: `true`) — The object presents a disclosed, waivered grouped summary of the actions it intentionally did not rerun or reauthorize after restore or reconnect — naming the withheld category rather than each action — while still disclosing that actions were withheld, so the not-resumed disclosure is narrowed and disclosed rather than silently absent.
- `preview_session` — `upstream_dependency_narrowed` (disclosed: `true`) — The object shows a disclosed partial lineage breadcrumb on a compact surface — dropping one facet such as the host/boundary detail while still naming the source class, actor/subsystem, and checkpoint lineage — so the breadcrumb lineage is narrowed and disclosed rather than collapsing into generic recovered wording.
- `profiler_capture` — `upstream_dependency_narrowed` (disclosed: `true`) — The object captures a disclosed reduced subset of its breadcrumb detail in a compact export while still capturing the provenance header and terminal breadcrumb, so the captured breadcrumb truth is narrowed and disclosed rather than absent.
- `companion_session` — `upstream_dependency_narrowed` (disclosed: `true`) — The object presents a disclosed coarse provenance grouping on a compact surface — for example grouping restored context and cached evidence under one disclosed recovered-context header — while still distinguishing the four classes, so the provenance labeling is narrowed and disclosed rather than ambiguous.

## Active waivers

- `waiver:collaboration-grouped-not-resumed:0001` (`collaboration_session`, owner: Collaboration owner, expires `2026-09-30T00:00:00Z`) — When a collaboration session reconnects after a dropped shared connection, the journey discloses a grouped summary of the actions it intentionally did not rerun or reauthorize — the pending control-transfer requests and outbound presence broadcasts are named as one withheld category rather than each request individually — while still disclosing that actions were withheld and offering the reconnect affordance to reauthorize them. The grouped summary is disclosed, never silent, and the itemized not-resumed set is restored the moment the collaboration lane rejoins.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_resume_breadcrumbs -- validate
cargo test -p aureline-shell --test m5_resume_breadcrumbs_fixtures
```
