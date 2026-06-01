# Finalize Tainted Context Fences

This fixture set exercises the finalized tainted-context record owned by
`aureline_ai::finalize_tainted_context_fences`. For one material AI / high-risk
command run and one evidence id, the packet binds the finalized tainted fences,
the content-boundary handling, the imported-data downgrade rules, and the
cross-surface command parity so no apply-capable path can widen authority behind
the user's back.

`finalize_tainted_context_packet.json` covers the clean stable case:

- two tainted fences — a fenced external tool response (`tainted_evidence`) and a
  fenced web search excerpt (`unknown_must_treat_as_tainted`) — each stripping
  instruction authority, blocking hidden provider/tool/workspace writes, naming a
  strategy and usage constraints, staying auditable, and forbidding raw bodies;
- three content boundaries: a `trusted_instruction_surface` (the only lane that
  carries instruction authority), an `untrusted_data_content` lane held in a
  delimited data channel and joined to the tool-response fence, and an
  `unknown_boundary_fail_closed` lane that fails closed to a summary ref;
- the five imported-data downgrade rows, each generated from a real artifact and
  preserving a rollback checkpoint:
  - `exact` keybinding profile → `no_downgrade_exact_mapping`, `full_run`;
  - `translated` color theme → `review_before_apply`, `preview_only`;
  - `partial` settings file → `narrowed_to_preview_only` with diagnostics;
  - `shimmed` extension manifest → `narrowed_to_local_only` with diagnostics;
  - `unsupported` automation macro → `blocked_unsupported`, `blocked`, with
    diagnostics;
- the seven command surfaces (palette, menu, keybinding, CLI/headless, deep link,
  automation recipe, AI assistant) all sharing the canonical command descriptor,
  preview, approval, result, and rollback model, enforcing the content boundary,
  honoring the import downgrade, disclosing route truth, and running policy
  checks; and
- the evidence export binding the in-product evidence id to the admin inspector
  and support export refs plus the rollback lineage refs a revert reconstructs the
  run from.

Verify the checked packet with:

```sh
cargo test -p aureline-ai finalize_tainted_context_fences --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-ai finalize_tainted_context_fences::tests::emit_artifact -- --ignored
```
