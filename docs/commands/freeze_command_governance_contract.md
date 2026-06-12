# Freeze the command-descriptor, invocation-session, result-packet, and lifecycle-dependency governance contract

This contract freezes the command-governance matrix that stable-facing command
surfaces use to decide whether they may keep Stable wording. It does not create
another command registry. The runtime owner is
`aureline_commands::freeze_command_governance_contract`, and the packet binds the
existing command descriptor, invocation/result, lifecycle, and claim-publication
contracts into one export-safe review object.

The packet covers five things:

- the canonical descriptor fields that new command families must publish:
  command identity, alias/deprecation state, capability class, preview class,
  automation labels, origin metadata, result-packet references, and lifecycle
  metadata;
- the invocation-session field freeze for issuing surface, argument provenance,
  context snapshot, enablement decision, execution intent, outcome, artifact
  joins, and timing/cost bands;
- the result-packet field freeze for canonical identity, outcome codes, artifact
  refs, rollback/checkpoint posture, notification/activity joins, evidence refs,
  and export posture;
- the lifecycle-dependency vocabulary used when a stable-facing command family
  still depends on Labs, Preview, Beta, policy-gated, or underqualified
  capability state; and
- the downgrade rules and per-family surface matrix that ensure Help/About,
  release, docs, and support surfaces never widen claims past the current packet.

## Contract rules

- `contract_refs` pins the canonical descriptor contract, invocation/result
  contract, lifecycle ADR, disabled-reason vocabulary, descriptor schema,
  invocation-session schema, result-packet schema, claim-publication manifest,
  and capability lifecycle registry.
- Every descriptor, invocation-session, result-packet, and lifecycle-dependency
  vocabulary row is closed and required. A missing row is a validation failure.
- Every covered feature family carries one governance row and every row covers
  desktop, CLI, AI, recipe, extension, and browser-companion surfaces.
- A family or surface that loses current descriptor proof, invocation proof,
  result proof, disabled-reason parity, preview truth, authority parity, route
  truth, or fresh evidence must auto-narrow below Stable.
- A family or surface with live lifecycle dependency markers may not keep Stable
  wording. The dependency must be disclosed and the claim must narrow.
- `published_stable_copy_allowed` must agree with the effective claim. Narrowed
  rows cannot keep stable-facing copy.

## Checked artifact

- `artifacts/commands/freeze_command_governance_contract/support_export.json`
  is the canonical export for docs, Help/About, release, and support consumers.
- `artifacts/commands/freeze_command_governance_contract/summary.md` is the
  reviewer-facing summary.
- `fixtures/commands/freeze_command_governance_contract/` contains the clean
  packet fixture.

Verify with:

```sh
cargo test -p aureline-commands freeze_command_governance_contract --no-fail-fast
```
