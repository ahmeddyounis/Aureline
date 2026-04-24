# Deployment-profile truth

This document is the narrative companion for Aureline's launch-bearing
deployment-profile descriptor register, the residual-dependency ledger
it binds to, and the claim-refresh rules that force downstream
surfaces to narrow or widen in lockstep with profile truth. It exists
so no surface — compatibility report, claim manifest, help/about,
support export, benchmark packet, or docs page — can imply a hidden
default deployment profile or silently widen the scope of a hosted
dependency.

Machine-readable companions:

- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — deployment-profile descriptor register. One row per launch-bearing
  profile (desktop-local / local-first, self-hosted / sovereign,
  hybrid remote-attach, air-gapped / mirror-only, managed-cloud with
  browser companion / handoff as the default home surface). Every row
  pins tenancy / residency, skew posture, companion-surface posture,
  remote-attach posture, fail-local continuity notes, a public-claim
  allowlist, a prohibited-implied-claims list, and change-trigger
  rule refs.
- [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml)
  — residual-dependency ledger. One row per dependency class (sign-in,
  package registry, remote mirror, remote agent, symbol service, AI
  provider, policy bundle, docs pack, browser handoff, companion
  notifications, hosted control-plane reachability). Every row declares
  a per-profile posture (required, optional, cached, mirrored,
  forbidden, or not-applicable-structural), a per-profile
  absence-impact class, and a per-profile continuity-fallback class.
- [`/fixtures/governance/deployment_profiles/`](../../fixtures/governance/deployment_profiles/)
  — baseline descriptor instances for each profile plus one worked
  narrowing case (`hybrid_ai_broker_outage_narrows_profile.yaml`) that
  shows a hosted dependency failure narrowing the profile truth
  without turning the desktop into a hidden thin client.

Related upstream contracts:

- [`/artifacts/deployment/locality_matrix.yaml`](../../artifacts/deployment/locality_matrix.yaml)
  and
  [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  — locality / tenancy / key-mode / control-plane vs data-plane
  matrix. The deployment-profile register consumes the frozen
  `deployment_profile_vocabulary` from this matrix verbatim.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  — qualification rows whose `claimed_deployment_profiles` lists MUST
  cite the same profile ids this register publishes.
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — public-truth claim-manifest seed whose `deployment_context` field
  MUST cite the same profile ids.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — boundary-manifest strawman that reserves the deployment-profile
  and residual-dependency slots this register fills.
- [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  — shared continuity and impairment drill catalog the profile rows
  cite by drill id.

Normative sources:

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53, §5.57, and Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  §9.8, and §26.5.
- `.t2/docs/Aureline_Technical_Design_Document.md` §9.9, §11.4.2, and
  §11.4.3.

If this document disagrees with those sources, those sources win and
this document plus the two YAML artifacts update in the same change.
If this document and the YAML artifacts disagree, the YAML artifacts
win and this document updates in the same change.

## Purpose

Launch-bearing deployment profiles are claim-bearing. A surface that
says "works offline" or "enterprise ready" or "no hosted dependency"
without naming which deployment profile it is narrowing to has made
a public claim whose scope cannot be reviewed. This register exists
so that:

- Every compatibility row, claim row, and support-export note resolves
  against one frozen deployment-profile id instead of re-inventing
  profile-local aliases.
- Every residual dependency the product carries (sign-in, registry,
  mirror, remote agent, symbol service, AI provider, policy bundle,
  docs pack, browser handoff, companion notifications, hosted control-
  plane reachability) has a machine-readable per-profile posture. No
  profile can silently require a dependency the ledger forbids, and
  no profile can silently forbid a dependency the ledger permits.
- Every profile row carries both a fail-local continuity statement
  (what remains usable when optional services degrade) and a
  prohibited-implied-claims list (what surfaces MUST NOT imply) so the
  honesty contract is inspectable.
- Every claim-bearing change propagates. When a profile's residual
  dependencies, tenancy / residency, skew window, or companion-surface
  posture changes, the change-trigger rules force claim-manifest,
  compatibility-report, docs/help/service-health, and evidence-packet
  updates in the same change.

## Frozen profile set

The register uses the frozen deployment-profile vocabulary from the
locality matrix. There are five profiles, all with stable ids; each
carries a product-facing label that names the launch-bearing flavor:

| Profile id | Product-facing label | Summary |
|---|---|---|
| `individual_local` | desktop-local / local-first | Baseline single-user, single-device profile. No account required, no hosted control plane claimed. Every `local_core` capability MUST remain available_local_safe without degradation. |
| `self_hosted` | self-hosted / sovereign | Customer-operated control plane, mirror, relay, or registry. Customer-managed keys, customer-pinned region, customer retention window. Local-core capabilities stay available_local_safe when the customer control plane is stale or unreachable. |
| `enterprise_online` | hybrid remote-attach | Hybrid / enterprise-online profile with customer-federated identity and vendor-managed or customer-federated services. Supports cross-device browser companion / handoff. Local-core capabilities remain available_local_safe when hybrid services degrade. |
| `air_gapped` | air-gapped / mirror-only | No public internet egress. Registry, docs-pack, policy, auth/identity, and catalog resolve against a signed mirror or offline bundle only. Remote agent, AI provider, browser handoff, and companion notifications are forbidden in this profile. |
| `managed_cloud` | browser companion / handoff default home | Vendor-operated SaaS control plane with browser companion / handoff as a first-class surface home. Vendor-managed keys by default, customer-pinned region, vendor retention window. Local-core capabilities remain available_local_safe when the managed plane degrades; companion surfaces narrow to cached read-only. |

Browser companion / handoff is **not** a sixth deployment profile.
The frozen deployment-profile vocabulary has five values. Companion /
handoff is modeled as a cross-cutting surface posture every profile
carries via `companion_surface_posture_class`. The managed-cloud
profile is the default home for companion-first flows; other profiles
declare their own in-window companion posture (or explicitly disallow
it, in the air-gapped case).

## Residual-dependency ledger contract

The ledger pins every residual-dependency class against every frozen
profile. The posture vocabulary is closed:

- `required` — the dependency MUST be reachable (or cached within the
  freshness floor) for the profile's baseline claims to hold. When
  absent, the absence-impact field names the degraded posture and the
  continuity-fallback field names the restore class.
- `optional` — the dependency widens the profile's claim scope when
  reachable and narrows the claim scope when absent. Local-core
  capabilities MUST remain available_local_safe regardless.
- `cached` — the dependency's last-known-good local state is allowed
  to serve reads under a stale-cache label. The label is not optional;
  cached reads MUST NOT be presented as current.
- `mirrored` — the dependency MUST resolve against a signed mirror or
  offline bundle in the named profile. Live fetch is not in window.
- `forbidden` — the dependency MUST NOT be used in the named profile.
  Surfaces that attempt to use a forbidden dependency MUST fail closed
  and cite the ledger row.
- `not_applicable_structural` — the dependency is structurally not
  applicable to the profile (for example, sign-in does not apply to
  individual-local). The posture is declared so reviewers can see the
  row was considered; silent absence is forbidden.

Every ledger row MUST declare a posture for every profile. No profile
can be skipped. If a dependency class is added or removed, the schema
version on both YAML artifacts bumps in the same change.

## Fail-local continuity rule

Every profile row carries a `fail_local_continuity_notes` field that
names what remains usable when optional services degrade. The rule is
simple and mechanical:

- Local editing, save, search, local git, tasks, export, and
  diagnostics are `local_core` capabilities on every profile. When a
  residual dependency degrades, these capabilities MUST remain at
  `available_local_safe` (or `available_mirror_backed` on the
  air-gapped profile, where docs inspection is mirror-backed by
  design).
- Hosted surfaces (AI assistance, companion notifications, browser
  handoff review, remote agent pair-programming, hosted search, hosted
  symbol navigation) narrow to a labeled cached-last-known-good or
  queued-for-reconnect posture. They MUST NOT silently present stale
  truth as current.
- "Service down" without a dependency-class reference and a named
  absence-impact class is not a valid surface state. Surfaces consume
  the ledger and the locality matrix; they do not author their own
  outage taxonomy.

The worked narrowing fixture
[`fixtures/governance/deployment_profiles/hybrid_ai_broker_outage_narrows_profile.yaml`](../../fixtures/governance/deployment_profiles/hybrid_ai_broker_outage_narrows_profile.yaml)
is the canonical example: a hybrid remote-attach deployment loses the
managed AI provider and the companion notification channel at the
same time; the profile narrows its AI and companion claims to cached
last-known-good posture while every local-core data-plane capability
remains `available_local_safe`. The desktop does not become a hidden
thin client.

## Public-claim allowlist and prohibited-implied-claims

Every profile row carries two lists whose contents are part of the
row's public-truth contract:

- `public_claim_allowlist` — wording the profile is allowed to bear
  publicly. Each entry is a claim the register stands behind and that
  downstream channels MAY quote verbatim.
- `prohibited_implied_claims` — wording the profile MUST NOT imply.
  Each entry names a concrete claim a downstream surface could be
  tempted to write and states why the row does not support it. The
  list is not exhaustive; it captures the honesty caveats that
  reviewers have flagged as likely to drift.

Both lists are non-empty on every seeded row. A row that left one
empty would be a silent widening of public truth.

## Change-trigger rules

The register declares five change-trigger rules:

- `rule:residual_dependency_posture_changed` — a dependency's
  per-profile posture in the ledger moves between required, optional,
  cached, mirrored, or forbidden.
- `rule:tenancy_or_residency_changed` — the profile's tenant scope,
  region scope, retention class, or key-mode class changes.
- `rule:skew_window_or_out_of_window_posture_changed` — the profile's
  skew-window class or out-of-window posture changes.
- `rule:companion_surface_posture_changed` — the profile's
  `companion_surface_posture_class` changes.
- `rule:hybrid_remote_attach_posture_changed` — the profile's
  `hybrid_remote_attach_posture_class` changes.

Each rule names the downstream surfaces that MUST be refreshed in the
same change: the claim manifest, the compatibility-report template,
the docs / help / service-health truth source, the evidence-packet
header, the qualification-matrix seed, the version-skew register, the
residual-dependency ledger, and the support-bundle contract, as
applicable. A posture change that lands without the paired refresh is
non-conforming; surfaces would continue implying the old scope.

## Stable id reuse across registers

The three registers that most often mis-align on deployment-profile
truth are the qualification matrix, the claim manifest, and the
locality matrix. This register closes that drift by contract:

- `artifacts/compat/qualification_matrix_seed.yaml` already names
  `deployment_profile_artifact` as a downstream consumer; its
  qualification rows cite `claimed_deployment_profiles` values from
  the same frozen vocabulary this register uses.
- `artifacts/governance/claim_manifest_seed.yaml` names the same five
  profile ids in `environment.deployment_context`; claim rows MUST
  NOT invent profile-local aliases.
- `artifacts/deployment/locality_matrix.yaml` publishes the frozen
  vocabulary this register reuses verbatim.

Because the three registers share one vocabulary, a compatibility
report, claim manifest, or deployment-profile descriptor that cites
`individual_local`, `self_hosted`, `enterprise_online`, `air_gapped`,
or `managed_cloud` is automatically talking about the same row
family. Free-text "desktop", "hybrid", "cloud", "offline", or
"sovereign" wording that is not backed by one of those ids is out of
contract.

## Current seed scope

Frozen at this revision:

- Five deployment-profile rows, each with stable id, owner, lane,
  review cadence, visibility, tenancy / residency, skew window,
  hybrid-remote-attach posture, companion-surface posture, fail-local
  continuity notes, public-claim allowlist, prohibited-implied-claims
  list, and change-trigger rule refs.
- An eleven-row residual-dependency ledger covering sign-in, package
  registry, remote mirror, remote agent, symbol service, AI provider,
  policy bundle, docs pack, browser handoff, companion notifications,
  and hosted control-plane reachability — each with a per-profile
  posture, absence-impact class, and continuity-fallback class.
- Five change-trigger rules naming every downstream surface that must
  be refreshed when a profile's residual dependencies, tenancy /
  residency, skew posture, companion-surface posture, or remote-attach
  posture changes.
- Six fixtures: five per-profile baseline descriptor instances and one
  worked narrowing case where the managed AI provider and the
  companion notification channel degrade together on the hybrid
  remote-attach profile without turning the desktop into a hidden
  thin client.

Deferred to a later milestone:

- Full M3 / M4 certification packets and enterprise-rollout
  playbooks (explicitly out of scope for this register).
- A JSON-schema boundary for the descriptor instance record
  (the register is self-contained at this revision; a schema under
  `/schemas/governance/` may land later once downstream validators
  need one). When that schema lands, it joins the `governance`
  family in `artifacts/governance/schema_families.yaml`.
