# Harden AI and command support-export parity, audit lineage, and shiproom packet inclusion

This stable lane hardens the write-capable AI and command surfaces into one
export-safe artifact proving that support and release evidence tell the same
story no matter how the action was invoked. The runtime owner is
`aureline_commands::harden_ai_and_command_support_export_parity_audit`.

A write-capable path is not stable unless UI, CLI, AI, automation, voice, and
support exports all preserve the same preview, approval, provider/route,
spend/egress, tainted-context fence, rollback, and audit-lineage metadata — and
unless shiproom can point to one checked packet in the stable proof index and
release checklist. The parity-audit packet binds those invariants into one
attributable artifact.

## Contract

The packet does **not** re-derive descriptor, registry, invocation, result,
preview, approval, rollback, or evidence-export truth. The frozen command
descriptor contract (`docs/commands/command_descriptor_contract.md`) and the
invocation-result and parity contract
(`docs/commands/invocation_result_and_parity_contract.md`) remain canonical for
those slices. The packet references those lineages by stable ref and adds:

- **Support-export parity** — every write-capable AI and command surface carries
  the same preview-record ref, approval-lineage ref, provider/route identity,
  spend/egress disclosure, tainted-context fence handling, rollback/revert
  handle ref, and audit-lineage trace; AI and command share one descriptor,
  preview, approval, result, and rollback model, and no surface hides a
  provider route.
- **Audit lineage** — actor identity, invocation surface, policy epoch,
  provider/route identity, decision ref, and recorded outcome are bound into a
  non-repudiable lineage that support and audit can replay without raw
  materials.
- **Shiproom inclusion** — the packet is indexed in the stable proof index,
  referenced by the release checklist, included in the support export bundle,
  and validated against checked-in artifact refs so release, support, and field
  exports project one truth.
- **Exportable evidence lineage** — the in-product evidence id and rollback
  lineage refs stay bound to the JSON/Markdown export refs without exposing raw
  prompts, raw command arguments, endpoint URLs, credentials, or other boundary
  material.

## Required behavior

`HardenAiAndCommandSupportExportParityAuditPacket::validate` rejects a packet
when:

- a claimed-stable row does not require support-export parity, audit lineage, or
  shiproom inclusion;
- the parity, audit-lineage, or shiproom inclusion requirement coverage is
  incomplete, or any required cue for those contracts is missing;
- a support-export surface is not covered, a stable reachable row drops preview,
  approval, provider/route, spend/egress, tainted-context, rollback, audit, or
  no-authority-widening truth, or a surface claims Stable without qualifying for
  it;
- evidence-export refs or rollback-lineage refs are missing; or
- any field carries raw boundary material.

## Boundary

The packet is export-safe. It carries refs, stable class tokens, booleans,
counts, and review labels only. Raw prompts, raw command arguments, raw diff
bodies, endpoint URLs, credentials, provider payloads, and signing-key material
stay outside the support boundary.

## Truth source

The checked artifact at
`artifacts/ai/m4/harden_ai_and_command_support_export_parity_audit/support_export.json`
is canonical for this lane. Dashboards, shiproom packets, docs, Help/About
surfaces, and support exports should ingest it instead of cloning status text.
The boundary schema is
`schemas/ai/harden_ai_and_command_support_export_parity_audit.schema.json`; the
protected fixture is
`fixtures/ai/m4/harden_ai_and_command_support_export_parity_audit/`. The frozen
contracts this lane projects against are
`docs/commands/command_descriptor_contract.md` and
`docs/commands/invocation_result_and_parity_contract.md`.

Verify the checked packet with:

```sh
cargo test -p aureline-commands harden_ai_and_command_support_export_parity_audit
```
