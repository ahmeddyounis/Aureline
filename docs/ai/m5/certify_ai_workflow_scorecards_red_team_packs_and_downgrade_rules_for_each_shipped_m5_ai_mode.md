# M5 AI Mode Certification — Scorecards, Red-Team Packs, and Downgrade Rules

This document certifies the canonical M5 depth qualification for every shipped AI mode. It binds, per mode, a workflow scorecard, a red-team pack, and a closed set of downgrade rules so that no shipped M5 AI mode may keep a public claim its evidence cannot support.

The shipped modes are:

1. **Inline Edit** — Composer inline quick-edit and scoped-apply.
2. **Patch Review** — AI review-assist findings, publish-to-review sheets, and resolution memory.
3. **Explain** — Explain flow with evidence links to logs, traces, runbooks, and profiles.
4. **Debug** — Debug flow with evidence links to logs, traces, and runbooks.
5. **Test** — Test-generation proposals with assumption review and isolated sandbox validation.
6. **Refactor** — Refactor planner with impact sets, candidate previews, and multi-file safety classes.
7. **Branch or Worktree Agents** — Background branch-agent lifecycle with isolated worktrees.

## Packet

The machine-readable packet is owned by `crates/aureline-ai/src/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/`.

- Record kind: `certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode`
- Schema version: `1`
- Schema: `schemas/ai/certify-ai-workflow-scorecards-red-team-packs-and-downgrade-rules-for-each-shipped-m5-ai-mode.schema.json`
- Checked-in export: `artifacts/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/support_export.json`

The packet reuses the qualification-class and downgrade-trigger vocabularies frozen by the M5 AI workflow matrix lane and references that matrix's schema and checked-in export as source contracts, so the certification can never drift greener than the matrix it certifies.

## Workflow Scorecards

Every mode is scored against the same seven trust dimensions, each on a `0..=100` scale with a per-dimension threshold and a `pass`/`warn`/`fail` status:

- `evidence_integrity` — Evidence packets exist, are current, and cite their sources.
- `context_visibility` — Used and omitted context stay inspectable.
- `scope_honesty` — The mode never claims wider scope than it qualifies for.
- `approval_gating` — Mutating side effects require preview and human approval.
- `rollback_safety` — Applied changes carry a rollback or checkpoint handle.
- `provider_trust_disclosure` — Provider, host, and trust posture are disclosed.
- `omitted_context_disclosure` — Omitted or truncated context is visibly disclosed.

A status is consistent only when `pass`/`warn` scores meet the threshold and `fail` scores fall short. A Stable-claimed mode must pass every dimension; a Beta-claimed mode may carry borderline `warn` dimensions but no `fail`.

## Red-Team Packs

Every mode covers all eight required attack vectors with a disposition (`blocked`, `mitigated`, or `not_applicable`) and a ref to the control that enforces it:

- `prompt_injection`
- `tainted_context_exfiltration`
- `scope_escape`
- `self_approved_mutation`
- `worktree_isolation_bypass`
- `unreviewed_apply`
- `credential_leak_in_export`
- `stale_evidence_promotion`

Read-only modes may mark the four apply-specific vectors (`scope_escape`, `self_approved_mutation`, `worktree_isolation_bypass`, `unreviewed_apply`) `not_applicable`. The four always-applicable vectors (`prompt_injection`, `tainted_context_exfiltration`, `credential_leak_in_export`, `stale_evidence_promotion`) must be blocked or mitigated on any mode carrying a public claim.

## Downgrade Rules

Each mode carries a closed set of downgrade rules. Every rule binds a frozen downgrade trigger to a narrowed qualification, an `auto_enforced` flag, and a review-safe rationale. Every mode must include a `proof_stale` rule, and every rule must narrow to a strictly lower qualification than the claim — downgrade narrows the claim, it never hides the mode and never widens it. Consumers and release tooling project `AiModeCertification::narrowed_qualification` rather than re-deriving narrowing locally.

## Proof Freshness

- Proof-freshness SLO: 168 hours.
- Last refresh tracked per packet.
- Auto-narrow on stale proof is enabled.

## Enforcement

`AiModeCertificationPacket::validate` returns the closed set of `AiModeCertificationViolation` tokens. The checked-in export is read and revalidated by `current_ai_mode_certification_export`, exercised in the crate test suite that runs in the PR gate. A stale, underqualified, evidence-missing, or non-narrowing certification fails validation and therefore fails CI, narrowing the claim before publication.
