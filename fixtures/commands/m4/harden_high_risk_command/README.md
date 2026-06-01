# Harden high-risk command preview, approval lineage, and rollback-handle issuance

This fixture set exercises the hardened high-risk command record owned by
`aureline_commands::harden_high_risk_command`. For one canonical high-risk command
family and one evidence id, the packet binds the declared risk classes, the
high-risk preview contract, the approval lineage, the rollback-handle issuance
contract, and the cross-surface authority parity so every invocation surface
(menu/button, keybinding, palette, CLI/headless, AI tool, voice, recipe, deep
link, browser companion) enforces the same preview, approval, and rollback and no
surface skips the preview, self-grants or fails to audit the approval, omits the
rollback handle, widens authority, or claims the Stable lane while narrowed below
it.

`harden_high_risk_command_packet.json` covers the clean stable case:

- the canonical `contract_refs` set — the single descriptor schema, registry-entry
  schema, seeded-registry artifact, invocation-session schema, result-packet
  schema, public-contract projection schema, parity-expectation schema, and
  structured disabled-reason vocabulary every surface projects from;
- the declared high-risk effect classes (irreversible VCS, external network
  effect) that drive preview depth, approval, and rollback posture;
- the required, non-bypassable preview enumerating the effect summary,
  scope/targets, diff/impact, route-and-spend disclosure, and destructive
  confirmation, with every disclosure cue shown, an apply-guard ref, and the
  no-blind-apply guard;
- the approval lineage covering requested, reviewed, granted, and recorded-in-audit
  steps, each carrying an authority class, decision ref, and basis snapshot ref,
  bound to one policy epoch, with a distinct second-human approver, the grant
  recorded in audit, and the no-self-approval, no-authority-widening, and
  enforced-expiry guards;
- the rollback-handle issuance contract with an issued, replayable handle bound to
  checkpoint refs and the in-product evidence id and a checkpoint-revert posture;
- the nine cross-surface authority-parity rows, each enforcing preview, approval,
  and rollback, disclosing the route, running policy checks, and never widening
  authority at the Stable lane; and
- the evidence export binding the in-product evidence id to the admin inspector
  and support export refs plus the rollback lineage refs a revert reconstructs the
  command from.

Verify the checked packet with:

```sh
cargo test -p aureline-commands harden_high_risk_command --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-commands harden_high_risk_command::tests::emit_artifact -- --ignored
```
