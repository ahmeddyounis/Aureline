# Policy, flag, and machine-schema stack selection

This document is the **normative** stack-selection note for three
repository-wide governance concerns that this milestone keeps as one
governed stack rather than three unrelated local conventions:

1. feature-flag evaluation (runtime control of experiments, toggles,
   benchmark modes, and rollout rows);
2. policy bundles and policy-decision explainability (managed
   narrowing, admin-policy ceilings, offline entitlement);
3. machine-readable contracts for JSON artifacts (schemas under
   `/schemas/**`) and any future HTTP service surface.

Companion artifacts (this doc is the narrative home; if they
disagree, this doc wins and the companion file is updated in the
same change):

- [`/artifacts/governance/feature_flag_provider_chain.yaml`](../../artifacts/governance/feature_flag_provider_chain.yaml)
  — provider-chain precedence register for every experiment, flag,
  benchmark mode, and rollout row, including emergency-disable
  precedence.
- [`/artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml)
  — schema-family registry: family name, schema home, publication
  path, versioning rule, unknown-field policy, and compatibility
  owner for every contract-bearing JSON directory.
- [`/schemas/governance/policy_decision_explain.schema.json`](../../schemas/governance/policy_decision_explain.schema.json)
  — boundary schema for the typed explain packet a policy-decision
  site emits instead of an opaque allow/deny string.
- [`./feature_flag_policy.md`](./feature_flag_policy.md) — the
  per-row control policy (ownership, lifecycle, kill switches,
  artifact dependencies). The stack note scopes the **technology
  posture**; the policy doc scopes the **per-row governance
  contract**. Both apply.
- [`../../artifacts/governance/standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  — the canonical industry-standards register. The rows named below
  are the authoritative posture; this doc cites them rather than
  redeclaring versions.
- [`../../artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml)
  — every named flag / experiment / benchmark / rollout row. The
  provider-chain register keys off this file's row ids.
- [`../identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md)
  and [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — policy-bundle source of truth; the explain-packet schema
  re-exports its frozen vocabularies rather than inventing parallel
  ones.

## Why one stack, not three

Three failure modes justify folding flag evaluation, policy bundles,
and machine-schema versioning into one governed stack:

1. **Silent-widening risk.** Any of the three surfaces, if
   ungoverned, can quietly widen trust, egress, write scope, or
   startup dependencies. A flag that flips default-on without a
   provider-chain row, a policy bundle that narrows without a typed
   explain packet, or a schema that adds an undeclared keyword each
   produce the same reviewer gap.
2. **Offline posture drift.** Flag rows, policy bundles, and schema
   loaders each have an "offline" story. If those stories are
   invented locally, editing-critical paths end up blocking startup
   on remote fetches the user never authorised. One stack note lets
   every row cite the same offline vocabulary.
3. **Explainability debt.** A policy decision without a typed explain
   packet ends up as an opaque "disabled by admin" banner, which
   forces every support case to re-ask why. A flag without a
   provider chain ends up as "I don't know which layer set this."
   A schema without a family home ends up as ad-hoc validation. The
   same fix — a typed, versioned, inspectable record — closes all
   three.

## Stack selection

The selections below are **ecosystem reference points**, not live
dependencies. Aureline does not ship a runtime flag service, a
hosted policy engine, or a service HTTP API at this milestone. The
choices fix the shape of the contract so the later runtime reads
from (and bridges to) the ecosystem instead of inventing a parallel
one.

### 1. Feature-flag evaluation — OpenFeature-shaped

- **Ecosystem reference:** the OpenFeature (CNCF) evaluation-API
  shape and provider contract. See
  [`standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  row `standard.openfeature`, `support_class:
  custom_but_mirrorable`, `deviation_policy: bridge_with_adr`.
- **What we adopt:** the evaluation-API shape (flag key → resolved
  value plus reason and variant plus error code plus provider name)
  as the **bridge seam**. A future adapter MAY project internal flag
  rows through an OpenFeature-shaped API when an external consumer
  requires it.
- **What we don't adopt (yet):** the OpenFeature SDK, an OpenFeature
  server, or OpenFeature flag definitions as the internal source of
  truth. Internal flag rows live in `experiments_register.yaml`
  with the control contract described in `feature_flag_policy.md`.
- **Why not adopt OpenFeature directly today:** the specification is
  pre-1.0 and Aureline does not yet ship a runtime flag service.
  Direct adoption would lock the internal contract to a moving
  target.
- **Deviation rule:** adopting OpenFeature as the primary internal
  flag contract, or embedding an OpenFeature SDK on a protected
  lane, requires a deviation ADR that cites the standards-matrix
  row.

### 2. Policy bundles and evaluation — OPA/Rego-shaped

- **Ecosystem reference:** Open Policy Agent bundle-service API and
  manifest shape; Rego as the policy expression language reference.
  See [`standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  row `standard.opa_rego`, `support_class: custom_but_mirrorable`,
  `deviation_policy: bridge_with_adr`.
- **What we adopt:** the **bundle discipline** — versioned, signed,
  distributable policy artifacts with a stable manifest shape, an
  epoch, and last-known-good fallback — via the frozen
  `policy_bundle_record` in
  [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json).
- **What we don't adopt (yet):** Rego as the in-product policy
  language, or a live OPA bundle service. Admin-policy narrowing,
  workspace-trust restrictions, and managed-capability gates are
  typed record decisions, not Rego expressions.
- **What replaces opaque allow/deny:** the typed explain packet
  defined in
  [`policy_decision_explain.schema.json`](../../schemas/governance/policy_decision_explain.schema.json).
  Every policy-decision site emits a record carrying the decision
  class, narrowing reason, opaque rule ref, policy epoch, fallback
  path, identity-mode context, trust-state context, and the
  local-vs-vendor-console explainability envelope so enterprise
  claims about narrowing are justifiable without privileged console
  state.
- **Deviation rule:** adopting Rego as the in-product policy
  language, or shipping a live OPA bundle service as the
  authoritative distribution path, requires a deviation ADR that
  cites the standards-matrix row.

### 3. JSON contracts — JSON Schema Draft 2020-12

- **Ecosystem reference:** JSON Schema Draft 2020-12. See
  [`standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  row `standard.json_schema_2020_12`,
  `support_class: standard_shaped_import_and_export`,
  `deviation_policy: narrow_with_adr`.
- **What we adopt:** every file under `/schemas/**` declares
  `"$schema": "https://json-schema.org/draft/2020-12/schema"` and
  carries a stable `$id` of the form
  `https://aureline.dev/schemas/<family>/<name>.schema.json`.
- **Narrowing rule:** schemas MAY narrow (require a subset of
  keywords, set `additionalProperties: false`, forbid specific
  string shapes) but MUST NOT invent custom keywords outside the
  JSON Schema vocabulary without a deviation ADR.
- **Family registry:** every contract-bearing JSON directory is one
  row in
  [`schema_families.yaml`](../../artifacts/governance/schema_families.yaml).
  A schema that has no family row is not yet ratified; reviewers
  reject ad-hoc validation outside the registry.

### 4. HTTP service API surface — OpenAPI 3.2+

- **Ecosystem reference:** OpenAPI 3.2 (or later once published).
  See [`standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  row `standard.openapi_3_2`,
  `support_class: standard_deferred_placeholder`,
  `deviation_policy: not_yet_committed_pending_standard_maturity`.
- **What we adopt today:** nothing yet ships a hosted HTTP service
  API, so there is no live OpenAPI document.
- **What the stack fixes:** when a service HTTP surface does land
  (CLI/headless runner, docs-help service health, extension
  registry), the description document is an OpenAPI 3.2+ document
  co-located under `/schemas/<family>/` (or a sibling folder the
  family row names), and the contract is ratified by adding an
  OpenAPI family row to `schema_families.yaml`.
- **Deviation rule:** admitting OpenAPI 3.1 as the interim shape, or
  emitting an ad-hoc service description, requires a deviation ADR
  that cites the standards-matrix row.

## Provider-chain and precedence rules

The provider-chain register
([`feature_flag_provider_chain.yaml`](../../artifacts/governance/feature_flag_provider_chain.yaml))
encodes one precedence ladder and one emergency-disable precedence.
Every experiment, flag, benchmark, or rollout row binds to one
provider-chain profile.

### Resolution precedence (highest wins, except where capped)

1. **`emergency_disable`** — any source authorised to fire a kill
   switch can force the row off or narrow it. Emergency disable is
   a one-way ratchet for the current session; it does not silently
   re-enable on next refresh.
2. **`admin_policy_narrowing`** — admin-policy bundles (signed,
   inspectable) MAY narrow or disable a row. Policy MAY NOT widen
   trust, egress, write scope, startup dependencies, or enable an
   `off_by_default` editing-critical row. This cap is the
   `policy_override_posture` field on the row.
3. **`optional_managed_override`** — a vendor-managed convenience
   layer layered over the self-hosted source, where the row's
   `policy_override_posture` allows it. Same widening cap as
   `admin_policy_narrowing`.
4. **`signed_local_admin_bundle`** — a signed local/admin bundle
   cached on device. Used as `last_known_good_signed` when a
   higher layer is offline.
5. **`ci_workflow_input`** — CI-only workflow inputs (scheduled
   runs, rollout rings). Does not apply to editing-critical paths
   at runtime.
6. **`local_cli_argument`** — local CLI or environment inputs. The
   primary invocation path for prototypes, benchmark modes, and
   hidden developer toggles at this milestone.
7. **`embedded_default`** — the value compiled into the build. Every
   row MUST declare `embedded_default_behavior`. This is the
   floor; no row may have no defined behavior.

A row's provider-chain profile names the **subset** of the ladder it
actually supports. A row MUST NOT imply a layer it does not have
(the standards-matrix row
`standard.openfeature` deviation notes this rule). A row that
declares only `embedded_default` and `local_cli_argument` MUST NOT
silently gain a managed layer later without a new decision row.

### Last-known-good and refresh rules

- Rows that can refresh from a higher layer MUST declare a
  `refresh_class` in the provider-chain register. Supported
  classes: `refresh_forbidden`, `refresh_on_explicit_invocation`,
  `refresh_on_session_start`, `refresh_on_cached_expiry`,
  `refresh_on_signed_push`.
- A refresh that fails MUST fall back to the
  `last_known_good_signed` cached value; it MUST NOT block startup
  on editing-critical paths. The editing-critical floor is the
  cross-cutting rule that `experiments_register.yaml` enforces per
  row (`remote_fetch_failure_may_block_startup: false`).
- A refresh that succeeds MUST preserve audit linkage: the previous
  value, the new value, the refresh source, and the policy epoch
  that authored it.

### Offline behavior

- Every row MUST map to exactly one `offline_resolution_posture`:
  `authoritative_local`, `last_known_good_signed`, `cache_only`,
  or `unavailable_offline` (ADR-0008 vocabulary).
- `authoritative_local` means the row resolves entirely from the
  embedded default plus local inputs; offline is the normal case.
- `last_known_good_signed` means the row resolves from the cached
  signed value when a higher layer is offline, with
  `last_known_good_max_age_class` naming the staleness ceiling.
- `cache_only` means the row resolves from a cached value but does
  not hold a signed proof of origin (appropriate for
  non-editing-critical rows only).
- `unavailable_offline` means the row has no resolved value when
  offline; a row using this posture MUST declare a surfaced
  `unavailable_reason_class` rather than silently falling through
  to a default.

### Emergency-disable precedence

Emergency disable is a narrow, ratchet-style source that MUST:

- name its `kill_switch_source_kind` (one of the
  experiments-register enum values);
- name a visible `kill_switch.fallback_behavior` sentence (not "just
  stop using it");
- produce an audit record via the typed explain packet (the
  decision class `emergency_disable_fired`);
- ratchet for the current session — a kill switch that fired does
  not silently unfire on the next refresh within the same session.

Emergency disable precedes admin policy, managed override, signed
local admin bundles, CI inputs, CLI inputs, and embedded defaults.
It does not precede the offline floor: if a row is
`unavailable_offline` and the kill switch cannot be reached, the
row remains unavailable with a typed `unavailable_reason_class`,
not silently enabled.

## Schema-family registry

Every contract-bearing JSON directory under `/schemas/**` is one row
in [`schema_families.yaml`](../../artifacts/governance/schema_families.yaml).
A row names:

- **family id** — stable dot id (e.g. `governance`, `identity`,
  `runtime`, `ux`).
- **schema home** — authoritative directory (`/schemas/<family>/`).
- **publication path** — where consumers read the family from. For
  most families this equals the schema home. For families that
  publish through a generated manifest, release artifact bundle, or
  docs projection, the publication path is distinct and named.
- **versioning rule** — one of:
  `document_const_schema_version_bump_on_breaking`,
  `document_const_schema_version_bump_on_additive_minor`,
  `inherited_from_parent_packet_header`,
  `single_record_no_version_field`, or
  `generated_artifact_versioning_follows_generator`.
- **unknown-field policy** — one of:
  `additional_properties_false_strict`,
  `additional_properties_false_strict_with_namespaced_extension_envelope`,
  `additional_properties_false_strict_with_property_bag`, or
  `additional_properties_true_explicit_reason`.
- **compatibility owner** — the named surface owner (team, role,
  forum) who ratifies version bumps, new enum values, and
  cross-family crosswalks.

Adding a new schema directory under `/schemas/**` without a
corresponding family row is a governance bug. Reviewers reject
ad-hoc validators, ad-hoc version fields, or ad-hoc unknown-field
handling outside the registry.

## Typed policy-decision explain packet

Every policy-decision site — admin-policy narrowing, workspace-trust
restriction, capability gate denial, entitlement grace expiry,
revocation enforcement, offline degradation, emergency disable —
emits a record conforming to
[`policy_decision_explain.schema.json`](../../schemas/governance/policy_decision_explain.schema.json).

The packet fixes the answer to three reviewer questions that opaque
allow/deny can never answer:

1. **What class of decision was this?** — `decision_class` names
   whether the site narrowed an admin policy, denied a capability
   gate, fired an emergency kill switch, fell back to
   last-known-good, or expired a grace window.
2. **Why?** — `narrowing_reason` names the closed vocabulary of
   reasons (stale bundle, admin enforcement, capability-not-in-plan,
   entitlement revoked, trust-state incompatible, etc.) without
   leaking raw rule bodies or raw claim payloads.
3. **How is the operator supposed to recover?** — `fallback_path`
   names the path the user remains on (local safe default,
   restricted mode, offline-only, grace state, none) and the
   `local_vs_vendor_console_explainability` envelope says what is
   visible without privileged console-only state.

The packet re-exports frozen vocabularies from the identity/policy
schemas (identity mode, deployment profile class, trust state,
redaction class, explainability envelope) rather than inventing
parallel enums. Adding a new decision class, narrowing reason, or
fallback path is additive-minor and bumps the packet's schema
version; repurposing an existing value is breaking and requires a
new decision row.

## How to add a new row to the stack

When a contributor introduces a new flag, a new policy-decision
site, or a new schema directory, the minimum ratification trail is:

1. **Flag / experiment / benchmark / rollout row** — add the row to
   `experiments_register.yaml` per `feature_flag_policy.md`, then
   bind it to a provider-chain profile in
   `feature_flag_provider_chain.yaml`. If the row cannot be bound
   to an existing profile, add a new profile row with the
   precedence subset it supports.
2. **Policy-decision site** — emit a record conforming to
   `policy_decision_explain.schema.json`. If the decision class or
   narrowing reason is missing, propose the additive-minor change
   in the schema with the reason documented; reviewers ratify the
   new value or reject the site.
3. **Schema directory** — add a family row to
   `schema_families.yaml` before any schema in the directory is
   consumed by a validator or CI lane. The row names the home,
   publication path, versioning rule, unknown-field policy, and
   compatibility owner. Schemas in the directory inherit those
   conventions.

No change to this stack — provider chain, policy decision, or
schema family — is routine prose. Every change lands with its
companion register updated in the same commit.
