# M1 and M2 dependency backlog

This document turns the M1 and M2 milestone prose into a dependency-aware
backlog that the repository can use for sprint planning without
re-deriving M0 gates from scratch.

Companion artifacts:

- [`/artifacts/planning/m1_m2_work_breakdown.yaml`](../../artifacts/planning/m1_m2_work_breakdown.yaml) —
  machine-readable epic and task breakdown.
- [`/artifacts/planning/commitment_class_rollup.yaml`](../../artifacts/planning/commitment_class_rollup.yaml) —
  commitment-class counts, critical path, implementation waves, and
  start posture.

Planning posture:

- Source of truth for milestone bars: `.t2/docs/Aureline_Milestones_Document.md`.
- Source of truth for epic intent: `.t2/docs/Aureline_PRD.md` Appendix Q.
- Source of truth for live gate status: `artifacts/governance/decision_index.yaml`.
- Current execution owner is still `@ahmeddyounis` under the
  `single-maintainer-backup` waiver; planned owner teams below are the
  milestone-team names, not current staffed teams.
- PRD Appendix Q references RFC rows that are not yet authored in-repo.
  The backlog therefore binds to landed ADRs, prototypes, schemas, and
  decision rows, and treats missing contract freeze work as part of the
  first implementation wave.
- The task spec names `M00-61` as a dependency, but there is no
  `.plans/M00-61.md` or `.t2/handoff/M00-61.md` in the repository as of
  2026-04-21. No row below depends on an invisible artifact.

## Immediate M1 posture

| Posture | Backlog row | Why |
|---|---|---|
| Ready now | `m1.editor_io_recovery_core` | Buffer and VFS/save prototypes, save fixtures, and benchmark corpus are already landed. |
| Ready now | `m1.navigation_search_workset_truth` | Search truth vocabulary, reference workspaces, and filesystem truth already exist. |
| Ready now | `m1.execution_terminal_target_truth` | Execution-context, provider-approval, identity, and secret contracts are already decided. |
| Ready now | `m1.settings_tokens_activity_state` | Settings ADR, docs/help truth, and design/token artifacts are already seeded. |
| Partially blocked | `m1.shell_command_entry_surfaces` | The renderer spike exists, but `D-0006` and `D-0007` still leave the shell home and full keyboard model open. |
| Partially blocked | `m1.supportability_public_truth_governance_seed` | Support/export can start now, but `D-0011` still leaves exact-build joins open. |
| Target, partially blocked | `m1.automation_cli_seed` | Shared command metadata can be reused now, but the long-term shell/command home still depends on `D-0006`. |
| Blocked to hook-only | `m1.accessibility_install_review_hooks` | `D-0008`, `D-0007`, `DEP-0003`, and the still-proposed extension ADR keep this row from becoming a claimed surface. |

The practical implication is simple: most M1 engineering can start
immediately, but the team should not wait for every open decision before
building. The rows that need decision closure are narrow and explicit:
non-throwaway shell home, keyboard/accessibility coverage, exact-build
join semantics, and extension/install-review vocabulary.

## M1 backlog

| Epic | Class | Owner | Evidence owner | Critical path | Primary M0 gates | Notes |
|---|---|---|---|---|---|---|
| `m1.shell_command_entry_surfaces` | Committed | Shell/Editor | QE/Performance | Yes | renderer spike, shell spike traces, `D-0006`, `D-0007`, docs/help truth ADR | Start under the spike-hosted narrowing; close the permanent shell home before M2. |
| `m1.editor_io_recovery_core` | Committed | Platform/VFS | QE/Performance | Yes | buffer prototype, VFS prototype, save-truth fixtures, benchmark corpus | This is the first protected-path implementation row and should open sprint 1. |
| `m1.navigation_search_workset_truth` | Committed | Shell/Editor | QE/Performance | Yes | VFS prototype, reference workspaces, search truth ADR | Use the M0 readiness vocabulary from the first day; do not ship anonymous heuristic rows. |
| `m1.execution_terminal_target_truth` | Committed | Tooling/Execution | Supportability | Yes | execution-context ADR, provider approval ADR, identity modes ADR, route taxonomy | This is the anchor for M2 tasks/test/debug and must share one target/execution truth. |
| `m1.settings_tokens_activity_state` | Committed | UX/Design | Docs/Public Truth | No | settings ADR, lifecycle ADR, docs/help truth ADR | Keep the shell token/state layer and the settings resolver unified from the start. |
| `m1.supportability_public_truth_governance_seed` | Committed | Supportability | Supportability | Yes | benchmark corpus, docs/help truth ADR, lifecycle ADR, build baseline, safe-preview contract | This row converts the M0 artifact pile into nightly evidence and support/export structure. |
| `m1.automation_cli_seed` | Target | Tooling/Execution | Docs/Public Truth | No | `D-0006`, execution-context ADR, docs/help truth ADR | Pull forward just enough CLI/headless work to stop parity drift before M2. |
| `m1.accessibility_install_review_hooks` | Parked / hook-only | UX/Design | Security/Trust | No | `D-0008`, `D-0007`, `DEP-0003`, extension ADR seed | Preserve seams only; do not count this as delivered M1 scope. |

Task clusters inside the committed M1 rows:

- Shell and entry surfaces: promote the shell lane out of the throwaway spike or explicitly accept the spike-hosted narrowing, then land Start Center, command IDs, disabled reasons, degraded-state chips, and the first activity-center shell chrome.
- Editor and save core: join the piece-tree and save-plan prototypes, then add local history, crash journal, restore placeholders, and large-file/decode-recovery handoff.
- Navigation and workset truth: build file tree, quick open, search shell, path-truth chips, and named workset/sparse-scope seeds on the existing reference workspaces.
- Execution and terminal truth: make terminal, task-prep, and debug-prep surfaces share one execution-context inspector, one target/origin badge set, and one account-free-local-versus-managed vocabulary.
- Supportability and public truth: turn the benchmark corpus into nightly dashboards, seed support bundles and recovery ladders, and publish About/Help provenance plus stale-example checks.

## M2 backlog

| Epic | Class | Owner | Evidence owner | Critical path | Primary upstream chain | Notes |
|---|---|---|---|---|---|---|
| `m2.language_search_graph_alpha` | Committed | Language Platform | QE/Performance | Yes | M1 navigation/workset truth -> reference workspaces -> search truth ADR | This is the main alpha switching proof: useful navigation before full indexing finishes. |
| `m2.git_review_daily_loop_alpha` | Committed | Tooling/Execution | QE/Performance | No | M1 editor/save core -> reference workspaces -> safe-preview contract | Keep Git tied to mutation lineage and target/origin truth from day one. |
| `m2.tasks_test_debug_execution_alpha` | Committed | Tooling/Execution | Supportability | Yes | M1 execution/terminal truth -> M2 language alpha -> task-event skeleton | This row proves the shared execution-context model outside the terminal. |
| `m2.onboarding_activity_docs_alpha` | Committed | UX/Design | Docs/Public Truth | No | M1 shell/start surfaces -> docs-pack contract -> M1 supportability/public truth | External alpha depends on honest entry, import, docs, and notification behavior. |
| `m2.supportability_recovery_trust_alpha` | Committed | Supportability | Supportability | Yes | M1 support/export seed -> M2 execution alpha -> safe-preview contract | Project Doctor, safe mode, support bundles, and restricted-mode honesty live here. |
| `m2.provider_identity_policy_alpha` | Committed | Security/Trust | Security/Trust | No | M1 target truth -> provider approval ADR -> identity/secret ADRs | This row can progress now because the core contracts are already decided. |
| `m2.profile_sync_portability_alpha` | Target | Platform/VFS | Docs/Public Truth | No | M1 settings/state -> provider/identity alpha | Start early enough to de-risk M3, but cut first if critical-path rows go yellow. |
| `m2.release_evidence_exact_build_alpha` | Committed | Release | Release | Yes | M1 support/export seed -> `D-0011`/`D-0010` closure -> clean-room and mirror drills | Alpha should not widen without one build identity and a repeatable release evidence lane. |
| `m2.extension_install_review_alpha` | Parked / hook-only | Security/Trust | Security/Trust | No | extension ADR seed -> M1 hook row -> release/support joins | Keep the row visible, but do not promote it beyond hook-only until the ADR is accepted. |

Task clusters inside the committed M2 rows:

- Language and indexing alpha: tree-sitter, LSP router, diagnostics bus, hot-set indexing, ranking reasons, partial-index truth, deep-link remap, and graph-readiness/imported-fact cues on the launch bundle.
- Task/test/debug alpha: task runner, test runner, DAP host, output/problem surfaces, canonical task-event packets, and target/host-boundary truth on one local and one remote/helper row.
- Onboarding/docs alpha: admission checkpoints, import diff review, rollback checkpoints, no-account fast path, activity-center alpha, quiet-hours taxonomy, docs/help freshness chrome, and design-partner packets.
- Supportability/trust alpha: Project Doctor probe and finding taxonomy, support bundle alpha, incident-workspace export, restricted mode, suspicious-content cues, safe-preview posture, and recovery ladder growth.
- Release/evidence alpha: exact-build symbolication, support-bundle correlation, clean-room rebuild dry runs, mirror/offline publication dry runs, and protected fitness-function review packets.

## Gate notes by M0 artifact

Implementation can proceed immediately from these landed M0 artifacts:

- `crates/aureline-shell-spike/` plus `artifacts/render/spike_trace_samples/` and `docs/design/shell_spike_composition_notes.md`
- `crates/aureline-buffer/src/prototype/`
- `crates/aureline-vfs/src/` plus `fixtures/fs/save_truth_cases/`
- `fixtures/benchmarks/corpus_manifest.yaml` and `fixtures/workspaces/reference/`
- `docs/adr/0001`, `0007`, `0008`, `0009`, `0010`, `0011`, `0013`, and `0014`
- `docs/ux/shell_interaction_safety_contract.md`
- `docs/security/safe_preview_trust_classes.md`
- `docs/runtime/origin_target_route_taxonomy.md`
- `docs/docs/docs_pack_manifest_contract.md`

Open decisions that allow only narrowed progress:

- `D-0006`: shell home can stay inside the spike for one more milestone, but the non-throwaway home must close before the backlog widens.
- `D-0007`: shared command metadata can land now, but full keyboard-complete claims are still blocked.
- `D-0008`: renderer and shell accessibility hooks can land now, but accessibility readiness stays frozen.
- `D-0010`: release work can proceed, but alpha channels fall back to a single preview if the release posture stays open.
- `D-0011`: build/evidence plumbing can proceed, but cross-family build identity and symbolication cannot close until the ADR closes.
- `D-0018`: install-review and extension trust remain hook-only until the proposed ADR is accepted.

Missing or late inputs that should stay visible:

- `DEP-0003` is still open, so keyboard/accessibility review packet work has no final artifact home yet.
- The task-spec dependency `M00-61` has no repository artifact today and therefore cannot be treated as a real gate.

## Recommended sequencing

1. Open M1 with the editor/save, search/workset, terminal/target, and support/export seeds. Those rows already have enough M0 truth to start immediately.
2. Run shell-home and keyboard/input closure in parallel, but treat them as narrow decision lanes, not reasons to stall the rest of M1.
3. Start M2 language, task/debug, and supportability alpha work as soon as the M1 shell/editor/workset surfaces are stable on the reference workspaces.
4. Keep release/evidence alpha on the critical path once M1 nightly evidence exists; do not defer exact-build and preview-channel closure to the end of alpha.
5. Leave extension install review explicitly parked unless `docs/adr/0012-extension-manifest-permission-publisher-policy.md` graduates during M2.
