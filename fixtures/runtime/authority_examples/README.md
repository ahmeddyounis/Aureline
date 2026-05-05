# Authority decision-example fixtures

These fixtures are short, reviewable scenarios that anchor the
authority-class matrix and canonical-writer rules defined in
[`/docs/runtime/authority_class_matrix.md`](../../../docs/runtime/authority_class_matrix.md)
to concrete subscription-envelope frames and user-visible outcomes.
They are not a test suite; they are examples future conformance tests
and UX reviews can cite.

**Scope rules**

- Every fixture names the authority classes it exercises and the
  canonical writer it expects to own the mutation path.
- Fixtures include one or more subscription-envelope frames expressed
  as YAML. The `frames[*].envelope` objects are intended to validate
  against
  [`/schemas/runtime/subscription_envelope.schema.json`](../../../schemas/runtime/subscription_envelope.schema.json).
- Fixtures describe observable outcomes (labels, disabled states,
  required refresh/revalidate steps) rather than internal
  implementation details.

**Index**

| Fixture | Primary authority classes | What it freezes |
|---|---|---|
| [`provider_overlay_patch_suggestion_is_evidence.yaml`](./provider_overlay_patch_suggestion_is_evidence.yaml) | `provider_overlay` + `buffer_editor` | Provider overlays can suggest but cannot mutate buffers out-of-band |
| [`stale_policy_blocks_managed_mutation.yaml`](./stale_policy_blocks_managed_mutation.yaml) | `policy_entitlement` + `execution` | Stale policy is visible before managed/egress mutations remain enabled |
| [`derived_diagnostics_apply_requires_revalidate.yaml`](./derived_diagnostics_apply_requires_revalidate.yaml) | `derived_knowledge` + `buffer_editor` | Derived “apply” actions revalidate against live buffer truth |
| [`workspace_identity_aliases_keep_overlay_secondary.yaml`](./workspace_identity_aliases_keep_overlay_secondary.yaml) | `workspace_vfs` + `provider_overlay` | Overlay anchors bind to canonical file identity, not a second mutable path |
