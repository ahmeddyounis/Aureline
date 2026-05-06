# Managed-service capacity assumptions, cost guardrails, and residual-dependency disclosure packet

This document publishes the shared planning + disclosure packet for
optional managed-service families so reviewers can reason about capacity,
cost, and failure posture **without** accidentally treating convenience
services as prerequisites for the desktop-local core.

This packet is **contract-first and vocabulary-only**. It does not:

- operate a managed service;
- set final business pricing;
- promise customer-facing SLAs beyond what the SLO seed freezes;
- replace the operating-mode, metering, or locality continuity contracts.

The intent is that release, ops, docs, and support work can cite one
shared set of rows instead of inventing ad hoc “service limits” or
“cloud required” language per surface.

## Companion artifacts

- [`/artifacts/service/capacity_assumption_rows.yaml`](../../artifacts/service/capacity_assumption_rows.yaml)
  — planning rows per managed-service family: demand assumptions, burst
  posture, required downgrade behavior, cost guardrails, and the local
  continuity floor each family must preserve.
- [`/artifacts/service/residual_dependency_cost_notes.yaml`](../../artifacts/service/residual_dependency_cost_notes.yaml)
  — disclosure rows per user/admin-visible surface: which service family
  and dependency classes the surface relies on, how quota/cost is
  surfaced, what degrades under impairment, and what local fallback or
  export paths remain available.
- [`/fixtures/service/capacity_guardrail_cases/`](../../fixtures/service/capacity_guardrail_cases/)
  — worked guardrail cases that exercise the “downgrade vs block” rules
  under capacity or cost pressure.

## Inherited contracts

This packet stands on top of earlier contracts and MUST NOT recast any
of them:

- [`/docs/service/managed_service_seed.md`](./managed_service_seed.md) and
  [`/artifacts/service/slo_rows.yaml`](../../artifacts/service/slo_rows.yaml)
  — the local-core non-dependence clause and the per-service degradation
  vocabulary remain the authority on what “optional” means.
- [`/docs/service/operating_mode_and_capacity_contract.md`](./operating_mode_and_capacity_contract.md)
  — the user/admin-visible operating-mode and capacity-row vocabulary is
  frozen there. This packet supplies *planning assumptions* and
  *disclosure bindings* that those surfaces cite.
- [`/docs/service/metering_and_chargeback_contract.md`](./metering_and_chargeback_contract.md)
  — quota owner, chargeback honesty, export parity, and fail-open /
  fail-closed rules for metering are frozen there. This packet reuses
  those terms; it does not invent new billing language.
- [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml)
  — the per-deployment-profile residual-dependency posture is frozen
  there. The disclosure rows in this packet bind *specific product
  surfaces* to those dependency classes.

## Normative sources

- `.t2/docs/Aureline_PRD.md` §5.56 (managed-service capacity, cost, and
  tenancy model).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix W
  (managed-service capacity and cost model).
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.1.1–§11.1.2
  (metering separation; capacity/tenancy/cost guardrails by plane).

If this document disagrees with those sources, those sources win and
this document plus the YAML rows update in the same change. If the
narrative and YAML rows disagree, this document wins and the YAML updates
in the same change.

## Scope

Frozen at this revision:

- one **capacity-assumption row shape** per managed-service family,
  capturing:
  - the family’s scaling axis and default tenancy posture;
  - baseline demand assumptions and burst posture;
  - required downgrade behavior under overload / outage;
  - cost guardrails that keep optional services economically bounded;
  - the “must-not-break local workflow” clause for the family.
- one **residual-dependency cost note row shape** per user/admin-visible
  managed surface, capturing:
  - which residual-dependency classes (sign-in, policy bundle, registry,
    etc) the surface relies on;
  - which service ids and API surfaces the surface calls (when defined);
  - which quota/meter surfaces disclose spend and ownership;
  - what degrades and what local fallback / export remains.
- one **guardrail case shape** used by fixtures to demonstrate:
  - when the correct behavior is “downgrade to local-only/local-safe”;
  - when the correct behavior is “block the managed-only action”;
  - what disclosure text or export options must remain reachable.

Out of scope at this revision:

- implementing quotas, throttles, ledgers, or billing backends;
- final pricing, plan packaging, or SKU definitions;
- operating a vendor-managed service or publishing production runbooks.

## Guardrail rules (review blockers)

These rules exist to prevent “optional” services from silently becoming
budget or availability prerequisites.

### 1) Local-core continuity is the floor

Any overload, quota exhaustion, outage, or missing entitlement MUST NOT
block:

- local editing, save, undo, search, and navigation;
- local Git workflows;
- local export and offline inspection of existing local state.

If a managed surface cannot preserve this floor, it is not admissible as
an optional managed surface.

### 2) Downgrade beats broad failure

When a managed-family guardrail is breached (capacity or cost), the
default response is:

- **downgrade the managed-only path** (pause, queue, or reject the
  bounded managed action), and
- **keep local-safe workflows available**, with explicit disclosure of
  what is paused and how to recover.

Whole-product “service unavailable” copy is forbidden on these paths.

### 3) Blocking is narrow and explicit

Blocking is only admissible when *both* are true:

- the action is **managed-only** (no documented local-safe fallback
  exists for the claimed deployment profile), and
- the action’s cost or safety cannot be bounded without a hard stop
  (for example: spend would be unbounded, or a policy/hold decision is
  ambiguous and the action is privileged/destructive).

When blocking is admissible, the surface MUST:

- cite the relevant dependency class and/or service family row;
- provide an export path where the governing contract requires it;
- provide a recovery cue that is not “contact support” when a mechanical
  recovery path exists.

### 4) Publishing or changing a guardrail requires evidence

Any change that materially alters a family’s demand assumptions, burst
posture, downgrade behavior, or cost guardrail MUST ship as a cohesive
packet:

- update `artifacts/service/capacity_assumption_rows.yaml`;
- update `artifacts/service/residual_dependency_cost_notes.yaml` for any
  affected surfaces;
- add or refresh at least one fixture in
  `fixtures/service/capacity_guardrail_cases/` that demonstrates the
  intended downgrade/block behavior.

If the change widens a managed dependency into a prerequisite or removes
an existing local-safe fallback, it follows the repo’s governance
workflow for decision/waiver artifacts (see `docs/governance/decision_workflow.md`)
in addition to the packet above.

