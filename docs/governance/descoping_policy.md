# Milestone descoping policy

This policy turns the roadmap's scope-control rules into one operating
path for backlog review, scorecards, and shiproom decisions. Its purpose
is to make schedule pressure visible and repeatable instead of letting it
quietly redefine quality bars.

Companion artifacts:

- [`/artifacts/milestones/cut_classes.yaml`](../../artifacts/milestones/cut_classes.yaml)
  — machine-readable cut-class ledger for backlog epics and protected
  requirement rows.
- [`/artifacts/milestones/kill_criteria.yaml`](../../artifacts/milestones/kill_criteria.yaml)
  — machine-readable kill rows for latency, trust, recovery,
  compatibility, accessibility, and launch-language quality.
- [`/artifacts/governance/correction_trigger_table.yaml`](../../artifacts/governance/correction_trigger_table.yaml)
  — current-milestone correction triggers that already bind scorecard
  lanes, risks, and dependencies to typed responses.
- [`./commitment_and_rebaseline_policy.md`](./commitment_and_rebaseline_policy.md)
  — commitment-class, rebaseline, and repeated-exception policy.
- [`./change_budget_workflow.md`](./change_budget_workflow.md)
  — protected-change budget and exception-packet workflow.
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  and [`./claim_manifest_contract.md`](./claim_manifest_contract.md)
  — canonical claim families and downgrade rules that descoping must
  reuse instead of replacing with ad hoc wording.

## Operating rules

- Reduce breadth before truthfulness. Fewer languages, frameworks,
  integrations, or channels are acceptable; stale or inflated claims are
  not.
- Reduce surface area before core guarantees. New panes, alternate
  surfaces, polish, or convenience tooling give way before local
  responsiveness, save fidelity, rollback, trust, or accessibility.
- Reduce support level before pretending parity. A lane may narrow from
  certified to supported-with-caveats, from replacement-grade to limited,
  or from stable to preview when the evidence says so.
- Reduce automation convenience before safety. Preview, approval,
  evidence, rollback, and export-safe recovery remain in place on
  multi-file, external-side-effect, and AI-bearing flows.
- A descoping decision is incomplete until the affected claim row,
  scorecard lane, and support/docs posture are narrowed in the same
  train.

## Descoping ladder

Use the ladder below in order when a milestone slips:

| Order | Cut first | Hold line on |
|---|---|---|
| **1** | nice-to-have AI affordances, agentic convenience, and non-essential assistant chrome | core language, search, edit, save, Git, terminal, trust, and recovery workflows |
| **2** | collaboration polish, shared-control breadth, and optional multi-user depth | single-user correctness, latency, and local continuity |
| **3** | long-tail language or framework breadth | launch-language excellence and honest support-class labeling |
| **4** | built-in convenience tools that still have extension, CLI, or external-tool paths | canonical developer workflows and protected truth surfaces |
| **5** | new panes, alternate entry surfaces, premium differentiators, and secondary chrome | command palette, keyboard-first paths, existing-pane flows, and degraded-state honesty |
| **6** | visual polish, theming depth, minimap-style flourish, and extra onboarding gloss | quality bars, accessibility cues, diagnostics/help truth, and release evidence |

## Never-cut bars

These rows do not get traded for schedule. When one is late, the answer
is to cut other breadth, narrow claims, or move the date.

| Protected row | Current repository binding | Default response when late |
|---|---|---|
| **Typing latency and editor correctness** | `PERF-EDITOR-002`, `ARCH-INV-001` | cut AI, chrome, or secondary surfaces before relaxing the bar |
| **Crash recovery and session restore** | `REL-CORE-003`, `m2.supportability_recovery_trust_alpha` | keep restore checkpoints, repair paths, and honest downgrade states; cut other breadth |
| **Launch-language navigation, rename, diagnostics, and debug basics** | spec rows `FR-SEARCH-002`, `FR-DEBUG-001`, `LANG-WEB-001`, `LANG-CORE-001` | reduce launch bundle breadth or support class; do not ship shallow parity |
| **Git fundamentals** | spec row `FR-VCS-001`, `m2.git_review_daily_loop_alpha` | cut review polish or secondary VCS depth before daily-loop basics |
| **Terminal reliability** | `m1.execution_terminal_target_truth`, `TOOL-CTX-001` | narrow tasks/debug breadth before degrading terminal/manual launch truth |
| **Accessibility baselines** | `A11Y-CORE-002`, spec row `A11Y-SR-003` | disable or cut the offending feature; do not lower the baseline |
| **Workspace trust and update integrity** | `GOV-OSS-001`, spec row `SEC-TRUST-001`, `OPS-BUILD-006` | narrow to local-only, restricted, or preview posture, or hold the train |
| **Public-truth and evidence freshness for live claims** | `GOV-TRUTH-901`, `GOV-EVID-901`, `CERT-WS-001` | downgrade the claim, badge, or report immediately; do not leave stale green wording in place |

## Milestone-at-risk defaults

The policy below adapts the PRD's milestone cut guidance to the current
repository state.

| Milestone | Cut first | Hold line on |
|---|---|---|
| **M0 - Architecture freeze and benchmark harness** | release-grade/public-proof wording, unresolved contract-family breadth, extension or managed-service ambition, and extra shell polish | architecture pack, benchmark corpus and ledgers, requirement register, ownership coverage, correction triggers, and exact-build/public-truth narrowing rules |
| **M1 - Core prototype** | theming depth, minimap-like chrome, non-essential UI polish, and AI surfaces | renderer latency, buffer correctness, VFS/watchers, command system, save/recovery, and truthful degraded-state vocabulary |
| **M2 - Alpha** | collaboration groundwork, notebooks, database tools, advanced framework packs, and provider breadth that weakens desktop quality | search usefulness, launch-language seed depth, Git basics, terminal, restricted mode, crash recovery, support/export skeleton, and onboarding honesty |
| **M3 - Beta** | cloud-workspace ambition, visual designers, voice coding, secondary importers, and broad extension marketing | stable extension host truth, trust/restricted mode, packaging, migration path, exact-build publication, compatibility reporting, and supportability drills |
| **M4 - v1.0 stable** | broad long-tail language expansion, deep built-in ops tooling, premium differentiators, and non-essential breadth | quality bars, launch-language bundles, certified reference workspaces, docs/help truth, benchmark proofs, security posture, accessibility readiness, and support evidence |

## How a slip is handled

1. Classify the affected epic and requirement rows with
   [`cut_classes.yaml`](../../artifacts/milestones/cut_classes.yaml).
   The reviewer should not invent a fresh category such as "probably
   optional" or "nice to have later."
2. If the row is `hard_block_never_cut`, cut other breadth or move the
   date. Do not lower the bar, hide the miss, or downgrade the guarantee
   itself.
3. If the row is `claim_narrowing_before_quality_trade`, lower the
   support class, launch bundle, archetype set, channel, or public
   wording before reducing proof or safety requirements.
4. If the row is `hooks_only_until_contracts_close`, keep the seam and
   its truthful labels, but do not promote it beyond hook-only or
   experimental posture.
5. When the same quality lane misses twice or lacks current evidence,
   apply the matching row in
   [`kill_criteria.yaml`](../../artifacts/milestones/kill_criteria.yaml)
   and the current-milestone trigger in
   [`correction_trigger_table.yaml`](../../artifacts/governance/correction_trigger_table.yaml)
   together. Repeated misses are correction work or explicit scope cuts,
   not quiet carry-forward.

## Repeated misses and stale proof

- **First miss:** cut target/stretch work first, narrow claims if the row
  allows it, and keep the lane out of green milestone-close language.
- **Second consecutive miss:** freeze additional breadth on the affected
  lane, choose explicit correction work or rebaseline, and update the
  scorecard or packet in the same change.
- **No current evidence:** treat the row as blocked for promotion or
  claim-bearing use. Stale, missing, or seed-only proof does not count as
  current evidence when the lane is trying to graduate.
- **No one-off language:** "almost ready," "good enough for now," and
  similar phrases are not valid governance outcomes. The output must be a
  cut, a narrowed claim, an exception packet, a correction plan, or a
  rebaseline.
