# AI tainted context beta

This beta lane makes tainted AI context inspectable before it can widen
authority. The runtime owner is `aureline_ai::tainted_context`.

## Contract

`TaintedContextBetaPacket` joins four axes:

- source rows that label externally sourced, runtime/tool-output, suspicious,
  quarantined, or unknown context;
- policy decisions that narrow requested runs to `explain_only`, `local_only`,
  `preview_only`, or `blocked`;
- approval fences that name the renewal or denial path for each decision;
- surface rows proving composer, context inspector, review workspace,
  docs/help, support export, and CLI projections read the same packet refs.

The packet is export-safe. It carries refs, enum tokens, detector class tokens,
retrieval labels, audit refs, and review labels only; raw prompt text, source
bodies, terminal bodies, tool return bodies, provider payloads, endpoint URLs,
credentials, raw paths, raw token counts, exact prices, and billing-account ids
stay outside the support boundary.

## Required behavior

Suspicious or externally sourced context cannot widen authority by itself.
External docs can be narrowed to explain-only, external tool output can remove
remote/provider widening and leave local inspection, terminal output can allow
preview while blocking direct apply, and suspicious pasted text can block a run
when it attempts authority widening.

Users and support must see why context was considered tainted. A tainted source
row names its input source class, taint class, origin locus, reason classes,
fence ref, fence strategy, usage constraints, retrieval truth label when needed,
and shared suspicious-content detector tokens when suspicious text is present.

Approval fences must be explicit and auditable. Every narrowing decision points
to one approval fence, and every fence carries audit refs plus the approval
requirement: no approval remains after explain-only narrowing, fresh approval is
required after tainted input for renewed authority, preview requires approval
before apply, or the mutation is blocked with no approval path admitted.

## Surfaces

The composer, context inspector, review workspace, docs/help, support export,
and CLI projections must preserve the same packet, source, decision, and
approval-fence refs. A projection that hides those refs, drops raw-body
exclusion, or remints its own truth is drift and blocks the beta claim.

## Export

The checked support export is:

`/artifacts/ai/m3/tainted_context_beta_support_export.json`

The JSON schema is:

`/schemas/ai/tainted_context.schema.json`

The reviewer fixture corpus is:

`/fixtures/ai/m3/tainted_context/`

## Verification

```sh
cargo test -p aureline-ai tainted_context --no-fail-fast
cargo run -q -p aureline-ai --example dump_tainted_context_beta
```
