# Admin policy artifact and signed bundle-cache contract

This contract freezes the local artifact family used for admin policy,
signed bundle-cache entries, local explainability, and export-safe
policy handoff. It makes policy reviewable as text and cache metadata
instead of treating a hosted console as the only source of truth.

## Companion artifacts

- [`/schemas/policy/admin_policy.schema.json`](../../schemas/policy/admin_policy.schema.json)
  — machine-readable boundary for `$AURELINE_POLICY/aureline.policy.json`.
- [`/schemas/policy/policy_bundle_cache_entry.schema.json`](../../schemas/policy/policy_bundle_cache_entry.schema.json)
  — machine-readable boundary for one signed bundle-cache entry under
  `$AURELINE_STATE/policy/`.
- [`/fixtures/policy/explain_and_diff_cases/`](../../fixtures/policy/explain_and_diff_cases/)
  — worked examples for offline continuity, expired-bundle degrade,
  mirror import, and local decision reconstruction.
- [`/docs/identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md)
  and [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — upstream signed policy-bundle vocabulary.
- [`/docs/admin/policy_explainability_contract.md`](../admin/policy_explainability_contract.md)
  and [`/schemas/admin/effective_policy_card.schema.json`](../../schemas/admin/effective_policy_card.schema.json)
  — product-facing card, diff, history, lock explanation, and handoff
  vocabulary.
- [`/docs/governance/policy_flag_schema_stack.md`](../governance/policy_flag_schema_stack.md)
  and [`/artifacts/governance/feature_flag_provider_chain.yaml`](../../artifacts/governance/feature_flag_provider_chain.yaml)
  — precedence, flag evaluation, emergency-disable, and schema-family
  governance.

Normative product sources are the policy, identity, admin-plane,
network, degraded-mode, and settings sections in `.t2/docs/`. If this
contract disagrees with those sources, the `.t2/docs/` source wins and
this contract plus the schemas must be updated in the same change.

## Scope

Frozen here:

- `$AURELINE_POLICY/aureline.policy.json` as the local, reviewable,
  admin-managed artifact that names the active signed policy bundle,
  scope, epoch, signature metadata, precedence layers, safe-default
  posture, emergency-disable refs, cache refs, and export-safe
  explainability fields;
- `$AURELINE_STATE/policy/*` signed bundle-cache entries as the local
  record of signed bundle metadata, refresh history, validation
  results, last-known-good selection, emergency-disable records, and
  local decision reconstruction evidence;
- the precedence and safety rules used when embedded defaults, signed
  local admin policy, user or workspace configuration, optional managed
  overrides, and emergency disables interact;
- local diff, explain, and export requirements for allow/deny decisions,
  flag evaluation, mirror selection, endpoint restrictions, and
  policy-driven disablement; and
- the redaction boundary for support exports, CLI JSON, Project Doctor,
  policy-center cards, and admin handoff packets.

Out of scope:

- implementing a policy evaluator, signature verifier, managed console,
  mirror server, or bundle authoring UI;
- choosing a policy language;
- embedding raw policy rule bodies, raw signatures, tokens, tenant
  directory payloads, provider payloads, raw URLs, raw hostnames, raw
  paths, or raw user identifiers in these local artifacts.

## Artifact family

`aureline.policy.json` is an admin-owned control artifact. It is not a
user preference file, not a profile export file, and not a workspace
setting. Users and workspaces can select values only inside the ceiling
that the effective signed policy allows.

The file carries reviewable metadata:

- artifact id, artifact version, schema version, and location class;
- tenant or org scope as opaque refs plus deployment and install-profile
  scope;
- active policy epoch and active bundle version;
- detached-signature metadata and verification posture, without raw
  signature bytes;
- the precedence layers the resolver must use;
- policy rules as target refs, effect classes, safe defaults, fallback
  paths, export projections, and local/vendor-console handoff notes;
- safe-default rules for service outage, expiry, revocation, signature
  failure, scope mismatch, and emergency disable;
- refs into signed bundle-cache entries and emergency-disable records;
  and
- the export-safe explainability contract that every UI, CLI, support,
  and admin packet uses.

Signed bundle-cache entries are local evidence records. A cache entry
MUST be able to explain why a bundle was active, refused, stale,
last-known-good, or degraded to safe defaults. It stores:

- bundle ref, version, epoch, scope, received/effective/expiry times,
  source class, payload digests, and signature verification metadata;
- refresh attempts, including failed attempts and the prior active entry;
- last-known-good selection and the managed actions paused by that
  selection;
- emergency-disable records that ratchet for the current session or
  until a signed successor clears them; and
- local reconstruction fields that join bundle history, effective
  configuration sources, diff refs, explain refs, and audit event refs.

## Precedence

The policy resolver has two related but distinct jobs:

1. resolve a candidate value or action from local product and user
   sources; and
2. apply authority ceilings, emergency disables, and managed narrowing
   without widening trust or hiding the source.

The resolver uses this order:

| Layer | Role | Widening rule |
|---|---|---|
| `embedded_product_defaults` | floor and startup-safe behavior | defines safe defaults; cannot depend on a network fetch |
| `signed_local_admin_bundle` | admin ceiling and local signed authority | may lock, constrain, or disable; must not widen user/workspace authority |
| `user_workspace_configuration` | user or workspace candidate value | may select within the active ceiling; cannot override a lock or constraint |
| `optional_managed_override` | managed convenience layer where configured | may narrow or replace within the signed ceiling; must not widen trust, egress, write scope, startup dependency, or local-core access |
| `emergency_disable` | highest-priority ratchet | may only disable, freeze, or narrow until superseded or expired by signed evidence |

The product may render a single effective value, but the local explain
packet MUST retain the whole source chain. A user value winning inside
an allowed set and a user value denied by policy are different states;
both must show the signed policy source that admitted or narrowed them.

## Safe Defaults

Failure and outage behavior is deterministic:

| Trigger | Required behavior |
|---|---|
| Managed service unreachable | Use the last-known-good signed bundle for local-safe behavior; pause fresh-managed actions. |
| Bundle refresh times out | Keep the prior active entry if still valid; emit a refresh-history failure row. |
| Bundle expires within grace | Continue existing narrowing, label stale, and deny new managed privilege. |
| Bundle expires past grace | Degrade to the bundle's declared safe defaults; local editing, search, local Git, local tasks, and local history continue unless an explicit signed fail-closed policy applies. |
| Signature verification fails | Reject the candidate bundle, keep last-known-good where allowed, and deny new privileged managed actions. |
| Scope mismatch | Reject and explain; never apply a broader scope silently. |
| Revocation or emergency disable | Apply the signed disable or freeze immediately; preserve evidence and do not silently re-enable on restart. |

Fail-closed is permitted only for the target classes that require fresh
authority or for an explicit signed fail-closed policy. It is not the
default for desktop-local workflows.

## Diff, Explain, And Export

Every policy-controlled decision that affects a visible capability MUST
be explainable locally. The explanation may be compact in the UI, but
the underlying packet carries:

- decision id, decision class, outcome, target class, target ref, and
  user-visible consequence;
- tenant scope, deployment profile, active policy epoch, bundle version,
  and cache entry refs;
- the source chain from embedded default through admin bundle, user or
  workspace configuration, optional managed override, and any emergency
  disable;
- the rule refs and value projections used for the result;
- fallback path and safe-default posture;
- refresh-history, last-known-good, and emergency-disable refs when the
  decision depends on degraded or cached state;
- redaction summary and omitted data classes; and
- browser or vendor-console handoff notes when parity is incomplete.

Diffs compare projections, not raw secrets. A diff row may show that an
endpoint moved from `public_route_allowed` to `mirror_only`, that a flag
resolved to `disabled_by_policy`, or that an AI provider route narrowed
to BYOK-only, but it must not include raw hostnames, policy rule bodies,
provider responses, tenant names, or tokens.

Export paths MUST produce a pair:

- a human-readable summary suitable for a user, admin, or support
  engineer; and
- a machine-readable packet conforming to the schema family named by the
  record, with no progress text, screenshots, rendered Markdown, raw
  signatures, raw URLs, raw hostnames, raw paths, raw identities, raw
  policy bodies, or secret material.

## Decision Classes Covered

The local contract covers at least:

- allow or deny decisions on commands, actions, settings, extension
  install, update channels, support exports, and provider routes;
- feature-flag evaluation where policy or emergency disable changes the
  result;
- mirror selection, including mirror-only and offline-import paths;
- endpoint restrictions such as egress denial, proxy policy, route
  freezing, or public-endpoint refusal;
- policy-driven disablement from an emergency action, stale bundle,
  revoked bundle, signature failure, or scope mismatch; and
- browser/vendor-console handoff where the local packet can explain the
  narrowed decision but not every hosted-console detail.

Browser or vendor-console handoff must be a note and opaque ref, not a
dependency for the baseline explanation. A local user or admin must still
be able to answer "what narrowed this decision?" from the local artifacts.

## Review And Change Discipline

Adding a new enum value, optional field, decision class, source layer, or
safe-default trigger is additive-minor and bumps the relevant schema
version. Repurposing a value, loosening precedence, widening export
content, or treating admin policy as an ordinary user preference is
breaking and requires governance review.

The schemas in this family are strict (`additionalProperties: false`).
Extensions must be explicit, namespaced, and reviewed before they are
accepted into any release, support, or enterprise validation packet.
