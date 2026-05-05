# Emergency disable bundle, precedence, and local-continuity card contract

This document freezes the pre-implementation contract for **emergency
disable bundles** and the **local-continuity cards** that explain their
effects.

The goal is to make emergency feature disablement a **durable, signed,
inspectable object** rather than a silent disappearance:

- compromised extensions, providers, routes, or channels can be narrowed
  quickly;
- support and administrators can predict outcomes from typed fields
  (without reading raw policy or transport bytes); and
- mirrored, manual-import, and offline environments preserve the same
  local-continuity vocabulary as connected environments.

Companion artifacts:

- [`/schemas/security/emergency_disable_bundle.schema.json`](../../schemas/security/emergency_disable_bundle.schema.json)
  - machine boundary for `emergency_disable_bundle_record`.
- [`/schemas/security/local_continuity_card.schema.json`](../../schemas/security/local_continuity_card.schema.json)
  - machine boundary for `local_continuity_card_record`.
- [`/fixtures/security/emergency_disable_cases/`](../../fixtures/security/emergency_disable_cases/)
  - worked examples for extension disablement, provider/route disablement,
    channel freeze, mirror-imported state, and superseded/expired states.

Normative source alignment:

- `.t2/docs/Aureline_PRD.md` — emergency response must support fast
  disable bundles for compromised extensions, providers, or service
  endpoints.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6.1 and
  §22.8 — signed policy bundles, emergency disable bundles, and
  mirror/offline distribution requirements.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.10 and
  §7.11.13 — threat model, emergency response levers, and local
  continuity obligations.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  - local precedence and safe-default rules, including emergency-disable
  as a highest-priority ratchet.
- [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  and [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  - shared signer-continuity, distribution freshness, trigger-source,
    and local-continuity vocabulary that this contract reuses.
- [`/docs/release/mirror_integrity_and_offline_verification_contract.md`](../release/mirror_integrity_and_offline_verification_contract.md)
  - offline and mirror-safe propagation vocabulary for disable-bundle
    metadata.

If this contract disagrees with `.t2/docs/` sources, the `.t2/docs/`
source wins and this document, schemas, and fixtures update together.

## Scope

Frozen at this revision:

- the `emergency_disable_bundle_record` boundary schema: a signed,
  inspectable object describing emergency disables (extension, provider,
  route, endpoint, and channel freeze targets) without embedding raw
  bundle bytes;
- the precedence and interaction rules across:
  - signed emergency disable bundles,
  - signed policy bundles,
  - registry metadata,
  - update metadata,
  - local overrides; and
  - mirror/manual/offline import state (freshness + validation labels);
- the `local_continuity_card_record` boundary schema: a stable
  explanation card stating what remains available locally, what is
  blocked, which source triggered the state, and the next safe recovery
  path.

Out of scope:

- issuing/signing real emergency bundles;
- transport formats, publication plumbing, and kill-switch delivery
  implementations;
- embedding raw bundle bytes, raw signatures, raw trust roots, raw
  registry payloads, raw URLs, hostnames, or absolute paths.

## Emergency disable bundle record

An `emergency_disable_bundle_record` is the signed object that freezes
emergency disablement as a first-class product artifact.

It carries:

- **Identity**: `disable_bundle_id` plus `record_kind` and schema
  version.
- **Targets**: `subject_refs[]` naming the extension, provider route,
  mirror route, endpoint, or channel being disabled/frozen.
- **Target scope**: `affected_install_linkage` and
  `deployment_profile_scope` bounding where the targets apply.
- **Minimum required version**: `minimum_required_version_ref` naming
  the minimum safe/required version when the recovery path is “update to
  a minimum version”.
- **Precedence**: `precedence_layer = emergency_disable` (pinned) so
  resolvers and explain packets can preserve ordering without inference.
- **Issuer and approval chain**: `issuer_ref` and `approval_chain[]`.
- **Expiry and supersedence**: `effective_at`, `expires_at`,
  `supersedes_disable_bundle_refs[]`, and
  `superseded_by_disable_bundle_ref`.
- **Follow-up rule**: what must happen next for recovery (for example:
  a signed successor bundle, a signed emergency-action record, or a
  post-incident review link).
- **Local continuity note**: a short, export-safe statement describing
  what still works locally while the bundle is enforced.
- **Mirrored/offline continuity**: `signer_continuity` and
  `distribution_statuses[]` rows so a mirror or air-gapped import does
  not pretend to be live-authoritative.

Non-negotiable rules:

1. **Disable bundles narrow only.** They MUST NOT widen capability,
   trust, or egress. A bundle that attempts to “re-enable” is
   non-conforming; re-enablement happens only by supersedence, expiry,
   and explicit follow-up state.
2. **Durable history.** Superseded or expired bundles remain
   inspectable; consumers MUST NOT treat them as missing state.
3. **Scope is explicit.** A bundle that cannot be proven to match the
   local target scope is refused with an explainable reason; it is not
   applied silently.
4. **Expiry is explicit.** If the emergency state is time-bounded, it
   must be stated in `expires_at`. If it is not time-bounded, the
   follow-up rule must state how it clears (superseding bundle or
   administrative clearance) without implying a hidden vendor-only
   console.

## Precedence and interaction rules

Emergency disablement must remain predictable even when multiple sources
contribute restrictions.

### Precedence order

When multiple sources apply to the same target, the resolver preserves
this order (low → high), aligned with the policy resolver contract:

1. **Signed policy bundle** — admin ceiling and narrowing-only locks.
2. **Registry metadata** — registry or mirror-projected disablement,
   quarantine, or revocation signals for extensions, publishers, and
   artifact families.
3. **Update metadata** — signed update-channel controls, pauses, and
   freezes distributed alongside release metadata.
4. **Local override** — user choice or local admin override that may
   further narrow but MUST NOT widen beyond signed ceilings.
5. **Signed emergency disable bundle** — highest-priority ratchet; may
   only disable/freeze/narrow until superseded or expired by signed
   evidence.

The resolver MAY render only one effective state, but explainability
surfaces MUST retain the full source chain and identify which layer
forced the final disable/freeze.

### Mirror/manual/offline import overlay

Mirror/manual/offline import does not change the meaning of a disable.
It changes how the product labels evidence:

- `distribution_statuses[]` rows carry `freshness_class` and
  `validation_state` so a mirror snapshot can be described as
  `mirrored_stale_past_grace` instead of being treated as silently
  authoritative or silently ignored.
- A stale or expired snapshot MUST remain visible and explainable. It
  may narrow what managed actions are permitted, but it MUST NOT be
  represented as “nothing happened”.

## Local continuity card record

A `local_continuity_card_record` is the durable explanation object a
surface renders when a capability is blocked, frozen, or narrowed.

It MUST state:

- **What still works** (`local_continuity.retained_capabilities[]`).
- **What is blocked** (`local_continuity.blocked_capabilities[]`).
- **What triggered the state** (`source.trigger_source_class`) and the
  stable `source_ref` (policy bundle ref, registry record ref, emergency
  disable bundle ref, or override ref).
- **What to do next** (`next_safe_paths[]`) as typed recovery actions
  (update to minimum version, import a newer snapshot, contact admin,
  inspect details).

Surfaces MUST NOT replace this card with a generic “disabled” tooltip or
silently hide the capability. The card is the required continuity truth
for support, admins, mirrored installs, and offline bundles.

## Worked examples

Worked examples live in:
[`/fixtures/security/emergency_disable_cases/`](../../fixtures/security/emergency_disable_cases/).

They cover:

- extension disablement;
- provider/route disablement;
- channel freeze disablement;
- mirror-imported emergency state; and
- superseded/expired disable bundles that remain inspectable.

