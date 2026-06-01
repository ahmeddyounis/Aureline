# Stabilize command descriptor, invocation/result packets, preview, and parity

This fixture set exercises the stabilized command-contract record owned by
`aureline_commands::stabilize_command_contract`. For one canonical command family
and one evidence id, the packet binds the finalized stable descriptor fields, the
invocation-session/result-packet contract, the palette diagnostics contract, the
structured disabled-reason cases, and the cross-surface authority parity so every
entry surface (menu/button, keybinding, palette, CLI/headless, AI tool, voice,
recipe, deep link, browser companion) explains the same command truth and no
surface widens authority or suppresses a preview.

`stabilize_command_contract_packet.json` covers the clean stable case:

- the canonical `contract_refs` set — the single descriptor schema,
  registry-entry schema, seeded-registry artifact, invocation-session schema,
  result-packet schema, public-contract projection schema, parity-expectation
  schema, and structured disabled-reason vocabulary every surface projects from;
- the thirteen stable descriptor fields (command id, invocation schema,
  capability class, enablement rules, discoverability record, automation labels,
  result contract, lifecycle metadata, origin metadata, alias/deprecation
  metadata, docs/help anchor, accessibility labels, shortcut narration), each
  exported and marked as a stable interface;
- the stabilized result contract enumerating the nine stable result codes and
  preserving canonical command identity, alias resolution, the issuing surface,
  artifact refs, a notification/activity join, a reversible rollback handle for
  durable commands, checkpoint support, evidence refs, and the strict no-bypass
  guards;
- the palette diagnostics contract showing the source badge, keybinding,
  dominant side-effect cue, disabled-with-reason state, and preview/approval
  posture, and exposing the Copy command ID, Copy CLI equivalent, Add to recipe,
  and Why not automatable? actions;
- the seven structured disabled-reason cases (disabled-by-policy, wrong-focus,
  missing-runtime, degraded-provider, preview-required, approval-required,
  UI-only), each mapped to a canonical machine reason code and resolving
  identically across support, CLI, palette, and automation;
- the nine cross-surface parity rows, all sharing the canonical command
  descriptor, preview, approval, rollback, and audit models, resolving aliases to
  the canonical command id, disclosing route truth, running policy checks, and
  keeping automation labels honest at the Stable lane; and
- the evidence export binding the in-product evidence id to the admin inspector
  and support export refs plus the rollback lineage refs a revert reconstructs
  the command from.

Verify the checked packet with:

```sh
cargo test -p aureline-commands stabilize_command_contract --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-commands stabilize_command_contract::tests::emit_artifact -- --ignored
```
