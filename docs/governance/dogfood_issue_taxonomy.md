# Dogfood issue taxonomy

This document defines the taxonomy for internal dogfood, design-partner,
and public-preview issues. It is written so issue labels, templates, and
intake automation can all resolve against one vocabulary instead of
inventing separate buckets for the same problem.

Use this taxonomy alongside:

- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  for the public/private route and owning forum;
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  for build identity fields every claim-bearing report should quote;
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  for `action_origin_class`, `action_target_class`,
  `action_route_class`, and `action_exposure_class`;
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  for dependency-marker and hidden-dependency rules;
- [`/docs/benchmarks/benchmark_lab_run_results.md`](../benchmarks/benchmark_lab_run_results.md)
  and
  [`/docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  for benchmark/public-proof evidence;
- [`/docs/security/severity_matrix.md`](../security/severity_matrix.md)
  for private security routing. Security issues do not use the public
  dogfood route.

## Goals

- Give every dogfood issue one primary category and one severity.
- Require enough evidence that support, engineering, and release can
  reconstruct what happened without guesswork.
- Keep public-truth, compatibility, benchmark, and hidden-dependency
  failures distinct from ordinary bugs.
- Reuse existing route, build, docs, and dependency vocabularies rather
  than inventing issue-only metadata.

## Suggested labels

Use these labels directly if you want an immediate label set:

- `dogfood:hot-path`
- `dogfood:fidelity`
- `dogfood:recovery`
- `dogfood:trust`
- `dogfood:ux`
- `dogfood:docs`
- `dogfood:compatibility`
- `dogfood:accessibility`
- `dogfood:ecosystem`
- `dogfood:public-proof`
- `dogfood:hidden-dependency`
- `sev:daily-blocker`
- `sev:major`
- `sev:scoped`
- `sev:clarity-gap`

## Intake record

Use the field names below in issue templates or intake automation.
Unknown values should stay explicit as `unknown_*`; they must not be
silently omitted or guessed.

```yaml
issue_route_class: oss_bug | perf_regression | supportability_issue | docs_truth_defect | design_system_defect | benchmark_dispute | security_issue | private_partner_case
dogfood_category: hot_path | fidelity | recovery | trust | ux | docs | compatibility | accessibility | ecosystem | public_proof | hidden_dependency
severity: daily_blocker | major | scoped | clarity_gap
exact_build_identity_ref: <required unless truly unavailable>
workspace_archetype: <reference workspace, partner repo, or local archetype>
workspace_size_class: micro | small | medium | large | monorepo | unknown
os_arch_profile: <os / arch / local-or-remote profile>
safe_mode_repro: yes | no | unknown | not_applicable
restricted_mode_repro: yes | no | unknown | not_applicable
headless_repro: yes | no | unknown | not_applicable
enabled_extensions_or_hosts:
  - <extension-or-host id/version>
route_context:
  command_id: <or unknown>
  invocation_session_id: <or unknown>
  action_origin_class: <token or unknown_origin_class>
  action_target_class: <token or unknown_target_class>
  action_route_class: <token or heuristic_unknown_route>
  action_exposure_class: <token or unknown_exposure_class>
  target_identity_ref: <or unknown>
finding_codes:
  - <Project Doctor or other stable finding code>
repair_candidate_ids:
  - <repair id>
evidence_refs:
  - <support bundle manifest / benchmark run / crash id / screenshot / trace / checkpoint / report ref>
docs_or_known_limit_refs:
  - <docs pack, known-limits, or claim ref>
dependency_marker_refs:
  - <dependency marker ref when a hidden dependency is suspected>
public_proof_refs:
  benchmark_packet_ref: <optional>
  compatibility_report_ref: <optional>
  migration_packet_ref: <optional>
```

## Severity rubric

| Severity | Use when | Typical response |
|---|---|---|
| `daily_blocker` | blocks daily dogfood on a claimed path, causes data-loss risk, breaks safe recovery, or makes a marketed claim actively misleading | route immediately to the owning lane; evidence or waiver needed before promotion |
| `major` | serious regression on a protected path or truth surface, but bounded by profile, archetype, or workaround | fix in the active train and attach fresh evidence |
| `scoped` | meaningful degradation with a clear narrow scope, fallback, or known limit | prioritize by affected profile and claim impact |
| `clarity_gap` | docs, copy, or workflow gap that does not currently break the protected path itself | fix before the mismatch becomes a claim or support burden |

If a report is actually a vulnerability, override this rubric and use the
private `security_issue` route.

## Category map

| Category | Use when | Default route | Required evidence links | Notes |
|---|---|---|---|---|
| `hot_path` | startup, typing, save, command dispatch, quick open, search, render, or another protected speed path regresses | `perf_regression` if measured, otherwise `oss_bug` | `exact_build_identity_ref`, host profile, benchmark run or journey trace, command or reproduction script, `safe_mode_repro` | Prefer benchmark or trace evidence over anecdote. |
| `fidelity` | the product acted on the wrong object, showed the wrong bytes, lost/source-misread state, or misreported canonical identity | `oss_bug` | exact build, expected vs observed, file/object identity, checkpoint or mutation refs, screenshot/diff | Includes wrong-target writes, stale result truth, and symbol/source-map mismatches. |
| `recovery` | crash loops, restore failures, bad safe-mode behavior, broken bisect/quarantine, repair-preview gaps, or rollback failures | `supportability_issue` | exact build, `finding_codes`, `repair_candidate_ids`, support-bundle manifest, checkpoint refs, `safe_mode_repro` | Use when blocked-user recovery is the failure, not just the underlying bug. |
| `trust` | policy, approval, route/origin disclosure, secret handling, preview safety, or provenance truth is wrong or missing | `supportability_issue`; escalate to `security_issue` when exploitability exists | exact build, route-context fields, policy epoch or approval refs, redaction/export manifest, trust state | Route truth and authority linkage matter more than screenshots here. |
| `ux` | discoverability, misleading state, missing next step, noisy attention behavior, or broken recovery path wording | `oss_bug` or `design_system_defect` | surface id, screenshot/video, expected recovery path, exact build | Use `design_system_defect` when the component/state contract is the issue. |
| `docs` | docs/help/known-limits/public text disagrees with product truth or lacks current evidence | `docs_truth_defect` | docs-pack or page ref, exact build, observed behavior, claim or known-limit ref | Treat docs drift as a product bug, not cleanup. |
| `compatibility` | archetype, OS, target, remote skew, import/export, or version-window support does not match the stated contract | `supportability_issue` or `oss_bug` | exact build, archetype/profile, target class, compatibility report or drift code, support bundle | Include whether the failure is local, remote, managed, or mixed-version. |
| `accessibility` | keyboard path, screen-reader output, focus return, IME, bidi, contrast, or motion contract failure | `design_system_defect` or `oss_bug` | exact build, platform/assistive tech, affected surface, reproduction steps, screenshots/audio/video where useful | Accessibility regressions on protected paths should usually be `major` or higher. |
| `ecosystem` | extension runtime, marketplace, language service, provider adapter, notebook/runtime host, or permission/dependency-marker issue | `supportability_issue` or `oss_bug` | exact build, extension/host ids and versions, host health/quarantine state, permission or route context | Use this instead of `compatibility` when the break lives in an attached capability, not the core archetype claim. |
| `public_proof` | benchmark packet, compatibility report, migration proof, claim language, or freshness state no longer matches reality | `benchmark_dispute` or `docs_truth_defect` | packet/report ref, exact build, freshness timestamp, docs/help version-match state, known-limit ref | This is the category for proof drift, not for the underlying bug the packet surfaced. |
| `hidden_dependency` | a stable-facing workflow secretly depends on Preview/Labs, a managed service, a paid seat, a missing mirror, or an undisclosed dependency marker | `docs_truth_defect` when claim drift is primary; otherwise `supportability_issue` or `oss_bug` | dependency-marker refs, effective lifecycle/support class, hidden prerequisite, exact build, fallback failure | Use when the issue is undeclared dependence, not just a broken feature. |

## Required field rules by category

| Field | Required for |
|---|---|
| `exact_build_identity_ref` | every category except pre-build planning issues |
| `safe_mode_repro` | `hot_path`, `fidelity`, `recovery`, `trust`, `compatibility`, `ecosystem` |
| route-context fields | `trust`, `compatibility`, `ecosystem`, `hidden_dependency`, and any report involving remote, browser, provider, or managed boundaries |
| `finding_codes` / `repair_candidate_ids` | `recovery`, `trust`, `compatibility`, `ecosystem` when Doctor or guided repair was involved |
| `docs_or_known_limit_refs` | `docs`, `compatibility`, `public_proof`, `hidden_dependency` |
| `dependency_marker_refs` | `hidden_dependency`, plus any `compatibility` or `ecosystem` issue caused by lifecycle narrowing |
| `public_proof_refs` | `public_proof`, plus claim-bearing `hot_path` or `compatibility` issues |

## Routing rules

- Pick exactly one primary `dogfood_category`.
- Pick the route from `issue_route_class`; the dogfood category does not
  replace the existing issue-routing table.
- Security reports never go to the public dogfood route even if the
  symptom looked like `trust` or `hidden_dependency`.
- Private partner cases may still use the same `dogfood_category`,
  severity, and evidence-link fields; only the route changes.

## Normalization rules

- Prefer stable IDs over prose: use finding codes, repair IDs, exact-
  build refs, invocation/session IDs, route tokens, dependency-marker
  refs, and packet refs whenever they exist.
- If a stable value is unavailable, record the typed unknown state and
  say why. Missing metadata is itself useful triage information.
- A public-proof or hidden-dependency issue stays open until the proof,
  docs, and dependency surface all agree again. Fixing the underlying bug
  but leaving stale claim language behind is incomplete closure.
