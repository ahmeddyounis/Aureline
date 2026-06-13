# Structured config, policy, and entitlement certification

This document is the narrative companion for Aureline's canonical certification
packet for structured configuration, signed policy bundles, offline
entitlements, and admin-audit explainability across claimed deployment
profiles.

Machine-readable companions:

- [`/artifacts/config/structured_config_policy_entitlement_certification.json`](../../artifacts/config/structured_config_policy_entitlement_certification.json)
  — the canonical checked-in packet consumed by release-center, Help/About,
  docs/help, support export, and shiproom review.
- [`/artifacts/config/structured_config_policy_entitlement_certification.md`](../../artifacts/config/structured_config_policy_entitlement_certification.md)
  — generated summary suitable for human review beside the JSON packet.
- [`/schemas/config/structured_config_policy_entitlement_certification.schema.json`](../../schemas/config/structured_config_policy_entitlement_certification.schema.json)
  — boundary schema for the packet.
- [`/fixtures/config/structured_config_policy_entitlement_certification/`](../../fixtures/config/structured_config_policy_entitlement_certification/)
  — replayable canonical and degraded fixture instances anchored to the same
  packet shape.

Related contracts:

- [`/docs/config/structured_config_policy_bundle_and_entitlement_matrix.md`](./structured_config_policy_bundle_and_entitlement_matrix.md)
  — freezes the family, bundle-taxonomy, and deployment-profile rows this
  certification packet narrows but never widens.
- [`/docs/config/structured_config_artifact_modes_and_layers.md`](./structured_config_artifact_modes_and_layers.md)
  — freezes the shared source/effective/live and environment-layer vocabulary
  the certification packet requires downstream rows to reuse.
- [`/docs/config/structured_config_parameter_source_and_round_trip_review.md`](./structured_config_parameter_source_and_round_trip_review.md)
  — freezes parameter provenance, secret/reference chips, compare-before-save
  review, and export/support disclosure for the structured-editor families.

Normative product sources remain the `.t2/docs/` design documents. If this file
disagrees with those sources, those sources win and this file, the schema, the
checked-in packet, and the fixtures update together.

## What this packet freezes

The packet is the release-safe certification source for these questions:

1. Which config-bearing artifact families currently hold a certified claim,
   and which are narrowed to `limited` or `retest_pending`?
2. Which deployment profiles currently hold a certified configuration truth
   claim, and which degrade to `offline_only` or `retest_pending` under stale
   policy, reauth, signer rotation, or mirror/offline fallback?
3. Whether source, effective, and live truth remain reviewable rather than
   collapsed on every claimed family.
4. Whether signed policy, entitlement, emergency-disable, and signer-rotation
   paths stay reviewable across managed, mirror, manual-import, and offline
   delivery.
5. Whether release-center, Help/About, docs/help, support export, and shiproom
   surfaces quote one packet instead of drifting after promotion.

## Artifact-family rows

Each `artifact_rows[]` entry binds one current config-bearing family to:

- its upstream qualification ceiling from the family matrix;
- its current published certification state;
- source/effective/live, mode/layer, parameter-provenance, secret/reference,
  and policy-lock explainability coverage;
- the admin-audit explainability state;
- supported deployment profiles;
- exact evidence age; and
- explicit narrowing reasons where the row sits below its ceiling.

The packet covers the existing family set:

- request-workspace environments,
- database profiles,
- API profiles,
- notebook runtime manifests,
- preview runtime config,
- workflow bundle manifests,
- CI environment descriptors,
- infrastructure environment descriptors,
- managed policy overlays,
- admin policy bundles,
- offline entitlement snapshots,
- emergency-disable bundles, and
- trust-root or signer-update review objects.

## Deployment-profile rows

Each `profile_rows[]` entry binds one claimed deployment profile to:

- required signed-bundle classes,
- allowed distribution paths,
- managed-auth dependency posture,
- the promised local-safe floor,
- known limits that must stay visible, and
- six explicit drills:
  - reference-workspace,
  - mirror/offline,
  - managed/self-hosted,
  - stale-policy,
  - reauth-required, and
  - signer-rotation.

## Core invariants

The packet is conforming only if all of these remain true:

- every known artifact family and claimed deployment profile is covered;
- no preview-scoped family publishes as fully certified;
- no certified row carries stale evidence or hidden narrowing reasons;
- structured-editor families keep parameter-source/save-review depth;
- signed-bundle families keep signed-path reviewability;
- every claimed profile preserves a visible local-safe floor; and
- release-center, Help/About, docs/help, support export, and shiproom all quote
  this packet directly.

## Regeneration

Regenerate the canonical packet, markdown summary, and degraded fixtures with:

```sh
cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- json
cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- markdown
cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- stale_policy
cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- reauth_required
cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- signer_rotation
```
