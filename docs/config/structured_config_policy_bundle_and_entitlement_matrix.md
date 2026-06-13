# Structured config, policy bundle, and offline entitlement matrix

This document is the narrative companion for Aureline's canonical matrix that
freezes:

- every config-bearing artifact family introduced in the current depth lane,
- the signed-bundle taxonomy for policy, entitlement, emergency-disable, and
  trust-root or signer-update envelopes, and
- the deployment-profile qualification rows that downstream surfaces must honor.

Machine-readable companions:

- [`/artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json`](../../artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json)
  — the canonical checked-in packet consumed by shell, CLI/headless inspect,
  Help/About, support export, and release evidence.
- [`/artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.md`](../../artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.md)
  — generated summary suitable for human review alongside the JSON packet.
- [`/schemas/config/structured_config_policy_bundle_and_entitlement_matrix.schema.json`](../../schemas/config/structured_config_policy_bundle_and_entitlement_matrix.schema.json)
  — boundary schema for the packet.
- [`/fixtures/config/structured_config_policy_bundle_and_entitlement_matrix/`](../../fixtures/config/structured_config_policy_bundle_and_entitlement_matrix/)
  — replayable fixture instances anchored to the same packet shape.

Related contracts:

- [`/docs/config/m4/structured-config-manifest-environment-editor-qualification.md`](./m4/structured-config-manifest-environment-editor-qualification.md)
  — source/effective/live vocabulary and round-trip-risk guardrails reused by
  this matrix.
- [`/docs/config/structured_config_parameter_source_and_round_trip_review.md`](./structured_config_parameter_source_and_round_trip_review.md)
  — per-parameter provenance, compare-before-save review, and export/support
  disclosure built on top of this family matrix.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — admin policy precedence, bundle-cache, last-known-good, and safe-default
  rules this matrix points to.
- [`/docs/security/emergency_disable_bundle_contract.md`](../security/emergency_disable_bundle_contract.md)
  — emergency-disable precedence and continuity rules this matrix reuses.
- [`/docs/governance/deployment_profile_truth.md`](../governance/deployment_profile_truth.md)
  — deployment-profile honesty and degraded-state vocabulary aligned with the
  profile qualification rows here.

Normative product sources remain the `.t2/docs/` design documents. If this file
disagrees with those sources, those sources win and this file, the schema, and
the checked-in packet update together.

## What this matrix freezes

The packet is intentionally metadata-only. It is the control source that later
settings, inspectors, support exports, and release evidence ingest instead of
copying prose. It freezes three things:

1. `artifact_families[]`
   Each row names a config-bearing family such as request-workspace
   environments, database/API profiles, notebook runtime manifests, preview
   runtime config, workflow bundles, CI/infra descriptors, managed policy
   overlays, and the signed bundle families themselves.

   Every row states:

   - whether there is a reviewable authored source object;
   - whether there is a reviewable effective/resolved projection;
   - whether live or observed state exists, and if so whether it is live,
     mirrored, or deferred;
   - how secrets appear (handle, redacted placeholder, or key path);
   - how policy lock state is shown; and
   - which downgrade or disclosure labels must remain visible.

2. `bundle_taxonomy[]`
   Each row freezes one signed-bundle class:

   - admin policy bundle,
   - offline entitlement snapshot,
   - emergency disable bundle, and
   - trust-root or signer update.

   The row pins required envelope fields, precedence layer, expiry guidance,
   supersedes/revokes behavior, distribution paths, stale-state label, and the
   local-safe continuation posture.

3. `profile_qualifications[]`
   Each row freezes one deployment posture:

   - `local_only`
   - `managed`
   - `self_hosted`
   - `mirrored`
   - `fully_air_gapped`

   The row states which bundle classes must remain reviewable on that profile,
   which distribution paths are allowed or required, what kind of managed-auth
   dependency exists, what local-safe continuation is promised, and which known
   limits must stay visible.

## Core invariants

The packet is conforming only if all of these remain true:

- authored source, effective projection, and live or observed state are never
  collapsed into one unlabeled view;
- signed bundles stay portable across signed-origin, signed-mirror,
  manual-import, offline-bundle, and preseeded-cache paths where the row claims
  them;
- stale, mirrored, or last-known-good authority is labeled explicitly rather
  than shown as current;
- local-safe continuation remains visible when managed auth, managed policy, or
  live distribution degrades;
- preview-dependent families carry explicit preview or narrower labels rather
  than stable-looking defaults; and
- support/export-safe truth remains available on every profile the packet claims
  to support.

## How downstream surfaces use it

This matrix is the ceiling for later feature work. A downstream surface may
narrow below the packet, but it may not claim stronger config, policy, bundle,
or entitlement truth than the packet allows.

In practice:

- settings and setup surfaces reuse the family rows instead of inventing
  per-surface source/effective/live vocabulary;
- policy and entitlement inspectors reuse the bundle taxonomy instead of
  minting bundle-local expiry or rotation semantics;
- Help/About and support export reuse the profile qualification rows instead of
  describing local-only, mirrored, and air-gapped behavior ad hoc; and
- release evidence can point to this packet when reviewing whether a later lane
  preserved the frozen control model.

## Regeneration

Regenerate the canonical packet and markdown summary with:

```sh
cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_matrix -- json
cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_matrix -- markdown
```
