# Publish-target and release-center case fixtures

These fixtures exercise the release-center object model and
publish-target contract in
[`docs/release/release_center_object_model_contract.md`](../../../docs/release/release_center_object_model_contract.md).
They are structural examples only; they do not claim release
automation, a signing service, a channel backend, or a registry
integration has been implemented.

Cases:

- `staged_preview.yaml` - internal ring to public preview promotion
  with current dry run, auth-source disclosure, audience semantics, and
  exact-build backreferences.
- `public_stable_publish.yaml` - stable publication with immutable
  artifacts, mutable channel pointer, current evidence, rollback
  target, and About/provenance parity refs.
- `mirror_only_emergency_push.yaml` - mirror-only emergency update that
  preserves origin exact-build identity, mirror freshness, manual-import
  receipt, and support linkage without advancing the public channel.
- `support_bundle_backreference.yaml` - rollback reconstruction through
  support bundle refs, update manifest refs, last-known-good refs, and
  release-center action ids.
- `break_glass_reconciliation.yaml` - emergency containment action that
  carries a break-glass event ref, mandatory evidence and waiver packet
  refs, and a reconciled superseding signed action.
