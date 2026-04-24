# Assurance-claim matrix, scope and exclusion model, and evidence-publication rules

This document is the normative contract for launch-facing assurance
language. It makes claims explicit, evidence-bound, and narrow by
default: every row names one claim-subject family, one declared claim
class, one effective claim class after automatic narrowing, an explicit
scope envelope, a closed proof-class breakdown, exclusion and
known-limit refs, a docs-version-match floor, publication destinations,
and downgrade triggers. Release, docs, support, and product owners
quote the same rows by `assurance_claim:` id; no surface may invent
claim-class wording, proof-class labels, or publication destinations
that are not enumerated here.

Companion artifacts:

- [`/schemas/release/assurance_claim.schema.json`](../../schemas/release/assurance_claim.schema.json)
- [`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml)
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
- [`/artifacts/governance/claim_propagation_rules.yaml`](../../artifacts/governance/claim_propagation_rules.yaml)
- [`/artifacts/governance/truth_class_matrix.yaml`](../../artifacts/governance/truth_class_matrix.yaml)
- [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
- [`/artifacts/governance/storage_modes.yaml`](../../artifacts/governance/storage_modes.yaml)
- [`/docs/governance/storage_and_retention_vocabulary.md`](../governance/storage_and_retention_vocabulary.md)
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
- [`/docs/release/release_evidence_packet_template.md`](./release_evidence_packet_template.md)
- [`/docs/release/certified_archetype_report_template.md`](./certified_archetype_report_template.md)

If this document and
[`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml)
disagree, the machine form is authoritative for tooling and this
narrative MUST update in the same change. If the schema and the machine
form disagree, the schema wins and both update in the same change.

## Purpose

Launch-facing wording regresses when it depends on prose memory: a
reader sees "supported", "replacement-grade", or "certified" without a
named evidence set, a named scope, or a named downgrade path. This
matrix removes that ambiguity. Every claim row MUST point to evidence,
exclusions, and downgrade conditions. Replacement-grade or similar
high-bar language is structurally impossible without the required
proof families because the schema rejects it.

## Claim-class vocabulary

The closed claim-class set (mirrored in
[`/schemas/release/assurance_claim.schema.json`](../../schemas/release/assurance_claim.schema.json)
and in `claim_class_vocabulary` in the machine form) is:

- **certified** — The strongest class. Requires `vendor_asserted`,
  `customer_provided`, `runtime_observed`, and `policy_declared` proof
  classes each present or `not_applicable`, with `runtime_observed`
  present. Requires at least one archetype ref, one workflow-bundle
  ref, and one evidence ref. Reserved for rows with the full launch-
  wedge proof set. No current row declares certified; the repository is
  pre-implementation.
- **supported** — Claim-bearing. Requires vendor-asserted and policy-
  declared posture plus runtime-observed OR customer-provided evidence
  present or partial so the row is not merely vendor-asserted.
- **limited** — Disclosure posture: evidence covers a smaller scope
  than a full support claim would imply. MUST carry at least one active
  downgrade reason.
- **community** — Community-reported or customer-reported. Requires
  vendor-asserted posture plus customer-provided or runtime-observed
  evidence present or partial. Aureline does not maintain runtime-
  observed coverage on community rows.
- **experimental** — Preview-grade. Default class for capabilities that
  have not earned claim-bearing wording yet.
- **retest_pending** — The row's prior evidence has been invalidated
  and is being refreshed. Short-lived.
- **exception_recorded** — The row is under a recorded exception
  (waiver, known-limit, or policy block) and MUST render the exception
  posture until it closes.
- **evidence_stale** — Evidence exists but its freshness window
  expired. Surfaces cannot render current-proof wording.
- **not_claimed** — Explicit non-claim. MUST carry at least one
  exclusion ref so the absence of claim is visible, not silent.
- **replacement_grade_candidate** — Ladder-crossing class reserved for
  successor rows that inherit the full certified-grade proof set AND
  name the superseded `assurance_claim:` id via
  `replacement_claim_row_ref`. Structurally impossible without the
  full proof set.

Class ordering (strongest to weakest; widening is forbidden, narrowing
is always admissible):

```
certified > supported > community > limited > experimental >
  retest_pending > exception_recorded > evidence_stale > not_claimed
```

`replacement_grade_candidate` is not comparable on this ladder: it
carries the full proof set of the row it supersedes and is not a
softer class.

## Proof-class breakdown

Every row carries a closed proof-class breakdown. The five proof
classes (mirrored in the schema's `proof_class` enum) are:

- **vendor_asserted** — Aureline-authored evidence: schemas,
  contracts, ADRs, vendor-owned fixtures.
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
  verification capability. A row MUST NOT project a high-bar class on
  the strength of `not_currently_verifiable` evidence alone.

Proof-presence values (mirrored in the schema's `proof_presence` enum)
are `present`, `partial`, `absent`, `not_applicable`. `partial` and
`absent` MUST raise a corresponding active downgrade reason when the
declared class requires that proof family.

## Scope envelope

Every row declares a scope envelope covering:

- **release_channels** (`dev_local`, `nightly`, `preview`, `beta`,
  `stable`, `lts`, `hotfix`) — aligned with
  [`/schemas/release/waiver_packet.schema.json`](../../schemas/release/waiver_packet.schema.json).
- **deployment_profiles** (`individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`) — aligned with
  [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml).
- **client_scopes** (`desktop_product`, `cli`, `managed_admin_surface`,
  `headless_agent`, `docs_site`, `support_center`) — drawn from the
  claim-manifest seed.

A row may not render its claim class on a channel or profile outside
this envelope. Surfaces that attempt to project a row on a wider scope
MUST render `not_claimed` wording.

## Exclusion and known-limit model

Every row MUST carry at least one entry in `exclusion_refs`. These
entries are stable non-claim refs (for example
`non_claim:voice.features_not_shipped`) that document what the row
does not assert. Rows in `not_claimed` or `limited` classes MUST
disclose the specific capability excluded; rows in claim-bearing
classes MUST disclose the specific caveat or narrowing condition that
keeps the claim honest.

Known-limit refs point to migration guidance, boundary strawman, or
waiver-packet refs that describe what the row's evidence does not
cover. `exception_recorded` rows MUST carry at least one
`known_limit_ref` naming the exception.

## Docs-version-match floor

Every row declares the minimum docs-version match state and minimum
freshness class required to project through `docs_site`, `help_about`,
and `service_health`:

- `minimum_match_state` ∈ {`exact_match`, `same_schema_epoch`,
  `warm_cached_allowed`, `not_applicable`}
- `minimum_freshness_class` ∈ {`authoritative_live`, `warm_cached`,
  `stale_allowed`, `not_applicable`}
- `behavior_when_unmet` ∈ {`downgrade_to_limited`,
  `downgrade_to_evidence_stale`, `downgrade_to_retest_pending`,
  `downgrade_to_not_claimed`}

If the floor is unmet, the docs-serving surface MUST render the
downgrade-behavior target; it MUST NOT render the declared class.

## Publication destinations

Every row MUST project through at least one publication destination
drawn from the shared claim-manifest publication-channel vocabulary:
`docs_site`, `migration_notes`, `help_about`, `service_health`,
`support_export`, `release_packet`, `release_notes`, `cli_help`,
`evaluation_artifact`, `marketplace_discovery`, `public_proof_packet`.
Surfaces that do not publish an `assurance_claim:` row MUST NOT mint
launch wording for that row's subject family.

## Automatic narrowing rules

Claims narrow automatically when any of the following triggers fires.
Triggers are closed (mirrored in the schema's `downgrade_trigger`
enum). The matrix's `downgrade_rules` table fixes the default target
class and mandatory publication destinations for each trigger; row-
level `downgrade_triggers` may narrow further but may not widen.

| Trigger                                   | Default target class      | Mandatory destinations                                             |
|-------------------------------------------|---------------------------|--------------------------------------------------------------------|
| `required_evidence_missing`               | `limited`                 | `release_packet`, `support_export`, `help_about`                   |
| `required_evidence_stale`                 | `evidence_stale`          | `public_proof_packet`, `release_notes`, `support_export`           |
| `required_evidence_narrower_than_claim`   | `limited`                 | `docs_site`, `release_packet`, `support_export`                    |
| `docs_version_match_unmet`                | `limited`                 | `docs_site`, `help_about`, `service_health`                        |
| `docs_freshness_floor_unmet`              | `evidence_stale`          | `docs_site`, `help_about`, `service_health`                        |
| `unresolved_migration_gap`                | `limited`                 | `migration_notes`, `release_packet`, `release_notes`               |
| `caveat_uncovered`                        | `limited`                 | `docs_site`, `release_notes`, `support_export`                     |
| `proof_class_not_currently_verifiable`    | `experimental`            | `docs_site`, `release_packet`, `public_proof_packet`               |
| `exception_window_expired`                | `not_claimed`             | `release_notes`, `migration_notes`, `support_export`               |
| `compatibility_row_degraded`              | `limited`                 | `migration_notes`, `release_packet`, `evaluation_artifact`         |
| `support_window_expired`                  | `not_claimed`             | `docs_site`, `migration_notes`, `cli_help`, `release_notes`        |
| `policy_blocked`                          | `exception_recorded`      | `help_about`, `service_health`, `support_export`                   |

Composition follows the worst-supporting-truth-wins rule frozen in
[`/artifacts/governance/claim_propagation_rules.yaml`](../../artifacts/governance/claim_propagation_rules.yaml):
when several triggers are active on the same row, the narrowest
effective class wins, and every mandatory destination from every
active rule MUST render the narrowed posture.

## Structural invariants

The schema at
[`/schemas/release/assurance_claim.schema.json`](../../schemas/release/assurance_claim.schema.json)
enforces these invariants with `additionalProperties: false` and
`allOf` gates:

1. `certified` or `replacement_grade_candidate` rows MUST have:
   - `archetype_refs` non-empty,
   - `workflow_bundle_refs` non-empty,
   - `required_evidence_refs` non-empty,
   - `proof_classes` containing `vendor_asserted`, `customer_provided`,
     `runtime_observed`, and `policy_declared` entries, each with
     `proof_presence` ∈ {`present`, `not_applicable`} (except
     `runtime_observed` which MUST be `present`).
2. `replacement_grade_candidate` rows MUST name the superseded row
   via `replacement_claim_row_ref`.
3. `supported` or `community` rows MUST have at least one of
   `runtime_observed` or `customer_provided` with `proof_presence` ∈
   {`present`, `partial`}.
4. `limited`, `retest_pending`, or `evidence_stale` rows MUST have
   `active_downgrade_reasons` non-empty.
5. `exception_recorded` rows MUST have `active_downgrade_reasons`
   non-empty and `known_limit_refs` non-empty.
6. `not_claimed` rows MUST have `exclusion_refs` non-empty.

Together these gates make replacement-grade or certified wording
structurally impossible to ship without the required proof families.

## Claim-subject family register

The matrix covers the subject families the PRD requires launch wording
to speak to. Every family listed below MUST have at least one row in
[`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml)
and at least one explicit non-claim or exclusion note when its
supporting evidence is absent or only partial:

1. **Provider-aware language intelligence** — editor-side semantic
   behavior, provider-bound completions, and multi-provider fallback
   posture.
2. **Trustworthy diagnostics and quick fixes** — diagnostics, lint
   routing, and quick-fix or code-action suggestions.
3. **Replay-safe execution history** — task, run, and debug execution
   history captured in a form safe to replay for audit or review.
4. **Provider-integrated review** — review threading that binds an
   Aureline review surface to an upstream provider.
5. **Localization readiness** — translated strings, locale-aware
   formatting, RTL layouts, IME behavior.
6. **Export and offboarding support** — support bundles, redaction
   posture, and data-portability workflows.
7. **Theme-package portability** — cross-display-technology fidelity
   and theme-package schema-revision behavior.
8. **Voice privacy** — voice input or voice-driven feature privacy
   posture (on-device processing, audio retention, transcripts).
9. **Repair and rollback safety** — undoing an Aureline-initiated
   change cleanly.
10. **Regulated-environment assurance** — sovereign posture, customer-
    managed keys, no vendor-hosted AI, and regulated-deploy evidence
    posture.

The matrix also covers adjacent families every launch surface leans
on: boundary-and-scope truth, compatibility and skew truth, benchmark
publication truth, docs and help freshness truth, supply-chain and
provenance truth, and accessibility readiness.

## Current posture

At this milestone no row declares or projects as `certified` or
`replacement_grade_candidate`; the repository is pre-implementation and
no runtime-observed evidence exists yet. Every row therefore sits at
`experimental`, `limited`, or `not_claimed`, and every row carries
explicit exclusions so downstream surfaces cannot silently widen the
claim. Running rows will cross ladder boundaries only when:

- `runtime_observed` proof becomes present via a certified-archetype
  report or a benchmark-pack execution, and
- the row's declared class is not forbidden at the current milestone by
  `class_rules.declared_use_allowed_at_milestone`, and
- the schema's structural gates are satisfied.

## Out of scope at this revision

- Final stable launch wording for every marketed row. The matrix
  freezes the **shape** of launch language; concrete marketing copy
  remains owned by release and docs reviewers, anchored to the canonical
  copy variants on each row.
- Pricing, commercial availability, or certification partner naming.
- The waiver-issuance workflow, which remains governed by
  [`/schemas/release/waiver_packet.schema.json`](../../schemas/release/waiver_packet.schema.json).
- Any OS-level, NTP-level, or external infrastructure posture outside
  Aureline's declared deployment profiles.
