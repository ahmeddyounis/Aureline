# Policy explain and diff cases

Worked examples for the admin-policy artifact and signed bundle-cache
contract.

Cases:

- [`aureline_policy_artifact_reviewable.json`](./aureline_policy_artifact_reviewable.json)
  — reviewable `$AURELINE_POLICY/aureline.policy.json` projection with
  precedence, signature metadata, safe defaults, cache refs, and local
  export fields.
- [`offline_continuity_last_known_good.json`](./offline_continuity_last_known_good.json)
  — managed policy refresh fails; the client keeps the last-known-good
  signed bundle for local-safe work and pauses fresh managed actions.
- [`expired_bundle_degrades_to_safe_defaults.json`](./expired_bundle_degrades_to_safe_defaults.json)
  — bundle is past grace; local-safe defaults continue and managed
  privileges fail closed with an explainable packet.
- [`mirror_import_private_registry.json`](./mirror_import_private_registry.json)
  — a private mirror import activates with receipt and mirror-selection
  explanation without exposing raw mirror URLs.
- [`reconstructed_ai_provider_policy_decision.json`](./reconstructed_ai_provider_policy_decision.json)
  — a deny decision is reconstructed locally from refresh history,
  last-known-good metadata, emergency-disable records, and effective
  configuration sources.

These examples are contract fixtures, not policy-engine output. They use
opaque refs and reviewable summaries instead of raw rule bodies,
signatures, URLs, hostnames, paths, tenant names, user identifiers, or
secret material.
