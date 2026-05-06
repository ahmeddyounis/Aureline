# Repair-class matrix: preview-before-repair, checkpoint honesty, and support-safe copy

This document freezes the **repair classes** Aureline uses when a surface proposes a “repair” action (Doctor, Support Center, CLI/headless, or recovery flows). The goal is to make repair actions honest about **what changes**, **what is preserved**, and **what reversal is actually available** so “fix” never overstates safety or reversibility.

Each class is a contract the UI copy, preview artifacts, and support exports MUST be able to describe **without improvising new semantics**.

This contract defines, per class:

- checkpoint requirement (when a checkpoint is required vs forbidden);
- reversal expectation (exact / compensating / regenerate-only / manual / audit-only);
- preview minimum (what the preview MUST show before any mutation);
- user-review requirement (what requires explicit confirmation or admin consent);
- side-effect scope (what may mutate and what MUST NOT); and
- marketing/support caveats (what surfaces must not claim).

Out of scope: implementing repair automation or UI. This document publishes semantics only.

## Companion artifacts

- [`/artifacts/recovery/repair_classes.yaml`](../../artifacts/recovery/repair_classes.yaml)
  — machine-readable repair-class matrix, preview minima, checkpoint rules, and caveats.
- [`/fixtures/recovery/repair_class_examples/`](../../fixtures/recovery/repair_class_examples/)
  — worked examples covering cache invalidation, extension quarantine, route revocation, policy/session refresh, rollback, and export/escalate.

## Normative sources projected here

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §24.2.3 and Appendix DM (repair transaction preview and repair-class matrix).
- `.t2/docs/Aureline_PRD.md` supportability, recovery ladder, and export consent rules.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md) and
  [`/schemas/support/repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json)
  (typed repair transactions, preview artifact contract, reversal vocabulary, forbidden-action assertions).
- [`/docs/reliability/recovery_scenario_contract.md`](../reliability/recovery_scenario_contract.md) and
  [`/artifacts/recovery/safe_first_action_matrix.yaml`](../../artifacts/recovery/safe_first_action_matrix.yaml)
  (scenario-coded safe-first-action rules and repair-class vocabulary reuse).
- [`/docs/runtime/browser_runtime_contract.md`](../runtime/browser_runtime_contract.md) and
  [`/schemas/runtime/preview_route.schema.json`](../../schemas/runtime/preview_route.schema.json)
  (route expiry and revocation semantics for route-related repairs).

If this document disagrees with a normative source above, the normative source wins and this document plus its companion artifacts MUST be updated in the same change.

## Terms used below

- **Repair preview**: a human+machine explanation produced *before* any mutation, sufficient for a user (or support) to judge blast radius and reversibility.
- **Checkpoint**: a captured pre-apply state reference used to enable exact undo or rollback-on-failure semantics for classes that claim them.
- **Adjacent durable state**: persisted state outside the declared impacted scope (for example: durable settings/profiles/history, trust store, credential store, managed policy, user-authored files).

## Repair-class matrix (semantic contract)

The class names below are the canonical human labels; tokens and detailed per-field rules are defined in the companion artifact.

| Repair class | What it does (honest job) | Checkpoint expectation | Reversal expectation | Preview minimum (before any write) |
|---|---|---|---|---|
| **dispose/rebuild** | Discard and rebuild *declared disposable* derived artifacts (caches, indexes, watcher backlogs, mirror snapshots). | Required when shared with evidence-bearing state or when durable adjacency exists; otherwise optional. | **regenerate** (never “restore authored state”). | Affected storage/state classes, deletion vs rebuild plan, preserved state assertions, and rebuild source summary. |
| **disable/quarantine** | Disable or quarantine a suspect extension/host without silently re-enabling it. | Not required unless coupled to other durable changes. | **exact** (re-enable requires review) or **compensating** (if a compensating safeguard is applied). | Exactly which extension(s)/host(s) are affected, what capabilities are disabled, and the explicit re-enable path. |
| **rollback/reinstall** | Revert an extension/build/agent/runtime artifact to a prior verified version or reinstall a known-good artifact. | Required when user-visible version/channel/runtime identity changes. | **exact** / **compensating** / **manual** (must be truthful). | Before/after artifact identity (exact-build refs where applicable), eligibility checks, and what will be replaced. |
| **re-resolve context** | Re-evaluate execution context mapping (toolchain/LSP/runtime handles) and invalidate stale context caches without rewriting user source. | Required when durable launch profile selections or stored context bindings change. | **compensating** or **manual** (depending on what the user must do). | Before/after execution-context summary, affected targets, and what bindings will be invalidated. |
| **revoke/expire route** | Revoke or expire a shared/forwarded route/session handle to restore safety boundaries. | Not required for route metadata only; required if session/layout/durable bindings also change. | **exact** (route invalidation is explicit and attributable). | Route/session handle summary, who/what is being revoked, and the user-visible consequence (link invalidation / reconnect required). |
| **refresh policy/session** | Refresh signed policy, entitlements, trust posture, or session snapshots without widening trust beyond the request. | Required when durable admin caches or session state changes. | **exact** or **manual** (reauth is allowed but never hidden). | Current policy/session posture, what will be refreshed, whether reauth/admin consent is required, and what will not change. |
| **export/escalate** | Produce an audit artifact (support bundle, trace, reproducibility capsule, escalation packet) instead of applying hidden mutations. | Forbidden (no mutation beyond artifact creation). | **audit-only**. | Included artifact classes, redaction/consent choices, and where the export will be written. |

## Cross-class rules (preview-before-repair contract)

### Rule: preview exists before mutation

Any repair that mutates state MUST own a preview artifact that exists before apply runs. Preview-only outcomes are valid: a repair may refuse to apply and route to export/escalation after preview determines the action would cross a forbidden boundary.

### Rule: previews must name both changed and unchanged state

Every preview MUST clearly name:

- the impacted state classes (what may change);
- the preserved state classes (what MUST NOT change); and
- any capability narrowing applied while the repair is in flight.

Previews that only say “this will fix things” are non-conforming.

### Rule: checkpoint honesty

If a repair claims an **exact** reversal path, it MUST be backed by a checkpoint or an equivalent verified exact-undo mechanism. A repair that does not capture a checkpoint MUST NOT imply exact undo.

### Rule: no hidden adjacent durable-state mutation

No repair path may silently mutate adjacent durable state outside the declared class and scope. If a broader mutation is required, the repair MUST:

1. stop and present a new class with the broader scope, or
2. refuse local repair and route to export/escalation.

### Rule: copy must not over-claim

Two class-specific caveats are mandatory across all surfaces:

- **Dispose/rebuild never equals restore.** Cache/index rebuilds MUST NOT be marketed as restoring user-authored state. They rebuild derived artifacts from authoritative sources and may recover *derived correctness*, not lost authored bytes.
- **Export/escalate is audit-only.** Export/escalation actions are audit artifacts, not hidden mutations. Any mutation beyond artifact creation is non-conforming.

## Worked examples

See [`/fixtures/recovery/repair_class_examples/`](../../fixtures/recovery/repair_class_examples/) for concrete examples covering:

- cache invalidation → dispose/rebuild;
- extension quarantine → disable/quarantine;
- unsafe share link / preview route → revoke/expire route;
- expired trust/session snapshot → refresh policy/session;
- extension regression → rollback/reinstall; and
- reproducibility/support export → export/escalate.

