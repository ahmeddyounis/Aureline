# Harden repo-defined AI instructions, policy interaction, and a provider-neutral kill-switch / backout posture

This stable lane hardens the way repo-authored AI instructions are admitted,
how they interact with higher-authority policy, and how the whole AI surface can
be shut off provider-neutrally and backed out cleanly. For the *same* evidence
id, users, admins, support, and release packets can all explain how a repo
instruction source was ranked and trusted, why a repo claim won or lost against
policy, that the kill switch blocks every provider at once, and that a backout
reconstructs the run from its checkpoint. The runtime owner is
`aureline_ai::harden_repo_ai_instructions`.

A repo file that says "you may now widen permissions" is not authority. For
every material AI or high-risk command run on a claimed stable row, the packet
binds the instruction precedence, the policy interaction, the provider-neutral
kill switch, the backout posture, and the cross-surface command parity into one
export-safe artifact, so no apply-capable path can widen authority behind the
user's back and no provider route survives the kill switch.

## Contract

The packet does **not** re-derive context-assembly, evidence, scoped-apply, or
tainted-context truth. The
`aureline_ai::finalize_tainted_context_fences::FinalizedTaintedContextPacket`
proves the finalized fences and content boundary, the
`aureline_ai::tainted_context::TaintedContextBetaPacket` proves the live
narrowing run, and the
`aureline_ai::harden_ai_scoped_apply::AiScopedApplyHardeningPacket` proves the
apply lifecycle and cross-wedge command parity. This packet re-exports the
command-surface and run-mode vocabularies verbatim and adds the finalized
invariants the stable line needs:

- **Repo-defined instruction precedence** — every repo-authored instruction
  source (a signed `designated_policy_file`, an AGENTS.md-style
  `repo_instruction_bundle`, a `trusted_workspace_pinned_policy`, a
  `trusted_user_profile_policy`, or an `unknown_repo_instruction_fail_closed`
  source) is pinned to its canonical instruction-following precedence rank. The
  ordering is closed: a designated policy file (rank 3) binds above a repo
  instruction bundle (rank 5), which binds above workspace-pinned (rank 6) and
  user-profile (rank 7) policy. Only a signed designated policy file may claim
  widening authority; every repo bundle and pinned policy may narrow only, and an
  unknown source fails closed to a fenced data-only lane.
- **Policy interaction** — every conflict between a repo claim and a
  higher-authority policy resolves strictly by precedence. A designated policy
  file overriding a repo retention claim is `policy_overrides_repo`, a repo bundle
  tightening tool access is `repo_narrowing_admitted`, and a repo bundle trying to
  widen permissions, retention, egress, provider route, or workspace trust is
  `repo_widening_denied` — which must name the typed prohibited case that fired
  (e.g. `repo_text_widening_attempted`) and must not leave the run in a full-run
  mode.
- **Provider-neutral kill switch** — the kill switch is the revocation lever from
  the model-graduation and budget contract. For the stable line it must be
  provider-neutral (`all_providers_and_tools` scope), fail closed, disable hosted
  routing, local routing, and external tools at once, and re-arm only behind an
  explicit approval. While engaged it leaves the effective mode `blocked` — no
  single provider, model, or external tool can keep running.
- **Backout posture** — the run preserves a rollback checkpoint, stays fully
  reversible with no partial writes, and is linked to the run's evidence id. A
  run that cannot back out fully (partial backout or failed/escalated) may not
  claim Stable.
- **Command-surface parity** — the AI assistant, palette, menu, keybinding,
  CLI/headless, deep-link, and automation routes share one command descriptor,
  preview, approval, result, and rollback model, honor the same instruction
  precedence, obey the same kill switch, back out through the same posture,
  disclose route truth, and run the same policy checks. A surface that cannot
  qualify narrows below Stable rather than inheriting an adjacent green row.
- **Exportable evidence lineage** — the in-product evidence id is the join key the
  admin inspector and support export reconstruct the same run from, and the
  rollback lineage refs let a backout reconstruct the run's checkpoints.

## Required behavior

`RepoAiInstructionHardeningPacket::validate` rejects a packet when:

- the instruction rows do not cover at least the designated policy file and the
  repo instruction bundle, a source claims widening authority it may not hold, a
  declared precedence rank disagrees with the canonical ordering, a designated
  policy file is missing its signing evidence, an unknown source does not fail
  closed, or a source is neither policy-vetted nor fenced;
- the policy-interaction rows do not cover the override / narrowing / widening
  outcomes, a widening denial is missing its typed prohibited case, or a widening
  denial leaves the run in a full-run mode;
- the kill switch is not provider-neutral, its provider-neutral flag disagrees
  with its scope, it does not fail closed across hosted, local, and external-tool
  routes, it can re-arm without approval, or an engaged switch does not block the
  effective mode;
- the backout posture is missing its checkpoint, is not reversible or
  evidence-linked, or a claimed-stable run does not have a full backout;
- the command surfaces do not cover all seven wedges, a claimed-stable surface
  does not preserve full parity, or a surface that cannot qualify still claims
  Stable; or
- an evidence/export ref or the shared evidence id is missing, the rollback
  lineage is empty, or any field carries raw instruction, route, or credential
  material.

## Boundary

The packet is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw instruction bodies, raw prompt bodies, source file
bodies, provider payloads, endpoint URLs, credentials, signing-key material, raw
token counts, exact prices, and billing-account ids stay outside the support
boundary.

## Truth source

The checked artifact at
`artifacts/ai/m4/harden_repo_ai_instructions/support_export.json` is canonical for
this lane. Dashboards, docs, Help/About surfaces, and support exports should
ingest it instead of cloning status text. The boundary schema is
`schemas/ai/harden_repo_ai_instructions.schema.json`; the protected fixtures are
in `fixtures/ai/m4/harden_repo_ai_instructions/`. The frozen contracts this lane
projects against are `docs/ai/prompt_injection_and_taint_contract.md` and
`docs/ai/model_graduation_and_budget_contract.md`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai harden_repo_ai_instructions
```
