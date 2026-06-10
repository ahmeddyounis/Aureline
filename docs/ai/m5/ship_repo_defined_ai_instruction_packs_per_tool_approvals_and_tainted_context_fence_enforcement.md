# Repo-Defined AI Instruction Packs, Per-Tool Approvals, and Tainted-Context Fence Enforcement

## Purpose

This document defines the M5 canonical packet that locks three interlocking AI
governance concerns into one export-safe artifact. Repo-defined **instruction
packs** may narrow the AI's policy posture but never widen it; **per-tool
approvals** gate every tool side effect behind a disclosed approval posture with
human review on first use; and **tainted-context fences** keep untrusted context
from widening policy or auto-approving a tool. The concerns are enforced together:
tainted context can never grant a tool approval, and a repo instruction can never
widen policy (`fence_enforcement_interlocked` must be true). The packet binds three
blocks:

- **Instruction packs** — the repo-defined instruction packs in effect. Each pack
  carries a source class (repo committed, repo local-uncommitted, workspace
  shared, or organization pinned), a trust posture (trusted repo, untrusted
  imported, or restricted), the scope effect it has on policy (narrows policy,
  neutral, or attempts widen), and its precedence rank. Every pack is repo-sourced,
  any pack that attempts to widen policy is blocked and never applied, and
  precedence is disclosed rather than hidden.
- **Per-tool approvals** — the per-tool approval decisions. Each decision carries
  the tool's capability class (read-only, workspace write, network egress, process
  exec, or external service), side-effect class (none, local mutation, network
  call, or irreversible), approval posture (denied, first-use review, per-call,
  session-scoped, or pre-approved read-only), and approval actor (human operator,
  repo policy, or unattributed). No tool side effect runs without an approval, a
  denied tool is never approved, and any side-effecting tool that is approved
  carries a human approval actor and requires human review on first use.
- **Tainted-context fences** — the tainted-context fences in force. Each fence
  carries the tainted source class (imported external, tool output, web fetch,
  untrusted repo doc, or pasted content), the enforcement applied (blocked,
  downgraded to advisory, quarantined, or allowed after review), and the usage
  constraint it imposes (no policy widening, no tool auto-approval, no apply
  authority, or display only). Every tainted source is fenced, no fence is
  bypassed, and each fence blocks both policy widening and tool auto-approval.

## Scope

The packet covers:

1. **Repo instruction packs** — source class, trust posture, scope effect, and
   precedence rank for every repo-defined pack, with widening attempts blocked.
2. **Per-tool approvals** — capability class, side-effect class, approval posture,
   and approval actor for every gated tool.
3. **Tainted-context fences** — source class, enforcement, and usage constraint
   for every fenced tainted input.
4. **Consumer surface parity** — cross-surface truth for the desktop composer,
   desktop settings, CLI/headless, browser companion, support export, and
   diagnostics.

## Repo-instruction posture

Repo instruction packs may narrow but never widen policy. The block sets
`repo_only`, `no_repo_policy_widening`, and `precedence_disclosed`, and every pack
row carries `disclosed = true`. Any pack whose `scope_effect` is `attempts_widen`
must set `widen_blocked = true` and `applied = false`; a widening pack that is
applied or unblocked narrows the lane via the `repo_instruction_widened_policy`
downgrade trigger.

## Per-tool approval posture

No tool side effect runs without an approval (`no_side_effect_without_approval`
must be true) and side-effecting tools require human review on first use
(`first_use_review_required` must be true). A denied tool
(`approval_posture = denied`) is never approved. Any tool whose `side_effect_class`
is not `none` that is approved must carry a `human_operator` approval actor and set
`requires_human_first_use`; an approved side-effecting tool without a human actor
or first-use review narrows the lane via the `tool_side_effect_unapproved` trigger.

## Tainted-context fence posture

Every tainted source is fenced (`all_tainted_fenced` must be true) and no fence is
bypassed (`no_fence_bypass` must be true). Each fence row blocks policy widening
(`widening_blocked`) and tool auto-approval (`auto_approval_blocked`). A tainted
input allowed to widen policy or auto-approve a tool narrows the lane via the
`tainted_context_bypassed_fence` and `tool_approval_granted_by_tainted_context`
downgrade triggers.

## Interlock posture

The three concerns are enforced together: `fence_enforcement_interlocked` must be
true. Tainted context can never grant a tool approval (the fence block enforces
`auto_approval_blocked`), and a repo instruction can never widen policy (the
instruction block enforces `no_repo_policy_widening`). The interlock not being
enforced is itself a validation failure.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Repo instruction widened policy
- Tool side effect unapproved
- Tainted context bypassed fence
- Tool approval granted by tainted context

A repo instruction widening policy, a tool side effect running unapproved, a
tainted input bypassing its fence, or tainted context granting a tool approval
narrows the lane claim rather than hiding the deficiency.

## Source contracts

This packet projects against:

- `docs/ai/context_assembly_contract.md` — frozen context-assembly contract for evidence-citation and omitted-context truth
- `docs/ai/m4/harden_repo_ai_instructions.md` — prior canonical repo-instruction hardening lane for instruction precedence, trust, and policy-interaction outcomes
- `docs/ai/prompt_injection_and_taint_contract.md` — prompt-injection and taint contract for tainted-context handling
- `docs/ai/m4/finalize_tainted_context_fences.md` — prior canonical tainted-context fence lane for fence strategy and enforcement
- `docs/ai/provider-model-tool-registry.md` — provider/model/external-tool registry contract for tool capability, side-effect, and approval posture
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `schemas/ai/ship-repo-defined-ai-instruction-packs-per-tool-approvals-and-tainted-context-fence-enforcement.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw instruction-pack bodies, raw prompt bodies, raw tool
arguments, raw tool output, raw tainted content, raw symbol names, raw file paths,
provider payloads, endpoint URLs, credentials, raw token counts, exact prices, and
billing-account ids stay outside the support boundary.
