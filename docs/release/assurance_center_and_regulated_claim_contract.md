# Assurance-center, regulated-claim packet, and compliance-evidence handoff contract

This document is the contract layer that turns regulated-environment and
assurance language into product-bound state. It defines the surfaces a
release operator, support engineer, procurement reviewer, customer
admin, auditor, or end-user inspects when they ask the same question:
*does the active path satisfy this claim right now?*

The assurance center does not invent a second assurance vocabulary.
Every card it renders is a projection of one
[`assurance_claim`](../../schemas/release/assurance_claim.schema.json)
row from
[`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml).
This contract adds the per-card surface, the per-claim control-proof
breakdown, the exception or waiver row, the linkage to the real
boundary (key ownership, residency, proxy or egress posture, update or
mirror provenance, policy source, and hosted-dependency blockers), and
the redaction-safe evaluation-packet export every reviewer can replay.

Companion artifacts:

- [`/schemas/release/assurance_claim_card.schema.json`](../../schemas/release/assurance_claim_card.schema.json)
  - boundary schema for `assurance_claim_card_record`.
- [`/schemas/release/evaluation_packet.schema.json`](../../schemas/release/evaluation_packet.schema.json)
  - boundary schema for `evaluation_packet_record`.
- [`/fixtures/release/assurance_center_cases/`](../../fixtures/release/assurance_center_cases/)
  - worked cases for an active regulated claim, a stale-evidence
    downgrade, a hosted-dependency blocker, a customer-managed-key
    claim, and a redacted evaluation-packet export.
- [`/docs/release/assurance_claim_matrix.md`](./assurance_claim_matrix.md)
  and
  [`/schemas/release/assurance_claim.schema.json`](../../schemas/release/assurance_claim.schema.json)
  - upstream claim-row truth this contract projects from. If the
    matrix and a card disagree, the matrix and its row schema win and
    the card updates in the same change.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml),
  [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml),
  and
  [`/docs/governance/deployment_profile_truth.md`](../governance/deployment_profile_truth.md)
  - deployment-profile, residency, key-mode, residual-dependency, and
    hosted-dependency vocabulary the card's `boundary_linkages` field
    quotes verbatim.
- [`/schemas/policy/admin_policy.schema.json`](../../schemas/policy/admin_policy.schema.json)
  and
  [`/schemas/policy/policy_bundle_cache_entry.schema.json`](../../schemas/policy/policy_bundle_cache_entry.schema.json)
  - precedence, redaction-class, and policy-source vocabulary the
    card's `policy_source` block quotes verbatim.
- [`/schemas/release/waiver_packet.schema.json`](../../schemas/release/waiver_packet.schema.json)
  - waiver-packet refs surfaced through the card's
    `exception_or_waiver_rows`.
- [`/schemas/governance/provenance_badge.schema.json`](../../schemas/governance/provenance_badge.schema.json)
  - provenance, signature, attestation, license, and notice vocabulary
    the card's `update_provenance` and `mirror_provenance` blocks
    quote.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` sections on regulated-environment posture,
  sovereign deployment, customer-managed keys, no-vendor-hosted-AI,
  no-telemetry posture, evaluation handoff, and assurance language.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections on
  managed-service separation, local-first defaults, mirror or
  offline posture, and trust-boundary review.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections on policy
  bundle cache, signature stack, evidence packets, and support handoff.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections on assurance
  center, claim cards, and procurement-facing review surfaces.

If this document disagrees with those sources, those sources win and
this document plus its companion schemas and fixtures update in the
same change.

## Scope

Frozen here:

- one `assurance_claim_card_record` shape rendered per claim row inside
  the assurance center;
- one `evaluation_packet_record` shape for the redaction-safe export
  bundle that procurement, regulated review, customer evaluation,
  release evidence, and support handoff all read;
- a closed `active_claim_state` vocabulary (`claimed`, `supported`,
  `limited`, `not_claimed`, `evidence_stale`, `exception_recorded`,
  `retest_pending`, `experimental`) aligned with the upstream
  `claim_class` ladder;
- a closed `proof_class` vocabulary (`vendor_asserted`,
  `customer_provided`, `runtime_observed`, `policy_declared`,
  `not_currently_verifiable`) reused verbatim from the assurance-claim
  row schema;
- the `boundary_linkages` block that binds every card to key ownership,
  residency, proxy/egress posture, update provenance, mirror
  provenance, policy source, and the hosted-dependency blocker list, so
  a reviewer can inspect the active path instead of only reading copy;
- the `exception_or_waiver_row` shape that records active waivers,
  known limits, policy blocks, compatibility exceptions, and
  hosted-dependency blocks;
- the redaction profile every external evaluation export MUST apply
  before crossing the customer, auditor, regulator, or procurement
  boundary; and
- structural invariants that make the prohibited launch wording
  (`regulated`, `sovereign`, `air-gapped safe`, `customer-managed
  keys`, `no vendor-hosted AI`, `no telemetry`) impossible to render
  unless the active path satisfies the claim now.

Out of scope:

- obtaining certifications (FedRAMP, ISO, SOC2) or operating a
  compliance program. The assurance center exposes the proof posture;
  the certification process is owned outside this milestone.
- the assurance-center UI implementation (layout, copy polish,
  interaction model). This contract freezes the record shape; the
  surface owners design the UI on top.
- the runtime check engine that decides whether a control proof is
  currently verified. This contract freezes the record the engine
  publishes.
- raw signatures, raw attestations, raw key material, raw policy
  bodies, raw provider payloads, raw tenant identifiers, raw user
  identifiers, raw hostnames, raw paths, raw secrets, and raw policy
  decision logs. The packet boundary forbids them.

## Surfaces

The assurance center is one product surface composed of five record
families:

1. **Assurance-center overview.** A list view that ranks every active
   `assurance_claim_card_record` in the current scope envelope, by
   subject family and effective claim state, so a reader can see at a
   glance which families are claim-bearing, which are limited, which
   are stale, and which are explicitly not claimed. Each row in the
   overview is a `card_id` plus `claim_subject_family`,
   `effective_claim_state`, `auto_narrowed`, `freshness_state`, and an
   `evaluation_packet_refs` list.
2. **Claim card.** The per-claim detail view. One
   `assurance_claim_card_record` per backing `assurance_claim` row.
   Carries the active state, the boundary linkages, the proof rows,
   the exception rows, the freshness window, the rendering contract,
   and an `evaluation_packet_refs` list.
3. **Control-proof row.** A row inside the card that names one
   `proof_class`, its `proof_presence`, its `verification_state`, the
   evidence refs that back it, and the freshness window beyond which it
   ages out. Reviewers navigate from the row to evidence, from the
   evidence to the runtime workflow, and from the workflow back to the
   card.
4. **Exception or waiver row.** A row inside the card that names the
   active exception class (`waiver`, `known_limit`, `policy_block`,
   `compatibility_exception`, `hosted_dependency_block`), the waiver
   packet ref if applicable, the effective window, the redaction class,
   and the publication destinations the exception MUST surface through
   while it is active.
5. **Evaluation-packet export.** The redaction-safe bundle of one or
   more cards, frozen for procurement, regulator, auditor, customer,
   support, or release-evidence handoff. The packet is the only
   export form for assurance-center state that crosses the Aureline
   boundary; raw evidence and raw policy bodies do not.

## Active claim states

The card's `active_claim_state` vocabulary mirrors the upstream
`claim_class` enum but uses surface-facing names:

| Active state          | Backing `claim_class`         | Renders when                                                                                                  |
|-----------------------|-------------------------------|---------------------------------------------------------------------------------------------------------------|
| `claimed`             | `certified`                   | Every claim-bearing proof family is present, runtime-observed proof is verified inside the freshness window, and no downgrade trigger is active. |
| `supported`           | `supported`                   | Vendor-asserted and policy-declared posture plus runtime-observed OR customer-provided evidence is present or partial. |
| `limited`             | `limited`                     | At least one downgrade reason is active and the card narrows to the disclosed scope.                          |
| `not_claimed`         | `not_claimed`                 | The row is an explicit non-claim. The card MUST disclose the specific capability excluded.                    |
| `evidence_stale`      | `evidence_stale`              | Evidence exists but its freshness window has expired.                                                         |
| `exception_recorded`  | `exception_recorded`          | The card is under a recorded exception (waiver, known-limit, policy block, compatibility exception, or hosted-dependency block) until it closes. |
| `retest_pending`      | `retest_pending`              | Prior evidence has been invalidated and is being refreshed.                                                   |
| `experimental`        | `experimental` / `community`  | Preview-grade or community-grade posture. Not claim-bearing.                                                  |

`claimed` and `supported` are the only states under which the card may
render claim-bearing wording. Every other state MUST render the
narrowed copy variant from
[`canonical_copy`](../../schemas/release/assurance_claim.schema.json#L594)
on the backing assurance-claim row.

## Proof classes

The five proof classes are reused verbatim from the assurance-claim
row schema:

- **vendor_asserted** — Aureline-authored evidence: schemas, contracts,
  ADRs, vendor-owned fixtures.
- **customer_provided** — Customer-bearing evidence such as deployment
  records, configuration attestations, key-material attestations, or
  customer-supplied conformance reports.
- **runtime_observed** — Live-observed evidence from a reference
  workspace, a certified-archetype run, a benchmark-pack execution, or
  a release-time workflow run.
- **policy_declared** — Policy or governance surface declaration with
  no runtime verification (for example, the deployment-profile policy
  map or the storage-and-retention vocabulary).
- **not_currently_verifiable** — Explicitly outside Aureline's current
  verification capability. A card MUST NOT project a claim-bearing
  state on the strength of `not_currently_verifiable` proof alone.

Each control-proof row also declares a `verification_state` so a
reviewer can distinguish a `runtime_observed` row that is `verified`
from one that is `expired`, `blocked_by_policy`, or
`blocked_by_hosted_dependency`. `verified_within_freshness_window` is
the canonical state for runtime proof inside its `stale_after` window.

## Boundary linkages

`boundary_linkages` is the field that prevents launch copy from
floating free of the active path. Every card MUST populate the
following sub-blocks:

- **key_ownership** — `key_mode_class` drawn from the deployment-
  profile vocabulary (`os_store`, `vendor_managed`, `customer_managed`,
  `offline_trust_root`, `not_applicable`). A card may only render
  customer-managed-keys wording when this block resolves to
  `customer_managed` and at least one runtime-observed or
  customer-provided proof row backs it.
- **residency** — `tenant_org_scope_class`, `region_scope_class`, and
  `retention_class` from the deployment-profile register. A card may
  only render sovereign or single-tenant wording when these resolve to
  the corresponding values; the schema rejects mismatches.
- **proxy_egress_posture** — `public_internet_expectation_class` and
  `outbound_proxy_class`. A card may only render no-vendor-hosted-AI or
  air-gapped-safe wording when this block resolves to
  `public_internet_forbidden` or `public_internet_not_expected` and
  `outbound_proxy_class` is `no_outbound`, `mirror_only`, or
  `customer_proxy_required`.
- **update_provenance** — `origin_path_class`, `exact_build_identity_refs`,
  `signature_state`, `attestation_state`. The card pulls these from
  the active update path so a reviewer can confirm the build the user
  is running matches the build the claim was qualified against.
- **mirror_provenance** — `mirror_class` and `mirror_freshness_class`.
  Air-gapped or sovereign cards MUST resolve `mirror_class` to
  `customer_mirror` or `offline_bundle_import`.
- **policy_source** — `policy_source_class` from
  [`schemas/policy/admin_policy.schema.json`](../../schemas/policy/admin_policy.schema.json),
  the active policy artifact refs, and the policy epoch ref. A card
  may only render policy-bound wording (for example "no telemetry") on
  a `signed_local_admin_bundle` source plus a runtime-observed
  policy_declared proof.
- **hosted_dependency_blockers** — A list of `dependency_class` rows
  drawn from
  [`artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml).
  Each blocker names a `blocker_state_class` and `active_for_workflow`.
  When `active_for_workflow=true` and `blocker_state_class` is
  `conflicts_with_claim_when_active` or
  `incompatible_in_active_profile`, the card MUST raise
  `hosted_dependency_blocker_active` in `active_downgrade_reasons` and
  MUST narrow `effective_claim_state`.

## Active claim states downgrade automatically

A card's `effective_claim_state` MUST narrow when any of the following
fires; the schema rejects a card that declares `claimed` or
`supported` while these triggers are active:

- `required_evidence_stale` — at least one runtime-observed control-
  proof row's `last_verified_at` is older than `stale_after`. The card
  narrows to `evidence_stale`.
- `required_evidence_missing` — a required proof row's `proof_presence`
  is `absent`. The card narrows to `limited`.
- `required_evidence_narrower_than_claim` — proof exists but its
  evidence covers a smaller scope than the declared claim. The card
  narrows to `limited`.
- `docs_version_match_unmet` / `docs_freshness_floor_unmet` — the
  docs-version-match floor frozen on the upstream claim row is unmet.
  The card narrows per the upstream `behavior_when_unmet` value.
- `proof_class_not_currently_verifiable` — the only available proof on
  a required class is `not_currently_verifiable`. The card narrows to
  `experimental` or `limited` per the upstream rule.
- `caveat_uncovered` — a required caveat is not published. The card
  narrows to `limited`.
- `exception_window_expired` — an active exception's `ends_at` has
  passed. The card narrows to `not_claimed` until a new exception or
  proof refresh closes the gap.
- `compatibility_row_degraded` — the linked compatibility row degrades.
  The card narrows to `limited`.
- `support_window_expired` — the active support window has expired.
  The card narrows to `not_claimed`.
- `policy_blocked` — `policy_source` resolves to a precedence layer
  that blocks the claim. The card narrows to `exception_recorded`.
- `hosted_dependency_blocker_active` — a hosted dependency in
  `hosted_dependency_blockers` is `active_for_workflow=true` and
  conflicts with the claim. The card narrows to `limited`.
- `control_path_drift_detected` — the active update path no longer
  matches the build the claim was qualified against, the active policy
  epoch differs from the qualified epoch, or the active mirror class
  differs from the declared mirror class. The card narrows to
  `retest_pending` until the drift closes.

The schema's `allOf` gates encode these structurally:

1. `claimed` or `supported` cards MUST contain a `runtime_observed`
   control-proof row whose `proof_presence` is `present` and whose
   `verification_state` is `verified` or
   `verified_within_freshness_window`.
2. `limited`, `evidence_stale`, `retest_pending`, and
   `exception_recorded` cards MUST have at least one
   `active_downgrade_reasons` entry.
3. `exception_recorded` cards MUST have at least one
   `exception_or_waiver_rows` entry.
4. When `effective_claim_state` is narrower than `declared_claim_state`,
   the card MUST set `auto_narrowed=true`.

These gates are why prohibited launch wording (`regulated`,
`sovereign`, `air-gapped safe`, `customer-managed keys`, `no
vendor-hosted AI`, `no telemetry`) is structurally impossible to ship
without the required proof families.

## Evaluation-packet export

The evaluation packet is the redaction-safe bundle every external
reviewer reads. It carries:

- `packet_class` — `customer_evaluation`, `auditor_evaluation`,
  `regulated_review`, `procurement_handoff`, `support_handoff`, or
  `release_evidence_handoff`. The class drives default redaction.
- `packet_state` — `draft`, `in_review`, `frozen`, `superseded`, or
  `withdrawn`. Frozen, superseded, and withdrawn packets MUST set
  `frozen_at`.
- `scope` — release channels, deployment profiles, client scopes,
  claim subject families, and publication destinations.
- `claim_card_index` — one entry per included card, with the active
  state, the auto-narrowed flag, the active downgrade reasons, and the
  refs into the proof and exception indexes.
- `control_proof_index` — flat list of every proof row with redacted
  evidence refs and the redaction class applied.
- `exception_index` — flat list of every exception row with the
  expiry state and redaction class.
- `redaction` — `redaction_state`, `applied_redaction_class`,
  `redaction_rules_ref`, and a closed `redacted_field_classes` list
  enumerating which raw field classes were excluded from the export.
- `handoff` — `recipient_class`, `recipient_label`,
  `delivery_channel_class`, `signed_at`, and `signing_evidence_ref`.
- `freshness` — capture timestamp, stale-after token, freshness
  state, and next review target.
- `limitations` — explicit limitations of the packet (for example
  "this packet asserts no FedRAMP authorization").

External-class packets (`customer_evaluation`, `auditor_evaluation`,
`regulated_review`, `procurement_handoff`) MUST apply a redaction
class narrower than `internal_support_restricted`. The schema
rejects external-class packets that try to ship internal-only
redaction.

`redacted_field_classes` is the only allowed enumeration of redacted
content. A packet may not invent ad-hoc redaction labels; readers can
reason about what is and is not in the packet from the closed list.

## Linkage rules

- A card's `claim_row_ref` MUST resolve to a row in
  [`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml).
- A card's `boundary_linkages.key_ownership.key_mode_class` MUST
  resolve to a value in
  [`artifacts/governance/deployment_profiles.yaml#key_mode_class_vocabulary`](../../artifacts/governance/deployment_profiles.yaml).
- A card's `boundary_linkages.residency.{tenant_org_scope_class,region_scope_class,retention_class}`
  MUST resolve to values in the corresponding vocabularies of
  [`artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml).
- A card's `boundary_linkages.policy_source.policy_source_class` MUST
  resolve to a precedence layer in
  [`schemas/policy/admin_policy.schema.json`](../../schemas/policy/admin_policy.schema.json).
- A card's `boundary_linkages.hosted_dependency_blockers[].dependency_class`
  MUST resolve to a value in
  [`artifacts/governance/residual_dependencies.yaml#dependency_class_vocabulary`](../../artifacts/governance/residual_dependencies.yaml).
- An exception row's `waiver_packet_ref`, when non-null, MUST resolve
  to a record conforming to
  [`schemas/release/waiver_packet.schema.json`](../../schemas/release/waiver_packet.schema.json).
- An evaluation packet's `claim_card_index[].card_ref` MUST resolve to
  a card emitted under this contract.

## Out of contract

- Final regulated launch wording for every customer scenario. The
  card freezes the **shape** of regulated language; concrete copy
  remains anchored to the `canonical_copy` field on each
  assurance-claim row.
- Pricing, commercial availability, or certification partner naming.
- The waiver-issuance workflow, which remains governed by
  [`schemas/release/waiver_packet.schema.json`](../../schemas/release/waiver_packet.schema.json).
- OS-level, NTP-level, or external infrastructure posture outside
  Aureline's declared deployment profiles.
- Any UI design choices on the assurance-center surface beyond the
  `rendering_contract` flags (which fields MUST be visible). Layout,
  ordering, and copy polish remain owned by surface designers.
