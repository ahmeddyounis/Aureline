# Harden Repo-Defined AI Instructions

This fixture set exercises the hardened repo-instruction record owned by
`aureline_ai::harden_repo_ai_instructions`. For one material AI / high-risk
command run and one evidence id, the packet binds the repo-defined instruction
precedence, the policy interaction, the provider-neutral kill switch, the backout
posture, and the cross-surface command parity so no apply-capable path can widen
authority behind the user's back and no provider route survives the kill switch.

`harden_repo_ai_instructions_packet.json` covers the clean stable case:

- four repo-defined instruction sources, each pinned to its canonical precedence
  rank and forbidding raw bodies:
  - a signed `designated_policy_file` (rank 3) carrying full admin authority with
    a signing-evidence ref;
  - an AGENTS.md-style `repo_instruction_bundle` (rank 5) that may narrow only;
  - a `trusted_workspace_pinned_policy` (rank 6) that may narrow only; and
  - an `unknown_repo_instruction_fail_closed` source fenced to a data-only lane;
- three policy interactions resolving repo claims against higher authority:
  - `policy_overrides_repo` — the designated policy file overrode a repo retention
    claim;
  - `repo_narrowing_admitted` — a repo bundle tightened tool access to read-only;
  - `repo_widening_denied` — a repo egress-widening attempt was denied with the
    typed `repo_text_widening_attempted` prohibited case and left blocked;
- a provider-neutral kill switch scoped to `all_providers_and_tools` that fails
  closed across hosted routing, local routing, and external tools, re-arms only
  behind an approval, and leaves the effective mode `blocked` when engaged;
- a backout posture that preserves a rollback checkpoint, is fully reversible and
  evidence-linked, and can be triggered by the kill switch engaging;
- the seven command surfaces (palette, menu, keybinding, CLI/headless, deep link,
  automation recipe, AI assistant) all sharing the canonical command descriptor,
  preview, approval, result, and rollback model, honoring the instruction
  precedence, obeying the kill switch, backing out through the same posture,
  disclosing route truth, and running policy checks; and
- the evidence export binding the in-product evidence id to the admin inspector
  and support export refs plus the rollback lineage refs a backout reconstructs
  the run from.

Verify the checked packet with:

```sh
cargo test -p aureline-ai harden_repo_ai_instructions --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-ai harden_repo_ai_instructions::tests::emit_artifact -- --ignored
```
