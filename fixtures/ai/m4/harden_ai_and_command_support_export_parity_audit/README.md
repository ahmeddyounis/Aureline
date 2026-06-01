# Harden AI and Command Support-Export Parity Audit

This fixture set exercises the stable AI and command support-export parity audit
record owned by
`aureline_commands::harden_ai_and_command_support_export_parity_audit`. It binds
support-export parity, audit lineage, shiproom packet inclusion, and exportable
evidence/rollback lineage into one export-safe packet.

`harden_ai_and_command_support_export_parity_audit_packet.json` covers:

- all seven support-export surfaces (UI palette/menu, keybinding, CLI/headless,
  AI tool, automation recipe, deep-link/browser companion, and voice) carrying
  the same preview, approval, provider/route, spend/egress, tainted-context,
  rollback, and audit-lineage metadata without authority widening;
- one shared export-parity contract proving AI and command surfaces share the
  descriptor, preview, approval, result, and rollback models and never hide a
  provider route;
- one audit-lineage contract binding actor identity, invocation surface, policy
  epoch, provider/route identity, decision traceability, recorded outcome, and
  non-repudiation; and
- one shiproom inclusion contract proving the packet is indexed, checklisted,
  bundled for support export, and validated against checked-in artifact refs.

Verify the checked packet with:

```sh
cargo test -p aureline-commands harden_ai_and_command_support_export_parity_audit --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-commands harden_ai_and_command_support_export_parity_audit::tests::emit_artifact -- --ignored
```
