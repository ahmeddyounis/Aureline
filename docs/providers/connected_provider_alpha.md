# Connected-provider registry alpha

This page is the reviewer-facing contract for the connected-provider
registry alpha. The canonical implementation is the `aureline-provider`
crate; the schema files and fixtures exist so non-owning surfaces can
read the same truth instead of restating provider metadata locally.

## Owned outputs

- [`/schemas/providers/connected_provider_registry.schema.json`](../../schemas/providers/connected_provider_registry.schema.json)
  defines the descriptor packet for code-host, issue, and CI/check
  providers; surface action states; pipeline run/log/artifact/annotation
  overlays; and auditable run controls.
- [`/schemas/providers/publish_later_queue_alpha.schema.json`](../../schemas/providers/publish_later_queue_alpha.schema.json)
  defines the alpha queue item that composes with the existing
  publish-later and deferred-publish contracts.
- [`/crates/aureline-provider`](../../crates/aureline-provider)
  is the first consumer. It parses the packet, validates coverage and
  guardrails, and emits an export-safe support projection.
- [`/fixtures/providers/connected_provider_alpha/registry_packet.json`](../../fixtures/providers/connected_provider_alpha/registry_packet.json)
  is the protected fixture covering the acceptance states.

## Composition

The alpha registry does not replace the existing provider contracts. It
references these as source-of-truth inputs:

- connected-account registry:
  [`connected_account_registry.schema.json`](../../schemas/providers/connected_account_registry.schema.json)
- publish-later records:
  [`publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json)
- deferred-publish records:
  [`deferred_publish_queue_item.schema.json`](../../schemas/providers/deferred_publish_queue_item.schema.json)
- CI run, log, artifact, annotation, and run-control records under
  [`/schemas/ci/`](../../schemas/ci/)

If this page disagrees with a referenced schema or the Rust consumer,
the schema and Rust consumer win and this page is updated in the same
change.

## Alpha invariants

- Code-host, issue, and CI/check providers are represented through one
  descriptor family with source, actor-scope, freshness, supported
  object, supported state, and fallback labels.
- Provider descriptors carry `region_mode`, `residency_mode`, and
  `key_mode` from the identity-mode vocabulary. Missing values project
  as explicit `unknown` warnings; they are never treated as allowed
  provider-default truth by omission.
- Claimed surfaces keep `local_draft`, `publish_now`,
  `open_in_provider`, `publish_later_queue`, and `inspect_only`
  distinct. A publish-later action must cite a queue row; an
  open-in-provider action must cite a browser-handoff packet; a
  publish-now action must cite an approval ticket.
- Queue rows carry ordered dependencies, stale-target risk,
  reauth/rescope posture, linked canonical publish-later and
  deferred-publish refs, audit refs, and an export-safe summary.
- CI overlays include run, log, artifact, and annotation descriptors.
  Artifact overlays disclose an artifact trust class.
- Rerun, cancel, and retry controls disclose upstream mutation scope,
  target ref, auth source, actor scope, freshness, stale-target risk,
  fallback mode, and audit refs before invocation.
- Provider overlays remain secondary to current local task/debug truth.
  At least one CI overlay must explicitly show local/provider truth
  disagreement rather than replacing the local result with imported
  provider state.

## How to inspect

```bash
cargo test -p aureline-provider
cargo run -p aureline-provider --bin aureline_provider_alpha -- --validate-only
cargo run -p aureline-provider --bin aureline_provider_alpha
```

The binary prints the validation report in `--validate-only` mode and
prints the support-export projection otherwise. Both outputs exclude
raw provider URLs, raw provider payloads, and credential material.
