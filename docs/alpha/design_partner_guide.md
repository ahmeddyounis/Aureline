# External Alpha Design Partner Guide

This guide is the external-alpha entry point for design partners. It points to
the canonical scope, task, feedback, and known-limit artifacts used by the
program so partner reports stay reproducible, privacy-reviewed, and routed to
the same proof rows as internal alpha review.

## Canonical Artifacts

- Alpha scope matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Alpha go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Intake packet: `artifacts/milestones/m2/design_partner_intake_packet.md`
- Task pack: `artifacts/milestones/m2/design_partner_task_pack.md`
- Feedback taxonomy: `artifacts/feedback/design_partner_feedback_taxonomy.yaml`
- Known limits: `artifacts/feedback/external_alpha_known_limits.md`
- Upstream intake checklist: `artifacts/program/design_partner_intake_checklist.yaml`
- Validator: `ci/check_design_partner_alpha.py`

## What This Alpha Covers

External alpha is limited to the two launch wedges named in the alpha scope
matrix:

| Wedge | Supported task surface | Deployment posture |
|---|---|---|
| TypeScript / JavaScript web app or service | first useful navigation, rename preview, targeted test/debug loop, Git/review basics | local desktop or helper-backed local services |
| Python service or data app | interpreter selection, pytest loop, debug/refactor basics, notebook handoff disclosure, Git/review basics | local desktop or helper-backed Python environment or devcontainer |

Managed-cloud daily-driver parity, browser or mobile companion parity, full
notebook parity, and additional language wedges are not alpha claims. Report
those as known-limit or scope requests rather than task failures.

## Before Starting

1. Complete the intake packet before sharing repository details, support
   bundles, logs, traces, screenshots, or transcripts.
2. Use a redacted partner reference, not a company, product, repository, user,
   host, account, or customer name.
3. Choose one task script from the task pack and run it against the required
   reference workspace or a privacy-cleared partner workspace.
4. Record the privacy posture before attaching evidence. Raw code, prompts,
   repo names, file paths, clipboard contents, customer data, and credentials
   require redaction review before sharing.
5. Route every report through the feedback taxonomy category and severity
   values. Do not invent one-off labels.

## Reporting Paths

| Report kind | Taxonomy category | Default route |
|---|---|---|
| Task completed or partially completed | `task_completion` | private partner channel |
| Task blocked by product behavior | `alpha_task_blocker` | private partner channel |
| Unsupported wedge, platform, or workflow request | `scope_or_known_limit` | governance packet queue |
| Raw artifact, support bundle, path, or transcript needs review | `privacy_redaction` | private partner channel |
| Support export or diagnostic packet is insufficient | `support_export_or_diagnostics` | private support channel |
| Docs, guide, or copy disagrees with observed alpha behavior | `docs_or_copy_truth` | public issue tracker when content is safe; otherwise private partner channel |
| Benchmark fixture or reference workspace does not match the task | `benchmark_fixture_gap` | benchmark council queue |
| Security, trust, or boundary concern | `trust_security_boundary` | security channel when sensitive; otherwise private partner channel |

## Blocker Severity

Use the smallest severity that preserves the actual consequence:

| Severity | Meaning |
|---|---|
| `external_alpha_blocker` | Partner cannot continue the claimed task safely or the report implies a privacy, trust, or claim-scope breach. |
| `task_blocking` | One task script cannot pass, but the partner can continue other alpha work. |
| `scoped_workaround` | A documented workaround exists and preserves the alpha claim honestly. |
| `clarity_gap` | The product, docs, or task pack is ambiguous but the task is still executable. |
| `observation` | Useful feedback that does not block completion. |

## Safe Sharing

Share evidence only after the intake packet records a `privacy_review_state`.
When in doubt, share references and summaries first:

- task script id, workflow id, wedge id, build or channel ref;
- redacted screenshot or clipped log with paths, names, and secrets removed;
- support-export id after redaction review;
- pass/fail summary and exact reproduction steps.

Do not attach raw repository archives, raw support bundles, raw terminal
transcripts, raw issue text, credentials, customer names, tokens, hostnames, or
unredacted paths until the redaction review is cleared.

## Rollback And Narrowing

External alpha feedback can narrow the public claim but cannot widen it. If a
task shows that a claimed workflow is unsafe or unsupported, the report routes
to the known-limits packet and the scoreboard row stays blocked or narrows until
fresh evidence lands. If a partner needs a workflow outside the matrix, the
request opens scope review rather than becoming an implied alpha promise.

