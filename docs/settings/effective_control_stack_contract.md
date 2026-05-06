# Effective control-stack contract

This contract freezes one **explainable control stack** for settings, policy
ceilings, experiments, and emergency kill switches so every surface can answer:

- what the effective value / allow / deny decision is;
- which layer produced it (and which layers were present but lost);
- whether the device is operating on cached / offline / degraded state; and
- how to reproduce the same explanation **without reading implementation code
  or logs**.

The control stack is **local-first**: editing-critical paths resolve locally
from embedded defaults and locally cached signed material even when managed
control planes are stale or unreachable.

## Companion artifacts

- [`/schemas/settings/effective_control_stack_row.schema.json`](../../schemas/settings/effective_control_stack_row.schema.json)
  — machine-readable row and matrix packet used by UI, CLI inspect, diagnostics,
  and support export.
- [`/artifacts/settings/control_stack_examples.yaml`](../../artifacts/settings/control_stack_examples.yaml)
  — compact examples and pointers to worked fixtures.
- [`/fixtures/settings/control_stack_cases/`](../../fixtures/settings/control_stack_cases/)
  — worked scenarios covering offline continuity, expiry, ceilings, mismatch,
  and emergency narrowing.

This contract composes with:

- [`/schemas/settings/effective_setting.schema.json`](../../schemas/settings/effective_setting.schema.json)
  — settings effective record; `effective_setting_record.control_stack` is the
  **per-setting projection** of this contract.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — signed admin policy bundles, last-known-good cache entries, mismatch rules,
  and emergency disable records.
- [`/artifacts/governance/feature_flag_provider_chain.yaml`](../../artifacts/governance/feature_flag_provider_chain.yaml)
  — provider-chain ladder for experiments / flags / rollouts, including kill
  switch precedence and offline posture.
- [`/schemas/governance/policy_decision_explain.schema.json`](../../schemas/governance/policy_decision_explain.schema.json)
  — typed explain records emitted by policy and emergency decision sites.

Normative product sources are the settings, governance, policy, and UX sections
in `.t2/docs/`. If this contract disagrees with those sources, the `.t2/docs/`
source wins and this contract plus the companion schemas and fixtures must be
updated in the same change.

## Control-stack ladder (precedence)

The effective control stack is an ordered ladder. Higher layers override lower
layers **unless a safety ceiling caps them**.

| Rank | Layer | Typical source | Offline / degraded rule |
|---:|---|---|---|
| 1 | Embedded default | shipped product default | always available locally |
| 2 | Signed local admin bundle | local signed policy/config bundle | last-known-good signed bundle remains active and inspectable offline unless a stricter signed safety rule says otherwise |
| 3 | User/profile/workspace configuration | settings UI, JSONC, profile import/export, workspace manifests | invalid entries degrade to explicit validation errors; no silent ignore |
| 4 | Optional managed override | connected control plane / tenant policy | remote fetch failure may not block startup or core editing-critical flows; must fall back to locally cached signed material |

### Safety ceiling rule (orthogonal cap)

Stricter safety/compliance policy or emergency controls may cap later layers.
When a cap applies:

- the trace MUST include an explicit ceiling row (policy ceiling or kill switch);
- the winning value MUST still identify the lower-layer request it capped; and
- the UI MUST say that a ceiling was applied (never pretend a lower layer won).

## Row contract (what every surface renders)

Surfaces render **the same row shape** for each layer and ceiling; no surface
reconstructs its own narrative from ad hoc internal state.

Required fields (see the schema for exact names and enums):

- **Source identity:** `control_authority` + `source_label`
- **Current binding:** `current_value` (branch/value summary suitable for UI and
  CLI)
- **Lifecycle:** `lifecycle_label`, `last_refresh_at`, `expires_at`
- **Offline posture:** `offline_fallback`
- **Explainability:** `explain_why_ref` (opaque handle for “Explain why”)
- **Schema reference:** `schema_ref` (points to the contract shape backing the
  row)
- **Export-safe evidence:** `evidence_refs` (opaque ids or safe refs; never raw
  secrets, raw URLs, raw policy bodies, raw signatures, or tenant identities)

### Projection rule (one trace, many surfaces)

The same matrix packet MUST be projectable by:

- desktop settings UI and any “Effective value” inspector;
- CLI inspect output (human + machine-readable);
- diagnostics / Project Doctor (summary + deep links);
- support export bundles.

If a surface needs additional copy, it may add **presentation**, but it must not
change meaning, precedence, or omit rows. Any additional narrative MUST cite the
row’s `schema_ref` and one of its `evidence_refs`.

## Local-offline resolution packet

The matrix packet is the **local proof** of resolution. A reviewer reconstructs
the effective decision from the packet alone:

- all considered layers are present as rows (including unavailable/expired);
- the winner is identified; and
- any ceiling (policy narrowing, emergency kill switch) is explicit.

Packets MUST remain export-safe: evidence refs are opaque handles, not raw
payloads.

## Guardrails

1. **No silent widening.** No control path may silently widen network use, trust
   posture, write scope, or retention behavior. Any widening attempt MUST be
   denied or require explicit approval, and the trace must include the attempted
   source plus the denial/ceiling row that prevented it.
2. **Editing-critical continuity.** Core editing-critical flows resolve locally
   even if managed overrides are stale/unreachable. Managed freshness is visible
   (`last_refresh_at`, `offline_fallback`) rather than inferred.
3. **Explain-why is a handle, not a story.** `explain_why_ref` is resolved by a
   dedicated explainer that returns a typed packet; surfaces do not invent
   their own “because …” strings.
4. **Mismatch is explicit.** Signature mismatch / scope mismatch / refused
   bundles must appear as blocked rows with evidence refs to the validation and
   last-known-good selection record.

## Worked cases

The fixtures under [`/fixtures/settings/control_stack_cases/`](../../fixtures/settings/control_stack_cases/)
cover:

- offline last-known-good signed admin bundle continuity;
- expired experiment/flag with deterministic fallback;
- policy ceiling overriding a local preference (narrowing);
- signed-bundle mismatch and last-known-good fallback; and
- emergency kill-switch narrowing that remains visible and auditable.

